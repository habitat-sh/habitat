// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

use ansi_term::Colour::{Yellow, Red, Green};
use time::{SteadyTime, Duration as TimeDuration};
use hcore::package::PackageIdent;
use hcore::service::ServiceGroup;
use hcore::crypto::{hash, default_cache_key_path};
use hcore::fs::{self, CACHE_ARTIFACT_PATH, FS_ROOT_PATH};
use hcore::util::perm::{set_owner, set_permissions};

use {PRODUCT, VERSION};
use depot_client::Client;
use common::ui::UI;
use supervisor::{Supervisor, RuntimeConfig};
use package::Package;
use manager::signals;
use manager::census::CensusList;
use manager::service::config::ServiceConfig;
use util;
use error::Result;
use config::{gconfig, UpdateStrategy, Topology};

static LOGKEY: &'static str = "SR";
const UPDATE_STRATEGY_FREQUENCY_MS: i64 = 60000;

#[derive(Debug, PartialEq, Eq)]
enum LastRestartDisplay {
    None,
    ElectionInProgress,
    ElectionNoQuorum,
    ElectionFinished,
}

#[derive(Debug)]
pub struct Service {
    service_group: ServiceGroup,
    supervisor: Supervisor,
    package: Package,
    pub needs_restart: bool,
    topology: Topology, //    config: ServiceConfig,
    update_strategy: UpdateStrategy,
    update_thread_rx: Option<Receiver<Package>>,
    last_restart_display: LastRestartDisplay,
    initialized: bool,
    pub service_config_incarnation: Option<u64>,
}

impl Service {
    pub fn new(service_group: ServiceGroup,
               package: Package,
               topology: Topology,
               update_strategy: UpdateStrategy)
               -> Result<Service> {
        let (svc_user, svc_group) = try!(util::users::get_user_and_group(&package.pkg_install));
        let sg = format!("{}.{}", service_group.service, service_group.group);
        outputln!(preamble sg, "Process will run as user={}, group={}",
                  &svc_user,
                  &svc_group);
        let runtime_config = RuntimeConfig::new(svc_user, svc_group);
        let supervisor = Supervisor::new(package.ident().clone(), runtime_config);
        let update_thread_rx = if update_strategy != UpdateStrategy::None {
            Some(run_update_strategy(package.ident().clone()))
        } else {
            None
        };
        Ok(Service {
            service_group: service_group,
            supervisor: supervisor,
            package: package,
            topology: topology,
            needs_restart: false,
            update_strategy: update_strategy,
            update_thread_rx: update_thread_rx,
            last_restart_display: LastRestartDisplay::None,
            initialized: false,
            service_config_incarnation: None,
        })
    }

    pub fn service_group_str(&self) -> String {
        format!("{}.{}",
                self.service_group.service,
                self.service_group.group)
    }

    pub fn start(&mut self) -> Result<()> {
        self.supervisor.start()
    }

    pub fn restart(&mut self, census_list: &CensusList) -> Result<()> {
        match self.topology {
            Topology::Leader | Topology::Initializer => {
                if let Some(census) = census_list.get(&format!("{}.{}",
                                                               self.service_group.service,
                                                               self.service_group.group)) {
                    let me = census.me();
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
        if self.supervisor.pid.is_some() {
            signals::send_signal(self.supervisor.pid.unwrap(), signal)
        } else {
            debug!("No process to send the signal to");
            Ok(())
        }
    }

    pub fn is_down(&self) -> bool {
        self.supervisor.pid.is_none()
    }

    pub fn check_process(&mut self) -> Result<()> {
        self.supervisor.check_process()
    }

    pub fn write_butterfly_service_config(&mut self, config: String) -> bool {
        let on_disk_path = fs::svc_path(&self.service_group.service).join("gossip.toml");
        let current_checksum = match hash::hash_file(&on_disk_path) {
            Ok(current_checksum) => current_checksum,
            Err(e) => {
                debug!("Failed to get current checksum for {:?}: {}",
                       on_disk_path,
                       e);
                String::new()
            }
        };
        let new_checksum = hash::hash_string(&config)
            .expect("We failed to hash a string in a method that can't return an error; not even \
                     sure what this means");
        if new_checksum != current_checksum {
            let new_filename = format!("{}.write", on_disk_path.to_string_lossy());

            let mut new_file = match File::create(&new_filename) {
                Ok(new_file) => new_file,
                Err(e) => {
                    outputln!(preamble self.service_group_str(), "Service configuration from butterfly failed to open the new file: {}", Red.bold().paint(format!("{}", e)));
                    return false;
                }
            };

            if let Err(e) = new_file.write_all(config.as_bytes()) {
                outputln!(preamble self.service_group_str(), "Service configuration from butterfly failed to write: {}", Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = std::fs::rename(&new_filename, &on_disk_path) {
                outputln!(preamble self.service_group_str(), "Service configuration from butterfly failed to rename: {}", Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_owner(&on_disk_path,
                                      &self.supervisor.runtime_config.svc_user,
                                      &self.supervisor.runtime_config.svc_group) {
                outputln!(preamble self.service_group_str(), "Service configuration from butterfly failed to set ownership: {}", Red.bold().paint(format!("{}", e)));
                return false;
            }

            if let Err(e) = set_permissions(&on_disk_path, 0o770) {
                outputln!(preamble self.service_group_str(), "Service configuration from butterfly failed to set permissions: {}", Red.bold().paint(format!("{}", e)));
                return false;
            }

            outputln!(preamble self.service_group_str(), "Service configuration updated from butterfly: {}", Green.bold().paint(new_checksum));
            true
        } else {
            false
        }
    }

    pub fn initialize(&mut self) {
        if !self.initialized {
            match self.package.initialize() {
                Ok(()) => outputln!(preamble self.service_group_str(), "{}", "Initializing"),
                Err(e) => {
                    outputln!(preamble self.service_group_str(), "Initialization failed: {}", e)
                }
            }
            self.initialized = true
        }
    }

    pub fn reconfigure(&mut self, census_list: &CensusList) {
        let sg = format!("{}", self.service_group);
        let mut service_config =
            match ServiceConfig::new(&sg, &self.package, census_list, gconfig().bind()) {
                Ok(sc) => sc,
                Err(e) => {
                    outputln!(preamble self.service_group_str(),
                              "Error generating Service Configuration; not reconfiguring: {}",
                              e);
                    return;
                }
            };

        self.package.create_svc_path();

        match service_config.write(&self.package) {
            Ok(true) => {
                self.needs_restart = true;
                match self.package.reconfigure() {
                    Ok(_) => {}
                    Err(e) => outputln!(preamble self.service_group_str(), "Reconfiguration hook failed: {}", e),
                }
            }
            Ok(false) => {}
            Err(e) => {
                outputln!(preamble self.service_group_str(), "Failed to write service configuration: {}", e);
            }
        }

        self.package.hooks().load_hooks();
        // Probably worth moving the run hook under compile all, eventually
        self.package.copy_run(&service_config);
        self.package.hooks().compile_all(&service_config);
    }

    pub fn check_for_updated_package(&mut self) {
        if self.update_thread_rx.is_some() {
            match self.update_thread_rx.as_mut().unwrap().try_recv() {
                Ok(package) => {
                    outputln!(preamble self.service_group_str(), "Updated {} to {}", self.package, package);
                    self.package = package;
                    self.needs_restart = true;
                }
                Err(TryRecvError::Disconnected) => {
                    outputln!(preamble self.service_group_str(), "Software update thread has died {}", "; disconnected");
                    let receiver = run_update_strategy(self.package.ident().clone());
                    self.update_thread_rx = Some(receiver);
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.package)
    }
}

pub fn run_update_strategy(package_ident: PackageIdent) -> Receiver<Package> {
    let (tx, rx) = sync_channel(0);
    let _ = thread::Builder::new()
        .name(format!("update-{}", package_ident))
        .spawn(move || update_strategy(package_ident, tx));
    rx
}

pub fn update_strategy(package_ident: PackageIdent, tx_to_service: SyncSender<Package>) {
    'check: loop {
        let next_check = SteadyTime::now() +
                         TimeDuration::milliseconds(UPDATE_STRATEGY_FREQUENCY_MS);
        let depot_client = match Client::new(gconfig().url(), PRODUCT, VERSION, None) {
            Ok(client) => client,
            Err(e) => {
                debug!("Failed to create HTTP client: {:?}", e);
                let time_to_wait = next_check - SteadyTime::now();
                thread::sleep(Duration::from_millis(time_to_wait.num_milliseconds() as u64));
                continue 'check;
            }
        };
        match depot_client.show_package(package_ident.clone()) {
            Ok(remote) => {
                let latest_ident: PackageIdent = remote.get_ident().clone().into();
                if latest_ident > package_ident {
                    let mut ui = UI::default();
                    match depot_client.fetch_package(latest_ident.clone(),
                                                     &Path::new(FS_ROOT_PATH)
                                                         .join(CACHE_ARTIFACT_PATH),
                                                     ui.progress()) {
                        Ok(archive) => {
                            debug!("Updater downloaded new package to {:?}", archive);
                            // JW TODO: actually handle verify and unpack results
                            archive.verify(&default_cache_key_path(None)).unwrap();
                            archive.unpack(None).unwrap();
                            let latest_package = Package::load(&latest_ident, None).unwrap();
                            tx_to_service.send(latest_package).unwrap_or_else(|e| {
                                error!("Main thread has gone away; this is a disaster, mate.")
                            });
                        }
                        Err(e) => {
                            debug!("Failed to download package: {:?}", e);
                        }
                    }
                } else {
                    debug!("Package found is not newer than ours");
                }
            }
            Err(e) => {
                debug!("Updater failed to get latest package: {:?}", e);
            }
        }
        let time_to_wait = next_check - SteadyTime::now();
        thread::sleep(Duration::from_millis(time_to_wait.num_milliseconds() as u64));
    }
}
