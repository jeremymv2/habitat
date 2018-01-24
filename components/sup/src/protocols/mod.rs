// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

pub mod ctl;
pub mod types;

use std::fmt;
use std::io;

use depot_client::DisplayProgress;
use hcore::package::PackageIdent;
use hcore::service::{ApplicationEnvironment, ServiceGroup};

use ctl_gateway::codec::*;
use ctl_gateway::server::CtlRequest;
use manager::service::ServiceBind;

pub struct NetProgressBar {
    inner: SrvMessage<ctl::NetProgress>,
    req: CtlRequest,
}

impl NetProgressBar {
    pub fn new(req: CtlRequest) -> Self {
        NetProgressBar {
            inner: SrvMessage::<ctl::NetProgress>::new(),
            req: req,
        }
    }
}

impl DisplayProgress for NetProgressBar {
    fn size(&mut self, size: u64) {
        self.inner.set_total(size);
    }

    fn finish(&mut self) {
        ()
    }
}

impl io::Write for NetProgressBar {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.set_delta(buf.len() as u64);
        self.req.reply_partial(self.inner.clone());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl fmt::Display for ctl::ConsoleLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_line())
    }
}

// JW TODO: These trait implementations to provide translations between protocol messages and
// concrete Rust types defined in the core crate will go away eventually. We need to put the
// core crate back into the Supervisor's repository and untangle our dependency hell before
// that can happen.

impl From<ApplicationEnvironment> for types::ApplicationEnvironment {
    fn from(app_env: ApplicationEnvironment) -> Self {
        let mut proto = types::ApplicationEnvironment::new();
        proto.set_application(app_env.application().to_string());
        proto.set_environment(app_env.environment().to_string());
        proto
    }
}

impl From<PackageIdent> for types::PackageIdent {
    fn from(ident: PackageIdent) -> Self {
        let mut proto = types::PackageIdent::new();
        proto.set_origin(ident.origin);
        proto.set_name(ident.name);
        if let Some(version) = ident.version {
            proto.set_version(version);
        }
        if let Some(release) = ident.release {
            proto.set_release(release);
        }
        proto
    }
}

impl Into<PackageIdent> for types::PackageIdent {
    fn into(mut self) -> PackageIdent {
        let version = if self.has_version() {
            Some(self.take_version())
        } else {
            None
        };
        let release = if self.has_release() {
            Some(self.take_release())
        } else {
            None
        };
        PackageIdent::new(self.take_origin(), self.take_name(), version, release)
    }
}

impl From<ServiceBind> for types::ServiceBind {
    fn from(bind: ServiceBind) -> Self {
        let mut proto = types::ServiceBind::new();
        proto.set_name(bind.name);
        proto.set_service_group(bind.service_group.into());
        proto
    }
}

impl From<ServiceGroup> for types::ServiceGroup {
    fn from(service_group: ServiceGroup) -> Self {
        let mut proto = types::ServiceGroup::new();
        if let Some(app_env) = service_group.application_environment() {
            proto.set_application_environment(app_env.into());
        }
        proto.set_group(service_group.group().to_string());
        proto.set_service(service_group.service().to_string());
        if let Some(organization) = service_group.org() {
            proto.set_organization(organization.to_string());
        }
        proto
    }
}

impl Into<ServiceBind> for types::ServiceBind {
    fn into(mut self) -> ServiceBind {
        ServiceBind {
            name: self.take_name(),
            service_group: self.take_service_group().into(),
        }
    }
}

impl Into<ServiceGroup> for types::ServiceGroup {
    fn into(mut self) -> ServiceGroup {
        let app_env = if self.has_application_environment() {
            Some(
                ApplicationEnvironment::new(
                    self.get_application_environment().get_application(),
                    self.get_application_environment().get_environment(),
                ).unwrap(),
            )
        } else {
            None
        };
        let service = self.take_service();
        let group = self.take_group();
        let organization = if self.has_organization() {
            Some(self.get_organization())
        } else {
            None
        };
        ServiceGroup::new(app_env.as_ref(), service, group, organization).unwrap()
    }
}
