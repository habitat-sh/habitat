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

use std::ffi::OsString;
use std::path::PathBuf;
use std::os::unix::process::CommandExt;
use std::process::Command;

use error::Result;

pub fn become_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    become_exec_command(command, args)
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
