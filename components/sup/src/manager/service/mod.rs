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

pub mod config;
pub mod health;
pub mod hooks;

use std;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use ansi_term::Colour::{Yellow, Red, Green};
use butterfly;
use butterfly::rumor::service::Service as ServiceRumor;
use common::ui::UI;
use hcore::fs::FS_ROOT_PATH;
use hcore::service::ServiceGroup;
use hcore::crypto::hash;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::url::DEFAULT_DEPOT_URL;
use hcore::util::perm::{set_owner, set_permissions};
use serde::Serializer;
use toml;

use self::hooks::{HOOK_PERMISSIONS, HookTable, HookType};
use config::GossipListenAddr;
use error::{Error, Result, SupError};
use http_gateway;
use fs;
use manager::signals;
use manager::census::{CensusList, CensusUpdate, ElectionStatus};
use prometheus::Opts;
use supervisor::{Supervisor, RuntimeConfig};
use util;

pub use self::config::ServiceConfig;
pub use self::health::{HealthCheck, SmokeCheck};

static LOGKEY: &'static str = "SR";
static DEFAULT_GROUP: &'static str = "default";
const HABITAT_PACKAGE_INFO_NAME: &'static str = "habitat_package_info";
const HABITAT_PACKAGE_INFO_DESC: &'static str = "package version information";

#[derive(Debug)]
pub struct ServiceSpec {
    pub ident: PackageIdent,
    pub group: String,
    pub organization: Option<String>,
    pub depot_url: String,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    pub binds: Vec<(String, ServiceGroup)>,
    pub config_from: Option<PathBuf>,
}

impl ServiceSpec {
    pub fn default_for(ident: PackageIdent) -> Self {
        ServiceSpec {
            ident: ident,
            group: DEFAULT_GROUP.to_string(),
            organization: None,
            depot_url: DEFAULT_DEPOT_URL.to_string(),
            topology: Topology::default(),
            update_strategy: UpdateStrategy::default(),
            binds: vec![],
            config_from: None,
        }
    }

    pub fn split_bindings<'a, T>(bindings: T) -> Result<Vec<(String, ServiceGroup)>>
        where T: Iterator<Item = &'a str>
    {
        let mut bresult = Vec::new();
        for bind in bindings {
            let values: Vec<&str> = bind.splitn(2, ':').collect();
            if values.len() != 2 {
                return Err(sup_error!(Error::InvalidBinding(bind.to_string())));
            } else {
                bresult.push((values[0].to_string(), ServiceGroup::from_str(values[1])?));
            }
        }
        Ok(bresult)
    }

    pub fn validate(&self, package: &PackageInstall) -> Result<()> {
        self.validate_binds(package)?;
        Ok(())
    }

    fn validate_binds(&self, package: &PackageInstall) -> Result<()> {
        let missing: Vec<String> = package.binds()?
            .into_iter()
            .filter(|bind| {
                self.binds.iter().find(|&&(ref service, _)| &bind.service == service).is_none()
            })
            .map(|bind| bind.service)
            .collect();
        if !missing.is_empty() {
            return Err(sup_error!(Error::MissingRequiredBind(missing)));
        }
        Ok(())
    }
}

fn serialize_lock<S>(x: &Arc<RwLock<PackageInstall>>,
                     s: &mut S)
                     -> std::result::Result<(), S::Error>
    where S: Serializer
{
    s.serialize_str(&x.read().expect("Package lock poisoned").to_string())
}

#[derive(Debug, Serialize)]
pub struct Service {
    pub config: ServiceConfig,
    pub current_service_files: HashMap<String, u64>,
    pub depot_url: String,
    pub health_check: HealthCheck,
    pub initialized: bool,
    pub last_election_status: ElectionStatus,
    pub needs_restart: bool,
    pub needs_reconfiguration: bool,
    #[serde(serialize_with="serialize_lock")]
    pub package: Arc<RwLock<PackageInstall>>,
    pub service_group: ServiceGroup,
    pub smoke_check: SmokeCheck,
    pub spec_ident: PackageIdent,
    pub supervisor: Supervisor,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    hooks: HookTable,
    config_from: Option<PathBuf>,
}

impl Service {
    pub fn new(package: PackageInstall,
               spec: ServiceSpec,
               gossip_listen: &GossipListenAddr,
               http_listen: &http_gateway::ListenAddr)
               -> Result<Service> {
        spec.validate(&package)?;
        let service_group = ServiceGroup::new(&package.ident.name,
                                              spec.group,
                                              spec.organization.as_ref().map(|x| &**x))?;
        let (svc_user, svc_group) = util::users::get_user_and_group(&package)?;
        let runtime_cfg = RuntimeConfig::new(svc_user, svc_group);
        let config_root = spec.config_from.clone().unwrap_or(package.installed_path.clone());
        let svc_cfg = ServiceConfig::new(&package,
                                         &runtime_cfg,
                                         config_root,
                                         spec.binds,
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
            hooks: HookTable::default().load_hooks(&runtime_cfg, hooks_path, hook_template_path),
            initialized: false,
            last_election_status: ElectionStatus::None,
            needs_restart: false,
            needs_reconfiguration: false,
            supervisor: Supervisor::new(locked_package.clone(), &service_group, runtime_cfg),
            package: locked_package,
            service_group: service_group,
            smoke_check: SmokeCheck::default(),
            spec_ident: spec.ident,
            topology: spec.topology,
            update_strategy: spec.update_strategy,
            config_from: spec.config_from,
        })
    }

    pub fn load(spec: ServiceSpec,
                gossip_listen: &GossipListenAddr,
                http_listen: &http_gateway::ListenAddr)
                -> Result<Service> {
        let mut ui = UI::default();
        let package = match PackageInstall::load(&spec.ident, Some(&Path::new(&*FS_ROOT_PATH))) {
            Ok(package) => {
                match spec.update_strategy {
                    UpdateStrategy::AtOnce | UpdateStrategy::Rolling => {
                        try!(util::pkg::maybe_install_newer(&mut ui, &spec, package))
                    }
                    UpdateStrategy::None => package,
                }
            }
            Err(_) => {
                outputln!("Package {} not found locally, installing from {}",
                          Yellow.bold().paint(spec.ident.to_string()),
                          &spec.depot_url);
                try!(util::pkg::install(&mut ui, &spec.depot_url, &spec.ident))
            }
        };
        Self::new(package, spec, gossip_listen, http_listen)
    }

    pub fn add(&self) -> Result<()> {
        outputln!("Adding {}",
                  Yellow.bold().paint(self.package().ident().to_string()));
        // JW (and now also FN) TODO: We can't just set the run path for the entire process here,
        // we need to set this on the supervisor which will be starting the process itself to
        // support multi services in hab-sup.
        let run_path = self.run_path()?;
        debug!("Setting the PATH to {}", run_path);
        env::set_var("PATH", &run_path);

        self.create_svc_path()?;
        self.register_metrics();
        Ok(())
    }

    pub fn remove(&self) -> Result<()> {
        outputln!("Finished with {}",
                  Yellow.bold().paint(self.package().ident().to_string()));
        Ok(())
    }

    pub fn config_root(&self) -> &Path {
        &*self.config.config_root
    }

    /// Create the service path for this package.
    pub fn create_svc_path(&self) -> Result<()> {
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
        // TODO: Not 100% if this directory is still needed, but for the moment it's still here -
        // FIN
        try!(Self::create_dir_all(self.svc_path().join("toml")));
        try!(set_permissions(self.svc_path().join("toml"), 0o700));
        Ok(())
    }

    pub fn start(&mut self) {
        if let Some(err) = self.supervisor.start().err() {
            outputln!(preamble self.service_group, "Service start failed: {}", err);
        } else {
            self.needs_restart = false;
            self.needs_reconfiguration = false;
        }
    }

    pub fn restart(&mut self) {
        if let Some(err) = self.supervisor.restart().err() {
            outputln!(preamble self.service_group, "Service restart failed: {}", err);
        } else {
            self.needs_restart = false;
        }
    }

    pub fn down(&mut self) -> Result<()> {
        self.supervisor.down()
    }

    pub fn last_config(&self) -> Result<String> {
        let mut file = try!(File::open(self.svc_path().join("config.toml")));
        let mut result = String::new();
        try!(file.read_to_string(&mut result));
        Ok(result)
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

    pub fn is_down(&self) -> bool {
        self.supervisor.child.is_none()
    }

    /// Instructs the service's process supervisor to reap dead children.
    pub fn check_process(&mut self) {
        self.supervisor.check_process()
    }

    pub fn register_metrics(&self) {
        let version_opts = Opts::new(HABITAT_PACKAGE_INFO_NAME, HABITAT_PACKAGE_INFO_DESC)
            .const_label("origin", &self.package().ident.origin.clone())
            .const_label("name", &self.package().ident.name.clone())
            .const_label("version",
                         &self.package().ident.version.as_ref().unwrap().clone())
            .const_label("release",
                         &self.package().ident.release.as_ref().unwrap().clone());
        let version_gauge = register_gauge!(version_opts).unwrap();
        version_gauge.set(1.0);
    }

    pub fn tick(&mut self,
                butterfly: &butterfly::Server,
                census_list: &CensusList,
                census_updated: bool,
                last_census_update: &mut CensusUpdate) {

        self.update_configuration(butterfly, census_list, census_updated, last_census_update);

        match self.topology {
            Topology::Standalone => {
                self.execute_hooks();
            }
            Topology::Leader => {
                let census = census_list.get(&*self.service_group)
                    .expect("Service Group's census entry missing from list!");
                let me = census.me().expect("Census corrupt, service can't find 'me'");
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
                        let leader_id = census.get_leader()
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

    fn execute_hooks(&mut self) {
        if !self.initialized {
            self.initialize();
            if self.initialized {
                self.start();
            }
        } else {
            self.check_process();

            if self.needs_restart || self.is_down() || self.needs_reconfiguration {
                self.restart();
                if self.needs_reconfiguration {
                    self.reconfigure()
                }
            }
        }
    }

    pub fn package(&self) -> RwLockReadGuard<PackageInstall> {
        self.package.read().expect("Package lock poisoned")
    }

    pub fn update_package(&mut self, package: PackageInstall) {
        let (svc_user, svc_group) = match util::users::get_user_and_group(&package) {
            Ok(user_and_group) => user_and_group,
            Err(err) => {
                outputln!(preamble self.service_group, "Unable to extract svc_user and svc_group from updated package, {}", err);
                return;
            }
        };

        let runtime_cfg = RuntimeConfig::new(svc_user, svc_group);
        let config_root = self.config_from.clone().unwrap_or(package.installed_path.clone());
        let hooks_path = fs::svc_hooks_path(self.service_group.service());
        self.hooks = HookTable::default()
            .load_hooks(&runtime_cfg, hooks_path, &config_root.join("hooks"));

        if let Some(err) = self.config.reload_package(&package, config_root, &runtime_cfg).err() {
            outputln!(preamble self.service_group, "Failed to reload service config with updated package: {}", err);
        }
        *self.package.write().expect("Package lock poisoned") = package;

        self.initialized = false;
        self.initialize();
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

    pub fn health_check(&self) -> HealthCheck {
        if self.hooks.health_check.is_some() {
            // JW TODO: This should leverage `run_hook()` directly but we would need to give a
            // mutable reference to the http-gateway to allow it to run this hook. We don't want
            // to allow the http-gateway to obtain a lock on any of the manager's memory, including
            // the service list, so we need to temporarily ensure that this hook never attempts
            // to compile on run.
            //
            // In the near future, we will periodically run this hook and cache it's results on
            // the service struct itself.
            match self.hooks.try_run(HookType::HealthCheck, &self.service_group) {
                Ok(()) => HealthCheck::Ok,
                Err(SupError { err: Error::HookFailed(_, 1), .. }) => HealthCheck::Warning,
                Err(SupError { err: Error::HookFailed(_, 2), .. }) => HealthCheck::Critical,
                Err(SupError { err: Error::HookFailed(_, 3), .. }) => HealthCheck::Unknown,
                Err(SupError { err: Error::HookFailed(_, code), .. }) => {
                    outputln!(preamble self.service_group,
                        "Health check exited with an unknown status code, {}", code);
                    HealthCheck::Unknown
                }
                Err(err) => {
                    outputln!(preamble self.service_group, "Health check couldn't be run, {}", err);
                    HealthCheck::Unknown
                }
            }
        } else {
            match self.supervisor.status() {
                (true, _) => HealthCheck::Ok,
                (false, _) => HealthCheck::Critical,
            }
        }
    }

    /// Run initialization hook if present
    pub fn initialize(&mut self) {
        if self.initialized {
            return;
        }
        outputln!(preamble self.service_group, "Initializing");
        if let Some(err) = self.run_hook(HookType::Init).err() {
            outputln!(preamble self.service_group, "Initialization failed: {}", err);
            return;
        }
        if let Some(err) = self.copy_run().err() {
            outputln!(preamble self.service_group, "Failed to copy run hook: {}", err);
        }
        self.initialized = true;
    }

    pub fn populate(&mut self, census: &CensusList) {
        self.config.populate(&self.service_group, census)
    }

    /// Run reconfigure hook if present. Return false if it is not present, to trigger default
    /// restart behavior.
    pub fn reconfigure(&mut self) {
        self.needs_reconfiguration = false;
        if let Some(err) = self.run_hook(HookType::Reconfigure).err() {
            outputln!(preamble self.service_group,
                      "Reconfiguration hook failed: {}",
                      err);
        }
    }

    /// Returns a string with the full run path for this package. This path is composed of any
    /// binary paths specified by this package, or its TDEPS, plus a path to a BusyBox(non-windows),
    /// plus the existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can invoke the Supervisor,
    /// without having to worry much about context.
    pub fn run_path(&self) -> Result<String> {
        let mut paths = match self.package().runtime_path() {
            Ok(r) => env::split_paths(&r).collect::<Vec<PathBuf>>(),
            Err(e) => return Err(sup_error!(Error::HabitatCore(e))),
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

        util::path::append_interpreter_and_path(&mut paths)
    }

    pub fn smoke_test(&mut self) -> SmokeCheck {
        match self.run_hook(HookType::SmokeTest) {
            Ok(()) => SmokeCheck::Ok,
            Err(SupError { err: Error::HookFailed(_, code), .. }) => SmokeCheck::Failed(code),
            Err(err) => {
                outputln!(preamble self.service_group, "Smoke test couldn't be run, {}", err);
                SmokeCheck::Failed(-1)
            }
        }
    }

    /// Returns the root path for service configuration, files, and data.
    pub fn svc_path(&self) -> PathBuf {
        fs::svc_path(&self.service_group.service())
    }

    /// Returns the path to the service configuration.
    pub fn svc_config_path(&self) -> PathBuf {
        fs::svc_config_path(&self.service_group.service())
    }

    /// Returns the path to the service data.
    pub fn svc_data_path(&self) -> PathBuf {
        fs::svc_data_path(&self.service_group.service())
    }

    /// Returns the path to the service's gossiped config files.
    pub fn svc_files_path(&self) -> PathBuf {
        fs::svc_files_path(&self.service_group.service())
    }

    /// Returns the path to the service hooks.
    pub fn svc_hooks_path(&self) -> PathBuf {
        fs::svc_hooks_path(&self.service_group.service())
    }

    /// Returns the path to the service static content.
    pub fn svc_static_path(&self) -> PathBuf {
        fs::svc_static_path(&self.service_group.service())
    }

    /// Returns the path to the service variable state.
    pub fn svc_var_path(&self) -> PathBuf {
        fs::svc_var_path(&self.service_group.service())
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

    // Copy the "run" file to the svc path.
    fn copy_run(&self) -> Result<()> {
        let svc_run = self.svc_path().join(hooks::RUN_FILENAME);
        match self.hooks.run {
            Some(ref hook) => {
                try!(std::fs::copy(&hook.path, &svc_run));
                try!(set_permissions(&svc_run.to_str().unwrap(), HOOK_PERMISSIONS));
            }
            None => {
                let run = self.package().installed_path().join(hooks::RUN_FILENAME);
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

    /// Run file_updated hook if present
    fn file_updated(&mut self) -> bool {
        if self.initialized {
            match self.run_hook(HookType::FileUpdated) {
                Ok(()) => {
                    outputln!(preamble self.service_group, "File update hook succeeded");
                    return true;
                }
                Err(err) => {
                    outputln!(preamble self.service_group, "File update hook failed: {}", err);
                }
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
            butterfly.service_files_for(&*self.service_group, &self.current_service_files)
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

    fn run_hook(&mut self, hook: HookType) -> Result<()> {
        self.hooks.compile(&self.service_group, &self.config);
        self.hooks.try_run(hook, &self.service_group)
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
            butterfly.service_store
                .with_rumor(&*self.service_group,
                            &me,
                            |rumor| if let Some(rumor) = rumor {
                                let mut rumor = rumor.clone();
                                let incarnation = rumor.get_incarnation() + 1;
                                rumor.set_incarnation(incarnation);
                                *rumor.mut_cfg() = toml::encode_str(&cfg).into_bytes();
                                updated = Some(rumor);
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
        self.current_service_files.insert(filename.clone(), incarnation);
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
        let encoded = toml::encode_str(&config);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Topology {
    Standalone,
    Leader,
}

impl FromStr for Topology {
    type Err = SupError;

    fn from_str(topology: &str) -> result::Result<Self, Self::Err> {
        match topology {
            "leader" => Ok(Topology::Leader),
            "standalone" => Ok(Topology::Standalone),
            _ => Err(sup_error!(Error::UnknownTopology(String::from(topology)))),
        }
    }
}

impl Default for Topology {
    fn default() -> Topology {
        Topology::Standalone
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum UpdateStrategy {
    None,
    AtOnce,
    Rolling,
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

impl Default for UpdateStrategy {
    fn default() -> UpdateStrategy {
        UpdateStrategy::None
    }
}
