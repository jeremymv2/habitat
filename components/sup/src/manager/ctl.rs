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

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::PathBuf;
use std::str::FromStr;

use common::command::package::install::InstallSource;
use hcore::channel;
use hcore::package::PackageIdent;
use hcore::service::{ApplicationEnvironment, ServiceGroup};
use hcore::url::default_bldr_url;
use manager::service::{BindMap, IntoServiceSpec, ServiceBind, ServiceSpec, Topology,
                       UpdateStrategy};

use ctl_gateway::codec::*;
use protocols::ctl::*;

lazy_static! {
    static ref DEFAULT_BLDR_URL: String = {
        default_bldr_url()
    };

    static ref DEFAULT_BLDR_CHANNEL: String = {
        channel::default()
    };
}

#[derive(Clone, Debug)]
pub struct SvcLoadOpts {
    pub application_environment: Option<ApplicationEnvironment>,
    pub binds: Option<Vec<ServiceBind>>,
    pub composite_binds: Option<HashMap<String, Vec<ServiceBind>>>,
    pub config_from: Option<PathBuf>,
    pub force: bool,
    pub group: Option<String>,
    pub svc_encrypted_password: Option<String>,
    pub topology: Option<Topology>,
    pub update_strategy: Option<UpdateStrategy>,
    pub source: InstallSource,
    pub ident: PackageIdent,
    bldr_url: Option<String>,
    bldr_channel: Option<String>,
}

impl SvcLoadOpts {
    pub fn new(mut msg: SrvMessage<SvcLoad>) -> Self {
        let source = InstallSource::from_str(msg.get_source()).unwrap();
        let ident = source.as_ref().clone();
        let bldr_url = if msg.has_bldr_url() {
            Some(msg.take_bldr_url())
        } else {
            None
        };
        let bldr_channel = if msg.has_bldr_channel() {
            Some(msg.take_bldr_channel())
        } else {
            None
        };
        let group = if msg.has_group() {
            Some(msg.take_group())
        } else {
            None
        };
        let app_env = if msg.has_application_environment() {
            Some(
                ApplicationEnvironment::new(
                    msg.get_application_environment().get_application(),
                    msg.get_application_environment().get_environment(),
                ).unwrap(),
            )
        } else {
            None
        };
        let binds = if msg.has_specified_binds() {
            Some(msg.take_binds().into_iter().map(Into::into).collect())
        } else {
            None
        };
        let composite_binds = if msg.has_specified_binds() {
            Some(HashMap::new())
        } else {
            None
        };
        let config_from = if msg.has_config_from() {
            Some(PathBuf::from(msg.take_config_from()))
        } else {
            None
        };
        let svc_encrypted_password = if msg.has_svc_encrypted_password() {
            Some(msg.take_svc_encrypted_password())
        } else {
            None
        };
        let topology = if msg.has_topology() {
            Some(msg.get_topology())
        } else {
            None
        };
        let update_strategy = if msg.has_update_strategy() {
            Some(msg.get_update_strategy())
        } else {
            None
        };
        SvcLoadOpts {
            application_environment: app_env,
            binds: binds,
            composite_binds: composite_binds,
            bldr_url: bldr_url,
            bldr_channel: bldr_channel,
            config_from: config_from,
            force: msg.get_force(),
            group: group,
            svc_encrypted_password: svc_encrypted_password,
            topology: topology,
            update_strategy: update_strategy,
            source: source,
            ident: ident,
        }
    }

    pub fn bldr_url(&self) -> &str {
        self.bldr_url.as_ref().unwrap_or_else(|| &DEFAULT_BLDR_URL)
    }

    pub fn bldr_channel(&self) -> &str {
        self.bldr_channel.as_ref().unwrap_or_else(
            || &DEFAULT_BLDR_CHANNEL,
        )
    }
}

impl IntoServiceSpec for SvcLoadOpts {
    fn into_spec(&self, spec: &mut ServiceSpec) {
        spec.ident = self.ident.clone();
        if let Some(ref group) = self.group {
            spec.group = group.clone();
        }
        spec.application_environment = self.application_environment.clone();
        spec.bldr_url = self.bldr_url().to_string();
        spec.channel = self.bldr_channel().to_string();
        if let Some(topology) = self.topology {
            spec.topology = topology;
        }
        if let Some(update_strategy) = self.update_strategy {
            spec.update_strategy = update_strategy;
        }
        spec.binds = self.binds.clone().unwrap_or_default();
        spec.config_from = self.config_from.clone();
        spec.svc_encrypted_password = self.svc_encrypted_password.clone();
        spec.composite = None;
    }

    /// All specs in a composite currently share a lot of the same
    /// information. Here, we create a "base spec" that we can clone and
    /// further customize for each individual service as needed.
    ///
    /// * All services will pull from the same channel in the same
    ///   Builder instance
    /// * All services will be in the same group and app/env. Binds among
    ///   the composite's services are generated based on this
    ///   assumption.
    ///   (We do not set binds here, though, because that requires
    ///   specialized, service-specific handling.)
    /// * For now, all a composite's services will also share the same
    ///   update strategy and topology, though we may want to revisit
    ///   this in the future (particularly for topology).
    fn into_composite_spec(
        &self,
        composite_name: String,
        services: Vec<PackageIdent>,
        mut bind_map: BindMap,
    ) -> Vec<ServiceSpec> {
        // All the service specs will be customized copies of this.
        let mut base_spec = ServiceSpec::default();
        self.into_spec(&mut base_spec);
        base_spec.composite = Some(composite_name);
        // TODO (CM): Not dealing with service passwords for now, since
        // that's a Windows-only feature, and we don't currently build
        // Windows composites yet. And we don't have a nice way target
        // them on a per-service basis.
        base_spec.svc_encrypted_password = None;
        // TODO (CM): Not setting the dev-mode service config_from value
        // because we don't currently have a nice way to target them on a
        // per-service basis.
        base_spec.config_from = None;

        let mut specs: Vec<ServiceSpec> = Vec::with_capacity(services.len());
        let mut composite_binds = self.composite_binds.clone().unwrap_or_default();
        for service in services {
            // Customize each service's spec as appropriate
            let mut spec = base_spec.clone();
            spec.ident = service;
            set_composite_binds(&mut spec, &mut bind_map, &mut composite_binds);
            specs.push(spec);
        }
        specs
    }

    fn update_composite(&self, bind_map: &mut BindMap, spec: &mut ServiceSpec) {
        let composite_binds = self.composite_binds.clone();
        // We only want to update fields that were set by LoadOpts
        if let Some(ref bldr_url) = self.bldr_url {
            spec.bldr_url = bldr_url.clone();
        }
        if let Some(ref bldr_channel) = self.bldr_channel {
            spec.channel = bldr_channel.clone();
        }
        if let Some(ref app_env) = self.application_environment {
            spec.application_environment = Some(app_env.clone());
        }
        if let Some(ref group) = self.group {
            spec.group = group.clone();
        }
        if let Some(strategy) = self.update_strategy {
            spec.update_strategy = strategy;
        }
        if let Some(topology) = self.topology {
            spec.topology = topology;
        }
        if let Some(mut composite_binds) = composite_binds {
            set_composite_binds(spec, bind_map, &mut composite_binds);
        }
    }
}

#[derive(Clone, Debug)]
pub struct SvcStartOpts {
    pub ident: PackageIdent,
}

impl SvcStartOpts {
    pub fn new(mut msg: SrvMessage<SvcStart>) -> Self {
        SvcStartOpts { ident: msg.take_ident().into() }
    }
}

/// Generate the binds for a composite's service, taking into account
/// both the values laid out in composite definition and any CLI value
/// the user may have specified. This allows the user to override a
/// composite-defined bind, but also (perhaps more usefully) to
/// declare binds for services within the composite that are not
/// themselves *satisfied* by other members of the composite.
///
/// The final list of bind mappings is generated and then set in the
/// `ServiceSpec`. Any binds that may have been present in the spec
/// before are completely ignored.
///
/// # Parameters
///
/// * bind_map: output of package.bind_map()
/// * cli_binds: per-service overrides given on the CLI
fn set_composite_binds(
    spec: &mut ServiceSpec,
    bind_map: &mut BindMap,
    cli_binds: &mut HashMap<String, Vec<ServiceBind>>,
) {
    // We'll be layering bind specifications from the composite
    // with any additional ones from the CLI. We'll store them here,
    // keyed to the bind name
    let mut final_binds: HashMap<String, ServiceBind> = HashMap::new();

    // First, generate the binds from the composite
    if let Some(bind_mappings) = bind_map.remove(&spec.ident) {
        // Turn each BindMapping into a ServiceBind

        // NOTE: We are explicitly NOT generating binds that include
        // "organization". This is a feature that never quite found
        // its footing, and will likely be removed / greatly
        // overhauled Real Soon Now (TM) (as of September 2017).
        //
        // As it exists right now, "organization" is a supervisor-wide
        // setting, and thus is only available for `hab sup run` and
        // `hab svc start`. We don't have a way from `hab svc load` to
        // access the organization setting of an active supervisor,
        // and so we can't generate binds that include organizations.
        for bind_mapping in bind_mappings.iter() {
            let group = ServiceGroup::new(
                spec.application_environment.as_ref(),
                &bind_mapping.satisfying_service.name,
                &spec.group,
                None, // <-- organization
            ).expect(
                "Failed to parse bind mapping into service group. Did you validate your input?",
            );
            let bind = ServiceBind {
                name: bind_mapping.bind_name.clone(),
                service_group: group,
            };
            final_binds.insert(bind.name.clone(), bind);
        }
    }

    // If anything was overridden or added on the CLI, layer that on
    // now as well. These will take precedence over anything in the
    // composite itself.
    //
    // Note that it consumes the values from cli_binds
    if let Entry::Occupied(b) = cli_binds.entry(spec.ident.name.clone()) {
        let binds = b.remove();
        for bind in binds {
            final_binds.insert(bind.name.clone(), bind);
        }
    }

    // Now take all the ServiceBinds we've collected.
    spec.binds = final_binds.drain().map(|(_, v)| v).collect();
}
