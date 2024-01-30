use crate::{core::{os::{process::{handle_from_pid,
                                  windows_child::{ExitStatus,
                                                  Handle}},
                        users::get_current_username},
                   util},
            error::ServiceRunError,
            protocol::{self,
                       ShutdownMethod},
            service::Service};
use anyhow::Result;
use log::{debug,
          error};
use std::{collections::HashMap,
          env,
          io,
          mem,
          time::{Duration,
                 Instant}};
use winapi::{shared::{minwindef::{DWORD,
                                  LPDWORD,
                                  MAX_PATH},
                      winerror::WAIT_TIMEOUT},
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

    pub fn id(&self) -> u32 { unsafe { processthreadsapi::GetProcessId(self.handle.raw()) } }

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

        let shutdown_timeout = Duration::from_secs(8);
        let start_time = Instant::now();
        loop {
            if ret == 0 || start_time.elapsed() > shutdown_timeout {
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

pub fn run(msg: protocol::Spawn) -> Result<Service, ServiceRunError> {
    debug!("launcher is spawning {}", msg.binary);
    let ps_cmd = format!("iex $(gc {} | out-string)", &msg.binary);
    let password = msg.svc_password.clone();

    let user = match msg.svc_user.as_ref() {
        Some(u) => {
            // In the case where we are spawning on behalf of an older Supervisor
            // we will need to revert to older 'get_current_username' behavior. When
            // running as the Local System account, the former behavior would return
            // the host name followed by a dollar sign. The new behavior returns
            // 'system'. Both the supervisor and the launcher behavior must match.
            // Otherwise if we are running under system, we will interpret the user
            // to spawn as different from ourselves and thus attempt to logon as ourselves
            // which will fail since you cannot simply logon as system. One day we can
            // remove this when we are confident everyone is on a recent supervisor
            // and launcher.
            let mut username = u.to_string();
            if get_current_username().map_err(ServiceRunError::GetCurrentUsername)?
               == Some("system".to_string())
            {
                if let Ok(cn) = env::var("COMPUTERNAME") {
                    if u == &(cn.to_lowercase() + "$") {
                        username = "system".to_string();
                    }
                }
            }
            username
        }
        None => {
            return Err(ServiceRunError::UserNotFound(String::from("")));
        }
    };

    let new_env = msg.env.clone().into_iter().collect();

    match util::spawn_pwsh(&ps_cmd, &new_env, user, password) {
        Ok(child) => {
            let process = Process::new(child.handle);
            Ok(Service::new(msg, process, child.stdout, child.stderr))
        }
        Err(_) => Err(ServiceRunError::Spawn(io::Error::last_os_error())),
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
                                    .or_default();
                (*children).push(process_entry.th32ProcessID);
                process_success =
                    unsafe { tlhelp32::Process32NextW(processes_snap_handle, &mut process_entry) };
            }
            unsafe { handleapi::CloseHandle(processes_snap_handle) };
        }
        _ => unsafe {
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
