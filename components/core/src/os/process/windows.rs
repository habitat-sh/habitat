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

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{self, Command};
use std::ptr;
use std::io;

use kernel32;
use winapi;

use error::{Error, Result};
use super::{OsSignal, Signal};

const STILL_ACTIVE: u32 = 259;

pub type Pid = winapi::DWORD;
pub type SignalCode = winapi::DWORD;

impl OsSignal for Signal {
    fn from_signal_code(code: SignalCode) -> Option<Signal> {
        None
    }

    fn os_signal(&self) -> SignalCode {
        0
    }
}

pub fn become_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    become_child_command(command, args)
}

/// Get process identifier of calling process.
pub fn current_pid() -> u32 {
    unsafe { kernel32::GetCurrentProcessId() as u32 }
}

pub fn handle_from_pid(pid: Pid) -> Option<winapi::HANDLE> {
    unsafe {
        let proc_handle = kernel32::OpenProcess(
            winapi::PROCESS_QUERY_LIMITED_INFORMATION | winapi::PROCESS_TERMINATE,
            winapi::FALSE,
            pid,
        );

        // we expect this to happen if the process died
        // before OpenProcess completes
        if proc_handle == ptr::null_mut() {
            return None;
        } else {
            return Some(proc_handle);
        }
    }
}

/// Determines if a process is running with the given process identifier.
pub fn is_alive(pid: Pid) -> bool {
    match handle_from_pid(pid) {
        Some(handle) => {
            let exit_status = exit_status(handle).expect("Failed to get exit status");
            unsafe {
                let _ = kernel32::CloseHandle(handle);
            }
            exit_status == STILL_ACTIVE
        }
        None => false,
    }
}

pub fn signal(pid: Pid, signal: Signal) -> Result<()> {
    debug!(
        "sending no-op(windows) signal {} to pid {}",
        signal.os_signal(),
        pid
    );
    Ok(())
}

/// Executes a command as a child process and exits with the child's exit code.
///
/// Note that if successful, this function will not return.
///
/// # Failures
///
/// * If the child process cannot be created
fn become_child_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    debug!(
        "Calling child process: ({:?}) {:?}",
        command.display(),
        &args
    );
    let status = Command::new(command).args(&args).status()?;
    // Let's honor the exit codes from the child process we finished running
    process::exit(status.code().unwrap())
}

fn exit_status(handle: winapi::HANDLE) -> Result<u32> {
    let mut exit_status: u32 = 0;

    unsafe {
        let ret = kernel32::GetExitCodeProcess(handle, &mut exit_status as winapi::LPDWORD);
        if ret == 0 {
            return Err(Error::GetExitCodeProcessFailed(format!(
                "Failed to retrieve Exit Code: {}",
                io::Error::last_os_error()
            )));
        }
    }

    Ok(exit_status)
}
