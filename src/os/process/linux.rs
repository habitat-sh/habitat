// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use libc;
use std::ffi::OsString;
use std::path::PathBuf;
use std::ops::Neg;
use std::os::unix::process::CommandExt;
use std::process::{self, Command};
use time::{Duration, SteadyTime};

use error::{Error, Result};

use super::{HabExitStatus, ExitStatusExt, OsSignal, ShutdownMethod, Signal};

pub type Pid = libc::pid_t;
pub type SignalCode = libc::c_int;

impl OsSignal for Signal {
    fn from_signal_code(code: SignalCode) -> Option<Signal> {
        match code {
            libc::SIGINT => Some(Signal::INT),
            libc::SIGILL => Some(Signal::ILL),
            libc::SIGABRT => Some(Signal::ABRT),
            libc::SIGFPE => Some(Signal::FPE),
            libc::SIGKILL => Some(Signal::KILL),
            libc::SIGSEGV => Some(Signal::SEGV),
            libc::SIGTERM => Some(Signal::TERM),
            _ => None,
        }
    }

    fn os_signal(&self) -> SignalCode {
        match *self {
            Signal::INT => libc::SIGINT,
            Signal::ILL => libc::SIGILL,
            Signal::ABRT => libc::SIGABRT,
            Signal::FPE => libc::SIGFPE,
            Signal::KILL => libc::SIGKILL,
            Signal::SEGV => libc::SIGSEGV,
            Signal::TERM => libc::SIGTERM,
            Signal::HUP => libc::SIGHUP,
            Signal::QUIT => libc::SIGQUIT,
            Signal::ALRM => libc::SIGALRM,
            Signal::USR1 => libc::SIGUSR1,
            Signal::USR2 => libc::SIGUSR2,
        }
    }
}

pub fn become_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    become_exec_command(command, args)
}

/// Get process identifier of calling process.
pub fn current_pid() -> u32 {
    unsafe { libc::getpid() as u32 }
}

/// Determines if a process is running with the given process identifier.
pub fn is_alive(pid: Pid) -> bool {
    let process_group_id = unsafe { libc::getpgid(pid) };
    process_group_id >= 0
}

pub fn signal(pid: Pid, signal: Signal) -> Result<()> {
    unsafe {
        match libc::kill(pid, signal.os_signal()) {
            0 => Ok(()),
            e => return Err(Error::SignalFailed(e)),
        }
    }
}

/// Makes an `execvp(3)` system call to become a new program.
///
/// Note that if successful, this function will not return.
///
/// # Failures
///
/// * If the system call fails the error will be returned, otherwise this function does not return
fn become_exec_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    debug!("Calling execvp(): ({:?}) {:?}", command.display(), &args);
    let error_if_failed = Command::new(command).args(&args).exec();
    // The only possible return for the above function is an `Error` so return it, meaning that we
    // failed to exec to our target program
    return Err(error_if_failed.into());
}

pub struct Child {
    pid: Pid,
    last_status: Option<i32>,
}

impl Child {
    pub fn new(child: &mut process::Child) -> Result<Child> {
        Ok(Child {
            pid: child.id() as Pid,
            last_status: None,
        })
    }

    pub fn id(&self) -> Pid {
        self.pid
    }

    pub fn status(&mut self) -> Result<HabExitStatus> {
        match self.last_status {
            Some(status) => Ok(HabExitStatus { status: Some(status as u32) }),
            None => {
                let mut exit_status: i32 = 0;

                match unsafe { libc::waitpid(self.pid as i32, &mut exit_status, libc::WNOHANG) } {
                    0 => Ok(HabExitStatus { status: None }),
                    -1 => {
                        Err(Error::WaitpidFailed(
                            format!("Error calling waitpid on pid: {}", self.pid),
                        ))
                    }
                    _ => {
                        self.last_status = Some(exit_status);
                        Ok(HabExitStatus { status: Some(exit_status as u32) })
                    }
                }
            }
        }
    }

    pub fn kill(&mut self) -> Result<ShutdownMethod> {
        // check the group of the process being killed
        // if it is the root process of the process group
        // we send our signals to the entire process group
        // to prevent orphaned processes.
        let pgid = unsafe { libc::getpgid(self.pid) };
        if self.pid == pgid {
            debug!(
                "pid to kill {} is the process group root. Sending signal to process group.",
                self.pid
            );
            // sending a signal to the negative pid sends it to the
            // entire process group instead just the single pid
            self.pid = self.pid.neg();
        }

        signal(self.pid, Signal::TERM)?;
        let stop_time = SteadyTime::now() + Duration::seconds(8);
        loop {
            if let Ok(status) = self.status() {
                if !status.no_status() {
                    break;
                }
            }
            if SteadyTime::now() > stop_time {
                signal(self.pid, Signal::KILL)?;
                return Ok(ShutdownMethod::Killed);
            }
        }
        Ok(ShutdownMethod::GracefulTermination)
    }
}

impl ExitStatusExt for HabExitStatus {
    fn code(&self) -> Option<u32> {
        unsafe {
            match self.status {
                None => None,
                Some(status) if libc::WIFEXITED(status as libc::c_int) => {
                    Some(libc::WEXITSTATUS(status as libc::c_int) as u32)
                }
                _ => None,
            }
        }
    }

    fn signal(&self) -> Option<u32> {
        unsafe {
            match self.status {
                None => None,
                Some(status) if !libc::WIFEXITED(status as libc::c_int) => {
                    Some(libc::WTERMSIG(status as libc::c_int) as u32)
                }
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use libc;
    use std::process::Command;
    use super::super::*;

    #[test]
    fn running_process_returns_no_exit_status() {
        let mut cmd = Command::new("/bin/bash");
        cmd.arg("-c").arg("while : ; do /bin/sleep 1; done");
        let mut child = cmd.spawn().unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();

        assert!(hab_child.status().unwrap().no_status())
    }

    #[test]
    fn successfully_run_process_exits_zero() {
        let mut cmd = Command::new("/bin/bash");
        cmd.arg("-c").arg("a='b'");
        let mut child = cmd.spawn().unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();
        let mut exit = hab_child.status().unwrap();

        while exit.no_status() {
            exit = hab_child.status().unwrap();
        }

        assert_eq!(exit.code(), Some(0))
    }

    #[test]
    fn terminated_process_returns_sigterm() {
        let mut cmd = Command::new("/bin/bash");
        cmd.arg("-c").arg("while : ; do /bin/sleep 1; done");
        let mut child = cmd.spawn().unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();
        let _ = hab_child.kill();

        let mut exit = hab_child.status().unwrap();
        while exit.no_status() {
            exit = hab_child.status().unwrap();
        }

        assert_eq!(exit.signal(), Some(libc::SIGTERM as u32))
    }

    #[test]
    fn calling_wait_multiple_times_after_exit_returns_same_status() {
        let mut cmd = Command::new("/bin/bash");
        cmd.arg("-c").arg("exit 5");
        let mut child = cmd.spawn().unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();
        let mut exit = hab_child.status().unwrap();

        while exit.no_status() {
            exit = hab_child.status().unwrap();
        }
        let next_exit = hab_child.status().unwrap();

        assert_eq!(next_exit.code(), exit.code())
    }

    #[test]
    fn process_that_exits_with_specific_code_has_same_exit_code() {
        let mut cmd = Command::new("/bin/bash");
        cmd.arg("-c").arg("exit 5");
        let mut child = cmd.spawn().unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();
        let mut exit = hab_child.status().unwrap();

        while exit.no_status() {
            exit = hab_child.status().unwrap();
        }

        assert_eq!(exit.code(), Some(5))
    }
}
