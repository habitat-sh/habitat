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

use std;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::result;
use std::str::FromStr;

use ansi_term::Colour::{Yellow, Red, Green};
use hcore::service::ServiceGroup;
use hcore::crypto::hash;
use hcore::fs;
use hcore::util::perm::{set_owner, set_permissions};
use toml;

pub use self::config::ServiceConfig;
use config::gconfig;
use error::{Error, Result, SupError};
use health_check;
use manager::signals;
use manager::census::CensusList;
use package::Package;
use supervisor::{Supervisor, RuntimeConfig};
use util;

static LOGKEY: &'static str = "SR";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum LastRestartDisplay {
    None,
    ElectionInProgress,
    ElectionNoQuorum,
    ElectionFinished,
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
    pub last_restart_display: LastRestartDisplay,
    pub supervisor: Supervisor,
}

impl Service {
    pub fn new<T>(package: Package,
                  group: T,
                  organization: Option<&str>,
                  topology: Topology,
                  update_strategy: UpdateStrategy)
                  -> Result<Service>
        where T: AsRef<str>
    {
        let service_group = ServiceGroup::new(&package.name, group, organization)?;
        let (svc_user, svc_group) = try!(util::users::get_user_and_group(&package.pkg_install));
        let runtime_config = RuntimeConfig::new(svc_user, svc_group);
        let supervisor = Supervisor::new(package.ident().clone(), &service_group, runtime_config);
        Ok(Service {
            service_group: service_group,
            supervisor: supervisor,
            package: package,
            topology: topology,
            needs_restart: false,
            update_strategy: update_strategy,
            current_service_files: HashMap::new(),
            last_restart_display: LastRestartDisplay::None,
            initialized: false,
            cfg_incarnation: 0,
        })
    }

    pub fn service_group_str(&self) -> String {
        format!("{}", self.service_group)
    }

    pub fn start(&mut self) -> Result<()> {
        self.supervisor.start()
    }

    pub fn restart(&mut self, census_list: &CensusList) -> Result<()> {
        match self.topology {
            Topology::Leader => {
                if let Some(census) = census_list.get(&self.service_group.to_string()) {
                    // We know perfectly well we are in this census, because we asked for
                    // our own service group *by name*
                    let me = census.me().unwrap();
                    if me.get_election_is_running() {
                        if self.last_restart_display != LastRestartDisplay::ElectionInProgress {
                            outputln!(preamble self.service_group_str(),
                                      "Not restarting service; {}",
                                      Yellow.bold().paint("election in progress."));
                            self.last_restart_display = LastRestartDisplay::ElectionInProgress;
                        }
                    } else if me.get_election_is_no_quorum() {
                        if self.last_restart_display != LastRestartDisplay::ElectionNoQuorum {
                            outputln!(preamble self.service_group_str(),
                                      "Not restarting service; {}, {}.",
                                      Yellow.bold().paint("election in progress"),
                                      Red.bold().paint("and we have no quorum"));
                            self.last_restart_display = LastRestartDisplay::ElectionNoQuorum;
                        }
                    } else if me.get_election_is_finished() {
                        // We know we have a leader, so this is fine
                        let leader_id = census.get_leader().unwrap().get_member_id();
                        if self.last_restart_display != LastRestartDisplay::ElectionFinished {
                            outputln!(preamble self.service_group_str(),
                                      "Restarting service; {} is the leader",
                                      Green.bold().paint(leader_id));
                            self.last_restart_display = LastRestartDisplay::ElectionFinished;
                        }
                        self.needs_restart = false;
                        try!(self.supervisor.restart());
                    }
                }
            }
            Topology::Standalone => {
                self.needs_restart = false;
                try!(self.supervisor.restart());
            }
        }
        Ok(())
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
                    outputln!(preamble self.service_group_str(),
                        "Service file from butterfly failed to open the new file {}: {}",
                        new_filename,
                        Red.bold().paint(format!("{}", e)));
                    return false;
                }
            };

            if let Err(e) = new_file.write_all(&body) {
                outputln!(preamble self.service_group_str(),
                    "Service file from butterfly failed to write {}: {}",
                    new_filename,
                    Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = std::fs::rename(&new_filename, &on_disk_path) {
                outputln!(preamble self.service_group_str(),
                    "Service file from butterfly failed to rename {} to {}: {}",
                    new_filename,
                    on_disk_path.to_string_lossy(),
                    Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_owner(&on_disk_path,
                                      &self.supervisor.runtime_config.svc_user,
                                      &self.supervisor.runtime_config.svc_group) {
                outputln!(preamble self.service_group_str(),
                    "Service file from butterfly failed to set ownership on {}: {}",
                    on_disk_path.to_string_lossy(),
                    Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_permissions(&on_disk_path, 0o640) {
                outputln!(preamble self.service_group_str(),
                    "Service file from butterfly failed to set permissions on {}: {}",
                    on_disk_path.to_string_lossy(),
                    Red.bold().paint(format!("{}", e)));
                return false;
            }

            outputln!(preamble self.service_group_str(),
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
                    outputln!(preamble self.service_group_str(),
                        "Service configuration from butterfly failed to open the new file: {}",
                        Red.bold().paint(format!("{}", e)));
                    return false;
                }
            };

            if let Err(e) = new_file.write_all(encoded.as_bytes()) {
                outputln!(preamble self.service_group_str(),
                    "Service configuration from butterfly failed to write: {}",
                    Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = std::fs::rename(&new_filename, &on_disk_path) {
                outputln!(preamble self.service_group_str(),
                    "Service configuration from butterfly failed to rename: {}",
                    Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_owner(&on_disk_path,
                                      &self.supervisor.runtime_config.svc_user,
                                      &self.supervisor.runtime_config.svc_group) {
                outputln!(preamble self.service_group_str(),
                    "Service configuration from butterfly failed to set ownership: {}",
                    Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_permissions(&on_disk_path, 0o640) {
                outputln!(preamble self.service_group_str(),
                    "Service configuration from butterfly failed to set permissions: {}",
                    Red.bold().paint(format!("{}", e)));
                return false;
            }

            outputln!(preamble self.service_group_str(),
                "Service configuration updated from butterfly: {}",
                Green.bold().paint(new_checksum));
            true
        } else {
            false
        }
    }

    pub fn health_check(&self) -> Result<health_check::CheckResult> {
        self.package.health_check(&self.supervisor, &self.service_group)
    }

    pub fn file_updated(&self) -> bool {
        if self.initialized {
            let sg = self.service_group_str();
            match self.package.file_updated(&self.service_group) {
                Ok(_) => {
                    outputln!(preamble sg, "{}", "File update hook succeeded.");
                    return true;
                }
                Err(e) => outputln!(preamble sg, "File update hook failed: {}", e),
            }
        }
        false
    }

    pub fn initialize(&mut self) {
        if !self.initialized {
            match self.package.initialize(&self.service_group) {
                Ok(()) => {
                    outputln!(preamble self.service_group_str(), "{}", "Initializing");
                    self.initialized = true
                }
                Err(e) => {
                    outputln!(preamble self.service_group_str(), "Initialization failed: {}", e)
                }
            }
        }
    }

    pub fn load_service_config(&self, census: &CensusList) -> Result<ServiceConfig> {
        ServiceConfig::new(&self.service_group_str(),
                           &self.package,
                           census,
                           gconfig().bind())
    }

    pub fn reconfigure(&mut self, census_list: &CensusList) -> Option<ServiceConfig> {
        let mut service_config = match self.load_service_config(census_list) {
            Ok(sc) => sc,
            Err(e) => {
                outputln!(preamble self.service_group_str(),
                          "Error generating Service Configuration; not reconfiguring: {}",
                          e);
                return None;
            }
        };
        match service_config.write(&self.package) {
            Ok(true) => {
                self.needs_restart = true;
                match self.package.reconfigure(&self.service_group) {
                    Ok(_) => {}
                    Err(e) => {
                        outputln!(preamble self.service_group_str(),
                            "Reconfiguration hook failed: {}", e);
                    }
                }
            }
            Ok(false) => {}
            Err(e) => {
                outputln!(preamble self.service_group_str(),
                    "Failed to write service configuration: {}", e);
            }
        }

        self.package.hooks().load_hooks();
        // Probably worth moving the run hook under compile all, eventually
        if let Err(e) = self.package.copy_run(&service_config) {
            outputln!("Failed to copy run hook: {}", e);
        }
        self.package.hooks().compile_all(&service_config);
        Some(service_config)
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
