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

//! This is the default topology. It's most useful for applications that stand alone, or that don't
//! share state between one another.
//!
//! ![Standalone Topology](../../images/standalone.png)
//!
//! * **Initializing**: Initializes the service by running the `init` hook if present.
//! * **Starting**: Starts the service under `runsv`, and starts a thread to process output and
//! handle errors.
//! * **Running**: The state for the 'normal' operating condition.

use error::{Result, SupError};
use package::Package;
use state_machine::StateMachine;
use topology::{self, State, Worker};

/// Sets up the topology and calls run_internal.
///
/// Add's the state transitions to the state machine, sets up the signal handlers, and runs the
/// `topology::run_internal` function.
pub fn run(package: Package) -> Result<()> {
    let mut worker = try!(Worker::new(package, String::from("standalone")));
    let mut sm: StateMachine<State, Worker, SupError> = StateMachine::new(State::Initializing);
    sm.add_dispatch(State::Initializing, state_initializing);
    sm.add_dispatch(State::Starting, state_starting);
    sm.add_dispatch(State::Running, state_running);
    topology::run_internal(&mut sm, &mut worker)
}

/// Initialize the service.
pub fn state_initializing(worker: &mut Worker) -> Result<(State, u64)> {
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
/// 1. Starts the package `run` script
/// 1. Spawns the supervisor thread
///
/// # Failures
///
/// * If we cannot find the package
/// * If we cannot start the supervisor
pub fn state_starting(worker: &mut Worker) -> Result<(State, u64)> {
    {
        let mut supervisor = worker.supervisor.write().unwrap();
        try!(supervisor.start());
    }
    Ok((State::Running, 0))
}

pub fn state_running(worker: &mut Worker) -> Result<(State, u64)> {
    if let Some(state) = worker.return_state {
        Ok((state, 0))
    } else {
        Ok((State::Running, 0))
    }
}
