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

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT, ATOMIC_BOOL_INIT};
use libc::{pid_t, c_int};
use ansi_term::Colour::White;
use std::process::{Command, Stdio, Child};
use std::io::prelude::*;
use std::thread;

use error::{BldrResult, BldrError};
use pkg::{self, Signal, Package};

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


pub fn set_signal_handlers() {
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

pub struct Statemachine<'a, T> {
    state: T,
    delay: Option<u32>,
    package: Package,
    supervisor_thread: Option<thread::JoinHandle<Result<(), BldrError>>>,
    dispatch: 'a ||
}

trait Topology {
    fn dispatch(&mut self) -> BldrResult<()>;

    fn next(&mut self) -> BldrResult<()> {
        if let Some(interval) = self.delay {
           thread::sleep_ms(interval);
        }

        let result = try!(self.dispatch());
        result
    }

    fn set_state<T: TopoState>(&mut self, state: T, delay: Option<u32>) {
        self.state = state;
        self.delay = delay;
    }

    fn preamble(&mut self) -> String {
        format!("{}({} {:?})", self.package.name, White.bold().paint("T"), self.state)
    }

    fn join_supervisor(&mut self) -> BldrResult<()> {
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

    fn run(&mut self) -> BldrResult<()> {
        loop {
            if CAUGHT_SIGNAL.load(Ordering::SeqCst) {
                match WHICH_SIGNAL.load(Ordering::SeqCst) {
                    1 => { // SIGHUP
                        println!("   {}: Sending SIGHUP", self.preamble());
                        try!(self.package.signal(Signal::Hup));
                    },
                    2 => { // SIGINT
                        println!("   {}: Sending 'force-shutdown' on SIGINT", self.preamble());
                        try!(self.package.signal(Signal::ForceShutdown));
                        self.join_supervisor();
                        break;
                    },
                    3 => { // SIGQUIT
                        try!(self.package.signal(Signal::Quit));
                        println!("   {}: Sending SIGQUIT", self.preamble());
                    },
                    14 => { // SIGALRM
                        try!(self.package.signal(Signal::Alarm));
                        println!("   {}: Sending SIGALRM", self.preamble());
                    },
                    15 => { // SIGTERM
                        println!("   {}: Sending 'force-shutdown' on SIGTERM", self.preamble());
                        try!(self.package.signal(Signal::ForceShutdown));
                        self.join_supervisor();
                        break;
                    },
                    30 => { //    SIGUSR1      terminate process    User defined signal 1
                        println!("   {}: Sending SIGUSR1", self.preamble());
                        try!(self.package.signal(Signal::One));
                    },
                    31 => { //    SIGUSR2      terminate process    User defined signal 25
                        println!("   {}: Sending SIGUSR1", self.preamble());
                        try!(self.package.signal(Signal::Two));
                    },
                    _ => unreachable!()
                }
                // Reset the signal handler flags
                CAUGHT_SIGNAL.store(false, Ordering::SeqCst);
                WHICH_SIGNAL.store(0 as usize, Ordering::SeqCst);
            }
            match self.state {
                State::Running => {
                    unsafe {
                        let mut status: c_int = 0;
                        match waitpid(-1 as pid_t, &mut status, 1 as c_int) {
                            0 => {}, // There is nothing to do, nobody has returned
                              _ => { // We don't care why it died - just that it did. It's the only child
                                  // we have directly, and it won't leak children unless something has
                                  // gone very, very, wrong.
                                  println!("   {}: The supervisor died - terminating", self.preamble());
                                  return Err(BldrError::SupervisorDied);
                              }
                        }
                    }
                },
                _ => {}
            }
            try!(self.next());
        }
        Ok(())
    }

    fn state_starting(&mut self) -> BldrResult<()> {
        println!("   {}: Starting", self.preamble());
        let busybox_pkg = try!(pkg::latest("busybox"));
        let mut child = try!(
            Command::new(busybox_pkg.join_path("bin/runsv"))
            .arg(&format!("/opt/bldr/srvc/{}", self.package.name))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        );
        let pkg = self.package.name.clone();
        let supervisor_thread = thread::spawn(move|| -> BldrResult<()> {
            {
                let mut c_stdout = match child.stdout {
                    Some(ref mut s) => s,
                    None => return Err(BldrError::UnpackFailed)
                };

                let mut line = format!("   {}({}): ", pkg, White.bold().paint("O"));
                loop {
                    let mut buf = [0u8; 1]; // Our byte buffer
                    let len = try!(c_stdout.read(&mut buf));
                    match len {
                        0 => { // 0 == EOF, so stop writing and finish progress
                            break;
                        },
                        _ => { // Write the buffer to the BufWriter on the Heap
                            let buf_vec = buf[0 .. len].to_vec();
                            let buf_string = String::from_utf8(buf_vec).unwrap();
                            line.push_str(&buf_string);
                            if line.contains("\n") {
                                print!("{}", line);
                                line = format!("   {}({}): ", pkg, White.bold().paint("O"));
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        self.supervisor_thread = Some(supervisor_thread);
        self.set_state(State::Running, None);
        Ok(())
    }

    fn state_running(&mut self) -> BldrResult<()> {
        self.set_state(State::Running, None);
        Ok(())
    }
}
