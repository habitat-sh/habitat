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

use std::collections::HashMap;
use std::io;
use std::mem;

use core::os::process::handle_from_pid;
use core::os::process::windows_child::{Child, ExitStatus, Handle};
use kernel32;
use protocol::{self, ShutdownMethod};
use time::{Duration, SteadyTime};
use winapi;

use error::{Error, Result};
use service::Service;

const PROCESS_ACTIVE: u32 = 259;
type ProcessTable = HashMap<winapi::DWORD, Vec<winapi::DWORD>>;

pub struct Process {
    handle: Handle,
    last_status: Option<ExitStatus>,
}

impl Process {
    fn new(handle: Handle) -> Self {
        Process {
            handle: handle,
            last_status: None,
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { kernel32::GetProcessId(self.handle.raw()) as u32 }
    }

    pub fn kill(&mut self) -> ShutdownMethod {
        if self.status().is_some() {
            return ShutdownMethod::AlreadyExited;
        }
        let ret = unsafe { kernel32::GenerateConsoleCtrlEvent(1, self.id()) };
        if ret == 0 {
            debug!(
                "Failed to send ctrl-break to pid {}: {}",
                self.id(),
                io::Error::last_os_error()
            );
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

    pub fn wait(&mut self) -> Result<ExitStatus> {
        unsafe {
            let res = kernel32::WaitForSingleObject(self.handle.raw(), winapi::INFINITE);
            if res != winapi::WAIT_OBJECT_0 {
                return Err(Error::ExecWait(io::Error::last_os_error()));
            }
            let mut status = 0;
            cvt(kernel32::GetExitCodeProcess(self.handle.raw(), &mut status))
                .map_err(Error::ExecWait)?;
            Ok(ExitStatus::from(status))
        }
    }

    pub fn try_wait(&mut self) -> Result<Option<ExitStatus>> {
        unsafe {
            match kernel32::WaitForSingleObject(self.handle.raw(), 0) {
                winapi::WAIT_OBJECT_0 => {}
                winapi::WAIT_TIMEOUT => return Ok(None),
                _ => return Err(Error::ExecWait(io::Error::last_os_error())),
            }
            let mut status = 0;
            cvt(kernel32::GetExitCodeProcess(self.handle.raw(), &mut status))
                .map_err(Error::ExecWait)?;
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

pub fn run(mut msg: protocol::Spawn) -> Result<Service> {
    debug!("launcher is spawning {}", msg.get_binary());
    let ps_cmd = format!("iex $(gc {} | out-string)", msg.get_binary());
    let password = if msg.get_svc_password().is_empty() {
        None
    } else {
        Some(msg.take_svc_password())
    };
    match Child::spawn(
        "powershell.exe",
        vec!["-NonInteractive", "-command", ps_cmd.as_str()],
        msg.get_env(),
        msg.get_svc_user(),
        password,
    ) {
        Ok(child) => {
            let process = Process::new(child.handle);
            Ok(Service::new(msg, process, child.stdout, child.stderr))
        }
        Err(_) => Err(Error::Spawn(io::Error::last_os_error())),
    }
}

fn build_proc_table() -> ProcessTable {
    let processes_snap_handle =
        unsafe { kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPPROCESS, 0) };

    if processes_snap_handle == winapi::INVALID_HANDLE_VALUE {
        error!(
            "Failed to call CreateToolhelp32Snapshot: {}",
            io::Error::last_os_error()
        );
        return ProcessTable::new();
    }
    let mut table = ProcessTable::new();
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
            // Loop through all processes until we find one where `szExeFile` == `name`.
            while process_success == 1 {
                let children = table.entry(process_entry.th32ParentProcessID).or_insert(
                    Vec::new(),
                );
                (*children).push(process_entry.th32ProcessID);
                process_success =
                    unsafe { kernel32::Process32NextW(processes_snap_handle, &mut process_entry) };
            }
            unsafe { kernel32::CloseHandle(processes_snap_handle) };
        }
        0 | _ => unsafe {
            kernel32::CloseHandle(processes_snap_handle);
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
        let ret = kernel32::GetExitCodeProcess(handle.raw(), &mut exit_code as winapi::LPDWORD);
        if ret == 0 {
            error!(
                "Failed to retrieve Exit Code: {}",
                io::Error::last_os_error()
            );
            return None;
        }
    }
    Some(exit_code)
}

fn terminate_process_descendants(table: &ProcessTable, pid: winapi::DWORD) {
    if let Some(children) = table.get(&pid) {
        for child in children {
            terminate_process_descendants(table, child.clone());
        }
    }
    unsafe {
        match handle_from_pid(pid) {
            Some(h) => {
                if kernel32::TerminateProcess(h, 1) == 0 {
                    error!(
                        "Failed to call TerminateProcess on pid {}: {}",
                        pid,
                        io::Error::last_os_error()
                    );
                }
            }
            None => {}
        }
    }
}
