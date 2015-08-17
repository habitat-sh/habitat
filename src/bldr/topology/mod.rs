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

pub mod standalone;
pub mod leader;
pub mod watcher;

use ansi_term::Colour::White;
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT, ATOMIC_BOOL_INIT};
use libc::{pid_t, c_int};

use state_machine::StateMachine;
use discovery;
use pkg::{Package, Signal};
use error::{BldrResult, BldrError};
use config::Config;
use regex::Regex;

use std::process::Command;

static CAUGHT_SIGNAL: AtomicBool = ATOMIC_BOOL_INIT;
static WHICH_SIGNAL: AtomicUsize = ATOMIC_USIZE_INIT;

extern "C" {
    fn signal(sig: u32, cb: extern fn(u32)) -> fn(u32);
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}

#[allow(non_camel_case_types)]
pub type idtype_t = c_int;
pub const P_ALL:  idtype_t = 0;
pub const P_PID:  idtype_t = 1;
pub const P_PGID: idtype_t = 2;

pub const WCONTINUED: c_int = 8;
pub const WNOHANG:    c_int = 1;
pub const WUNTRACED:  c_int = 2;
pub const WEXITED:    c_int = 4;
pub const WNOWAIT:    c_int = 16777216;
pub const WSTOPPED:   c_int = 2;

#[allow(non_snake_case)]
pub fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[allow(non_snake_case)]
pub fn WIFCONTINUED(status: c_int) -> bool {
    status == 0xffff
}

#[allow(non_snake_case)]
pub fn WIFEXITED(status: c_int) -> bool {
    WTERMSIG(status) == 0
}

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

extern fn handle_signal(sig: u32) {
    CAUGHT_SIGNAL.store(true, Ordering::SeqCst);
    WHICH_SIGNAL.store(sig as usize, Ordering::SeqCst);
}

fn set_signal_handlers() {
    unsafe {
        signal(1, handle_signal);  //    SIGHUP       terminate process    terminal line hangup
        signal(2, handle_signal);  //    SIGINT       terminate process    interrupt program
        signal(3, handle_signal);  //    SIGQUIT      create core image    quit program
        signal(14, handle_signal); //    SIGALRM      terminate process    real-time timer expired
        signal(15, handle_signal); //    SIGTERM      terminate process    software termination signal
        signal(30, handle_signal); //    SIGUSR1      terminate process    User defined signal 1
        signal(31, handle_signal); //    SIGUSR2      terminate process    User defined signal 2
    }
}

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

pub struct Worker<'a> {
    pub package: Package,
    pub config: &'a Config,
    pub topology: String,
    pub discovery: discovery::Discovery,
    pub supervisor_thread: Option<thread::JoinHandle<Result<(), BldrError>>>,
    pub configuration_thread: Option<thread::JoinHandle<Result<(), BldrError>>>,
}

fn run_internal<'a>(sm: &mut StateMachine<State, Worker<'a>, BldrError>, worker: &mut Worker<'a>) -> BldrResult<()> {
    loop {
        if CAUGHT_SIGNAL.load(Ordering::SeqCst) {
            match WHICH_SIGNAL.load(Ordering::SeqCst) {
                1 => { // SIGHUP
                    println!("   {}: Sending SIGHUP", worker.preamble());
                    try!(worker.package.signal(Signal::Hup));
                },
                2 => { // SIGINT
                    println!("   {}: Sending 'force-shutdown' on SIGINT", worker.preamble());
                    try!(worker.package.signal(Signal::ForceShutdown));
                    worker.discovery.stop();
                    try!(worker.join_supervisor());
                    break;
                },
                3 => { // SIGQUIT
                    try!(worker.package.signal(Signal::Quit));
                    println!("   {}: Sending SIGQUIT", worker.preamble());
                },
                14 => { // SIGALRM
                    try!(worker.package.signal(Signal::Alarm));
                    println!("   {}: Sending SIGALRM", worker.preamble());
                },
                15 => { // SIGTERM
                    println!("   {}: Sending 'force-shutdown' on SIGTERM", worker.preamble());
                    try!(worker.package.signal(Signal::ForceShutdown));
                    worker.discovery.stop();
                    try!(worker.join_supervisor());
                    break;
                },
                30 => { //    SIGUSR1      terminate process    User defined signal 1
                    println!("   {}: Sending SIGUSR1", worker.preamble());
                    try!(worker.package.signal(Signal::One));
                },
                31 => { //    SIGUSR2      terminate process    User defined signal 25
                    println!("   {}: Sending SIGUSR1", worker.preamble());
                    try!(worker.package.signal(Signal::Two));
                },
                _ => unreachable!()
            }
            // Reset the signal handler flags
            CAUGHT_SIGNAL.store(false, Ordering::SeqCst);
            WHICH_SIGNAL.store(0 as usize, Ordering::SeqCst);
        }
        match sm.state {
            State::Running => {
                unsafe {
                    let mut status: c_int = 0;
                    // As soon as we can get the pid of the child we spawned, we
                    // really want this to be a specific wait. Right now, a badly
                    // behaved daemon will trigger a race for the reaping, and we
                    // will likely exit inappropriately.
                    match waitpid(0 as pid_t, &mut status, 1 as c_int) {
                        0 => {}, // There is nothing to do, nobody has returned
                        pid => { // We don't care why it died - just that it did. It's the only child
                              // we have directly, and it won't leak children unless something has
                              // gone very, very, wrong.
                              let output = try!(Command::new("ps").arg("wl").output());
                              let re = Regex::new(r"runsv").unwrap();
                              let stdout = String::from_utf8_lossy(&output.stdout);
                              if ! re.is_match(&stdout) {
                                  if WIFEXITED(status) {
                                      let exit_code = WEXITSTATUS(status);
                                      println!("   {}: The supervisor died - terminating {} with exit code {}", worker.preamble(), pid, exit_code);
                                  } else if WIFSIGNALED(status) {
                                      let exit_signal = WTERMSIG(status);
                                      println!("   {}: The supervisor died - terminating {} with signal {}", worker.preamble(), pid, exit_signal);
                                  } else {
                                      println!("   {}: The supervisor over {} died, but I don't know how.", worker.preamble(), pid);
                                  }
                                  worker.discovery.stop();
                                  return Err(BldrError::SupervisorDied);
                              }
                          },
                    }
                }
            },
            _ => {}
        }
        try!(worker.discovery.next());
        try!(sm.next(worker));
        thread::sleep_ms(100);
    }
    Ok(())
}

impl<'a> Worker<'a> {
    pub fn new(package: Package, topology: String, config: &'a Config) -> Worker<'a> {
        Worker{
            package: package,
            topology: topology,
            config: config,
            discovery: discovery::Discovery::new(discovery::Backend::Etcd),
            supervisor_thread: None,
            configuration_thread: None,
        }
    }

    pub fn preamble(&self) -> String {
        format!("{}({})", self.package.name, White.bold().paint("T"))
    }

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
