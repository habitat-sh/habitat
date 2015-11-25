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
use std::process::{Command, Stdio};
use std::io::prelude::*;

use fs::SERVICE_HOME;
use error::{BldrResult, BldrError, ErrorKind};
use package::Package;
use state_machine::StateMachine;
use topology::{self, State, Worker};
use config::Config;

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
pub fn state_initializing(worker: &mut Worker) -> BldrResult<(State, u32)> {
    let service_config = worker.service_config.read().unwrap();
    let package = worker.package.read().unwrap();
    match package.initialize(&service_config) {
        Ok(()) => Ok((State::Starting, 0)),
        Err(e) => Err(e),
    }
}

/// Start the service.
///
/// 1. Finds the package
/// 1. Starts runit for the `run` script
/// 1. Spawns the supervisor thread
///
/// # Failures
///
/// * If we cannot find the package
/// * If we cannot start the supervisor
pub fn state_starting(worker: &mut Worker) -> BldrResult<(State, u32)> {
    outputln!(P: &worker.package_name, "Starting");
    let package = worker.package_name.clone();
    let runit_pkg = try!(Package::latest("chef", "runit", None, None));
    let mut child = try!(Command::new(runit_pkg.join_path("bin/runsv"))
                             .arg(&format!("{}/{}", SERVICE_HOME, worker.package_name))
                             .stdin(Stdio::null())
                             .stdout(Stdio::piped())
                             .stderr(Stdio::piped())
                             .spawn());
    worker.supervisor_id = Some(child.id());
    let supervisor_thread = try!(thread::Builder::new()
                 .name(String::from("supervisor"))
                 .spawn(move || -> BldrResult<()> {
                     {
                         let mut c_stdout = match child.stdout {
                             Some(ref mut s) => s,
                             None => return Err(bldr_error!(ErrorKind::UnpackFailed)),
                         };

                         let mut line = output_format!(P: &package, L: "SO");
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
                                     let buf_vec = buf[0..len].to_vec();
                                     let buf_string = String::from_utf8(buf_vec).unwrap();
                                     line.push_str(&buf_string);
                                     if line.contains("\n") {
                                         print!("{}", line);
                                         line = output_format!(P: &package, L: "O");
                                     }
                                 }
                             }
                         }
                     }
                     Ok(())
                 }));
    worker.supervisor_thread = Some(supervisor_thread);
    Ok((State::Running, 0))
}

pub fn state_running(_worker: &mut Worker) -> BldrResult<(State, u32)> {
    Ok((State::Running, 0))
}
