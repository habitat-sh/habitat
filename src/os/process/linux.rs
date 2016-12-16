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

use libc;
use std::ffi::OsString;
use std::path::PathBuf;
use std::os::unix::process::CommandExt;
use std::process::{self, Command};

use error::{Error, Result};

use super::{HabExitStatus, ExitStatusExt};

extern "C" {
    fn kill(pid: i32, sig: u32) -> u32;
    fn waitpid(pid: libc::pid_t, status: *mut libc::c_int, options: libc::c_int) -> libc::pid_t;
}

pub fn become_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    become_exec_command(command, args)
}

/// send a Unix signal to a pid
pub fn send_signal(pid: u32, sig: u32) -> u32 {
    unsafe { kill(pid as i32, sig) }
}

/// Makes an `execvp(3)` system call to become a new program.
///
/// Note that if sucessful, this function will not return.
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
    pid: u32,
}

impl Child {
    pub fn new(child: &mut process::Child) -> Result<Child> {
        Ok(Child { pid: child.id() })
    }

    pub fn id(&self) -> u32 {
        self.pid
    }

    pub fn status(&mut self) -> Result<HabExitStatus> {
        let mut exit_status: i32 = 0;

        match unsafe { waitpid(self.pid as i32, &mut exit_status, libc::WNOHANG) } {
            0 => Ok(HabExitStatus { status: None }),
            -1 => Err(Error::WaitpidFailed(format!("Error calling waitpid on pid: {}", self.pid))),
            _ => Ok(HabExitStatus { status: Some(exit_status as u32) }),
        }
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
    fn succesfully_run_process_exits_zero() {
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
    fn terminated_process_returns_non_zero_exit() {
        let mut cmd = Command::new("/bin/bash");
        cmd.arg("-c").arg("while : ; do /bin/sleep 1; done");
        let mut child = cmd.spawn().unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();
        let _ = child.kill();

        let mut exit = hab_child.status().unwrap();
        while exit.no_status() {
            exit = hab_child.status().unwrap();
        }

        assert_eq!(exit.signal(), Some(libc::SIGKILL as u32))
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
