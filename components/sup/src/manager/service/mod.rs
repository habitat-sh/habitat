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

mod health;
mod spec;
mod config;
pub mod hooks;

use std;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::time::{Duration, Instant};

use ansi_term::Colour::{Yellow, Red, Green};
use butterfly;
use butterfly::rumor::service::Service as ServiceRumor;
use common::ui::UI;
use hcore::fs::FS_ROOT_PATH;
use hcore::service::ServiceGroup;
use hcore::crypto::hash;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::util::deserialize_using_from_str;
use hcore::util::perm::{set_owner, set_permissions};
use serde;
use toml;

use self::hooks::{HOOK_PERMISSIONS, Hook, HookTable};
use config::GossipListenAddr;
use error::{Error, Result, SupError};
use http_gateway;
use fs;
use manager::{self, signals};
use manager::census::{CensusList, CensusUpdate, ElectionStatus};
use supervisor::{Supervisor, RuntimeConfig};
use util;

pub use self::config::{ServiceConfig, Pkg};
pub use self::health::{HealthCheck, SmokeCheck};
pub use self::spec::{DesiredState, ServiceBind, ServiceSpec, StartStyle};

static LOGKEY: &'static str = "SR";

lazy_static! {
    static ref HEALTH_CHECK_INTERVAL: Duration = {
        Duration::from_millis(30_000)
    };
}

#[derive(Debug, Serialize)]
pub struct Service {
    pub config: ServiceConfig,
    current_service_files: HashMap<String, u64>,
    pub depot_url: String,
    health_check: HealthCheck,
    initialized: bool,
    last_election_status: ElectionStatus,
    needs_reload: bool,
    needs_reconfiguration: bool,
    #[serde(serialize_with="serialize_lock")]
    package: Arc<RwLock<PackageInstall>>,
    pub service_group: ServiceGroup,
    smoke_check: SmokeCheck,
    pub spec_file: PathBuf,
    pub spec_ident: PackageIdent,
    pub start_style: StartStyle,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    #[serde(skip_serializing)]
    spec_binds: Vec<ServiceBind>,
    hooks: HookTable,
    config_from: Option<PathBuf>,
    #[serde(skip_serializing)]
    last_health_check: Instant,
    #[serde(skip_serializing)]
    manager_fs_cfg: Arc<manager::FsCfg>,
    supervisor: Supervisor,
}

impl Service {
    fn new(package: PackageInstall,
           spec: ServiceSpec,
           gossip_listen: &GossipListenAddr,
           http_listen: &http_gateway::ListenAddr,
           manager_fs_cfg: Arc<manager::FsCfg>,
           organization: Option<&str>)
           -> Result<Service> {
        spec.validate(&package)?;
        let spec_file = manager_fs_cfg.specs_path.join(spec.file_name());
        let service_group = ServiceGroup::new(&package.ident.name, spec.group, organization)?;
        let runtime_cfg = Self::runtime_config_from(&package)?;
        let config_root = spec.config_from
            .clone()
            .unwrap_or(package.installed_path.clone());
        let svc_cfg = ServiceConfig::new(&package,
                                         &runtime_cfg,
                                         config_root,
                                         spec.binds.clone(),
                                         &gossip_listen,
                                         &http_listen)?;
        let hook_template_path = svc_cfg.config_root.join("hooks");
        let hooks_path = fs::svc_hooks_path(service_group.service());
        let locked_package = Arc::new(RwLock::new(package));
        Ok(Service {
               config: svc_cfg,
               current_service_files: HashMap::new(),
               depot_url: spec.depot_url,
               health_check: HealthCheck::default(),
               hooks: HookTable::default().load_hooks(&service_group,
                                                      &hooks_path,
                                                      &hook_template_path),
               initialized: false,
               last_election_status: ElectionStatus::None,
               needs_reload: false,
               needs_reconfiguration: false,
               manager_fs_cfg: manager_fs_cfg,
               supervisor: Supervisor::new(locked_package.clone(), &service_group, runtime_cfg),
               package: locked_package,
               service_group: service_group,
               smoke_check: SmokeCheck::default(),
               spec_binds: spec.binds,
               spec_ident: spec.ident,
               spec_file: spec_file,
               start_style: spec.start_style,
               topology: spec.topology,
               update_strategy: spec.update_strategy,
               config_from: spec.config_from,
               last_health_check: Instant::now() - *HEALTH_CHECK_INTERVAL,
           })
    }

    fn runtime_config_from(package: &PackageInstall) -> Result<RuntimeConfig> {
        let (svc_user, svc_group) = util::users::get_user_and_group(&package)?;
        let mut env = match package.runtime_environment() {
            Ok(r) => r,
            Err(e) => return Err(sup_error!(Error::HabitatCore(e))),
        };

        // FIXME: Devise a way to make OS independent so we don't have to muck with env.
        Self::run_path(&mut env)?;

        Ok(RuntimeConfig::new(svc_user, svc_group, env))
    }

    pub fn load(spec: ServiceSpec,
                gossip_listen: &GossipListenAddr,
                http_listen: &http_gateway::ListenAddr,
                manager_fs_cfg: Arc<manager::FsCfg>,
                organization: Option<&str>)
                -> Result<Service> {
        let mut ui = UI::default();
        let package = match PackageInstall::load(&spec.ident, Some(&Path::new(&*FS_ROOT_PATH))) {
            Ok(package) => {
                match spec.update_strategy {
                    UpdateStrategy::AtOnce => {
                        try!(util::pkg::maybe_install_newer(&mut ui, &spec, package))
                    }
                    UpdateStrategy::None | UpdateStrategy::Rolling => package,
                }
            }
            Err(_) => {
                outputln!("Package {} not found locally, installing from {}",
                          Yellow.bold().paint(spec.ident.to_string()),
                          &spec.depot_url);
                try!(util::pkg::install(&mut ui, &spec.depot_url, &spec.ident))
            }
        };
        Self::new(package,
                  spec,
                  gossip_listen,
                  http_listen,
                  manager_fs_cfg,
                  organization)
    }

    pub fn add(&self) -> Result<()> {
        outputln!("Adding {}",
                  Yellow.bold().paint(self.package().ident().to_string()));
        self.create_svc_path()?;
        Ok(())
    }

    /// Create the service path for this package.
    fn create_svc_path(&self) -> Result<()> {
        let (user, group) = try!(util::users::get_user_and_group(&self.package()));

        debug!("Creating svc paths");

        if let Err(e) = Self::create_dir_all(self.svc_path()) {
            outputln!("Can't create directory {}",
                      self.svc_path().to_str().unwrap());
            outputln!("If this service is running as non-root, you'll need to create \
                       {} and give the current user write access to it",
                      self.svc_path().to_str().unwrap());
            return Err(e);
        }

        try!(Self::create_dir_all(self.svc_config_path()));
        try!(set_owner(self.svc_config_path(), &user, &group));
        try!(set_permissions(self.svc_config_path(), 0o700));
        try!(Self::create_dir_all(self.svc_data_path()));
        try!(set_owner(self.svc_data_path(), &user, &group));
        try!(set_permissions(self.svc_data_path(), 0o700));
        try!(Self::create_dir_all(self.svc_files_path()));
        try!(set_owner(self.svc_files_path(), &user, &group));
        try!(set_permissions(self.svc_files_path(), 0o700));
        try!(Self::create_dir_all(self.svc_hooks_path()));
        try!(Self::create_dir_all(self.svc_var_path()));
        try!(set_owner(self.svc_var_path(), &user, &group));
        try!(set_permissions(self.svc_var_path(), 0o700));
        try!(Self::remove_symlink(self.svc_static_path()));
        try!(Self::create_dir_all(self.svc_static_path()));
        try!(set_owner(self.svc_static_path(), &user, &group));
        try!(set_permissions(self.svc_static_path(), 0o700));
        try!(Self::create_dir_all(self.svc_logs_path()));
        // TODO: Not 100% if this directory is still needed, but for the moment it's still here -
        // FIN
        try!(Self::create_dir_all(self.svc_path().join("toml")));
        try!(set_permissions(self.svc_path().join("toml"), 0o700));
        Ok(())
    }

    fn start(&mut self) {
        if let Some(err) = self.supervisor.start().err() {
            outputln!(preamble self.service_group, "Service start failed: {}", err);
        } else {
            self.needs_reload = false;
            self.needs_reconfiguration = false;
        }
    }

    pub fn stop(&mut self) {
        if let Err(err) = self.supervisor.stop() {
            outputln!(preamble self.service_group, "Service stop failed: {}", err);
        }
    }

    fn reload(&mut self) {
        self.needs_reload = false;
        if self.is_down() || self.hooks.reload.is_none() {
            if let Some(err) = self.supervisor.restart().err() {
                outputln!(preamble self.service_group, "Service restart failed: {}", err);
            }
        } else {
            let hook = self.hooks.reload.as_ref().unwrap();
            hook.run(&self.service_group, self.runtime_cfg());
        }
    }

    pub fn down(&mut self) -> Result<()> {
        self.supervisor.down()
    }

    pub fn send_signal(&self, signal: u32) -> Result<()> {
        match self.supervisor.child {
            Some(ref child) => signals::send_signal(child.id(), signal),
            None => {
                debug!("No process to send the signal to");
                Ok(())
            }
        }
    }

    fn is_down(&self) -> bool {
        self.supervisor.child.is_none()
    }

    /// Instructs the service's process supervisor to reap dead children.
    fn check_process(&mut self) {
        self.supervisor.check_process()
    }

    pub fn tick(&mut self,
                butterfly: &butterfly::Server,
                census_list: &CensusList,
                census_updated: bool,
                last_census_update: &mut CensusUpdate) {
        if !self.initialized {
            if !self.is_bindings_present(census_list) {
                outputln!(preamble self.service_group, "Waiting to initialize service.");
                return;
            }
        }
        self.update_configuration(butterfly, census_list, census_updated, last_census_update);

        match self.topology {
            Topology::Standalone => {
                self.execute_hooks();
            }
            Topology::Leader => {
                let census = census_list
                    .get(&*self.service_group)
                    .expect("Service Group's census entry missing from list!");
                let me = census
                    .me()
                    .expect("Census corrupt, service can't find 'me'");
                let current_election_status = me.get_election_status();
                match current_election_status {
                    ElectionStatus::None => {
                        if self.last_election_status != current_election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; {}",
                                      Yellow.bold().paint("election hasn't started"));
                            self.last_election_status = current_election_status;
                        }
                    }
                    ElectionStatus::ElectionInProgress => {
                        if self.last_election_status != current_election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; {}",
                                      Yellow.bold().paint("election in progress."));
                            self.last_election_status = current_election_status;
                        }
                    }
                    ElectionStatus::ElectionNoQuorum => {
                        if self.last_election_status != current_election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; {}, {}.",
                                      Yellow.bold().paint("election in progress"),
                                      Red.bold().paint("and we have no quorum"));
                            self.last_election_status = current_election_status
                        }
                    }
                    ElectionStatus::ElectionFinished => {
                        let leader_id = census
                            .get_leader()
                            .expect("No leader with finished election")
                            .get_member_id();
                        if self.last_election_status != current_election_status {
                            outputln!(preamble self.service_group,
                                      "Executing hooks; {} is the leader",
                                      Green.bold().paint(leader_id));
                            self.last_election_status = current_election_status;
                        }
                        self.execute_hooks()
                    }
                }
            }
        }
    }

    pub fn to_spec(&self) -> ServiceSpec {
        let mut spec = ServiceSpec::default_for(self.spec_ident.clone());
        spec.group = self.service_group.group().to_string();
        spec.depot_url = self.depot_url.clone();
        spec.topology = self.topology;
        spec.update_strategy = self.update_strategy;
        spec.binds = self.spec_binds.clone();
        spec.start_style = self.start_style;
        spec.config_from = self.config_from.clone();
        spec
    }

    fn is_bindings_present(&self, census_list: &CensusList) -> bool {
        let mut ret = true;
        for ref bind in self.spec_binds.iter() {
            if census_list.get(&*bind.service_group).is_none() {
                ret = false;
                outputln!(preamble self.service_group,
                          "The specified service group '{}' for binding '{}' is not (yet?) present in the census data.",
                          Green.bold().paint(format!("{}", bind.service_group)), Green.bold().paint(format!("{}", bind.name)));
            }
        }
        ret
    }

    fn update_configuration(&mut self,
                            butterfly: &butterfly::Server,
                            census_list: &CensusList,
                            census_updated: bool,
                            last_census_update: &mut CensusUpdate) {
        self.config.populate(&self.service_group, census_list);
        self.persist_service_files(butterfly);

        let svc_cfg_updated = self.persist_service_config(butterfly);
        if svc_cfg_updated || census_updated {
            if svc_cfg_updated {
                self.update_service_rumor_cfg(butterfly, last_census_update);
                if let Some(err) = self.config.reload_gossip().err() {
                    outputln!(preamble self.service_group, "error loading gossip config, {}", err);
                }
            }
            match self.config.write() {
                Ok(true) => {
                    self.needs_reconfiguration = true;
                    self.hooks.compile(&self.service_group, &self.config);
                    if let Some(err) = self.copy_run().err() {
                        outputln!(preamble self.service_group, "Failed to copy run hook: {}", err);
                    }
                }
                Ok(false) => (),
                Err(e) => {
                    outputln!(preamble self.service_group,
                              "Failed to write service configuration: {}",
                              e);
                }
            }
        }
    }

    pub fn package(&self) -> RwLockReadGuard<PackageInstall> {
        self.package.read().expect("Package lock poisoned")
    }

    pub fn update_package(&mut self, package: PackageInstall) {
        let runtime_cfg = match Self::runtime_config_from(&package) {
            Ok(c) => c,
            Err(err) => {
                outputln!(preamble self.service_group,
                          "Unable to extract svc_user, svc_group, and env_vars \
                          from updated package, {}", err);
                return;
            }
        };
        let config_root = self.config_from
            .clone()
            .unwrap_or(package.installed_path.clone());
        let hooks_path = fs::svc_hooks_path(self.service_group.service());
        self.hooks = HookTable::default().load_hooks(&self.service_group,
                                                     hooks_path,
                                                     &config_root.join("hooks"));

        if let Some(err) = self.config
               .reload_package(&package, config_root, &runtime_cfg)
               .err() {
            outputln!(preamble self.service_group,
                "Failed to reload service config with updated package: {}", err);
        }
        *self.package.write().expect("Package lock poisoned") = package;

        if let Err(err) = self.supervisor.down() {
            outputln!(preamble self.service_group,
                      "Error stopping process while updating package: {}", err);
        }
        self.initialized = false;
    }

    pub fn to_rumor<T: ToString>(&self, member_id: T) -> ServiceRumor {
        let exported = match self.config.to_exported() {
            Ok(exported) => Some(exported),
            Err(err) => {
                outputln!(preamble self.service_group,
                          "Failed to generate exported cfg for service rumor: {}",
                          Red.bold().paint(format!("{}", err)));
                None
            }
        };
        ServiceRumor::new(member_id.to_string(),
                          &self.package().ident,
                          &self.service_group,
                          &*self.config.sys,
                          exported.as_ref())
    }

    /// Run initialization hook if present
    fn initialize(&mut self) {
        if self.initialized {
            return;
        }
        outputln!(preamble self.service_group, "Initializing");
        self.hooks.compile(&self.service_group, &self.config);
        if let Some(err) = self.copy_run().err() {
            outputln!(preamble self.service_group, "Failed to copy run hook: {}", err);
        }

        self.initialized = true;
        if let Some(ref hook) = self.hooks.init {
            self.initialized = hook.run(&self.service_group, self.runtime_cfg())
        }
    }

    pub fn populate(&mut self, census: &CensusList) {
        self.config.populate(&self.service_group, census)
    }

    /// Run reconfigure hook if present. Return false if it is not present, to trigger default
    /// restart behavior.
    fn reconfigure(&mut self) {
        self.needs_reconfiguration = false;
        if let Some(ref hook) = self.hooks.reconfigure {
            hook.run(&self.service_group, self.runtime_cfg());
        }
    }

    fn post_run(&mut self) {
        if let Some(ref hook) = self.hooks.post_run {
            hook.run(&self.service_group, self.runtime_cfg());
        }
    }

    /// Modifies PATH env with the full run path for this package. This path is composed of any
    /// binary paths specified by this package, or its TDEPS, plus a path to a BusyBox(non-windows),
    /// plus the existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can invoke the Supervisor,
    /// without having to worry much about context.
    fn run_path(run_env: &mut HashMap<String, String>) -> Result<()> {
        let path_key = "PATH".to_string();
        let mut paths: Vec<PathBuf> = match run_env.get(&path_key) {
            Some(path) => env::split_paths(&path).collect(),
            None => vec![],
        };

        // Lets join the run paths to the FS_ROOT
        // In most cases, this does nothing and should only mutate
        // the paths in a windows studio where FS_ROOT_PATH will
        // be the studio root path (ie c:\hab\studios\...). In any other
        // environment FS_ROOT will be "/" and this will not make any
        // meaningful change.
        for i in 0..paths.len() {
            if paths[i].starts_with("/") {
                paths[i] = Path::new(&*FS_ROOT_PATH).join(paths[i].strip_prefix("/").unwrap());
            }
        }
        run_env.insert(path_key,
                       util::path::append_interpreter_and_path(&mut paths)?);
        Ok(())
    }

    fn runtime_cfg(&self) -> &RuntimeConfig {
        &self.supervisor.runtime_config
    }

    pub fn suitability(&self) -> Option<u64> {
        if !self.initialized {
            return None;
        }
        self.hooks
            .suitability
            .as_ref()
            .and_then(|hook| hook.run(&self.service_group, self.runtime_cfg()))
    }


    /// Returns the root path for service configuration, files, and data.
    fn svc_path(&self) -> PathBuf {
        fs::svc_path(&self.service_group.service())
    }

    /// Returns the path to the service configuration.
    fn svc_config_path(&self) -> PathBuf {
        fs::svc_config_path(&self.service_group.service())
    }

    /// Returns the path to the service data.
    fn svc_data_path(&self) -> PathBuf {
        fs::svc_data_path(&self.service_group.service())
    }

    /// Returns the path to the service's gossiped config files.
    fn svc_files_path(&self) -> PathBuf {
        fs::svc_files_path(&self.service_group.service())
    }

    /// Returns the path to the service hooks.
    fn svc_hooks_path(&self) -> PathBuf {
        fs::svc_hooks_path(&self.service_group.service())
    }

    /// Returns the path to the service static content.
    fn svc_static_path(&self) -> PathBuf {
        fs::svc_static_path(&self.service_group.service())
    }

    /// Returns the path to the service variable state.
    fn svc_var_path(&self) -> PathBuf {
        fs::svc_var_path(&self.service_group.service())
    }

    /// Returns the path to the service logs.
    fn svc_logs_path(&self) -> PathBuf {
        fs::svc_logs_path(&self.service_group.service())
    }

    /// this function wraps create_dir_all so we can give friendly error
    /// messages to the user.
    fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
        debug!("Creating dir with subdirs: {:?}", &path.as_ref());
        if let Err(e) = std::fs::create_dir_all(&path) {
            Err(sup_error!(Error::Permissions(format!("Can't create {:?}, {}", &path.as_ref(), e))))
        } else {
            Ok(())
        }
    }

    fn cache_health_check(&self, check_result: HealthCheck) {
        let state_file = self.manager_fs_cfg
            .data_path
            .join(format!("{}.health", self.service_group.service()));
        let tmp_file = state_file.with_extension("tmp");
        let file = match File::create(&tmp_file) {
            Ok(file) => file,
            Err(err) => {
                warn!("Couldn't open temporary health check file, {}, {}",
                      self.service_group,
                      err);
                return;
            }
        };
        let mut writer = BufWriter::new(file);
        if let Some(err) = writer
               .write_all((check_result as i8).to_string().as_bytes())
               .err() {
            warn!("Couldn't write to temporary health check state file, {}, {}",
                  self.service_group,
                  err);
        }
        if let Some(err) = std::fs::rename(&tmp_file, &state_file).err() {
            warn!("Couldn't finalize health check state file, {}, {}",
                  self.service_group,
                  err);
        }
    }

    // Copy the "run" file to the svc path.
    fn copy_run(&self) -> Result<()> {
        let svc_run = self.svc_path().join(hooks::RunHook::file_name());
        match self.hooks.run {
            Some(ref hook) => {
                try!(std::fs::copy(hook.path(), &svc_run));
                try!(set_permissions(&svc_run.to_str().unwrap(), HOOK_PERMISSIONS));
            }
            None => {
                let run = self.package()
                    .installed_path()
                    .join(hooks::RunHook::file_name());
                match std::fs::metadata(&run) {
                    Ok(_) => {
                        try!(std::fs::copy(&run, &svc_run));
                        try!(set_permissions(&svc_run, HOOK_PERMISSIONS));
                    }
                    Err(err) => {
                        outputln!(preamble self.service_group, "Error finding run file: {}", err);
                    }
                }
            }
        }
        Ok(())
    }

    fn execute_hooks(&mut self) {
        if !self.initialized {
            self.initialize();
            if self.initialized {
                self.start();
                self.post_run();
            }
        } else {
            self.check_process();
            if Instant::now().duration_since(self.last_health_check) >= *HEALTH_CHECK_INTERVAL {
                self.run_health_check_hook();
            }

            if self.needs_reload || self.is_down() || self.needs_reconfiguration {
                self.reload();
                if self.needs_reconfiguration {
                    self.reconfigure()
                }
            }
        }
    }

    /// Run file_updated hook if present
    fn file_updated(&self) -> bool {
        if self.initialized {
            if let Some(ref hook) = self.hooks.file_updated {
                return hook.run(&self.service_group, self.runtime_cfg());
            }
        }
        false
    }

    /// Write service configuration from gossip data to disk.
    ///
    /// Returns true if a change was made and false if there were no updates.
    fn persist_service_config(&mut self, butterfly: &butterfly::Server) -> bool {
        if let Some((incarnation, config)) =
            butterfly.service_config_for(&*self.service_group, Some(self.config.incarnation)) {
            self.config.incarnation = incarnation;
            self.write_butterfly_service_config(config)
        } else {
            false
        }
    }

    /// Write service files from gossip data to disk.
    ///
    /// Returnst rue if a file was changed, added, or removed, and false if there were no updates.
    fn persist_service_files(&mut self, butterfly: &butterfly::Server) -> bool {
        let mut updated = false;
        for (incarnation, filename, body) in
            butterfly
                .service_files_for(&*self.service_group, &self.current_service_files)
                .into_iter() {
            if self.write_butterfly_service_file(filename, incarnation, body) {
                updated = true;
            }
        }
        if updated { self.file_updated() } else { false }
    }

    /// attempt to remove a symlink in the /svc/run/foo/ directory if
    /// the link exists.
    fn remove_symlink<P: AsRef<Path>>(p: P) -> Result<()> {
        let p = p.as_ref();
        if !p.exists() {
            return Ok(());
        }
        // note: we're NOT using p.metadata() here as that will follow the
        // symlink, which returns smd.file_type().is_symlink() == false in all cases.
        let smd = try!(p.symlink_metadata());
        if smd.file_type().is_symlink() {
            try!(std::fs::remove_file(p));
        }
        Ok(())
    }

    fn run_health_check_hook(&mut self) {
        let check_result = if let Some(ref hook) = self.hooks.health_check {
            hook.run(&self.service_group, self.runtime_cfg())
        } else {
            match self.supervisor.status() {
                (true, _) => HealthCheck::Ok,
                (false, _) => HealthCheck::Critical,
            }
        };
        self.last_health_check = Instant::now();
        self.cache_health_check(check_result);
    }

    /// Update our own service rumor with a new configuration from the packages exported
    /// configuration data.
    ///
    /// The run loop's last updated census is a required parameter on this function to inform the
    /// main loop that we, ourselves, updated the service counter when we updated ourselves.
    fn update_service_rumor_cfg(&self,
                                butterfly: &butterfly::Server,
                                last_update: &mut CensusUpdate) {
        if let Some(cfg) = self.config.to_exported().ok() {
            let me = butterfly.member_id().to_string();
            let mut updated = None;
            butterfly
                .service_store
                .with_rumor(&*self.service_group, &me, |rumor| {
                    if let Some(rumor) = rumor {
                        let mut rumor = rumor.clone();
                        let incarnation = rumor.get_incarnation() + 1;
                        rumor.set_incarnation(incarnation);
                        // TODO FN: the updated toml API returns a `Result` when
                        // serializing--we should handle this and not potentially panic
                        *rumor.mut_cfg() =
                            toml::ser::to_vec(&cfg).expect("Can't serialize to TOML bytes");
                        updated = Some(rumor);
                    }
                });
            if let Some(rumor) = updated {
                butterfly.insert_service(rumor);
                last_update.service_counter += 1;
            }
        }
    }

    fn write_butterfly_service_file(&mut self,
                                    filename: String,
                                    incarnation: u64,
                                    body: Vec<u8>)
                                    -> bool {
        self.current_service_files
            .insert(filename.clone(), incarnation);
        let on_disk_path = self.svc_files_path().join(filename);
        let current_checksum = match hash::hash_file(&on_disk_path) {
            Ok(current_checksum) => current_checksum,
            Err(e) => {
                debug!("Failed to get current checksum for {:?}: {}",
                       on_disk_path,
                       e);
                String::new()
            }
        };
        let new_checksum = hash::hash_bytes(&body)
            .expect("We failed to hash a Vec<u8> in a method that can't return an error; not \
                     even sure what this means");
        if new_checksum != current_checksum {
            let new_filename = format!("{}.write", on_disk_path.to_string_lossy());

            let mut new_file = match File::create(&new_filename) {
                Ok(new_file) => new_file,
                Err(e) => {
                    outputln!(preamble self.service_group,
                                  "Service file from butterfly failed to open the new file {}: {}",
                                  new_filename,
                                  Red.bold().paint(format!("{}", e)));
                    return false;
                }
            };

            if let Err(e) = new_file.write_all(&body) {
                outputln!(preamble self.service_group,
                              "Service file from butterfly failed to write {}: {}",
                              new_filename,
                              Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = std::fs::rename(&new_filename, &on_disk_path) {
                outputln!(preamble self.service_group,
                              "Service file from butterfly failed to rename {} to {}: {}",
                              new_filename,
                              on_disk_path.to_string_lossy(),
                              Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_owner(&on_disk_path,
                                      &self.supervisor.runtime_config.svc_user,
                                      &self.supervisor.runtime_config.svc_group) {
                outputln!(preamble self.service_group,
                              "Service file from butterfly failed to set ownership on {}: {}",
                              on_disk_path.to_string_lossy(),
                              Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_permissions(&on_disk_path, 0o640) {
                outputln!(preamble self.service_group,
                              "Service file from butterfly failed to set permissions on {}: {}",
                              on_disk_path.to_string_lossy(),
                              Red.bold().paint(format!("{}", e)));
                return false;
            }

            outputln!(preamble self.service_group,
                          "Service file updated from butterfly {}: {}",
                          on_disk_path.to_string_lossy(),
                          Green.bold().paint(new_checksum));
            true
        } else {
            false
        }
    }

    fn write_butterfly_service_config(&mut self, config: toml::Value) -> bool {
        let encoded = toml::ser::to_string(&config)
            .expect("Failed to serialize service configuration to a string in a method that \
                     can't return an error; this could be made better");
        let on_disk_path = self.svc_path().join("gossip.toml");
        let current_checksum = match hash::hash_file(&on_disk_path) {
            Ok(current_checksum) => current_checksum,
            Err(e) => {
                debug!("Failed to get current checksum for {:?}: {}",
                       on_disk_path,
                       e);
                String::new()
            }
        };
        let new_checksum = hash::hash_string(&encoded)
            .expect("We failed to hash a string in a method that can't return an error; not even \
                     sure what this means");
        if new_checksum != current_checksum {
            let new_filename = format!("{}.write", on_disk_path.to_string_lossy());

            let mut new_file = match File::create(&new_filename) {
                Ok(new_file) => new_file,
                Err(e) => {
                    outputln!(preamble self.service_group,
                              "Service configuration from butterfly failed to open the new file: \
                               {}",
                              Red.bold().paint(format!("{}", e)));
                    return false;
                }
            };

            if let Err(e) = new_file.write_all(encoded.as_bytes()) {
                outputln!(preamble self.service_group,
                          "Service configuration from butterfly failed to write: {}",
                          Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = std::fs::rename(&new_filename, &on_disk_path) {
                outputln!(preamble self.service_group,
                          "Service configuration from butterfly failed to rename: {}",
                          Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_owner(&on_disk_path,
                                      &self.supervisor.runtime_config.svc_user,
                                      &self.supervisor.runtime_config.svc_group) {
                outputln!(preamble self.service_group,
                          "Service configuration from butterfly failed to set ownership: {}",
                          Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_permissions(&on_disk_path, 0o640) {
                outputln!(preamble self.service_group,
                          "Service configuration from butterfly failed to set permissions: {}",
                          Red.bold().paint(format!("{}", e)));
                return false;
            }

            outputln!(preamble self.service_group,
                      "Service configuration updated from butterfly: {}",
                      Green.bold().paint(new_checksum));
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.package().to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Topology {
    Standalone,
    Leader,
}

impl Topology {
    fn as_str(&self) -> &str {
        match *self {
            Topology::Leader => "leader",
            Topology::Standalone => "standalone",
        }
    }
}

impl FromStr for Topology {
    type Err = SupError;

    fn from_str(topology: &str) -> result::Result<Self, Self::Err> {
        match topology {
            "leader" => Ok(Topology::Leader),
            "standalone" => Ok(Topology::Standalone),
            _ => Err(sup_error!(Error::InvalidTopology(String::from(topology)))),
        }
    }
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Topology {
    fn default() -> Topology {
        Topology::Standalone
    }
}

impl serde::Deserialize for Topology {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: serde::Deserializer
    {
        deserialize_using_from_str(deserializer)
    }
}

impl serde::Serialize for Topology {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UpdateStrategy {
    None,
    AtOnce,
    Rolling,
}

impl UpdateStrategy {
    fn as_str(&self) -> &str {
        match *self {
            UpdateStrategy::None => "none",
            UpdateStrategy::AtOnce => "at-once",
            UpdateStrategy::Rolling => "rolling",
        }
    }
}

impl FromStr for UpdateStrategy {
    type Err = SupError;

    fn from_str(strategy: &str) -> result::Result<Self, Self::Err> {
        match strategy {
            "none" => Ok(UpdateStrategy::None),
            "at-once" => Ok(UpdateStrategy::AtOnce),
            "rolling" => Ok(UpdateStrategy::Rolling),
            _ => Err(sup_error!(Error::InvalidUpdateStrategy(String::from(strategy)))),
        }
    }
}

impl fmt::Display for UpdateStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for UpdateStrategy {
    fn default() -> UpdateStrategy {
        UpdateStrategy::None
    }
}

impl serde::Deserialize for UpdateStrategy {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: serde::Deserializer
    {
        deserialize_using_from_str(deserializer)
    }
}

impl serde::Serialize for UpdateStrategy {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}

fn serialize_lock<S>(x: &Arc<RwLock<PackageInstall>>, s: S) -> result::Result<S::Ok, S::Error>
    where S: serde::Serializer
{
    s.serialize_str(&x.read().expect("Package lock poisoned").to_string())
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use toml;

    use super::{Topology, UpdateStrategy};
    use error::Error::*;

    #[test]
    fn topology_default() {
        // This should always be the default topology, if this default gets changed, we have
        // a failing test to confirm we changed our minds
        assert_eq!(Topology::default(), Topology::Standalone);
    }

    #[test]
    fn topology_from_str() {
        let topology_str = "leader";
        let topology = Topology::from_str(topology_str).unwrap();

        assert_eq!(topology, Topology::Leader);
    }

    #[test]
    fn topology_from_str_invalid() {
        let topology_str = "dope";

        match Topology::from_str(topology_str) {
            Err(e) => {
                match e.err {
                    InvalidTopology(s) => assert_eq!("dope", s),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),

        }
    }

    #[test]
    fn topology_to_string() {
        let topology = Topology::Standalone;

        assert_eq!("standalone", topology.to_string())
    }

    #[test]
    fn topology_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: Topology,
        }
        let toml = r#"
            key = "leader"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.key, Topology::Leader);
    }

    #[test]
    fn topology_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: Topology,
        }
        let data = Data { key: Topology::Leader };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "leader""#))
    }

    #[test]
    fn update_strategy_default() {
        // This should always be the default update strategy, if this default gets changed, we have
        // a failing test to confirm we changed our minds
        assert_eq!(UpdateStrategy::default(), UpdateStrategy::None);
    }

    #[test]
    fn update_strategy_from_str() {
        let strategy_str = "at-once";
        let strategy = UpdateStrategy::from_str(strategy_str).unwrap();

        assert_eq!(strategy, UpdateStrategy::AtOnce);
    }

    #[test]
    fn update_strategy_from_str_invalid() {
        let strategy_str = "dope";

        match UpdateStrategy::from_str(strategy_str) {
            Err(e) => {
                match e.err {
                    InvalidUpdateStrategy(s) => assert_eq!("dope", s),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),

        }
    }

    #[test]
    fn update_strategy_to_string() {
        let strategy = UpdateStrategy::AtOnce;

        assert_eq!("at-once", strategy.to_string())
    }

    #[test]
    fn update_strategy_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: UpdateStrategy,
        }
        let toml = r#"
            key = "at-once"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.key, UpdateStrategy::AtOnce);
    }

    #[test]
    fn update_strategy_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: UpdateStrategy,
        }
        let data = Data { key: UpdateStrategy::AtOnce };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "at-once""#));
    }
}
