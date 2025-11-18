use crate::os::process::can_run_services_as_svc_user;
#[cfg(not(target_os = "macos"))]
use log::warn;
use nix::{sys::signal::{SigSet,
                        SigmaskHow,
                        pthread_sigmask},
          unistd::{Gid,
                   Uid,
                   setgid,
                   setuid}};
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

    // https://github.com/rust-lang/rust/pull/101077/ drove this pre_exec call in which we we reset
    // the spawned process to be able to receive all signals as this change prevented SIGTERM and 7
    // other signals from reaching processes spawned via our launcher.
    unsafe {
        cmd.pre_exec(|| {
               let newset = SigSet::all();
               pthread_sigmask(SigmaskHow::SIG_UNBLOCK, Some(&newset), None)?;
               Ok(())
           });
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
                use nix::unistd::{User,
                                  getgrouplist,
                                  setgroups};
                use std::ffi::CString;

                match User::from_uid(user_id).map_err(|e| {
                                                 eprintln!("Error resolving user from ID: {:?}", e);
                                                 io::Error::last_os_error()
                                             })? {
                    Some(user) => {
                        let user = CString::new(user.name).map_err(|e| {
                                                              eprintln!("User name cannot \
                                                                         convert to CString!: \
                                                                         {:?}",
                                                                        e);
                                                              e
                                                          })?;

                        // There are some platforms (ex. SUSE 12 sp5) that may return
                        // EINVAL from getgrouplist. This only appears to occur from
                        // statically compiled (MUSL) executables like the hab CLI. The
                        // error has not been reproducible from a dynamic executable like
                        // the supervisor.
                        let groups =
                            getgrouplist(&user, group_id).unwrap_or_else(|e| {
                                                             warn!("unable to get supplementary \
                                                                    groups with getgrouplist: {}",
                                                                   e);
                                                             vec![group_id]
                                                         });
                        setgroups(&groups).map_err(|e| {
                                              eprintln!("setgroups failed! {:?}", e);
                                              io::Error::last_os_error()
                                          })?; // CAP_SETGID
                    }
                    _ => {
                        eprintln!("Could not find user from user ID. Wil not set supplementary \
                                   groups.");
                    }
                }
            }

            // These calls replace `CommandExt::uid` and `CommandExt::gid`
            setgid(group_id).map_err(|e| {
                                eprintln!("setgid failed! {:?}", e);
                                io::Error::last_os_error()
                            })?; // CAP_SETGID
            setuid(user_id).map_err(|e| {
                               eprintln!("setuid failed! {:?}", e);
                               io::Error::last_os_error()
                           })?; // CAP_SETUID
        }

        Ok(())
    }
}
