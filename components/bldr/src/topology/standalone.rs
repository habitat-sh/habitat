// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! This is the default topology. It's most useful for applications that stand alone, or that don't
//! share state between one another.
//!
//! ![Standalone Topology](../../images/standalone.png)
//!
//! * **Initializing**: Initializes the service by running the `init` hook if present.
//! * **Starting**: Starts the service under `runsv`, and starts a thread to process output and
//! handle errors.
//! * **Running**: The state for the 'normal' operating condition.

use std::thread;
use std::process::{Command, Stdio, Child};
use std::io::prelude::*;

use core::fs::SERVICE_HOME;

use error::{BldrResult, BldrError, ErrorKind};
use package::Package;
use state_machine::StateMachine;
use topology::{self, State, Worker, ChildInfo};
use config::Config;
use time;

static LOGKEY: &'static str = "TS";

/// Sets up the topology and calls run_internal.
///
/// Add's the state transitions to the state machine, sets up the signal handlers, and runs the
/// `topology::run_internal` function.
pub fn run(package: Package, config: &Config) -> BldrResult<()> {
    let mut worker = try!(Worker::new(package, String::from("standalone"), config));
    let mut sm: StateMachine<State, Worker, BldrError> = StateMachine::new(State::Initializing);
    sm.add_dispatch(State::Initializing, state_initializing);
    sm.add_dispatch(State::Starting, state_starting);
    sm.add_dispatch(State::Running, state_running);
    topology::run_internal(&mut sm, &mut worker)
}

/// Initialize the service.
pub fn state_initializing(worker: &mut Worker) -> BldrResult<(State, u64)> {
    let service_config = worker.service_config.read().unwrap();
    let package = worker.package.read().unwrap();
    match package.initialize(&service_config) {
        Ok(()) => Ok((State::Starting, 0)),
        Err(e) => Err(e),
    }
}


/// Consume output from a child process until EOF, then finish
fn child_reader(child: &mut Child, package_name: String) -> BldrResult<()> {
    let mut c_stdout = match child.stdout {
        Some(ref mut s) => s,
        None => return Err(bldr_error!(ErrorKind::UnpackFailed)),
    };

    let mut line = output_format!(preamble &package_name, logkey "SO");
    loop {
        let mut buf = [0u8; 1]; // Our byte buffer
        let len = try!(c_stdout.read(&mut buf));
        match len {
            0 => {
                // 0 == EOF, so stop writing and finish progress
                break;
            }
            _ => {
                // Write the buffer to the BufWriter on the Heap
                let buf_string = String::from_utf8_lossy(&buf[0 .. len]);
                line.push_str(&buf_string);
                if line.contains("\n") {
                    print!("{}", line);
                    line = output_format!(preamble &package_name, logkey "O");
                }
            }
        }
    }
    debug!("child_reader exiting");
    Ok(())
}

/// Start the service.
///
/// 1. Finds the package
/// 1. Starts the package `run` script
/// 1. Spawns the supervisor thread
///
/// # Failures
///
/// * If we cannot find the package
/// * If we cannot start the supervisor
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn state_starting(worker: &mut Worker) -> BldrResult<(State, u64)> {
    outputln!(preamble &worker.package_name, "Starting");
    let package_name = worker.package_name.clone();
    let service_dir = format!("{}/{}", SERVICE_HOME, worker.package_name);
    // call the "run" script in the specified package
    let cmd = format!("{}/run", service_dir);

    // create a PID file so we can find the supervisor by
    // package name elsewhere in the code.
    let package = worker.package.read().unwrap();

    // if we are restarting, then remove the previous pidfile just to
    // be a good citizen
    try!(package.cleanup_pidfile());

    let mut child = try!(Command::new(cmd)
                         .stdin(Stdio::null())
                         .stdout(Stdio::piped())
                         .stderr(Stdio::piped())
                         .spawn());
    let child_info = ChildInfo {pid: child.id(), start_time: time::now().to_timespec()};
    worker.child_info = Some(child_info);
    try!(package.create_pidfile(child.id()));
    let supervisor_thread = try!(thread::Builder::new()
                                 .name(String::from("supervisor"))
                                 .spawn(move || -> BldrResult<()> {
                                    child_reader(&mut child, package_name)
                                 }));
    worker.supervisor_thread = Some(supervisor_thread);
    Ok((State::Running, 0))
}

pub fn state_running(worker: &mut Worker) -> BldrResult<(State, u64)> {
    if let Some(state) = worker.return_state {
        Ok((state,0))
    } else {
        Ok((State::Running, 0))
    }
}
