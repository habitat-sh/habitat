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

use libc::c_int;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{self, Command};

use error::Result;

pub fn become_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    become_child_command(command, args)
}

//TODO: REALLY wait for exit
pub fn wait_for_exit(pid: u32, status: *mut c_int) -> u32 {
    0
}

pub fn send_signal(pid: u32, sig: u32) -> u32 {
    unimplemented!();
}

/// Executes a command as a child process and exits with the child's exit code.
///
/// Note that if sucessful, this function will not return.
///
/// # Failures
///
/// * If the child process cannot be created
fn become_child_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    debug!("Calling child process: ({:?}) {:?}",
           command.display(),
           &args);
    let status = try!(Command::new(command).args(&args).status());
    // Let's honor the exit codes from the child process we finished running
    process::exit(status.code().unwrap())
}
