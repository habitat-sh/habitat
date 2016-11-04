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

use libc::{pid_t, c_int};
use std::ffi::OsString;
use std::path::PathBuf;
use std::os::unix::process::CommandExt;
use std::process::Command;

use error::Result;

extern "C" {
    fn kill(pid: i32, sig: u32) -> u32;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}

pub fn become_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    become_exec_command(command, args)
}

pub fn wait_for_exit(pid: u32, status: *mut c_int) -> u32 {
    unsafe { waitpid(pid as i32, status, 1 as c_int) as u32 }
}

/// send a Unix signal to a pid
pub fn send_signal(pid: u32, sig: u32) -> u32 {
    unsafe {
        kill(pid as i32, sig)
    }
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
