use crate::error::{Error,
                   Result};
use std::{ffi::OsString,
          io,
          path::PathBuf,
          process::{self,
                    Command}};
use winapi::{shared::minwindef::{DWORD,
                                 FALSE,
                                 LPDWORD},
             um::{handleapi,
                  processthreadsapi,
                  winnt::{HANDLE,
                          PROCESS_QUERY_LIMITED_INFORMATION,
                          PROCESS_TERMINATE,
                          SYNCHRONIZE}}};

const STILL_ACTIVE: u32 = 259;

pub type Pid = DWORD;

pub fn become_command(command: PathBuf, args: &[OsString]) -> Result<()> {
    become_child_command(command, args)
}

/// Get process identifier of calling process.
pub fn current_pid() -> u32 { unsafe { processthreadsapi::GetCurrentProcessId() as u32 } }

pub fn handle_from_pid(pid: Pid) -> Option<HANDLE> {
    unsafe {
        let proc_handle = processthreadsapi::OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION
                                                         | PROCESS_TERMINATE
                                                         | SYNCHRONIZE,
                                                         FALSE,
                                                         pid);

        // we expect this to happen if the process died
        // before OpenProcess completes
        //
        // This will also happen if pid is 0 (i.e., the system
        // process)
        if proc_handle.is_null() {
            None
        } else {
            Some(proc_handle)
        }
    }
}

/// Determines if a process is running with the given process
/// identifier.
///
/// Note that if pid is 0, this function will return `false`. That is
/// currently desirable, though, because this function should never be
/// called with a pid of 0 anyway.
///
/// See the Unix implementation in `src/os/process/unix.rs` for
/// additional background.
pub fn is_alive(pid: Pid) -> bool {
    match handle_from_pid(pid) {
        Some(handle) => {
            let exit_status = exit_status(handle).expect("Failed to get exit status");
            unsafe {
                let _ = handleapi::CloseHandle(handle);
            }
            exit_status == STILL_ACTIVE
        }
        None => false,
    }
}

/// Executes a command as a child process and exits with the child's exit code.
///
/// Note that if successful, this function will not return.
///
/// # Failures
///
/// * If the child process cannot be created
fn become_child_command(command: PathBuf, args: &[OsString]) -> Result<()> {
    debug!("Calling child process: ({:?}) {:?}",
           command.display(),
           &args);
    let status = Command::new(command).args(args).status()?;
    // Let's honor the exit codes from the child process we finished running
    process::exit(status.code().unwrap())
}

fn exit_status(handle: HANDLE) -> Result<u32> {
    let mut exit_status: u32 = 0;

    unsafe {
        let ret = processthreadsapi::GetExitCodeProcess(handle, &mut exit_status as LPDWORD);
        if ret == 0 {
            return Err(Error::GetExitCodeProcessFailed(format!(
                "Failed to retrieve Exit Code: {}",
                io::Error::last_os_error()
            )));
        }
    }

    Ok(exit_status)
}
