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

mod composite_spec;
pub mod config;
mod dir;
pub mod health;
pub mod hooks;
mod package;
pub mod spec;
mod supervisor;

use std;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::result;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use crate::butterfly::rumor::service::Service as ServiceRumor;
use crate::hcore;
use crate::hcore::crypto::hash;
use crate::hcore::fs::FS_ROOT_PATH;
use crate::hcore::package::metadata::Bind;
use crate::hcore::package::{PackageIdent, PackageInstall};
use crate::hcore::service::{HealthCheckInterval, ServiceGroup};
use crate::launcher_client::LauncherCli;
pub use crate::protocol::types::{BindingMode, ProcessState, Topology, UpdateStrategy};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use time::Timespec;

pub use self::composite_spec::CompositeSpec;
use self::config::CfgRenderer;
pub use self::config::{Cfg, UserConfigPath};
use self::dir::SvcDir;
pub use self::health::HealthCheck;
use self::hooks::{Hook, HookTable};
pub use self::package::{Env, Pkg, PkgProxy};
pub use self::spec::{BindMap, DesiredState, IntoServiceSpec, ServiceBind, ServiceSpec, Spec};
use self::supervisor::Supervisor;
use super::ShutdownReason;
use super::Sys;
use crate::census::{CensusGroup, CensusRing, ElectionStatus, ServiceFile};
use crate::error::{Error, Result, SupError};
use crate::fs;
use crate::manager;
use crate::templating::RenderContext;

static LOGKEY: &'static str = "SR";

#[cfg(not(windows))]
pub const GOSSIP_FILE_PERMISSIONS: u32 = 0o640;

/// When evaluating whether a particular service group can satisfy a
/// bind of the Service, there are several states it can be
/// in. Depending on which point in the lifecycle of the Service we
/// are in, we may want to take different actions depending on the
/// current status.
enum BindStatus<'a> {
    /// The bound group is not present in the census
    NotPresent,
    /// The bound group is present in the census, but has no active
    /// members.
    Empty,
    /// The bound group is present in the census, has active members,
    /// but does not satisfy the contract of the bind; the set of
    /// unsatisfied exports is returned.
    Unsatisfied(HashSet<&'a String>),
    /// The bound group is present, has active members, and fully
    /// satisfies the contract of the bind.
    Satisfied,
    /// An error was encountered determining the status
    Unknown(SupError),
}

#[derive(Debug, Serialize)]
pub struct Service {
    pub service_group: ServiceGroup,
    pub bldr_url: String,
    pub channel: String,
    pub desired_state: DesiredState,
    pub spec_file: PathBuf,
    pub spec_ident: PackageIdent,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    pub cfg: Cfg,
    pub pkg: Pkg,
    pub sys: Arc<Sys>,
    pub initialized: bool,
    pub user_config_updated: bool,

    #[serde(skip_serializing)]
    config_renderer: CfgRenderer,
    health_check: HealthCheck,
    last_election_status: ElectionStatus,
    needs_reload: bool,
    needs_reconfiguration: bool,
    /// The mapping of bind name to a service group, specified by the
    /// user when the service definition was loaded into the Supervisor.
    binds: Vec<ServiceBind>,
    /// The binds that the current service package declares, both
    /// required and optional. We don't differentiate because this is
    /// used to validate the user-specified bindings against the
    /// current state of the census; once you get into the actual
    /// running of the service, the distinction is immaterial.
    all_pkg_binds: Vec<Bind>,
    /// Controls how the presence or absence of bound service groups
    /// impacts the service's start-up.
    binding_mode: BindingMode,
    /// Binds specified by the user that are currently mapped to
    /// service groups that do _not_ satisfy the bind's contract, as
    /// defined in the service's current package.
    ///
    /// They may not satisfy them because they do not have the
    /// requisite exports, because no live members of the group exist,
    /// or because the group itself does not exist in the census.
    ///
    /// We don't serialize because this is purely runtime information
    /// that should be reconciled against the current state of the
    /// census.
    #[serde(skip_serializing)]
    unsatisfied_binds: HashSet<ServiceBind>,
    hooks: HookTable,
    config_from: Option<PathBuf>,
    #[serde(skip_serializing)]
    scheduled_health_check: Option<Instant>,
    manager_fs_cfg: Arc<manager::FsCfg>,
    #[serde(rename = "process")]
    supervisor: Supervisor,
    svc_encrypted_password: Option<String>,
    health_check_interval: HealthCheckInterval,
    composite: Option<String>,

    #[serde(skip_serializing)]
    /// Whether a service's default configuration changed on a package
    /// update. Used to control when templates are re-rendered.
    defaults_updated: bool,
    #[serde(skip_serializing)]
    gateway_state: Arc<RwLock<manager::GatewayState>>,
}

impl Service {
    fn new(
        sys: Arc<Sys>,
        package: PackageInstall,
        spec: ServiceSpec,
        manager_fs_cfg: Arc<manager::FsCfg>,
        organization: Option<&str>,
        gateway_state: Arc<RwLock<manager::GatewayState>>,
    ) -> Result<Service> {
        spec.validate(&package)?;
        let all_pkg_binds = package.all_binds()?;
        let pkg = Pkg::from_install(package)?;
        let spec_file = manager_fs_cfg.specs_path.join(spec.file_name());
        let service_group = ServiceGroup::new(
            spec.application_environment.as_ref(),
            &pkg.name,
            spec.group,
            organization,
        )?;
        let config_root = Self::config_root(&pkg, spec.config_from.as_ref());
        let hooks_root = Self::hooks_root(&pkg, spec.config_from.as_ref());
        Ok(Service {
            sys: sys,
            cfg: Cfg::new(&pkg, spec.config_from.as_ref())?,
            config_renderer: CfgRenderer::new(&config_root)?,
            bldr_url: spec.bldr_url,
            channel: spec.channel,
            desired_state: spec.desired_state,
            health_check: HealthCheck::default(),
            hooks: HookTable::load(
                &service_group,
                &hooks_root,
                fs::svc_hooks_path(&service_group.service()),
            ),
            initialized: false,
            last_election_status: ElectionStatus::None,
            needs_reload: false,
            needs_reconfiguration: false,
            user_config_updated: false,
            manager_fs_cfg: manager_fs_cfg,
            supervisor: Supervisor::new(&service_group),
            pkg: pkg,
            service_group: service_group,
            binds: spec.binds,
            all_pkg_binds: all_pkg_binds,
            unsatisfied_binds: HashSet::new(),
            binding_mode: spec.binding_mode,
            spec_ident: spec.ident,
            spec_file: spec_file,
            topology: spec.topology,
            update_strategy: spec.update_strategy,
            config_from: spec.config_from,
            scheduled_health_check: None,
            svc_encrypted_password: spec.svc_encrypted_password,
            health_check_interval: spec.health_check_interval,
            composite: spec.composite,
            defaults_updated: false,
            gateway_state: gateway_state,
        })
    }

    /// Returns the config root given the package and optional config-from path.
    fn config_root(package: &Pkg, config_from: Option<&PathBuf>) -> PathBuf {
        config_from
            .and_then(|p| Some(p.as_path()))
            .unwrap_or(&package.path)
            .join("config")
    }

    /// Returns the hooks root given the package and optional config-from path.
    fn hooks_root(package: &Pkg, config_from: Option<&PathBuf>) -> PathBuf {
        config_from
            .and_then(|p| Some(p.as_path()))
            .unwrap_or(&package.path)
            .join("hooks")
    }

    pub fn load(
        sys: Arc<Sys>,
        spec: ServiceSpec,
        manager_fs_cfg: Arc<manager::FsCfg>,
        organization: Option<&str>,
        gateway_state: Arc<RwLock<manager::GatewayState>>,
    ) -> Result<Service> {
        // The package for a spec should already be installed.
        let fs_root_path = Path::new(&*FS_ROOT_PATH);
        let package = PackageInstall::load(&spec.ident, Some(fs_root_path))?;
        Ok(Self::new(
            sys,
            package,
            spec,
            manager_fs_cfg,
            organization,
            gateway_state,
        )?)
    }

    /// Create the service path for this package.
    pub fn create_svc_path(&self) -> Result<()> {
        debug!("{}, Creating svc paths", self.service_group);
        SvcDir::new(&self.pkg).create()
    }

    fn start(&mut self, launcher: &LauncherCli) {
        if let Some(err) = self
            .supervisor
            .start(
                &self.pkg,
                &self.service_group,
                launcher,
                self.svc_encrypted_password.as_ref(),
            )
            .err()
        {
            outputln!(preamble self.service_group, "Service start failed: {}", err);
        } else {
            self.needs_reload = false;
            self.needs_reconfiguration = false;
        }
    }

    pub fn stop(&mut self, launcher: &LauncherCli, cause: ShutdownReason) {
        match self.supervisor.stop(launcher, cause) {
            Ok(_) => self.post_stop(),
            Err(err) => outputln!(preamble self.service_group, "Service stop failed: {}", err),
        }
    }

    /// Runs the reconfigure hook if present, otherwise restarts the service.
    fn reload(&mut self, launcher: &LauncherCli) {
        self.needs_reload = false;
        if self.process_down() || self.hooks.reload.is_none() {
            if let Some(err) = self
                .supervisor
                .restart(
                    &self.pkg,
                    &self.service_group,
                    launcher,
                    self.svc_encrypted_password.as_ref(),
                )
                .err()
            {
                outputln!(preamble self.service_group, "Service restart failed: {}", err);
            }
        } else {
            let hook = self.hooks.reload.as_ref().unwrap();
            hook.run(
                &self.service_group,
                &self.pkg,
                self.svc_encrypted_password.as_ref(),
            );
        }
    }

    pub fn last_state_change(&self) -> Timespec {
        self.supervisor.state_entered
    }

    /// Performs updates and executes hooks.
    ///
    /// Returns `true` if the service was updated.
    pub fn tick(&mut self, census_ring: &CensusRing, launcher: &LauncherCli) -> bool {
        // We may need to block the service from starting until all
        // its binds are satisfied
        if !self.initialized {
            match self.binding_mode {
                BindingMode::Relaxed => (),
                BindingMode::Strict => {
                    self.validate_binds(census_ring);
                    if !self.unsatisfied_binds.is_empty() {
                        outputln!(preamble self.service_group, "Waiting for service binds...");
                        return false;
                    }
                }
            }
        }

        // Binds may become unsatisfied as a service is running (e.g.,
        // service members disappear, etc.) This can affect the data
        // we pass to templates, so we must account for it here.
        if census_ring.changed() {
            self.validate_binds(census_ring);
        }

        let svc_updated = self.update_templates(census_ring);
        if self.update_service_files(census_ring) {
            self.file_updated();
        }

        match self.topology {
            Topology::Standalone => {
                self.execute_hooks(launcher);
            }
            Topology::Leader => {
                let census_group = census_ring
                    .census_group_for(&self.service_group)
                    .expect("Service Group's census entry missing from list!");
                match census_group.election_status {
                    ElectionStatus::None => {
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; election hasn't started");
                            self.last_election_status = census_group.election_status;
                        }
                    }
                    ElectionStatus::ElectionInProgress => {
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; election in progress.");
                            self.last_election_status = census_group.election_status;
                        }
                    }
                    ElectionStatus::ElectionNoQuorum => {
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; election in progress, \
                                      and we have no quorum.");

                            self.last_election_status = census_group.election_status
                        }
                    }
                    ElectionStatus::ElectionFinished => {
                        let leader_id = census_group
                            .leader_id
                            .as_ref()
                            .expect("No leader with finished election");
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Executing hooks; {} is the leader",
                                      leader_id.to_string());
                            self.last_election_status = census_group.election_status;
                        }
                        self.execute_hooks(launcher)
                    }
                }
            }
        }
        if svc_updated {
            self.schedule_special_health_check();
        }

        svc_updated
    }

    pub fn to_spec(&self) -> ServiceSpec {
        let mut spec = ServiceSpec::default_for(self.spec_ident.clone());
        spec.group = self.service_group.group().to_string();
        if let Some(appenv) = self.service_group.application_environment() {
            spec.application_environment = Some(appenv)
        }
        if let Some(ref composite) = self.composite {
            spec.composite = Some(composite.clone())
        }
        spec.bldr_url = self.bldr_url.clone();
        spec.channel = self.channel.clone();
        spec.topology = self.topology;
        spec.update_strategy = self.update_strategy;
        spec.binds = self.binds.clone();
        spec.binding_mode = self.binding_mode;
        spec.config_from = self.config_from.clone();
        if let Some(ref password) = self.svc_encrypted_password {
            spec.svc_encrypted_password = Some(password.clone())
        }
        spec.health_check_interval = self.health_check_interval;
        spec
    }

    /// Iterate through all the service binds, marking any that are
    /// unsatisfied in `self.unsatisfied_binds`.
    ///
    /// When starting with a "strict" binding mode, the presence of
    /// any unsatisfied binds will block service startup.
    ///
    /// Thereafter, if binds become unsatisfied during the running of
    /// the service, those binds will be removed from the rendering
    /// context, allowing services to take appropriate action.
    fn validate_binds(&mut self, census_ring: &CensusRing) {
        for bind in self.binds.iter() {
            let mut bind_is_unsatisfied = true;

            match self.current_bind_status(census_ring, bind) {
                BindStatus::NotPresent => {
                    outputln!(preamble self.service_group,
                                  "The specified service group '{}' for binding '{}' is not (yet?) present \
                                   in the census data.",
                                  bind.service_group,
                                  bind.name);
                }
                BindStatus::Empty => {
                    outputln!(preamble self.service_group,
                                  "The specified service group '{}' for binding '{}' is present in the \
                                   census, but currently has no active members.",
                                  bind.service_group,
                                  bind.name);
                }
                BindStatus::Unsatisfied(ref unsatisfied) => {
                    outputln!(preamble self.service_group,
                                  "The group '{}' cannot satisfy the `{}` bind because it does not export \
                                   the following required fields: {:?}",
                                  bind.service_group,
                                  bind.name,
                                  unsatisfied);
                }
                BindStatus::Satisfied => {
                    // Since this function is currently called any
                    // time the census changes, and this is the
                    // expected steady-state of a properly running
                    // service, we won't log anything here. Otherwise
                    // we'd just spam the logs. Instead, log only on a
                    // state change (see below).
                    bind_is_unsatisfied = false;
                }
                BindStatus::Unknown(ref e) => {
                    outputln!(preamble self.service_group,
                                  "Error validating bind for {}=>{}: {}",
                                  bind.name,
                                  bind.service_group,
                                  e);
                }
            };

            if bind_is_unsatisfied {
                // TODO (CM): use Entry API to clone only when necessary
                self.unsatisfied_binds.insert((bind).clone())
            } else if self.unsatisfied_binds.remove(bind) {
                // We'll log if the bind was previously
                // unsatisfied, but now it is satisfied.
                outputln!(preamble self.service_group,
                              "The group '{}' satisfies the `{}` bind",
                              bind.service_group,
                              bind.name);
                true
            } else {
                false
            };
        }
    }

    /// Evaluate the suitability of the given `ServiceBind` based on
    /// current census information.
    fn current_bind_status<'a>(
        &'a self,
        census_ring: &'a CensusRing,
        service_bind: &'a ServiceBind,
    ) -> BindStatus<'a> {
        match census_ring.census_group_for(&service_bind.service_group) {
            None => BindStatus::NotPresent,
            Some(group) => {
                if group.active_members().count() == 0 {
                    BindStatus::Empty
                } else {
                    match self.unsatisfied_bind_exports(group, &service_bind.name) {
                        Ok(unsatisfied) => {
                            if unsatisfied.is_empty() {
                                BindStatus::Satisfied
                            } else {
                                BindStatus::Unsatisfied(unsatisfied)
                            }
                        }
                        Err(e) => BindStatus::Unknown(e),
                    }
                }
            }
        }
    }

    /// Does the service we've bound to actually satisfy the bind's
    /// contract (i.e., does it export everything we need)?
    ///
    /// Returns the set of unsatisfied exports. If everything is
    /// present, though, you get an empty set.
    ///
    /// Can return `Error::NoSuchBind` if there's not a bind with the
    /// given name.
    /// Can return `Error::NoActiveMembers` if there are no active members
    /// of the group.
    fn unsatisfied_bind_exports<'a>(
        &'a self,
        group: &'a CensusGroup,
        bind_name: &'a str,
    ) -> Result<HashSet<&'a String>> {
        let exports = self.exports_required_for_bind(bind_name)?;
        let group_exports = group.group_exports()?;

        let diff: HashSet<&String> = exports
            .difference(&group_exports)
            .cloned() // &&String -> &String
            .collect();

        Ok(diff)
    }

    /// Returns the list of exported values a given bind requires
    ///
    /// Returns Err if there is no bind by the given name... by the
    /// time we get to this code, though, that shouldn't happen.
    fn exports_required_for_bind<'a>(&'a self, binding_name: &str) -> Result<HashSet<&'a String>> {
        // TODO (CM): Really, we want a HashMap of name => HashSet instead of a
        // Vec<Bind>... this finding is for the birds
        self.all_pkg_binds
            .iter()
            .find(|b| b.service == binding_name)
            .ok_or(sup_error!(Error::NoSuchBind(binding_name.to_string())))
            .map(|b| b.exports.iter().collect())
    }

    /// Updates the process state of the service's supervisor
    fn check_process(&mut self) -> bool {
        self.supervisor.check_process()
    }

    fn process_down(&self) -> bool {
        self.supervisor.state == ProcessState::Down
    }

    /// Compares the current state of the service to the current state of the census ring and the
    /// user-config, and re-renders all templatable content to disk.
    ///
    /// Returns `true` if any modifications were made.
    fn update_templates(&mut self, census_ring: &CensusRing) -> bool {
        let census_group = census_ring
            .census_group_for(&self.service_group)
            .expect("Service update failed; unable to find own service group");
        let cfg_updated_from_rumors = self.cfg.update(census_group);
        let cfg_changed =
            self.defaults_updated || cfg_updated_from_rumors || self.user_config_updated;

        if self.user_config_updated {
            if let Err(e) = self.cfg.reload_user() {
                outputln!(preamble self.service_group, "Reloading user-config failed: {}", e);
            }

            self.user_config_updated = false;
        }

        self.defaults_updated = false;

        if cfg_changed || census_ring.changed() {
            let (reload, reconfigure) = {
                let ctx = self.render_context(census_ring);

                // If any hooks have changed, execute the `reload` hook (if present) or restart the
                // service.
                let reload = self.compile_hooks(&ctx);

                // If the configuration has changed, execute the `reload` and `reconfigure` hooks.
                // Note that the configuration does not necessarily change every time the user
                // config has (e.g. when only a comment has been added to the latter)
                let reconfigure = self.compile_configuration(&ctx);

                (reload, reconfigure)
            };

            self.schedule_special_health_check();

            self.needs_reload = reload;
            self.needs_reconfiguration = reconfigure;
        }

        cfg_changed
    }

    /// Replace the package of the running service and restart its system process.
    pub fn update_package(&mut self, package: PackageInstall, launcher: &LauncherCli) {
        match Pkg::from_install(package) {
            Ok(pkg) => {
                outputln!(preamble self.service_group,
                            "Updating service {} to {}", self.pkg.ident, pkg.ident);

                self.schedule_special_health_check();

                match CfgRenderer::new(&Self::config_root(&pkg, self.config_from.as_ref())) {
                    Ok(renderer) => self.config_renderer = renderer,
                    Err(e) => {
                        outputln!(preamble self.service_group,
                                  "Failed to load config templates after updating package, {}", e);
                        return;
                    }
                }
                self.hooks = HookTable::load(
                    &self.service_group,
                    &Self::hooks_root(&pkg, self.config_from.as_ref()),
                    fs::svc_hooks_path(self.service_group.service()),
                );
                self.pkg = pkg;
            }
            Err(err) => {
                outputln!(preamble self.service_group,
                          "Unexpected error while updating package, {}", err);
                return;
            }
        }
        if let Err(err) = self.supervisor.stop(launcher, ShutdownReason::PkgUpdating) {
            outputln!(preamble self.service_group,
                      "Error stopping process while updating package: {}", err);
        }

        match self.cfg.update_defaults_from_package(&self.pkg) {
            Ok(maybe_updated) => {
                self.defaults_updated = maybe_updated;
            }
            Err(err) => {
                outputln!(preamble self.service_group,
                          "Unexpected error while checking for updated package defaults: {}", err);
            }
        }

        self.initialized = false;
    }

    pub fn to_rumor(&self, incarnation: u64) -> ServiceRumor {
        let exported = match self.cfg.to_exported(&self.pkg) {
            Ok(exported) => Some(exported),
            Err(err) => {
                outputln!(preamble self.service_group,
                          "Failed to generate exported cfg for service rumor: {}",
                           err);
                None
            }
        };
        let mut rumor = ServiceRumor::new(
            self.sys.member_id.as_str(),
            &self.pkg.ident,
            self.service_group.clone(),
            self.sys.as_sys_info().clone(),
            exported,
        );
        rumor.incarnation = incarnation;
        rumor
    }

    /// Run initialization hook if present.
    fn initialize(&mut self) {
        if self.initialized {
            return;
        }
        outputln!(preamble self.service_group, "Initializing");
        self.initialized = true;
        if let Some(ref hook) = self.hooks.init {
            self.initialized = hook.run(
                &self.service_group,
                &self.pkg,
                self.svc_encrypted_password.as_ref(),
            )
        }
    }

    /// Run reconfigure hook if present.
    fn reconfigure(&mut self) {
        self.needs_reconfiguration = false;
        if let Some(ref hook) = self.hooks.reconfigure {
            hook.run(
                &self.service_group,
                &self.pkg,
                self.svc_encrypted_password.as_ref(),
            );
        }
    }

    fn post_run(&mut self) {
        if let Some(ref hook) = self.hooks.post_run {
            hook.run(
                &self.service_group,
                &self.pkg,
                self.svc_encrypted_password.as_ref(),
            );
        }
    }

    fn post_stop(&mut self) {
        if let Some(ref hook) = self.hooks.post_stop {
            hook.run(
                &self.service_group,
                &self.pkg,
                self.svc_encrypted_password.as_ref(),
            );
        }

        self.gateway_state
            .write()
            .expect("GatewayState lock is poisoned")
            .health_check_data
            .remove(&self.service_group);
    }

    pub fn suitability(&self) -> Option<u64> {
        if !self.initialized {
            return None;
        }
        self.hooks.suitability.as_ref().and_then(|hook| {
            hook.run(
                &self.service_group,
                &self.pkg,
                self.svc_encrypted_password.as_ref(),
            )
        })
    }

    fn cache_health_check(&self, check_result: HealthCheck) {
        debug!(
            "Caching HealthCheck = '{}' for '{}'",
            check_result, self.service_group
        );
        self.gateway_state
            .write()
            .expect("GatewayState lock is poisoned")
            .health_check_data
            .insert(self.service_group.clone(), check_result);
    }

    /// Helper for compiling configuration templates into configuration files.
    ///
    /// Returns `true` if the configuration has changed.
    fn compile_configuration(&self, ctx: &RenderContext<'_>) -> bool {
        match self.config_renderer.compile(&self.pkg, ctx) {
            Ok(true) => true,
            Ok(false) => false,
            Err(e) => {
                outputln!(preamble self.service_group,
                          "Failed to compile configuration: {}",
                          e);
                false
            }
        }
    }

    /// Helper for compiling hook templates into hooks.
    ///
    /// This function will also perform any necessary post-compilation tasks.
    ///
    /// Returns `true` if any hooks have changed.
    fn compile_hooks(&self, ctx: &RenderContext<'_>) -> bool {
        let changed = self.hooks.compile(&self.service_group, ctx);
        if let Some(err) = self.copy_run().err() {
            outputln!(preamble self.service_group, "Failed to copy run hook: {}", err);
        }
        if changed {
            outputln!(preamble self.service_group, "Hooks recompiled");
        }
        changed
    }

    // Copy the "run" file to the svc path.
    fn copy_run(&self) -> Result<()> {
        let svc_run = self.pkg.svc_path.join(hooks::RunHook::file_name());
        match self.hooks.run {
            Some(ref hook) => {
                std::fs::copy(hook.path(), &svc_run)?;
                Self::set_hook_permissions(&svc_run.to_str().unwrap())?;
            }
            None => {
                let run = self.pkg.path.join(hooks::RunHook::file_name());
                match std::fs::metadata(&run) {
                    Ok(_) => {
                        std::fs::copy(&run, &svc_run)?;
                        Self::set_hook_permissions(&svc_run)?;
                    }
                    Err(err) => {
                        outputln!(preamble self.service_group, "Error finding run file: {}", err);
                    }
                }
            }
        }
        Ok(())
    }

    #[cfg(not(windows))]
    fn set_hook_permissions<T: AsRef<Path>>(path: T) -> hcore::error::Result<()> {
        use self::hooks::HOOK_PERMISSIONS;
        use crate::hcore::util::posix_perm;

        posix_perm::set_permissions(path.as_ref(), HOOK_PERMISSIONS)
    }

    #[cfg(windows)]
    fn set_hook_permissions<T: AsRef<Path>>(path: T) -> hcore::error::Result<()> {
        use hcore::util::win_perm;

        win_perm::harden_path(path.as_ref())
    }

    fn execute_hooks(&mut self, launcher: &LauncherCli) {
        if !self.initialized {
            if self.check_process() {
                outputln!("Reattached to {}", self.service_group);
                self.initialized = true;
                return;
            }
            self.initialize();
            if self.initialized {
                self.start(launcher);
                self.post_run();
            }
        } else {
            self.check_process();

            let now = Instant::now();
            match self.scheduled_health_check {
                Some(scheduled_check_instant) if scheduled_check_instant > now => {
                    trace!(
                        "Skipping health check; next scheduled for {:?} (now: {:?})",
                        scheduled_check_instant,
                        now
                    );
                }
                _ => self.run_health_check_hook(),
            }

            // NOTE: if you need reconfiguration and you DON'T have a
            // reload script, you're going to restart anyway.
            if self.needs_reload || self.process_down() || self.needs_reconfiguration {
                self.reload(launcher);
                if self.needs_reconfiguration {
                    // NOTE this only runs the hook if it's defined
                    self.reconfigure()
                }
            }
        }
    }

    /// Run file-updated hook if present.
    fn file_updated(&self) -> bool {
        if self.initialized {
            if let Some(ref hook) = self.hooks.file_updated {
                return hook.run(
                    &self.service_group,
                    &self.pkg,
                    self.svc_encrypted_password.as_ref(),
                );
            }
        }
        false
    }

    /// Write service files from gossip data to disk under
    /// [`svc_files_path()`](../../fs/fn.svc_files_path.html).
    ///
    /// Returns `true` if a file was changed, added, or removed, and
    /// `false` if there were no updates.
    fn update_service_files(&mut self, census_ring: &CensusRing) -> bool {
        let census_group = census_ring
            .census_group_for(&self.service_group)
            .expect("Service update service files failed; unable to find own service group");
        let mut updated = false;
        for service_file in census_group.changed_service_files() {
            if self.cache_service_file(&service_file) {
                outputln!(preamble self.service_group, "Service file updated, {}",
                    service_file.filename);
                updated = true;
            }
        }
        updated
    }

    /// Helper for constructing a new render context for the service.
    fn render_context<'a>(&'a self, census: &'a CensusRing) -> RenderContext<'a> {
        // Unsatisfied binds are filtered out; you only get bind
        // information in the render context if they actually satisfy
        // the contract!
        RenderContext::new(
            &self.service_group,
            &self.sys,
            &self.pkg,
            &self.cfg,
            census,
            self.binds
                .iter()
                .filter(|b| !self.unsatisfied_binds.contains(b)),
        )
    }

    fn run_health_check_hook(&mut self) {
        debug!("Running Health Check hook for ({})", self.spec_ident);
        let check_result = if let Some(ref hook) = self.hooks.health_check {
            hook.run(
                &self.service_group,
                &self.pkg,
                self.svc_encrypted_password.as_ref(),
            )
        } else {
            match self.supervisor.status() {
                (true, _) => HealthCheck::Ok,
                (false, _) => HealthCheck::Critical,
            }
        };

        // We have just ran a check; therefore we must unset the next scheduled check time
        // in anticipation of `None` value being used in the next scheduled check time calculation.
        self.scheduled_health_check = None;

        if check_result == HealthCheck::Ok {
            self.schedule_routine_health_check();
            debug!(
                "Service ({}) health check is: {}",
                self.spec_ident, check_result
            );
        } else {
            debug!(
                "Service ({}) health check is: {}; scheduling special health check",
                self.spec_ident, check_result
            );
            self.schedule_special_health_check();
        }
        self.health_check = check_result;
        self.cache_health_check(check_result);
    }

    fn schedule_routine_health_check(&mut self) {
        let interval = self.health_check_interval;
        self.schedule_health_check(interval);
    }

    fn schedule_special_health_check(&mut self) {
        self.schedule_health_check(HealthCheckInterval::default());
    }

    fn schedule_health_check(&mut self, interval: HealthCheckInterval) {
        let instant_to_schedule = Instant::now() + interval.into();
        match self.scheduled_health_check {
            Some(already_scheduled_instant) if instant_to_schedule > already_scheduled_instant => {
                trace!(
                        "Skipping health check schedule request for {:?}; there is already one scheduled sooner at {:?}",
                        instant_to_schedule,
                        already_scheduled_instant
                    );
            }
            _ => {
                debug!(
                    "Scheduling next health check for ({}) in {}",
                    self.spec_ident, interval
                );
                self.scheduled_health_check = Some(instant_to_schedule);
            }
        }
    }

    // Returns `false` if the write fails.
    fn cache_service_file(&mut self, service_file: &ServiceFile) -> bool {
        let file = self.pkg.svc_files_path.join(&service_file.filename);
        self.write_cache_file(file, &service_file.body)
    }

    // Returns `false` if the write fails.
    fn write_cache_file<T>(&self, file: T, contents: &[u8]) -> bool
    where
        T: AsRef<Path>,
    {
        let current_checksum = match hash::hash_file(&file) {
            Ok(current_checksum) => current_checksum,
            Err(err) => {
                outputln!(preamble self.service_group, "Failed to get current checksum for {}, {}",
                       file.as_ref().display(),
                       err);
                String::new()
            }
        };
        let new_checksum = hash::hash_bytes(&contents);
        if new_checksum == current_checksum {
            return false;
        }
        let new_filename = format!("{}.write", file.as_ref().to_string_lossy());
        let mut new_file = match File::create(&new_filename) {
            Ok(new_file) => new_file,
            Err(e) => {
                outputln!(preamble self.service_group,
                          "Failed to create cache file {}, {}",
                          file.as_ref().display(), e);
                return false;
            }
        };
        if let Err(e) = new_file.write_all(contents) {
            outputln!(preamble self.service_group,
                      "Failed to write to cache file {}, {}",
                      file.as_ref().display(), e);
            return false;
        }
        if let Err(e) = std::fs::rename(&new_filename, &file) {
            outputln!(preamble self.service_group,
                      "Failed to move cache file {}, {}",
                      file.as_ref().display(), e);
            return false;
        }

        self.set_gossip_permissions(&file)
    }

    #[cfg(not(windows))]
    fn set_gossip_permissions<T: AsRef<Path>>(&self, path: T) -> bool {
        use crate::hcore::util::posix_perm;
        use crate::sys::abilities;

        if abilities::can_run_services_as_svc_user() {
            let result =
                posix_perm::set_owner(path.as_ref(), &self.pkg.svc_user, &self.pkg.svc_group);
            if let Err(e) = result {
                outputln!(preamble self.service_group,
                          "Failed to set ownership of cache file {}, {}",
                          path.as_ref().display(), e);
                return false;
            }
        }

        if let Err(e) = posix_perm::set_permissions(path.as_ref(), GOSSIP_FILE_PERMISSIONS) {
            outputln!(preamble self.service_group,
                      "Failed to set permissions on cache file {}, {}",
                      path.as_ref().display(), e);
            return false;
        }
        true
    }

    #[cfg(windows)]
    fn set_gossip_permissions<T: AsRef<Path>>(&self, path: T) -> bool {
        use hcore::util::win_perm;

        if let Err(e) = win_perm::harden_path(path.as_ref()) {
            outputln!(preamble self.service_group,
                      "Failed to set permissions on cache file {}, {}",
                      path.as_ref().display(), e);
            return false;
        }
        true
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.service_group, self.pkg.ident)
    }
}

/// This enum represents whether or not we want to render config information when we serialize this
/// service via the ServiceProxy struct below. Choosing ConfigRendering::Full will render the
/// config, and choosing ConfigRendering::Redacted will not render it. This matches up to the
/// feature flag we have in place to redact config information from a service's serialized output,
/// which shows up in the supervisor's HTTP API responses.
///
/// Please note that this enum derives the Copy trait, so that it behaves more like the boolean
/// that it is, and so that we don't have to clone() it everywhere. Adding anything to this enum
/// that consumes a large amount of memory would be a bad idea (without removing Copy first)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigRendering {
    Full,
    Redacted,
}

/// This is a proxy struct to represent what information we're writing to the dat file, and
/// therefore what information gets sent out via the HTTP API. Right now, we're just wrapping the
/// actual Service struct, but this will give us something we can refactor against without
/// worrying about breaking the data returned to users.
pub struct ServiceProxy<'a> {
    service: &'a Service,
    config_rendering: ConfigRendering,
}

impl<'a> ServiceProxy<'a> {
    pub fn new(s: &'a Service, c: ConfigRendering) -> Self {
        ServiceProxy {
            service: &s,
            config_rendering: c,
        }
    }
}

impl<'a> Serialize for ServiceProxy<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let num_fields: usize = if self.config_rendering == ConfigRendering::Full {
            28
        } else {
            27
        };

        let s = &self.service;
        let mut strukt = serializer.serialize_struct("service", num_fields)?;
        strukt.serialize_field("all_pkg_binds", &s.all_pkg_binds)?;
        strukt.serialize_field("binding_mode", &s.binding_mode)?;
        strukt.serialize_field("binds", &s.binds)?;
        strukt.serialize_field("bldr_url", &s.bldr_url)?;

        if self.config_rendering == ConfigRendering::Full {
            strukt.serialize_field("cfg", &s.cfg)?;
        }

        strukt.serialize_field("channel", &s.channel)?;
        strukt.serialize_field("composite", &s.composite)?;
        strukt.serialize_field("config_from", &s.config_from)?;
        strukt.serialize_field("desired_state", &s.desired_state)?;
        strukt.serialize_field("health_check", &s.health_check)?;
        strukt.serialize_field("hooks", &s.hooks)?;
        strukt.serialize_field("initialized", &s.initialized)?;
        strukt.serialize_field("last_election_status", &s.last_election_status)?;
        strukt.serialize_field("manager_fs_cfg", &s.manager_fs_cfg)?;
        strukt.serialize_field("needs_reconfiguration", &s.needs_reconfiguration)?;
        strukt.serialize_field("needs_reload", &s.needs_reload)?;

        let pkg_proxy = PkgProxy::new(&s.pkg);
        strukt.serialize_field("pkg", &pkg_proxy)?;

        strukt.serialize_field("process", &s.supervisor)?;
        strukt.serialize_field("service_group", &s.service_group)?;
        strukt.serialize_field("spec_file", &s.spec_file)?;
        strukt.serialize_field("spec_ident", &s.spec_ident)?;
        strukt.serialize_field("spec_identifier", &s.spec_ident.to_string())?;
        strukt.serialize_field("svc_encrypted_password", &s.svc_encrypted_password)?;
        strukt.serialize_field("health_check_interval", &s.health_check_interval)?;
        strukt.serialize_field("sys", &s.sys)?;
        strukt.serialize_field("topology", &s.topology)?;
        strukt.serialize_field("update_strategy", &s.update_strategy)?;
        strukt.serialize_field("user_config_updated", &s.user_config_updated)?;
        strukt.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::hcore::package::{ident::PackageIdent, PackageInstall};
    use serde_json;

    use self::{
        manager::{sys::Sys, FsCfg},
        ServiceSpec,
    };
    use crate::common::types::ListenCtlAddr;
    use crate::config::GossipListenAddr;
    use crate::http_gateway;
    use crate::test_helpers::*;

    #[test]
    fn service_proxy_conforms_to_the_schema() {
        let listen_ctl_addr =
            ListenCtlAddr::from_str("127.0.0.1:1234").expect("Can't parse IP into SocketAddr");
        let http_addr = http_gateway::ListenAddr::default();
        let sys = Sys::new(
            false,
            GossipListenAddr::default(),
            listen_ctl_addr,
            http_addr,
        );

        let ident = if cfg!(target_os = "linux") {
            PackageIdent::new("core", "tree", Some("1.7.0"), Some("20180609045201"))
        } else if cfg!(target_os = "windows") {
            PackageIdent::new("core", "7zip", Some("16.04"), Some("20170131110814"))
        } else {
            panic!("This is being run on a platform that's not currently supported");
        };

        let spec = ServiceSpec::default_for(ident);

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("pkgs");

        let install = PackageInstall::load(&spec.ident, Some(&path))
            .expect("PackageInstall should've loaded my spec, but it didn't");
        let asys = Arc::new(sys);
        let fscfg = FsCfg::new("/tmp");
        let afs = Arc::new(fscfg);
        let gs = Arc::new(RwLock::new(manager::GatewayState::default()));
        let service = Service::new(asys, install, spec, afs, Some("haha"), gs)
            .expect("I wanted a service to load, but it didn't");

        // With config
        let proxy_with_config = ServiceProxy::new(&service, ConfigRendering::Full);
        let proxies_with_config = vec![proxy_with_config];
        let json_with_config = serde_json::to_string(&proxies_with_config)
            .expect("Expected to convert proxies_with_config to JSON but failed");
        assert_valid(&json_with_config, "http_gateway_services_schema.json");

        // Without config
        let proxy_without_config = ServiceProxy::new(&service, ConfigRendering::Redacted);
        let proxies_without_config = vec![proxy_without_config];
        let json_without_config = serde_json::to_string(&proxies_without_config)
            .expect("Expected to convert proxies_without_config to JSON but failed");
        assert_valid(&json_without_config, "http_gateway_services_schema.json");
    }
}
