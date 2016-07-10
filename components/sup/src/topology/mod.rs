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

//! The service topologies.
//!
//! A service topology is a [state machine](../state_machine) that wraps the lifecycle events of a service around the
//! process supervisor and package manager. It is responsible for:
//!
//! 1. Processing the main event loop
//! 1. Registering callbacks with the [discovery](../discovery) system
//!
//! Typically, topologies are created from the [start](../command/start) command.

pub mod standalone;
pub mod leader;
pub mod initializer;

use std::mem;
use std::net::SocketAddrV4;
use std::ops::DerefMut;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use wonder;

use state_machine::StateMachine;
use census::{self, CensusList};
use common::gossip_file::GossipFileList;
use package::{self, Package, PackageUpdaterActor};
use util::signals::SignalNotifier;
use error::{Result, SupError};
use config::Config;
use service_config::ServiceConfig;
use sidecar;
use supervisor::{RuntimeConfig, Supervisor};
use gossip;
use gossip::rumor::{Rumor, RumorList};
use gossip::member::MemberList;
use election::ElectionList;
use time::SteadyTime;
use util::signals;
use util::users as hab_users;
use config::UpdateStrategy;

static LOGKEY: &'static str = "TP";
static MINIMUM_LOOP_TIME_MS: i64 = 200;

#[derive(PartialEq, Eq, Debug, RustcEncodable)]
pub enum Topology {
    Standalone,
    Leader,
    Initializer,
}

impl Default for Topology {
    fn default() -> Topology {
        Topology::Standalone
    }
}

/// Viable states for the topologies. Not every topology will implement every state.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum State {
    Init,
    MinimumQuorum,
    WaitingForQuorum,
    RestoreDataset,
    DetermineViability,
    BecomeLeader,
    BecomeFollower,
    CheckForElection,
    Election,
    StartElection,
    InElection,
    Leader,
    Follower,
    Configure,
    Initializing,
    Starting,
    Running,
}

/// The topology `Worker` is where everything our state machine needs between states lives.
pub struct Worker<'a> {
    /// The package we are supervising
    pub package: Arc<RwLock<Package>>,
    /// Name of the package being supervised
    pub package_name: String,
    /// A pointer to our current Config
    pub config: &'a Config,
    /// The topology we are running
    pub topology: String,
    /// Our Service Configuration; manages changes to our configuration,
    pub service_config: Arc<RwLock<ServiceConfig>>,
    /// The Gossip Server; listens for inbound gossip traffic
    pub gossip_server: gossip::server::Server,
    pub census_list: Arc<RwLock<CensusList>>,
    pub rumor_list: Arc<RwLock<RumorList>>,
    pub election_list: Arc<RwLock<ElectionList>>,
    pub member_list: Arc<RwLock<MemberList>>,
    pub gossip_file_list: Arc<RwLock<GossipFileList>>,
    /// Our Sidecar Actor; exposes a restful HTTP interface to the outside world
    pub sidecar_actor: sidecar::SidecarActor,
    /// Watches a package Depot for updates and signals the main thread when an update is available. Optionally
    /// started if a value is passed for the url option on startup.
    pub pkg_updater: Option<PackageUpdaterActor>,
    /// The service supervisor
    pub supervisor: Arc<RwLock<Supervisor>>,
    pub return_state: Option<State>,
}

impl<'a> Worker<'a> {
    /// Create a new worker
    ///
    /// Automatically sets the backend to Etcd.
    pub fn new(package: Package, topology: String, config: &'a Config) -> Result<Worker<'a>> {
        let mut pkg_updater = None;
        let package_name = package.name.clone();

        let (svc_user, svc_group) = try!(hab_users::get_user_and_group(&package.pkg_install));
        outputln!("Child process will run as user={}, group={}",
                  &svc_user,
                  &svc_group);
        let runtime_config = RuntimeConfig::new(svc_user, svc_group);

        let package_exposes = package.exposes().clone();
        let package_port = package_exposes.first().map(|e| e.clone());
        let package_ident = package.ident().clone();
        let pkg_lock = Arc::new(RwLock::new(package));
        let pkg_lock_1 = pkg_lock.clone();


        match config.update_strategy() {
            UpdateStrategy::None => {}
            _ => {
                let pkg_lock_2 = pkg_lock.clone();
                pkg_updater = Some(package::PackageUpdater::start(config.url(), pkg_lock_2));
            }
        }

        let gossip_server = gossip::server::Server::new(String::from(config.gossip_listen_ip()),
                                                        config.gossip_listen_port(),
                                                        config.gossip_permanent(),
                                                        config.ring().clone(),
                                                        package_name.clone(),
                                                        config.group().to_string(),
                                                        config.organization().clone(),
                                                        Some(package_exposes),
                                                        package_port);

        try!(gossip_server.start_inbound());
        try!(gossip_server.initial_peers(config.gossip_peer()));
        gossip_server.start_outbound();
        gossip_server.start_failure_detector();
        census::start_health_adjuster(gossip_server.census_list.clone(),
                                      gossip_server.member_list.clone());

        // Setup the Service Configuration
        let service_config = {
            let cl = gossip_server.census_list.read().unwrap();
            let pkg = pkg_lock.read().unwrap();
            let sc = try!(ServiceConfig::new(&config, &pkg, &cl, config.bind()));
            sc
        };
        let service_config_lock = Arc::new(RwLock::new(service_config));
        let service_config_lock_1 = service_config_lock.clone();

        let supervisor = Arc::new(RwLock::new(Supervisor::new(package_ident, runtime_config)));

        let sidecar_ml = gossip_server.member_list.clone();
        let sidecar_rl = gossip_server.rumor_list.clone();
        let sidecar_cl = gossip_server.census_list.clone();
        let sidecar_gfl = gossip_server.gossip_file_list.clone();
        let sidecar_detector = gossip_server.detector.clone();
        let sidecar_el = gossip_server.election_list.clone();
        let sidecar_sup = supervisor.clone();
        let sidecar_listen = try!(SocketAddrV4::from_str(&format!("{}:{}",
                                                                  &config.http_listen_ip(),
                                                                  config.http_listen_port())));
        Ok(Worker {
            package: pkg_lock,
            package_name: package_name,
            topology: topology,
            config: config,
            census_list: gossip_server.census_list.clone(),
            rumor_list: gossip_server.rumor_list.clone(),
            election_list: gossip_server.election_list.clone(),
            gossip_file_list: gossip_server.gossip_file_list.clone(),
            member_list: gossip_server.member_list.clone(),
            gossip_server: gossip_server,
            service_config: service_config_lock,
            sidecar_actor: sidecar::Sidecar::start(sidecar_listen,
                                                   pkg_lock_1,
                                                   service_config_lock_1,
                                                   sidecar_ml,
                                                   sidecar_rl,
                                                   sidecar_cl,
                                                   sidecar_detector,
                                                   sidecar_el,
                                                   sidecar_sup,
                                                   sidecar_gfl),
            supervisor: supervisor,
            pkg_updater: pkg_updater,
            return_state: None,
        })
    }

    /// update a package, but does NOT restart the service
    pub fn update_package(&self, updated: Package) -> Result<()> {
        let service_config = self.service_config.read().unwrap();
        {
            let mut package = self.package.write().unwrap();
            mem::replace(package.deref_mut(), updated);
        }
        let package = self.package.read().unwrap();
        try!(package.copy_run(&service_config));
        Ok(())
    }
}

/// The main loop of a topology.
///
/// 1. Loops forever
/// 1. Checks if we have caught a signal; if so, acts on the signal. (May exit entirely)
/// 1. Checks the current `state` of our [StateMachine](../state_machine)
/// 1. If it is running, we run a non-blocking `waitpid`, and inspect why the child died;
///    depending on the circumstances, we may exit with an error here
/// 1. Process any discovery events
/// 1. Trigger the next iteration of the state machine
///
/// # Failures
///
/// * We fail to send a signal to the supervisor
/// * We fail to join the supervisor thread
/// * The supervisor dies unexpectedly
/// * The discovery subsystem returns an error
/// * The topology state machine returns an error
fn run_internal<'a>(sm: &mut StateMachine<State, Worker<'a>, SupError>,
                    worker: &mut Worker<'a>)
                    -> Result<()> {
    {
        let package = worker.package.read().unwrap();
        let service_config = worker.service_config.read().unwrap();
        try!(package.create_svc_path());
        try!(package.copy_run(&service_config));
    }
    let handler = wonder::actor::Builder::new(SignalNotifier)
        .name("signal-handler".to_string())
        .start(())
        .unwrap();
    loop {
        let start_time = SteadyTime::now();
        match handler.receiver.try_recv() {
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(sig))) => {
                debug!("SIG = {:?}", sig);
                match sig {
                    signals::Signal::SIGINT | signals::Signal::SIGTERM => {
                        let mut supervisor = worker.supervisor.write().unwrap();
                        try!(supervisor.down());
                        break;
                    }
                    _ => {
                        outputln!("Forwarding {:?} on to the supervised process", sig);
                        let supervisor = worker.supervisor.write().unwrap();
                        try!(supervisor.send_unix_signal(sig.clone()));
                    }
                };
            }
            Ok(_) => {}
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                panic!("signal handler crashed!");
            }
        }

        {
            let mut supervisor = worker.supervisor.write().unwrap();
            try!(supervisor.check_process());
        }

        let mut restart_process = false;

        // This section, and the following really need to be refactored:
        //
        // 1. We check to see if we are in an event a bunch of times
        // 2. We potentially restart 3 different times - twice for reconfigure, and once for
        //    file_udpated. That's pretty yucky.
        //
        // Love, Adam
        {
            // Manage the census
            let (write_census, in_event, write_rumor, me_clone) = {
                let cl = worker.census_list.read().unwrap();
                let census = cl.local_census();
                (cl.needs_write(), census.in_event, census.me().needs_write(), census.me().clone())
            };

            if write_census {
                if !in_event {
                    let mut service_config = worker.service_config.write().unwrap();
                    let cl = worker.census_list.read().unwrap();
                    service_config.svc(&cl);
                    service_config.bind(worker.config.bind(), &cl);
                }
                if write_rumor {
                    debug!("Writing our census rumor: {:#?}", me_clone);
                    let mut rl = worker.rumor_list.write().unwrap();
                    rl.add_rumor(Rumor::census_entry(me_clone));
                }
                let mut cl = worker.census_list.write().unwrap();
                cl.written();
            }

            // Don't bother trying to reconfigure if we are in an event - just wait till
            // everything settles down.
            {
                let census_list = worker.census_list.read().unwrap();
                let census = census_list.local_census();
                if !census.in_event {
                    let mut service_config = worker.service_config.write().unwrap();
                    if service_config.needs_write {
                        let package = worker.package.read().unwrap();
                        // Write the configuration, and restart if needed
                        if try!(service_config.write(&package)) {
                            try!(package.copy_run(&service_config));
                            try!(package.reconfigure(&service_config));
                            outputln!("Restarting because the service config was updated via the \
                                       census");
                            restart_process = true;
                        }
                    }
                }
            }
        }

        {
            let in_event = {
                let census_list = worker.census_list.read().unwrap();
                let census = census_list.local_census();
                census.in_event
            };
            let (needs_file_updated, needs_reconfigure) = if in_event {
                (false, false)
            } else {
                let mut gossip_file_list = worker.gossip_file_list.write().unwrap();
                let needs_write = {
                    gossip_file_list.needs_write()
                };
                if needs_write {
                    let supervisor = worker.supervisor.read().unwrap();
                    let svc_user = &supervisor.runtime_config.svc_user;
                    let svc_group = &supervisor.runtime_config.svc_group;

                    try!(gossip_file_list.write(svc_user, svc_group))
                } else {
                    (false, false)
                }
            };
            if needs_file_updated {
                let service_config = worker.service_config.read().unwrap();
                let package = worker.package.read().unwrap();
                let existed = try!(package.file_updated(&service_config));
                if !existed {
                    restart_process = true;
                }
            }
            if needs_reconfigure {
                let mut service_config = worker.service_config.write().unwrap();
                let package = worker.package.read().unwrap();
                service_config.cfg(&package);
                if try!(service_config.write(&package)) {
                    try!(package.copy_run(&service_config));
                    let existed = try!(package.reconfigure(&service_config));
                    if !existed {
                        restart_process = true;
                    }
                }
            }
        }

        if let Some(ref updater) = worker.pkg_updater {
            match updater.receiver.try_recv() {
                Ok(wonder::actor::Message::Cast(package::UpdaterMessage::Update(package))) => {
                    debug!("Main loop received package update notification: {:?}",
                           &package);
                    try!(worker.update_package(package));
                    try!(package::PackageUpdater::run(&updater));
                    // force the package to restart
                    outputln!("Restarting because the package was updated");
                    restart_process = true;
                }
                Ok(_) => {}
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    panic!("package updater crashed!");
                }
            }
        }

        {
            let mut supervisor = worker.supervisor.write().unwrap();
            // If our target is that the process is up
            if supervisor.is_up() {
                // And no process is running
                if supervisor.pid.is_none() {
                    // Start a new one
                    try!(supervisor.start());
                } else {
                    // If we were supposed to restart
                    if restart_process {
                        // And we have ever started before...
                        if supervisor.has_started {
                            // Restart
                            try!(supervisor.restart());
                        }
                    }
                }
            }
        }

        // Next state!
        try!(sm.next(worker));

        // Slow down our loop
        let elapsed_time = SteadyTime::now() - start_time;
        let elapsed_millis = elapsed_time.num_milliseconds();

        if elapsed_millis < MINIMUM_LOOP_TIME_MS {
            thread::sleep(Duration::from_millis((MINIMUM_LOOP_TIME_MS - elapsed_millis) as u64));
        }
    }
    Ok(())
}
