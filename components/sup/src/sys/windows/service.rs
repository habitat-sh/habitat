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

use crate::{manager::action::ShutdownSpec,
            sys::ShutdownMethod};
use habitat_core::os::process::{handle_from_pid,
                                windows_child::{ExitStatus,
                                                Handle},
                                Pid};
use std::{collections::HashMap,
          io,
          mem,
          thread,
          time::Duration as StdDuration};
use time::{Duration as TimeDuration,
           SteadyTime};
use winapi::{shared::minwindef::{DWORD,
                                 LPDWORD,
                                 MAX_PATH},
             um::{handleapi::{self,
                              INVALID_HANDLE_VALUE},
                  processthreadsapi,
                  tlhelp32::{self,
                             LPPROCESSENTRY32W,
                             PROCESSENTRY32W,
                             TH32CS_SNAPPROCESS},
                  wincon}};

const PROCESS_ACTIVE: u32 = 259;
type ProcessTable = HashMap<DWORD, Vec<DWORD>>;

/// Kill a service process
pub fn kill(pid: Pid, shutdown_spec: ShutdownSpec) -> ShutdownMethod {
    match handle_from_pid(pid) {
        None => {
            // Assume it's already gone if we can't resolve a proper process handle
            ShutdownMethod::AlreadyExited
        }
        Some(handle_ptr) => {
            let mut process = Process::new(Handle::new(handle_ptr));
            process.kill(shutdown_spec)
        }
    }
}

///////////////////////////////////////////////////////////////////////
// Private Code

struct Process {
    handle:      Handle,
    last_status: Option<ExitStatus>,
}

impl Process {
    fn new(handle: Handle) -> Self {
        Process { handle,
                  last_status: None }
    }

    fn id(&self) -> u32 { unsafe { processthreadsapi::GetProcessId(self.handle.raw()) as u32 } }

    /// Attempt to gracefully terminate a process and then forcefully kill it after
    /// 8 seconds if it has not terminated.
    fn kill(&mut self, shutdown_spec: ShutdownSpec) -> ShutdownMethod {
        let ShutdownSpec { timeout } = shutdown_spec;

        if self.status().is_some() {
            return ShutdownMethod::AlreadyExited;
        }
        let ret = unsafe { wincon::GenerateConsoleCtrlEvent(1, self.id()) };
        if ret == 0 {
            debug!("Failed to send ctrl-break to pid {}: {}",
                   self.id(),
                   io::Error::last_os_error());
        }

        let timeout: TimeDuration = timeout.into();
        trace!("Waiting up to {} seconds before terminating process {}",
               timeout.num_seconds(),
               self.id());
        let stop_time = SteadyTime::now() + timeout;
        loop {
            if ret == 0 || SteadyTime::now() > stop_time {
                let proc_table = build_proc_table();
                terminate_process_descendants(&proc_table, self.id());
                return ShutdownMethod::Killed;
            }

            if self.status().is_some() {
                return ShutdownMethod::GracefulTermination;
            }
            thread::sleep(StdDuration::from_millis(5));
        }
    }

    fn status(&mut self) -> Option<ExitStatus> {
        if self.last_status.is_some() {
            return self.last_status;
        }
        match exit_code(&self.handle) {
            Some(PROCESS_ACTIVE) => None,
            Some(code) => {
                self.last_status = Some(ExitStatus::from(code));
                self.last_status
            }
            None => None,
        }
    }
}

fn build_proc_table() -> ProcessTable {
    let processes_snap_handle =
        unsafe { tlhelp32::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    if processes_snap_handle == INVALID_HANDLE_VALUE {
        error!("Failed to call CreateToolhelp32Snapshot: {}",
               io::Error::last_os_error());
        return ProcessTable::new();
    }
    let mut table = ProcessTable::new();
    let mut process_entry = PROCESSENTRY32W { dwSize:              mem::size_of::<PROCESSENTRY32W>()
                                                                   as u32,
                                              cntUsage:            0,
                                              th32ProcessID:       0,
                                              th32DefaultHeapID:   0,
                                              th32ModuleID:        0,
                                              cntThreads:          0,
                                              th32ParentProcessID: 0,
                                              pcPriClassBase:      0,
                                              dwFlags:             0,
                                              szExeFile:           [0; MAX_PATH], };
    // Get the first process from the snapshot.
    match unsafe {
              tlhelp32::Process32FirstW(processes_snap_handle,
                                        &mut process_entry as LPPROCESSENTRY32W)
          } {
        1 => {
            // First process worked, loop to find the process with the correct name.
            let mut process_success: i32 = 1;
            // Loop through all processes until we find one where `szExeFile` == `name`.
            while process_success == 1 {
                let children = table.entry(process_entry.th32ParentProcessID)
                                    .or_insert_with(Vec::new);
                (*children).push(process_entry.th32ProcessID);
                process_success =
                    unsafe { tlhelp32::Process32NextW(processes_snap_handle, &mut process_entry) };
            }
            unsafe { handleapi::CloseHandle(processes_snap_handle) };
        }
        0 | _ => unsafe {
            handleapi::CloseHandle(processes_snap_handle);
        },
    }
    table
}

fn exit_code(handle: &Handle) -> Option<u32> {
    let mut exit_code: u32 = 0;
    unsafe {
        let ret = processthreadsapi::GetExitCodeProcess(handle.raw(), &mut exit_code as LPDWORD);
        if ret == 0 {
            error!("Failed to retrieve Exit Code: {}",
                   io::Error::last_os_error());
            return None;
        }
    }
    Some(exit_code)
}

fn terminate_process_descendants(table: &ProcessTable, pid: DWORD) {
    if let Some(children) = table.get(&pid) {
        for child in children {
            terminate_process_descendants(table, *child);
        }
    }
    unsafe {
        if let Some(h) = handle_from_pid(pid) {
            if processthreadsapi::TerminateProcess(h, 1) == 0 {
                error!("Failed to call TerminateProcess on pid {}: {}",
                       pid,
                       io::Error::last_os_error());
            }
        }
    }
}
