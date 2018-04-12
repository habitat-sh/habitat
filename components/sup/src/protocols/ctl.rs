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

//! Specific request, responses, and types used to specifically communicate with the Supervisor's
//! Control Gateway.
//!
//! Note: See `protocols/ctl.proto` for type level documentation for generated types.

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use hcore;
use hcore::package::PackageIdent;
use hcore::service::ServiceGroup;

pub use super::generated::ctl::*;
use manager::service::{BindMap, IntoServiceSpec, ServiceBind, ServiceSpec};

impl fmt::Display for ConsoleLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_line())
    }
}

impl IntoServiceSpec for SvcLoad {
    fn into_spec(&self, spec: &mut ServiceSpec) {
        spec.ident = self.get_ident().clone().into();
        if self.has_group() {
            spec.group = self.get_group().to_string();
        }
        if self.has_application_environment() {
            spec.application_environment = Some(
                hcore::service::ApplicationEnvironment::new(
                    self.get_application_environment().get_application(),
                    self.get_application_environment().get_environment(),
                ).unwrap(),
            )
        }
        if self.has_bldr_url() {
            spec.bldr_url = self.get_bldr_url().to_string();
        }
        if self.has_bldr_channel() {
            spec.channel = self.get_bldr_channel().to_string();
        }
        if self.has_topology() {
            spec.topology = self.get_topology();
        }
        if self.has_update_strategy() {
            spec.update_strategy = self.get_update_strategy();
        }
        if self.has_specified_binds() {
            let binds: Vec<ServiceBind> = self.get_binds()
                .into_iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect();
            let (_, standard) = binds.into_iter().partition(|ref bind| bind.is_composite());
            spec.binds = standard;
        }
        if self.has_config_from() {
            spec.config_from = Some(PathBuf::from(self.get_config_from()));
        }
        if self.has_svc_encrypted_password() {
            spec.svc_encrypted_password = Some(self.get_svc_encrypted_password().to_string());
        }
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

        let composite_binds = if self.has_specified_binds() {
            let binds: Vec<ServiceBind> = self.get_binds()
                .into_iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect();
            let (composite, _) = binds.into_iter().partition(|ref bind| bind.is_composite());
            Some(composite)
        } else {
            None
        };
        let mut specs: Vec<ServiceSpec> = Vec::with_capacity(services.len());
        for service in services {
            // Customize each service's spec as appropriate
            let mut spec = base_spec.clone();
            spec.ident = service;
            if let Some(ref binds) = composite_binds {
                set_composite_binds(&mut spec, &mut bind_map, &binds);
            }
            specs.push(spec);
        }
        specs
    }

    fn update_composite(&self, bind_map: &mut BindMap, spec: &mut ServiceSpec) {
        // We only want to update fields that were set by SvcLoad
        if self.has_group() {
            spec.group = self.get_group().to_string();
        }
        if self.has_application_environment() {
            spec.application_environment = Some(
                hcore::service::ApplicationEnvironment::new(
                    self.get_application_environment().get_application(),
                    self.get_application_environment().get_environment(),
                ).unwrap(),
            )
        }
        if self.has_bldr_url() {
            spec.bldr_url = self.get_bldr_url().to_string();
        }
        if self.has_bldr_channel() {
            spec.channel = self.get_bldr_channel().to_string();
        }
        if self.has_topology() {
            spec.topology = self.get_topology();
        }
        if self.has_update_strategy() {
            spec.update_strategy = self.get_update_strategy();
        }
        if self.has_specified_binds() {
            let binds: Vec<ServiceBind> = self.get_binds()
                .iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect();
            let (composite, standard) = binds.into_iter().partition(|ref bind| bind.is_composite());
            spec.binds = standard;
            set_composite_binds(spec, bind_map, &composite);
        }
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
fn set_composite_binds(spec: &mut ServiceSpec, bind_map: &mut BindMap, binds: &Vec<ServiceBind>) {
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
        // setting, and thus is only available for `hab sup run`.
        // We don't have a way from `hab svc load` to access the organization setting of an
        // active supervisor, and so we can't generate binds that include organizations.
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
                service_name: Some(bind_mapping.bind_name.clone()),
            };
            final_binds.insert(bind.name.clone(), bind);
        }
    }

    // If anything was overridden or added on the CLI, layer that on
    // now as well. These will take precedence over anything in the
    // composite itself.
    //
    // Note that it consumes the values from cli_binds
    for bind in binds.iter().filter(|bind| {
        bind.service_name.as_ref().unwrap() == &spec.ident.name
    })
    {
        final_binds.insert(bind.name.clone(), bind.clone());
    }

    // Now take all the ServiceBinds we've collected.
    spec.binds = final_binds.drain().map(|(_, v)| v).collect();
}
