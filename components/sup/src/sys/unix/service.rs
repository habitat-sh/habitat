// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

// NOTE: All this code is basically copied verbatim from its previous home in
// the Launcher module. Once all the service-related functionality that we're
// going to move over to the Supervisor has been moved, we can take a look at
// perhaps refactoring some of this a bit.

use crate::{manager::action::ShutdownSpec,
            sys::ShutdownMethod};
use habitat_core::os::process::{is_alive,
                                signal,
                                Pid,
                                Signal};
use libc::{self,
           pid_t};
use std::{ops::Neg,
          thread,
          time::Duration as StdDuration};
use time::{Duration as TimeDuration,
           SteadyTime};

/// Kill a service process.
pub fn kill(pid: Pid, shutdown_spec: ShutdownSpec) -> ShutdownMethod {
    let process = Process::new(pid);
    process.kill(shutdown_spec)
}

///////////////////////////////////////////////////////////////////////
// Private Code

// TODO (CM): We may not want this struct in the end... keeping it for
// now to keep some parity with the Windows implementation. Once we
// pull over all the "service" functionality from the Launcher, we can
// re-evaluate.
struct Process {
    pid: pid_t,
}

impl Process {
    fn new(pid: Pid) -> Self { Process { pid } }

    /// Attempt to gracefully terminate a proccess and then forcefully
    /// kill it after 8 seconds if it has not terminated.
    fn kill(&self, shutdown_spec: ShutdownSpec) -> ShutdownMethod {
        let ShutdownSpec { signal: shutdown_signal,
                           timeout, } = shutdown_spec;
        let shutdown_signal = shutdown_signal.into();

        let mut pid_to_kill = self.pid;
        // check the group of the process being killed
        // if it is the root process of the process group
        // we send our signals to the entire process group
        // to prevent orphaned processes.

        let pgid = unsafe { libc::getpgid(self.pid) };
        if self.pid == pgid {
            debug!("pid to kill {} is the process group root. Sending signal to process group.",
                   self.pid);
            // sending a signal to the negative pid sends it to the
            // entire process group instead just the single pid
            pid_to_kill = self.pid.neg();
        }

        // JW TODO: Determine if the error represents a case where the
        // process was already exited before we return out and assume
        // so.
        trace!("Sending {:?} signal to process {}",
               shutdown_signal,
               pid_to_kill);
        if signal(pid_to_kill, shutdown_signal).is_err() {
            return ShutdownMethod::AlreadyExited;
        }

        let timeout: TimeDuration = timeout.into();
        trace!("Waiting up to {} seconds before sending KILL to process {}",
               timeout.num_seconds(),
               pid_to_kill);
        let stop_time = SteadyTime::now() + timeout;
        loop {
            if !is_alive(pid_to_kill) {
                return ShutdownMethod::GracefulTermination;
            }
            if SteadyTime::now() >= stop_time {
                break;
            }
            thread::sleep(StdDuration::from_millis(5));
        }

        trace!("Timeout exceeded; killing process {}", pid_to_kill);
        match signal(pid_to_kill, Signal::KILL) {
            Ok(_) => ShutdownMethod::Killed,
            Err(_) => {
                // JW TODO: Determine if the error represents a case
                // where the process was already exited before we
                // return out and assume so.
                ShutdownMethod::GracefulTermination
            }
        }
    }
}
