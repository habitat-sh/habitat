use super::Signal;
use crate::error::{Error,
                   Result};
use libc::{self,
           pid_t};
use std::{ffi::OsString,
          io,
          os::unix::process::CommandExt,
          path::PathBuf,
          process::Command};

pub type Pid = libc::pid_t;
pub(crate) type SignalCode = libc::c_int;

pub fn become_command(command: PathBuf, args: &[OsString]) -> Result<()> {
    become_exec_command(command, args)
}

/// Get process identifier of calling process.
pub fn current_pid() -> Pid { unsafe { libc::getpid() as pid_t } }

/// Determines if a process is running with the given process identifier.
pub fn is_alive(pid: Pid) -> bool {
    // Sending a signal to 0 is sending a signal to yourself, so this
    // would just be checking that the current process itself is alive,
    // which... *duh*.
    //
    // (This is in service of discovering how we would ever end up
    // with pid == 0 down here in the first place.  I think places
    // where a PID of 0 would be generated have been addressed in the
    // code, but this is belt-and-suspenders code at the moment.)
    if pid == 0 {
        error!(target: "pidfile_tracing","Trying to determine if PID 0 is alive; this is an unexpected situation; returning false!");
        return false;
    }

    match unsafe { libc::kill(pid, 0) } {
        0 => true,
        _ => {
            match io::Error::last_os_error().raw_os_error() {
                Some(libc::EPERM) => true,
                Some(libc::ESRCH) => false,
                _ => false,
            }
        }
    }
}

pub fn signal(pid: Pid, signal: Signal) -> Result<()> {
    unsafe {
        match libc::kill(pid as pid_t, signal.into()) {
            0 => Ok(()),
            e => Err(Error::SignalFailed(e, io::Error::last_os_error())),
        }
    }
}

// This only makes sense on Unix platforms, because not all of these
// symbols are actually defined on Windows. Also, this is only used
// for actually sending the given signal to a process, which only
// happens on Unix platforms anyway.
impl From<Signal> for SignalCode {
    fn from(value: Signal) -> SignalCode {
        match value {
            Signal::INT => libc::SIGINT,
            Signal::ILL => libc::SIGILL,
            Signal::ABRT => libc::SIGABRT,
            Signal::FPE => libc::SIGFPE,
            Signal::KILL => libc::SIGKILL,
            Signal::SEGV => libc::SIGSEGV,
            Signal::TERM => libc::SIGTERM,
            Signal::HUP => libc::SIGHUP,
            Signal::QUIT => libc::SIGQUIT,
            Signal::ALRM => libc::SIGALRM,
            Signal::USR1 => libc::SIGUSR1,
            Signal::USR2 => libc::SIGUSR2,
            Signal::CHLD => libc::SIGCHLD,
        }
    }
}
/// Makes an `execvp(3)` system call to become a new program.
///
/// Note that if successful, this function will not return.
///
/// # Failures
///
/// * If the system call fails the error will be returned, otherwise this function does not return
fn become_exec_command(command: PathBuf, args: &[OsString]) -> Result<()> {
    debug!("Calling execvp(): ({:?}) {:?}", command.display(), &args);
    let error_if_failed = Command::new(command).args(args).exec();
    // The only possible return for the above function is an `Error` so return it, meaning that we
    // failed to exec to our target program
    Err(error_if_failed.into())
}
