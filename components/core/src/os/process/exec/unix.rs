use crate::os::process::can_run_services_as_svc_user;
use nix::unistd::{setgid,
                  setuid,
                  Gid,
                  Uid};
use std::{ffi::OsStr,
          io,
          os::unix::process::CommandExt,
          process::{Command,
                    Stdio},
          result};

/// Prepare a `Command` to execute a lifecycle hook.
// TODO (CM): Ideally, `ids` would not be an `Option`, but separate
// `Uid` and `Gid` inputs. However, the `Option` interface provides
// the least disruption to other existing code for the time being.
pub fn hook_command<X, I, K, V>(executable: X, env: I, ids: Option<(Uid, Gid)>) -> Command
    where X: AsRef<OsStr>,
          I: IntoIterator<Item = (K, V)>,
          K: AsRef<OsStr>,
          V: AsRef<OsStr>
{
    let mut cmd = Command::new(executable);

    // NOTE: CommandExt::uid and CommandExt::guid should *not* be
    // called here! They are set in `with_user_and_group_information`;
    // see there for further details.
    cmd.stdin(Stdio::null())
       .stdout(Stdio::piped())
       .stderr(Stdio::piped())
       .envs(env);

    with_own_process_group(&mut cmd);
    if let Some((uid, gid)) = ids {
        with_user_and_group_information(&mut cmd, uid, gid);
    }

    cmd
}

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
fn with_own_process_group(cmd: &mut Command) -> &mut Command {
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
/// Sets uid, gid, and supplementary groups on command.
///
/// DO NOT call `CommandExt#uid` or `CommandExt#gid` on this command,
/// either before or after calling this function, or it will probably
/// not work like you want it to.
fn with_user_and_group_information(cmd: &mut Command, uid: Uid, gid: Gid) -> &mut Command {
    unsafe {
        cmd.pre_exec(set_supplementary_groups(uid, gid));
    }
    cmd
}

/// Stupid little private helper macro to make mapping `Nix` errors to
/// IO errors for our `pre_exec` hooks.
///
/// The format string should have a single variable placeholder for
/// the actual error.
///
/// e.g. `result.map_err(io_err!("blah blah {:?}"))`
macro_rules! io_error {
    ($format_string:tt) => {
        move |e| io::Error::new(io::ErrorKind::Other, format!($format_string, e))
    };
}

/// Returns a function that sets the supplementary group IDs of the
/// process to those that `user_id` belongs to.
///
/// Also sets the uid and gid of the process. We must do that here,
/// rather than using the `CommandExt::uid` and `CommandExt::gid`
/// methods to ensure that all the IDs are set on the process in the
/// correct order (that is, supplementary groups, gid, and finally uid).
///
/// Once https://github.com/rust-lang/rust/pull/72160 merges, we can
/// use all `CommandExt` methods, and thus simplify things a (little)
/// bit.
fn set_supplementary_groups(user_id: Uid,
                            group_id: Gid)
                            -> impl Fn() -> result::Result<(), io::Error> {
    // Note: since this function will be run a separate process that doesn't
    // inherit RUST_LOG, none of the log! macros will work actually
    // work here.

    move || {
        // Note that if we *can't* run services as another user,
        // that's OK; not an error. We just won't set supplementary
        // groups, and run all hooks as the user we currently are.
        if can_run_services_as_svc_user() {
            // These calls don't have a macOS counterpart (well, not a
            // direct one in nix, at least) and we don't need to
            // execute hooks on macOS, so it's not important to
            // implement this. Our crates aren't really factored well
            // enough to let us cut out larger chunks of code on macOS
            // at the moment, so we just won't compile this particular
            // bit for now.
            #[cfg(not(target_os = "macos"))]
            {
                use nix::unistd::{getgrouplist,
                                  setgroups,
                                  User};
                use std::ffi::CString;

                if let Some(user) = User::from_uid(user_id).map_err(io_error!("Error resolving \
                                                                               user from ID: \
                                                                               {:?}"))?
                {
                    let user = CString::new(user.name).map_err(io_error!("User name cannot \
                                                                          convert to CString!: \
                                                                          {:?}"))?;
                    let groups = getgrouplist(&user, group_id).map_err(io_error!("getgrouplist \
                                                                                  failed!: {:?}"))?;
                    setgroups(&groups).map_err(io_error!("setgroups failed! {:?}"))?; // CAP_SETGID
                } else {
                    return Err(io::Error::new(io::ErrorKind::Other,
                                              "Could not find user from user ID"));
                }
            }

            // These calls replace `CommandExt::uid` and `CommandExt::gid`
            setgid(group_id).map_err(io_error!("setgid failed! {:?}"))?; // CAP_SETGID
            setuid(user_id).map_err(io_error!("setuid failed! {:?}"))?; // CAP_SETUID
        }

        Ok(())
    }
}
