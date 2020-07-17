use std::{io,
          os::unix::process::CommandExt,
          process::Command,
          result};

/// Ensures that the `Command` is executed within its own process
/// group, and not that of its parent process.
///
/// This should be used when spawning all hooks. This ensures that
/// they are not the same process group as the Launcher (for `run`
/// hooks) or the Supervisor (for all other hooks). Otherwise, if a
/// child process were to send `SIGTERM`, the Launcher could be
/// terminated. Similarly, it prevents a `^C` sent to a foregrounded
/// Supervisor from terminating any hooks prematurely.
///
/// This basically ensures that all hooks are properly isolated,
/// without signaling cross-talk between them and the Launcher /
/// Supervisor.
pub fn with_own_process_group(cmd: &mut Command) -> &mut Command {
    unsafe {
        cmd.pre_exec(set_own_process_group);
    }
    cmd
}

/// Set the process group of the calling process to be the same as its
/// PID.
///
/// Intended for use in a
/// `std::os::unix::process::CommandExt::pre_exec` callback.
fn set_own_process_group() -> result::Result<(), io::Error> {
    unsafe {
        if libc::setpgid(0, 0) == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
