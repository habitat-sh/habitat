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
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::result;
use std::str::FromStr;

use ansi_term::Colour::{Yellow, Red, Green};
use hcore::service::ServiceGroup;
use hcore::crypto::hash;
use hcore::fs;
use hcore::package::PackageIdent;
use hcore::url::DEFAULT_DEPOT_URL;
use hcore::util::perm::{set_owner, set_permissions};
use toml;

pub use self::config::ServiceConfig;
pub use self::health::{HealthCheck, SmokeCheck};
use self::hooks::{HOOK_PERMISSIONS, HookType};
use error::{Error, Result, SupError};
use manager::signals;
use manager::census::{CensusList, ElectionStatus};
use package::Package;
use supervisor::{Supervisor, RuntimeConfig};
use util;

static LOGKEY: &'static str = "SR";

static DEFAULT_GROUP: &'static str = "default";

pub struct ServiceSpec {
    pub ident: PackageIdent,
    pub group: String,
    pub organization: Option<String>,
    pub depot_url: String,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    pub binds: Vec<String>,
    pub dev_config_from: Option<PathBuf>,
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
            dev_config_from: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Service {
    pub needs_restart: bool,
    pub package: Package,
    pub cfg_incarnation: u64,
    pub service_group: ServiceGroup,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    pub current_service_files: HashMap<String, u64>,
    pub initialized: bool,
    pub last_election_status: ElectionStatus,
    pub supervisor: Supervisor,
    pub depot_url: String,
    pub spec_ident: PackageIdent,
    pub binds: Vec<String>,
}

impl Service {
    pub fn new(package: Package, spec: ServiceSpec) -> Result<Service> {
        let service_group = ServiceGroup::new(&package.name,
                                              spec.group,
                                              spec.organization.as_ref().map(|x| &**x))?;
        let (svc_user, svc_group) = try!(util::users::get_user_and_group(&package.pkg_install));
        let runtime_config = RuntimeConfig::new(svc_user, svc_group);
        let supervisor = Supervisor::new(package.ident().clone(), &service_group, runtime_config);
        Ok(Service {
            service_group: service_group,
            supervisor: supervisor,
            package: package,
            topology: spec.topology,
            depot_url: spec.depot_url,
            spec_ident: spec.ident,
            binds: spec.binds,
            needs_restart: false,
            update_strategy: spec.update_strategy,
            current_service_files: HashMap::new(),
            last_election_status: ElectionStatus::None,
            initialized: false,
            cfg_incarnation: 0,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        self.supervisor.start()
    }

    pub fn restart(&mut self, census_list: &CensusList) -> bool {
        match self.topology {
            Topology::Leader => {
                if let Some(census) = census_list.get(&*self.service_group) {
                    // We know perfectly well we are in this census, because we asked for
                    // our own service group *by name*
                    let me = census.me().unwrap();
                    let current_status = me.get_election_status();
                    if self.last_election_status != current_status {
                        match me.get_election_status() {
                            ElectionStatus::ElectionInProgress => {
                                outputln!(preamble self.service_group,
                                          "Not restarting service; {}",
                                          Yellow.bold().paint("election in progress."));
                            }
                            ElectionStatus::ElectionNoQuorum => {
                                outputln!(preamble self.service_group,
                                          "Not restarting service; {}, {}.",
                                          Yellow.bold().paint("election in progress"),
                                          Red.bold().paint("and we have no quorum"));
                            }
                            ElectionStatus::ElectionFinished => {
                                // We know we have a leader, so this is fine
                                let leader_id = census.get_leader().unwrap().get_member_id();
                                outputln!(preamble self.service_group,
                                          "Restarting service; {} is the leader",
                                          Green.bold().paint(leader_id));
                                self.last_election_status = ElectionStatus::ElectionFinished;
                                self.needs_restart = false;
                                try!(self.supervisor.restart());
                                return true;
                            }
                            ElectionStatus::None => {}
                        }
                        self.last_election_status = current_status;
                    }
                }
            }
            Topology::Standalone => {
                self.needs_restart = false;
                try!(self.supervisor.restart());
                return true;
            }
        }
        return false;
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

    pub fn is_down(&self) -> bool {
        self.supervisor.child.is_none()
    }

    /// Instructs the service's process supervisor to reap dead children.
    pub fn check_process(&mut self) {
        self.supervisor.check_process()
    }

    pub fn write_butterfly_service_file(&mut self,
                                        filename: String,
                                        incarnation: u64,
                                        body: Vec<u8>)
                                        -> bool {
        self.current_service_files.insert(filename.clone(), incarnation);
        let on_disk_path = fs::svc_files_path(self.service_group.service()).join(filename);
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

    pub fn write_butterfly_service_config(&mut self, config: toml::Value) -> bool {
        let encoded = toml::encode_str(&config);
        let on_disk_path = fs::svc_path(self.service_group.service()).join("gossip.toml");
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

    pub fn health_check(&self) -> HealthCheck {
        if let Some(hook) = self.hooks().health_check {
            match hook.run(&self.service_group) {
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

    pub fn hooks(&self) -> hooks::HookTable {
        let mut hooks = hooks::HookTable::new(&self.package, &self.service_group);
        hooks.load_hooks();
        hooks
    }

    /// Run file_updated hook if present
    pub fn file_updated(&self) -> bool {
        if self.initialized {
            match self.hooks().try_run(HookType::FileUpdated) {
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

    /// Run initialization hook if present
    pub fn initialize(&mut self, census_list: &CensusList) -> bool {
        if !self.initialized {
            match self.topology {
                Topology::Leader => {
                    if let Some(census) = census_list.get(&*self.service_group) {
                        // We know perfectly well we are in this census, because we asked for
                        // our own service group *by name*
                        let me = census.me().unwrap();
                        let current_status = me.get_election_status();
                        if self.last_election_status != current_status {
                            match current_status {
                                ElectionStatus::ElectionInProgress => {
                                    outputln!(preamble self.service_group,
                                              "Waiting to initialize service; {}",
                                              Yellow.bold().paint("election in progress."));
                                }
                                ElectionStatus::ElectionNoQuorum => {
                                    outputln!(preamble self.service_group,
                                              "Waiting to initialize service; {}, {}.",
                                              Yellow.bold().paint("election in progress"),
                                              Red.bold().paint("and we have no quorum"));
                                }
                                ElectionStatus::ElectionFinished => {
                                    outputln!(preamble self.service_group, "Initializing");
                                    if let Some(err) = self.hooks().try_run(HookType::Init).err() {
                                        outputln!(preamble self.service_group, "Initialization failed: {}", err);
                                        return false;
                                    }
                                    self.initialized = true;
                                }
                                ElectionStatus::None => {}
                            }
                            self.last_election_status = current_status
                        }
                    }
                }
                Topology::Standalone => {
                    outputln!(preamble self.service_group, "Initializing");
                    if let Some(err) = self.hooks().try_run(HookType::Init).err() {
                        outputln!(preamble self.service_group, "Initialization failed: {}", err);
                        return false;
                    }
                    self.initialized = true;
                }
            }
        }
        self.initialized
    }

    pub fn load_service_config(&self, census: &CensusList) -> Result<ServiceConfig> {
        ServiceConfig::new(&self.service_group, &self.package, census, &self.binds)
    }

    pub fn render_service_config(&mut self, census_list: &CensusList) -> Option<ServiceConfig> {
        let mut service_config = match self.load_service_config(census_list) {
            Ok(sc) => sc,
            Err(e) => {
                outputln!(preamble self.service_group,
                          "Error generating Service Configuration: {}",
                          e);
                return None;
            }
        };
        match service_config.write(&self.package) {
            Ok(true) => {
                self.needs_restart = true;
            }
            Ok(false) => {}
            Err(e) => {
                outputln!(preamble self.service_group,
                          "Failed to write service configuration: {}",
                          e);
            }
        }
        // JW TODO: We probably don't need to reload everything. Just updating them is more
        // appropriate.
        self.hooks().compile(&service_config);
        if let Some(err) = self.copy_run().err() {
            outputln!(preamble self.service_group, "Failed to copy run hook: {}", err);
        }
        Some(service_config)
    }

    /// Run reconfigure hook if present.
    pub fn reconfigure(&self) {
        if let Some(err) = self.hooks().try_run(HookType::Reconfigure).err() {
            outputln!(preamble self.service_group,
                      "Reconfiguration hook failed: {}",
                      err);
        }
    }

    pub fn smoke_test(&self) -> SmokeCheck {
        match self.hooks().try_run(HookType::SmokeTest) {
            Ok(()) => SmokeCheck::Ok,
            Err(SupError { err: Error::HookFailed(_, code), .. }) => SmokeCheck::Failed(code),
            Err(err) => {
                outputln!(preamble self.service_group, "Smoke test couldn't be run, {}", err);
                SmokeCheck::Failed(-1)
            }
        }
    }

    // Copy the "run" file to the svc path.
    fn copy_run(&self) -> Result<()> {
        let svc_run = self.package.pkg_install.svc_path().join(hooks::RUN_FILENAME);
        match self.hooks().run {
            Some(hook) => {
                try!(std::fs::copy(hook.path, &svc_run));
                try!(set_permissions(&svc_run.to_str().unwrap(), HOOK_PERMISSIONS));
            }
            None => {
                let run = self.package.path().join(hooks::RUN_FILENAME);
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
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.package)
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
