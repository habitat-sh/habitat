// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

use super::spawned_future::SpawnedFuture;
use crate::{manager::action::ShutdownSpec,
            sys::{service,
                  ShutdownMethod}};
use futures::sync::oneshot;
use habitat_common::outputln;
use habitat_core::os::process::Pid;
use std::thread;

static LOGKEY: &str = "ST"; // "Service Terminator"

/// Shut a service process down.
///
/// This is performed in a separate thread in order to prevent
/// blocking the rest of the Supervisor.
pub fn terminate_service(pid: Pid,
                         service_group: String,
                         shutdown_spec: ShutdownSpec)
                         -> SpawnedFuture<ShutdownMethod> {
    let (tx, rx) = oneshot::channel();

    let handle_result = thread::Builder::new()
        .name(format!("{}-{}", LOGKEY, pid))
        .spawn(move || {
            outputln!(preamble service_group, "Terminating service (PID: {})", pid);
            let shutdown = service::kill(pid, shutdown_spec);
            outputln!(preamble service_group, "{} (PID: {})", shutdown, pid);
            tx.send(shutdown)
                .expect("Couldn't send oneshot signal from terminate_service: receiver went away");
        });

    match handle_result {
        Ok(_handle) => rx.into(),
        Err(io_err) => io_err.into(),
    }
}
