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

use std::collections::HashMap;
use std::ffi::OsString;
use std::mem;
use std::path::PathBuf;
use std::process::{self, Command};
use std::ptr;
use std::io;
use time::{Duration, SteadyTime};

use kernel32;
use winapi;

use error::{Error, Result};

use super::windows_child;
use super::{HabExitStatus, ExitStatusExt, ShutdownMethod, OsSignal, Signal};

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
    let status = try!(Command::new(command).args(&args).status());
    // Let's honor the exit codes from the child process we finished running
    process::exit(status.code().unwrap())
}

fn handle_from_pid(pid: Pid) -> Option<winapi::HANDLE> {
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

pub struct Child {
    handle: Option<winapi::HANDLE>,
    last_status: Option<u32>,
    pid: u32,
}

impl Child {
    // On windows we need the process handle to capture status
    // Here we will attempt to get the handle from the pid but if the
    // process dies before we can get it, we will just wait() on the
    // std::process::Child and cache the exit_status which we will return
    // when status is called.
    pub fn new(child: &mut windows_child::Child) -> Result<Child> {
        let (win_handle, status) = match handle_from_pid(child.id()) {
            Some(handle) => (Some(handle), Ok(None)),
            _ => {
                (None, {
                    match child.wait() {
                        Ok(exit) => Ok(Some(exit.code().unwrap() as u32)),
                        Err(e) => {
                            Err(format!(
                                "Failed to retrieve exit code for pid {} : {}",
                                child.id(),
                                e
                            ))
                        }
                    }
                })
            }
        };

        match status {
            Ok(status) => {
                Ok(Child {
                    handle: win_handle,
                    last_status: status,
                    pid: child.id(),
                })
            }
            Err(e) => Err(Error::GetHabChildFailed(e)),
        }
    }

    pub fn id(&self) -> u32 {
        self.pid
    }

    pub fn status(&mut self) -> Result<HabExitStatus> {
        if self.last_status.is_some() {
            return Ok(HabExitStatus { status: Some(self.last_status.unwrap()) });
        }

        let exit_status = exit_status(self.handle.unwrap())?;

        if exit_status == STILL_ACTIVE {
            return Ok(HabExitStatus { status: None });
        };

        Ok(HabExitStatus { status: Some(exit_status) })
    }

    pub fn kill(&mut self) -> Result<ShutdownMethod> {
        if self.last_status.is_some() {
            return Ok(ShutdownMethod::AlreadyExited);
        }

        let ret;
        unsafe {
            // Send a ctrl-BREAK
            ret = kernel32::GenerateConsoleCtrlEvent(1, self.pid);
            if ret == 0 {
                debug!(
                    "Failed to send ctrl-break to pid {}: {}",
                    self.pid,
                    io::Error::last_os_error()
                );
            }
        }

        let stop_time = SteadyTime::now() + Duration::seconds(8);

        let result;
        loop {
            if ret == 0 || SteadyTime::now() > stop_time {
                let proc_table = self.build_proc_table()?;
                self.terminate_process_descendants(&proc_table, self.pid)?;
                result = Ok(ShutdownMethod::Killed);
                break;
            }

            match self.status() {
                Ok(status) => {
                    if !status.no_status() {
                        result = Ok(ShutdownMethod::GracefulTermination);
                        break;
                    }
                }
                _ => {}
            }
        }

        result
    }

    fn terminate_process_descendants(
        &self,
        table: &HashMap<winapi::DWORD, Vec<winapi::DWORD>>,
        pid: winapi::DWORD,
    ) -> Result<()> {
        if let Some(children) = table.get(&pid) {
            for child in children {
                self.terminate_process_descendants(table, child.clone())?;
            }
        }
        unsafe {
            match handle_from_pid(pid) {
                Some(h) => {
                    if kernel32::TerminateProcess(h, 1) == 0 {
                        return Err(Error::TerminateProcessFailed(format!(
                            "Failed to call TerminateProcess on pid {}: {}",
                            pid,
                            io::Error::last_os_error()
                        )));
                    }
                }
                None => {}
            }
        }
        Ok(())
    }

    fn build_proc_table(&self) -> Result<HashMap<winapi::DWORD, Vec<winapi::DWORD>>> {
        let processes_snap_handle =
            unsafe { kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPPROCESS, 0) };

        if processes_snap_handle == winapi::INVALID_HANDLE_VALUE {
            return Err(Error::CreateToolhelp32SnapshotFailed(format!(
                "Failed to call CreateToolhelp32Snapshot: {}",
                io::Error::last_os_error()
            )));
        }

        let mut table: HashMap<winapi::DWORD, Vec<winapi::DWORD>> = HashMap::new();
        let mut process_entry = winapi::PROCESSENTRY32W {
            dwSize: mem::size_of::<winapi::PROCESSENTRY32W>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; winapi::MAX_PATH],
        };

        // Get the first process from the snapshot.
        match unsafe { kernel32::Process32FirstW(processes_snap_handle, &mut process_entry) } {
            1 => {
                // First process worked, loop to find the process with the correct name.
                let mut process_success: i32 = 1;

                // Loop through all processes until we find one hwere `szExeFile` == `name`.
                while process_success == 1 {
                    let children = table.entry(process_entry.th32ParentProcessID).or_insert(
                        Vec::new(),
                    );
                    (*children).push(process_entry.th32ProcessID);

                    process_success = unsafe {
                        kernel32::Process32NextW(processes_snap_handle, &mut process_entry)
                    };
                }

                unsafe { kernel32::CloseHandle(processes_snap_handle) };
            }
            0 | _ => {
                unsafe { kernel32::CloseHandle(processes_snap_handle) };
            }
        }

        Ok(table)
    }
}

// Have to implement these due to our HANDLE field
unsafe impl Send for Child {}
unsafe impl Sync for Child {}

impl Drop for Child {
    fn drop(&mut self) {
        match self.handle {
            None => {}
            Some(handle) => unsafe {
                let _ = kernel32::CloseHandle(handle);
            },
        }
    }
}

impl ExitStatusExt for HabExitStatus {
    fn code(&self) -> Option<u32> {
        self.status
    }

    fn signal(&self) -> Option<u32> {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::super::*;

    #[test]
    fn running_process_returns_no_exit_status() {
        let mut child = windows_child::Child::spawn(
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
            vec!["-noprofile", "-command", "while($true) { Start-Sleep 1 }"],
            &HashMap::new(),
        ).unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();

        assert!(hab_child.status().unwrap().no_status())
    }

    #[test]
    fn successfully_run_process_exits_zero() {
        let mut child = windows_child::Child::spawn(
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
            vec!["-noprofile", "-command", "$a='b'"],
            &HashMap::new(),
        ).unwrap();
        let mut hab_child = HabChild::from(&mut child).unwrap();

        let _ = child.wait();

        assert_eq!(hab_child.status().unwrap().code(), Some(0))
    }

    #[test]
    fn terminated_process_returns_non_zero_exit() {
        let mut child = windows_child::Child::spawn(
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
            vec!["-noprofile", "-command", "while($true) { Start-Sleep 1 }"],
            &HashMap::new(),
        ).unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();
        let _ = child.kill();

        assert!(hab_child.status().unwrap().code() != Some(0))
    }

    #[test]
    fn process_that_exits_with_specific_code_has_same_exit_code() {
        let mut child = windows_child::Child::spawn(
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
            vec!["-noprofile", "-command", "exit 5000"],
            &HashMap::new(),
        ).unwrap();

        let mut hab_child = HabChild::from(&mut child).unwrap();
        let _ = child.wait();

        assert_eq!(hab_child.status().unwrap().code(), Some(5000))
    }
}
