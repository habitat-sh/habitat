//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

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
pub mod watcher;

use ansi_term::Colour::White;
use std::thread;
use std::sync::mpsc::{TryRecvError};
use libc::{pid_t, c_int};

use wonder;

use state_machine::StateMachine;
use discovery;
use pkg::{Package, Signal};
use util::signals;
use util::signals::SignalNotifier;
use error::{BldrResult, BldrError};
use config::Config;

// Functions from POSIX libc.
extern "C" {
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}

/// A simple compatability type for external functions
#[allow(non_camel_case_types)]
pub type idtype_t = c_int;

pub const P_ALL:  idtype_t = 0;
pub const P_PID:  idtype_t = 1;
pub const P_PGID: idtype_t = 2;

// Process flags
pub const WCONTINUED: c_int = 8;
pub const WNOHANG:    c_int = 1;
pub const WUNTRACED:  c_int = 2;
pub const WEXITED:    c_int = 4;
pub const WNOWAIT:    c_int = 16777216;
pub const WSTOPPED:   c_int = 2;

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

/// Viable states for the topologies. Not every topology will implement every state.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum State {
    Init,
    CreateDataset,
    RestoreDataset,
    DetermineViability,
    BecomeLeader,
    BecomeFollower,
    Leader,
    Follower,
    Configure,
    Starting,
    Running,
    Finished,
}

/// The topology `Worker` is where everything our state machine needs between states lives.
pub struct Worker<'a> {
    /// The package we are supervising
    pub package: Package,
    /// A pointer to our current Config
    pub config: &'a Config,
    /// The topology we are running
    pub topology: String,
    /// Our discovery service
    pub discovery: wonder::actor::Actor<discovery::Message>,
    /// A pointer to the supervisor thread
    pub supervisor_thread: Option<thread::JoinHandle<Result<(), BldrError>>>,
    /// A pointer to the configuration thread
    pub configuration_thread: Option<thread::JoinHandle<Result<(), BldrError>>>,
    /// The PID of the Supervisor itself
    pub supervisor_id: Option<u32>
}

impl<'a> Worker<'a> {
    /// Create a new worker
    ///
    /// Automatically sets the backend to Etcd.
    pub fn new(package: Package, topology: String, config: &'a Config) -> Worker<'a> {
        Worker{
            package: package,
            topology: topology,
            config: config,
            discovery: wonder::actor::Builder::new(discovery::DiscoveryActor).name("discovery".to_string()).start(discovery::Discovery::new(discovery::Backend::Etcd)).unwrap(),
            supervisor_thread: None,
            configuration_thread: None,
            supervisor_id: None,
        }
    }

    /// Prints a preamble for the topology's println statements
    pub fn preamble(&self) -> String {
        format!("{}({})", self.package.name, White.bold().paint("T"))
    }

    /// Join the supervisor thread, and check for errors
    ///
    /// # Failures
    ///
    /// * Supervisor thread fails
    pub fn join_supervisor(&mut self) -> BldrResult<()> {
        let preamble = self.preamble();
        if self.supervisor_thread.is_some() {
            println!("   {}: Waiting for supervisor to finish", preamble);
            let st = self.supervisor_thread.take().unwrap().join();
            match st {
                Ok(result) => {
                    match result {
                        Ok(()) => println!("   {}: Supervisor has finished", preamble),
                        Err(_) => println!("   {}: Supervisor has an error", preamble),
                    }
                },
                Err(e) => println!("Supervisor thread paniced: {:?}", e),
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
fn run_internal<'a>(sm: &mut StateMachine<State, Worker<'a>, BldrError>, worker: &mut Worker<'a>) -> BldrResult<()> {
    let handler = wonder::actor::Builder::new(SignalNotifier).name("signal-handler".to_string()).start(()).unwrap();
    try!(discovery::setup_standard_watches(&worker.discovery, worker.package.clone(), worker.config));
    loop {
        match handler.receiver.try_recv() {
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGHUP))) => {
                println!("   {}: Sending SIGHUP", worker.preamble());
                try!(worker.package.signal(Signal::Hup));
            },
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGINT))) => {
                println!("   {}: Sending 'force-shutdown' on SIGINT", worker.preamble());
                try!(worker.package.signal(Signal::ForceShutdown));
                try!(discovery::stop(&worker.discovery));
                try!(worker.join_supervisor());
                break;
            },
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGQUIT))) => {
                try!(worker.package.signal(Signal::Quit));
                println!("   {}: Sending SIGQUIT", worker.preamble());
            },
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGALRM))) => {
                try!(worker.package.signal(Signal::Alarm));
                println!("   {}: Sending SIGALRM", worker.preamble());
            },
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGTERM))) => {
                println!("   {}: Sending 'force-shutdown' on SIGTERM", worker.preamble());
                try!(worker.package.signal(Signal::ForceShutdown));
                try!(discovery::stop(&worker.discovery));
                try!(worker.join_supervisor());
                break;
            },
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGUSR1))) => {
                println!("   {}: Sending SIGUSR1", worker.preamble());
                try!(worker.package.signal(Signal::One));
            },
            Ok(wonder::actor::Message::Cast(signals::Message::Signal(signals::Signal::SIGUSR2))) => {
                println!("   {}: Sending SIGUSR1", worker.preamble());
                try!(worker.package.signal(Signal::Two));
            },
            Ok(_) => {},
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => {
                panic!("signal handler crashed!");
            },
        }
        match sm.state {
            State::Running => {
                unsafe {
                    let mut status: c_int = 0;
                    let supervisor_pid = worker.supervisor_id.unwrap() as pid_t;
                    match waitpid(supervisor_pid, &mut status, 1 as c_int) {
                        0 => {}, // Nothing returned,
                        pid if pid == supervisor_pid => {
                            if WIFEXITED(status) {
                                let exit_code = WEXITSTATUS(status);
                                println!("   {}: The supervisor died - terminating {} with exit code {}", worker.preamble(), pid, exit_code);
                            } else if WIFSIGNALED(status) {
                                let exit_signal = WTERMSIG(status);
                                println!("   {}: The supervisor died - terminating {} with signal {}", worker.preamble(), pid, exit_signal);
                            } else {
                                println!("   {}: The supervisor over {} died, but I don't know how.", worker.preamble(), pid);
                            }
                            try!(discovery::stop(&worker.discovery));
                            return Err(BldrError::SupervisorDied);
                        },
                        // ZOMBIES! Bad zombies! We listen for zombies. ZOMBOCOM!
                        pid => {
                            if WIFEXITED(status) {
                                let exit_code = WEXITSTATUS(status);
                                debug!("   {}: Process {} died with exit code {}", worker.preamble(), pid, exit_code);
                            } else if WIFSIGNALED(status) {
                                let exit_signal = WTERMSIG(status);
                                debug!("   {}: Process {} terminated with signal {}", worker.preamble(), pid, exit_signal);
                            } else {
                                debug!("   {}: Process {} died, but I don't know how.", worker.preamble(), pid);
                            }
                        }
                    }
                }
            },
            _ => {}
        }
        try!(sm.next(worker));
    }
    try!(SignalNotifier::stop(&handler));
    Ok(())
}
