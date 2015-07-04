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

static CAUGHT_SIGNAL: AtomicBool = ATOMIC_BOOL_INIT;
static WHICH_SIGNAL: AtomicUsize = ATOMIC_USIZE_INIT;

extern "C" {
    fn signal(sig: u32, cb: extern fn(u32)) -> fn(u32);
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
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

pub struct Worker {
    pub package: Package,
    pub topology: String,
    pub discovery: discovery::Discovery,
    pub supervisor_thread: Option<thread::JoinHandle<Result<(), BldrError>>>,
    pub configuration_thread: Option<thread::JoinHandle<Result<(), BldrError>>>,
}

fn run_internal(sm: &mut StateMachine<State, Worker, BldrError>, worker: &mut Worker) -> BldrResult<()> {
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
                    match waitpid(-1 as pid_t, &mut status, 1 as c_int) {
                        0 => {}, // There is nothing to do, nobody has returned
                          _ => { // We don't care why it died - just that it did. It's the only child
                              // we have directly, and it won't leak children unless something has
                              // gone very, very, wrong.
                              println!("   {}: The supervisor died - terminating", worker.preamble());
                              worker.discovery.stop();
                              return Err(BldrError::SupervisorDied);
                          }
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

impl Worker {
    pub fn new(package: Package, topology: String) -> Worker {
        Worker{
            package: package,
            topology: topology,
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
