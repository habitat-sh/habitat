// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::{collections::HashMap,
          io,
          mem};

use crate::protocol::{self,
                      ShutdownMethod};
use core::os::process::{handle_from_pid,
                        windows_child::{Child,
                                        ExitStatus,
                                        Handle}};
use time::{Duration,
           SteadyTime};
use winapi::{shared::{minwindef::{DWORD,
                                  LPDWORD,
                                  MAX_PATH},
                      winerror::{ERROR_FILE_NOT_FOUND,
                                 WAIT_TIMEOUT}},
             um::{handleapi::{self,
                              INVALID_HANDLE_VALUE},
                  processthreadsapi,
                  synchapi,
                  tlhelp32::{self,
                             LPPROCESSENTRY32W,
                             PROCESSENTRY32W,
                             TH32CS_SNAPPROCESS},
                  winbase::{INFINITE,
                            WAIT_OBJECT_0},
                  wincon}};

use crate::{error::{Error,
                    Result},
            service::Service};

const PROCESS_ACTIVE: u32 = 259;
type ProcessTable = HashMap<DWORD, Vec<DWORD>>;

pub struct Process {
    handle:      Handle,
    last_status: Option<ExitStatus>,
}

impl Process {
    fn new(handle: Handle) -> Self {
        Process { handle,
                  last_status: None }
    }

    pub fn id(&self) -> u32 { unsafe { processthreadsapi::GetProcessId(self.handle.raw()) as u32 } }

    /// Attempt to gracefully terminate a process and then forcefully kill it after
    /// 8 seconds if it has not terminated.
    pub fn kill(&mut self) -> ShutdownMethod {
        if self.status().is_some() {
            return ShutdownMethod::AlreadyExited;
        }
        let ret = unsafe { wincon::GenerateConsoleCtrlEvent(1, self.id()) };
        if ret == 0 {
            debug!("Failed to send ctrl-break to pid {}: {}",
                   self.id(),
                   io::Error::last_os_error());
        }

        let stop_time = SteadyTime::now() + Duration::seconds(8);
        loop {
            if ret == 0 || SteadyTime::now() > stop_time {
                let proc_table = build_proc_table();
                terminate_process_descendants(&proc_table, self.id());
                return ShutdownMethod::Killed;
            }

            if self.status().is_some() {
                return ShutdownMethod::GracefulTermination;
            }
        }
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        unsafe {
            let res = synchapi::WaitForSingleObject(self.handle.raw(), INFINITE);
            if res != WAIT_OBJECT_0 {
                return Err(io::Error::last_os_error());
            }
            let mut status = 0;
            cvt(processthreadsapi::GetExitCodeProcess(self.handle.raw(), &mut status))?;
            Ok(ExitStatus::from(status))
        }
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        unsafe {
            match synchapi::WaitForSingleObject(self.handle.raw(), 0) {
                WAIT_OBJECT_0 => {}
                WAIT_TIMEOUT => return Ok(None),
                _ => return Err(io::Error::last_os_error()),
            }
            let mut status = 0;
            cvt(processthreadsapi::GetExitCodeProcess(self.handle.raw(), &mut status))?;
            Ok(Some(ExitStatus::from(status)))
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

pub fn run(msg: protocol::Spawn) -> Result<Service> {
    // Supervisors prior to version 0.53.0 pulled in beta versions of
    // powershell. The official 6.0.0 version of powershell changed
    // the name of the powershell binary to pwsh.exe. Here we will
    // first attempt the latest binary name and fall back to the
    // former name.
    match spawn_pwsh("pwsh.exe", msg.clone()) {
        Ok(service) => Ok(service),
        Err(Error::Spawn(err)) => {
            if err.raw_os_error() == Some(ERROR_FILE_NOT_FOUND as i32) {
                spawn_pwsh("powershell.exe", msg)
            } else {
                Err(Error::Spawn(err))
            }
        }
        Err(err) => Err(err),
    }
}

fn spawn_pwsh(ps_binary_name: &str, msg: protocol::Spawn) -> Result<Service> {
    debug!("launcher is spawning {}", msg.binary);
    let ps_cmd = format!("iex $(gc {} | out-string)", &msg.binary);
    let password = msg.svc_password.clone();

    let user = match msg.svc_user.as_ref() {
        Some(u) => u.to_string(),
        None => {
            return Err(Error::UserNotFound(String::from("")));
        }
    };

    match Child::spawn(ps_binary_name,
                       vec!["-NonInteractive", "-command", ps_cmd.as_str()],
                       &msg.env,
                       user,
                       password)
    {
        Ok(child) => {
            let process = Process::new(child.handle);
            Ok(Service::new(msg, process, child.stdout, child.stderr))
        }
        Err(_) => Err(Error::Spawn(io::Error::last_os_error())),
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

fn cvt(i: i32) -> io::Result<i32> {
    if i == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
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
