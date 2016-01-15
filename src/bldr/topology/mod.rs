// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

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
use std::ops::DerefMut;
use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::TryRecvError;

use libc::{pid_t, c_int};
use wonder;

use state_machine::StateMachine;
use census;
use package::{self, Package, PackageUpdaterActor, Signal};
use util::signals;
use util::signals::SignalNotifier;
use error::{BldrResult, BldrError, ErrorKind};
use config::Config;
use service_config::ServiceConfig;
use sidecar;
use user_config;
use watch_config;
use gossip;

static LOGKEY: &'static str = "TP";

// Functions from POSIX libc.
extern {
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}

/// A simple compatability type for external functions
#[allow(non_camel_case_types)]
pub type idtype_t = c_int;

pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 2;

// Process flags
pub const WCONTINUED: c_int = 8;
pub const WNOHANG: c_int = 1;
pub const WUNTRACED: c_int = 2;
pub const WEXITED: c_int = 4;
pub const WNOWAIT: c_int = 16777216;
pub const WSTOPPED: c_int = 2;

/// Get the exit status from waitpid's errno
#[allow(non_snake_case)]
pub fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/// Get the exit status from waitpid's errno
#[allow(non_snake_case)]
pub fn WIFCONTINUED(status: c_int) -> bool {
    status == 0xffff
}

#[allow(non_snake_case)]
pub fn WIFEXITED(status: c_int) -> bool {
    WTERMSIG(status) == 0
}

/// Has a value if our child was signaled
#[allow(non_snake_case)]
pub fn WIFSIGNALED(status: c_int) -> bool {
    ((((status) & 0x7f) + 1) as i8 >> 1) > 0
}

#[allow(non_snake_case)]
pub fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

#[allow(non_snake_case)]
pub fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}

#[allow(non_snake_case)]
pub fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

#[derive(PartialEq, Eq, Debug)]
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
    RestoreDataset,
    DetermineViability,
    BecomeLeader,
    BecomeFollower,
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
/// Our census
    pub census: census::Census,
/// Our Service Configuration; manages changes to our configuration,
    pub service_config: Arc<RwLock<ServiceConfig>>,
/// Our Census Entry Actor; writes our entry periodically
    pub census_entry_actor: wonder::actor::Actor<census::Message>,
/// Our Census Actor; reads the census periodically
    pub census_actor: wonder::actor::Actor<census::CensusMessage>,
/// The Gossip Server; listens for inbound gossip traffic
    pub gossip_server: wonder::actor::Actor<gossip::server::ServerActorMessage>,
/// Our Sidecar Actor; exposes a restful HTTP interface to the outside world
    pub sidecar_actor: sidecar::SidecarActor,
/// Our User Configuration; reads the config periodically
    pub user_actor: wonder::actor::Actor<user_config::Message>,
/// Our User Configuration; reads the config periodically
    pub watch_actor: wonder::actor::Actor<watch_config::Message>,
/// Watches a package repo for updates and signals the main thread when an update is available. Optionally
/// started if a value is passed for the url option on startup.
    pub pkg_updater: Option<PackageUpdaterActor>,
/// A pointer to the supervisor thread
    pub supervisor_thread: Option<thread::JoinHandle<Result<(), BldrError>>>,
/// The PID of the Supervisor itself
    pub supervisor_id: Option<u32>,
}

impl<'a> Worker<'a> {
    /// Create a new worker
    ///
    /// Automatically sets the backend to Etcd.
    pub fn new(package: Package, topology: String, config: &'a Config) -> BldrResult<Worker<'a>> {
        let mut pkg_updater = None;
        // Setup our Census Entry
        let port = package.exposes().pop().unwrap_or(String::from("0"));
        let exposes = package.exposes().clone();
        let mut ce = census::CensusEntry::new();
        ce.port(Some(port));
        ce.exposes(Some(exposes));
        let census_data = ce.as_etcd_write(&package, &config);
        let package_name = package.name.clone();

        outputln!("Supervise ID {}", ce.candidate_string());

        // Setup the Census
        let census = census::Census::new(ce);
        let census_actor_state = census::CensusActorState::new(format!("{}/{}/census",
                                                                       package_name,
                                                                       config.group()));

        // Setup the Service Configuration
        let service_config = try!(ServiceConfig::new(&package));

        // Setup the User Data Configuration
        let user_actor_state = user_config::UserActorState::new(format!("{}/{}/config",
                                                                        package_name,
                                                                        config.group()));

        // Setup the Watches
        let mut watch_actor_state = watch_config::WatchActorState::new();
        try!(watch_actor_state.set_watches(&config));

        let pkg_lock = Arc::new(RwLock::new(package));
        let pkg_lock_1 = pkg_lock.clone();

        let service_config_lock = Arc::new(RwLock::new(service_config));
        let service_config_lock_1 = service_config_lock.clone();

        if let Some(ref url) = *config.url() {
            let pkg_lock_2 = pkg_lock.clone();
            pkg_updater = Some(package::PackageUpdater::start(url, pkg_lock_2));
        }

        Ok(Worker {
            package: pkg_lock,
            package_name: package_name,
            topology: topology,
            config: config,
            census: census,
            census_entry_actor: wonder::actor::Builder::new(census::CensusEntryActor)
                                    .name("census-entry".to_string())
                                    .start(census_data)
                                    .unwrap(),
            census_actor: wonder::actor::Builder::new(census::CensusActor)
                              .name("census-reader".to_string())
                              .start(census_actor_state)
                              .unwrap(),
            service_config: service_config_lock,
            sidecar_actor: sidecar::Sidecar::start(pkg_lock_1, service_config_lock_1),
            user_actor: wonder::actor::Builder::new(user_config::UserActor)
                            .name("user-config".to_string())
                            .start(user_actor_state)
                            .unwrap(),
            watch_actor: wonder::actor::Builder::new(watch_config::WatchActor)
                             .name("watch-config".to_string())
                             .start(watch_actor_state)
                             .unwrap(),
            gossip_server: wonder::actor::Builder::new(gossip::server::ServerActor)
                               .name("gossip-server".to_string())
                               .start(gossip::server::ServerState {
                                   listen: String::from(config.gossip_listen()),
                               })
                               .unwrap(),
            pkg_updater: pkg_updater,
            supervisor_thread: None,
            supervisor_id: None,
        })
    }

    pub fn signal_package(&self, signal: Signal) -> BldrResult<String> {
        let package = self.package.read().unwrap();
        package.signal(signal)
    }

    pub fn update_package(&self, updated: Package) -> BldrResult<()> {
        let service_config = self.service_config.read().unwrap();
        {
            let mut package = self.package.write().unwrap();
            mem::replace(package.deref_mut(), updated);
        }
        let package = self.package.read().unwrap();
        try!(package.copy_run(&service_config));
        try!(package.signal(Signal::Restart));
        Ok(())
    }

    /// Join the supervisor thread, and check for errors
    ///
    /// # Failures
    ///
    /// * Supervisor thread fails
    pub fn join_supervisor(&mut self) -> BldrResult<()> {
        if self.supervisor_thread.is_some() {
            outputln!("Waiting for supervisor to finish");
            let st = self.supervisor_thread.take().unwrap().join();
            match st {
                Ok(result) => {
                    match result {
                        Ok(()) => outputln!("Supervisor has finished"),
                        Err(_) => outputln!("Supervisor has an error"),
                    }
                }
                Err(e) => outputln!("Supervisor thread paniced: {:?}", e),
            }
        }
        Ok(())
    }
}

/// The main loop of a topology.
///
/// 1. Loops forever
/// 1. Checks if we have caught a signal; if so, acts on the signal. (May exit entirely)
/// 1. Checks the current `state` of our [StateMachine](../state_machine)
/// 1. If it is running, we run a non-blocking `waitpid`, and inspect why the supervisor died;
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
fn run_internal<'a>(sm: &mut StateMachine<State, Worker<'a>, BldrError>,
                    worker: &mut Worker<'a>)
                    -> BldrResult<()> {
    {
        let package = worker.package.read().unwrap();
        let service_config = worker.service_config.read().unwrap();
        try!(package.create_srvc_path());
        try!(package.copy_run(&service_config));
    }

    let handler = wonder::actor::Builder::new(SignalNotifier)
                      .name("signal-handler".to_string())
                      .start(())
                      .unwrap();
    outputln!("Watching census");
    outputln!("Watching config");
    loop {
        match handler.receiver.try_recv() {
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGHUP))) => {
                outputln!("Sending SIGHUP");
                try!(worker.signal_package(Signal::Hup));
            }
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGINT))) => {
                outputln!("Sending 'force-shutdown' on SIGINT");
                try!(worker.signal_package(Signal::ForceShutdown));
                try!(worker.join_supervisor());
                break;
            }
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGQUIT))) => {
                try!(worker.signal_package(Signal::Quit));
                outputln!("Sending SIGQUIT");
            }
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGALRM))) => {
                try!(worker.signal_package(Signal::Alarm));
                outputln!("Sending SIGALRM");
            }
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGTERM))) => {
                outputln!("Sending 'force-shutdown' on SIGTERM");
                try!(worker.signal_package(Signal::ForceShutdown));
                try!(worker.join_supervisor());
                break;
            }
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGUSR1))) => {
                outputln!("Sending SIGUSR1");
                try!(worker.signal_package(Signal::One));
            }
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGUSR2))) => {
                outputln!("Sending SIGUSR1");
                try!(worker.signal_package(Signal::Two));
            }
            Ok(_) => {}
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                panic!("signal handler crashed!");
            }
        }
        if worker.supervisor_id.is_some() {
            unsafe {
                let mut status: c_int = 0;
                let supervisor_pid = worker.supervisor_id.unwrap() as pid_t;
                match waitpid(supervisor_pid, &mut status, 1 as c_int) {
                    0 => {} // Nothing returned,
                    pid if pid == supervisor_pid => {
                        if WIFEXITED(status) {
                            let exit_code = WEXITSTATUS(status);
                            outputln!("The supervisor died - terminating {} with exit code {}",
                                      pid,
                                      exit_code);
                        } else if WIFSIGNALED(status) {
                            let exit_signal = WTERMSIG(status);
                            outputln!("The supervisor died - terminating {} with signal {}",
                                      pid,
                                      exit_signal);
                        } else {
                            outputln!("The supervisor over {} died, but I don't know how.", pid);
                        }
                        return Err(bldr_error!(ErrorKind::SupervisorDied));
                    }
                    // ZOMBIES! Bad zombies! We listen for zombies. ZOMBOCOM!
                    pid => {
                        if WIFEXITED(status) {
                            let exit_code = WEXITSTATUS(status);
                            debug!("Process {} died with exit code {}", pid, exit_code);
                        } else if WIFSIGNALED(status) {
                            let exit_signal = WTERMSIG(status);
                            debug!("Process {} terminated with signal {}", pid, exit_signal);
                        } else {
                            debug!("Process {} died, but I don't know how.", pid);
                        }
                    }
                }
            }
        }

        // Write our census entry immediately if something is dirty
        {
            let mut ce = try!(worker.census.me_mut());
            if ce.needs_write.is_some() {
                let package = worker.package.read().unwrap();
                try!(census::CensusEntryActor::write(&worker.census_entry_actor,
                                                     ce.as_etcd_write(&package, &worker.config)));
            }
        }

        // Obtain write lock on service config
        {
            let mut service_config = worker.service_config.write().unwrap();

            // Manage the entire census
            {
                if let Some(census_string) =
                       try!(census::CensusActor::census_string(&worker.census_actor)) {
                    try!(worker.census.update(&census_string));
                }
                if !worker.census.in_event {
                    if worker.census.needs_write {
                        let census_toml = try!(worker.census.to_toml());
                        service_config.census(census_toml);
                    }
                }
            }

            // Manage the user configuration from discovery
            {
                match try!(user_config::UserActor::config_string(&worker.user_actor)) {
                    Some(user_string) => service_config.user(user_string),
                    None => service_config.user(String::new()),
                }
            }

            // Manage the watch configuration from discovery
            {
                match try!(watch_config::WatchActor::config_string(&worker.watch_actor)) {
                    Some(watch_string) => service_config.watch(watch_string),
                    None => service_config.watch(String::new()),
                }
            }

            // Don't bother trying to reconfigure if we are in an event - just wait till
            // everything settles down.
            if !worker.census.in_event {
                let package = worker.package.read().unwrap();
                // Write the configuration, and restart if needed
                if try!(service_config.write(&package)) {
                    try!(package.copy_run(&service_config));
                    try!(package.reconfigure(&service_config));
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
                }
                Ok(_) => {}
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    panic!("package updater crashed!");
                }
            }
        }

        // Next state!
        try!(sm.next(worker));
    }
    Ok(())
}
