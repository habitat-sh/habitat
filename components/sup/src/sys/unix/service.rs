// NOTE: All this code is basically copied verbatim from its previous home in
// the Launcher module. Once all the service-related functionality that we're
// going to move over to the Supervisor has been moved, we can take a look at
// perhaps refactoring some of this a bit.

use crate::sys::ShutdownMethod;
use habitat_core::os::process::{is_alive,
                                signal,
                                Pid,
                                Signal};
use libc::{self,
           pid_t};
use std::{ops::Neg,
          thread,
          time::Duration as StdDuration};
use time::{Duration,
           SteadyTime};

/// Kill a service process.
pub fn kill(pid: Pid) -> ShutdownMethod {
    let process = Process::new(pid);
    process.kill()
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
    fn kill(&self) -> ShutdownMethod {
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
        if signal(pid_to_kill, Signal::TERM).is_err() {
            return ShutdownMethod::AlreadyExited;
        }

        let stop_time = SteadyTime::now() + Duration::seconds(8);
        loop {
            if !is_alive(pid_to_kill) {
                return ShutdownMethod::GracefulTermination;
            }
            if SteadyTime::now() >= stop_time {
                break;
            }
            thread::sleep(StdDuration::from_millis(5));
        }

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
