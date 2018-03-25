// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs, SocketAddr, SocketAddrV4};
use std::ops::{Deref, DerefMut};
use std::option;
use std::path::Path;
use std::result;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use hcore::service::{ApplicationEnvironment, ServiceGroup};
use iron::prelude::*;
use iron::{headers, status, typemap};
use iron::modifiers::Header;
use persistent;
use router::Router;
use serde_json::{self, Value as Json};

use error::{Result, Error, SupError};
use manager;
use manager::service::HealthCheck;
use manager::service::hooks::{self, HealthCheckHook};

static LOGKEY: &'static str = "HG";
const APIDOCS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/api.html"));

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListenAddr(SocketAddr);

impl ListenAddr {
    pub fn new(ip: IpAddr, port: u16) -> ListenAddr {
        ListenAddr(SocketAddr::new(ip, port))
    }
}

impl Default for ListenAddr {
    fn default() -> ListenAddr {
        ListenAddr(SocketAddr::V4(
            SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9631),
        ))
    }
}

impl Deref for ListenAddr {
    type Target = SocketAddr;

    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}

impl DerefMut for ListenAddr {
    fn deref_mut(&mut self) -> &mut SocketAddr {
        &mut self.0
    }
}

impl FromStr for ListenAddr {
    type Err = SupError;

    fn from_str(val: &str) -> Result<Self> {
        match SocketAddr::from_str(val) {
            Ok(addr) => Ok(ListenAddr(addr)),
            Err(_) => {
                match IpAddr::from_str(val) {
                    Ok(ip) => {
                        let mut addr = ListenAddr::default();
                        addr.set_ip(ip);
                        Ok(addr)
                    }
                    Err(_) => Err(sup_error!(Error::IPFailed)),
                }
            }
        }
    }
}

impl ToSocketAddrs for ListenAddr {
    type Iter = option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.0.to_socket_addrs()
    }
}

impl fmt::Display for ListenAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

struct ManagerFs;

impl typemap::Key for ManagerFs {
    type Value = manager::FsCfg;
}

pub struct Server(Iron<Chain>, ListenAddr);

impl Server {
    pub fn new(manager_state: Arc<manager::FsCfg>, listen_addr: ListenAddr) -> Self {
        let router =
            router!(
            doc: get "/" => doc,
            butterfly: get "/butterfly" => butterfly,
            census: get "/census" => census,
            services: get "/services" => services,
            service: get "/services/:svc/:group" => service,
            service_org: get "/services/:svc/:group/:org" => service,
            service_config: get "/services/:svc/:group/config" => config,
            service_health: get "/services/:svc/:group/health" => health,
            service_config_org: get "/services/:svc/:group/:org/config" => config,
            service_health_org: get "/services/:svc/:group/:org/health" => health,
        );
        let mut chain = Chain::new(router);
        chain.link(persistent::Read::<ManagerFs>::both(manager_state));
        Server(Iron::new(chain), listen_addr)
    }

    pub fn start(self) -> Result<JoinHandle<()>> {
        let handle = thread::Builder::new()
            .name("http-gateway".to_string())
            .spawn(move || {
                self.0.http(*self.1).expect(
                    "unable to start http-gateway thread",
                );
            })?;
        Ok(handle)
    }
}

#[derive(Default, Serialize)]
struct HealthCheckBody {
    stdout: String,
    stderr: String,
}

fn butterfly(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    match File::open(&state.butterfly_data_path) {
        Ok(file) => Ok(Response::with(
            (status::Ok, Header(headers::ContentType::json()), file),
        )),
        Err(_) => Ok(Response::with(status::ServiceUnavailable)),
    }
}

fn census(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    match File::open(&state.census_data_path) {
        Ok(file) => Ok(Response::with(
            (status::Ok, Header(headers::ContentType::json()), file),
        )),
        Err(_) => Ok(Response::with(status::ServiceUnavailable)),
    }
}

fn config(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    let service_group = match build_service_group(req) {
        Ok(sg) => sg,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    match service_from_file(&service_group, &state.services_data_path) {
        Ok(Some(service)) => {
            Ok(Response::with((
                status::Ok,
                Header(headers::ContentType::json()),
                service["cfg"].to_string(),
            )))
        }
        Ok(None) => Ok(Response::with(status::NotFound)),
        Err(_) => Ok(Response::with(status::ServiceUnavailable)),
    }
}

fn health(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    let (health_file, stdout_path, stderr_path) = match build_service_group(req) {
        Ok(sg) => {
            (
                state.health_check_cache(&sg),
                hooks::stdout_log_path::<HealthCheckHook>(&sg),
                hooks::stderr_log_path::<HealthCheckHook>(&sg),
            )
        }
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    match File::open(&health_file) {
        Ok(mut file) => {
            let mut buf = String::new();
            let mut body = HealthCheckBody::default();
            file.read_to_string(&mut buf).unwrap();
            let code = i8::from_str(buf.trim()).unwrap();
            let status: status::Status = HealthCheck::from(code).into();
            if let Ok(mut file) = File::open(&stdout_path) {
                let _ = file.read_to_string(&mut body.stdout);
            }
            if let Ok(mut file) = File::open(&stderr_path) {
                let _ = file.read_to_string(&mut body.stderr);
            }
            Ok(Response::with((
                status,
                Header(headers::ContentType::json()),
                serde_json::to_string(&body).unwrap(),
            )))
        }
        Err(_) => Ok(Response::with(status::NotFound)),
    }
}

fn service(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    let service_group = match build_service_group(req) {
        Ok(sg) => sg,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    match service_from_file(&service_group, &state.services_data_path) {
        Ok(Some(service)) => {
            Ok(Response::with((
                status::Ok,
                Header(headers::ContentType::json()),
                service.to_string(),
            )))
        }
        Ok(None) => Ok(Response::with(status::NotFound)),
        Err(_) => Ok(Response::with(status::ServiceUnavailable)),
    }
}

fn services(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    match File::open(&state.services_data_path) {
        Ok(file) => Ok(Response::with(
            (status::Ok, Header(headers::ContentType::json()), file),
        )),
        Err(_) => Ok(Response::with(status::ServiceUnavailable)),
    }
}

fn doc(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(
        (status::Ok, Header(headers::ContentType::html()), APIDOCS),
    ))
}

impl Into<Response> for HealthCheck {
    fn into(self) -> Response {
        let status: status::Status = self.into();
        Response::with(status)
    }
}

impl Into<status::Status> for HealthCheck {
    fn into(self) -> status::Status {
        match self {
            HealthCheck::Ok | HealthCheck::Warning => status::Ok,
            HealthCheck::Critical => status::ServiceUnavailable,
            HealthCheck::Unknown => status::InternalServerError,
        }
    }
}

fn build_service_group(req: &mut Request) -> Result<ServiceGroup> {
    let app_env = match req.extensions.get::<Router>().unwrap().find(
        "application_environment",
    ) {
        Some(s) => {
            match ApplicationEnvironment::from_str(s) {
                Ok(app_env) => Some(app_env),
                Err(_) => None,
            }
        }
        None => None,
    };
    let sg = ServiceGroup::new(
        app_env.as_ref(),
        req.extensions
            .get::<Router>()
            .unwrap()
            .find("svc")
            .unwrap_or(""),
        req.extensions
            .get::<Router>()
            .unwrap()
            .find("group")
            .unwrap_or(""),
        req.extensions.get::<Router>().unwrap().find("org"),
    )?;
    Ok(sg)
}

fn service_from_file<T>(
    service_group: &ServiceGroup,
    services_data_path: T,
) -> result::Result<Option<Json>, io::Error>
where
    T: AsRef<Path>,
{
    match File::open(services_data_path) {
        Ok(file) => {
            match serde_json::from_reader(file) {
                Ok(Json::Array(services)) => {
                    Ok(services.into_iter().find(|s| {
                        s["service_group"] == service_group.as_ref()
                    }))
                }
                _ => Ok(None),
            }
        }
        Err(err) => Err(err),
    }
}
