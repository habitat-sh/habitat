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

use error::{BldrResult, BldrError};
use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::thread;
use discovery;
use pkg::{self, Signal};
use ansi_term::Colour::White;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT, ATOMIC_BOOL_INIT};
use libc::{pid_t, c_int};

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

/// Starts the given package.
///
/// * Registers signal handlers
/// * Starts runsv with the "run" script
/// * If discovery is enabled, spawn a thread and block on configuring
/// * Spawn a thread that launches the supervisor, then reads stdout
///   into a (very small) buffer, and prints the output
/// * Loop checking the CAUGHT_SIGNAL boolean. If we catch a signal,
///   take the appropriate action. If that involves shutting down
///   the supervisor, block on joining that thread, and shut down.
///   Then wait for your child (we should have exactly 1) to return.
/// * Return ()
pub fn package(package: &str) -> BldrResult<()> {
    // Set up all the signal handlers
    unsafe {
         signal(1, handle_signal);  //    SIGHUP       terminate process    terminal line hangup
         signal(2, handle_signal);  //    SIGINT       terminate process    interrupt program
         signal(3, handle_signal);  //    SIGQUIT      create core image    quit program
         signal(14, handle_signal); //    SIGALRM      terminate process    real-time timer expired
         signal(15, handle_signal); //    SIGTERM      terminate process    software termination signal
         signal(30, handle_signal); //    SIGUSR1      terminate process    User defined signal 1
         signal(31, handle_signal); //    SIGUSR2      terminate process    User defined signal 2
    }
    println!("   {}: Starting", package);
    let busybox_pkg = try!(pkg::latest("busybox"));
    let mut child = try!(
        Command::new(busybox_pkg.join_path("bin/runsv"))
        .arg(&format!("/opt/bldr/srvc/{}", package))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    );

    let pkg_name = package.to_string();

    match discovery::etcd::enabled() {
        Some(_) => {
            thread::spawn(move || -> BldrResult<()> {
                let package = try!(pkg::latest(&pkg_name));
                loop {
                    try!(package.config_data(true));
                    println!("   {}({}): Waiting 30 seconds before reconfiguring", pkg_name, White.bold().paint("C"));
                    thread::sleep_ms(30000);
                }
            });
        },
        None => {}
    }

    let pkg_names = package.to_string();
    let supervisor_thread = thread::spawn(move|| -> BldrResult<()> {
        {
            let mut c_stdout = match child.stdout {
                Some(ref mut s) => s,
                None => return Err(BldrError::UnpackFailed)
            };

            let mut line = format!("   {}({}): ", pkg_names, White.bold().paint("O"));
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
                            line = format!("   {}({}): ", pkg_names, White.bold().paint("O"));
                        }
                    }
                }
            }
        }
        Ok(())
    });
    let current_pkg = try!(pkg::latest(package));
    loop {
        if CAUGHT_SIGNAL.load(Ordering::SeqCst) {
            match WHICH_SIGNAL.load(Ordering::SeqCst) {
                1 => { // SIGHUP
                    println!("   {}: Sending SIGHUP", package);
                    try!(current_pkg.signal(Signal::Hup));
                },
                2 => { // SIGINT
                    println!("   {}: Sending 'force-shutdown' on SIGINT", package);
                    try!(current_pkg.signal(Signal::ForceShutdown));
                    println!("   {}: Waiting for supervisor to finish", package);
                    match supervisor_thread.join() {
                        Ok(result) => {
                            match result {
                                Ok(()) => println!("   {}: Supervisor has finished", package),
                                Err(_) => println!("   {}: Supervisor has an error", package),
                            }
                        },
                        Err(e) => println!("Supervisor thread paniced: {:?}", e),
                    }
                    break;
                },
                3 => { // SIGQUIT
                    try!(current_pkg.signal(Signal::Quit));
                    println!("   {}: Sending SIGQUIT", package);
                },
                14 => { // SIGALRM
                    try!(current_pkg.signal(Signal::Alarm));
                    println!("   {}: Sending SIGALRM", package);
                },
                15 => { // SIGTERM
                    println!("   {}: Sending 'force-shutdown' on SIGTERM", package);
                    try!(current_pkg.signal(Signal::ForceShutdown));
                    println!("   {}: Waiting for supervisor to finish", package);
                    match supervisor_thread.join() {
                        Ok(result) => {
                            match result {
                                Ok(()) => println!("   {}: Supervisor has finished", package),
                                Err(_) => println!("   {}: Supervisor has an error", package),
                            }
                        },
                        Err(e) => println!("Supervisor thread paniced: {:?}", e),
                    }
                    break;
                },
                30 => { //    SIGUSR1      terminate process    User defined signal 1
                    println!("   {}: Sending SIGUSR1", package);
                    try!(current_pkg.signal(Signal::One));
                },
                31 => { //    SIGUSR2      terminate process    User defined signal 25
                    println!("   {}: Sending SIGUSR1", package);
                    try!(current_pkg.signal(Signal::Two));
                },
                _ => unreachable!()
            }
            // Reset the signal handler flags
            CAUGHT_SIGNAL.store(false, Ordering::SeqCst);
            WHICH_SIGNAL.store(0 as usize, Ordering::SeqCst);
        }
        unsafe {
            let mut status: c_int = 0;
            match waitpid(-1 as pid_t, &mut status, 1 as c_int) {
                0 => {}, // There is nothing to do, nobody has returned
                _ => { // We don't care why it died - just that it did. It's the only child
                       // we have directly, and it won't leak children unless something has
                       // gone very, very, wrong.
                    println!("   {}: The supervisor died - terminating", package);
                    return Err(BldrError::SupervisorDied);
                }
            }
        }
        thread::sleep_ms(1000);
    }
    Ok(())
}

