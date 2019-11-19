/// Supervise a service.
///
/// The Supervisor is responsible for running any services we are asked to start. It handles
/// spawning the new process, watching for failure, and ensuring the service is either up or
/// down. If the process dies, the Supervisor will restart it.
use super::{terminator,
            ProcessState};
use crate::{error::{Error,
                    Result},
            manager::{ServicePidSource,
                      ShutdownConfig}};
use futures::{future,
              Future};
use habitat_common::{outputln,
                     templating::package::Pkg,
                     types::UserInfo};
#[cfg(unix)]
use habitat_core::os::users;
use habitat_core::{fs,
                   os::process::{self,
                                 Pid},
                   service::ServiceGroup};
use habitat_launcher_client::LauncherCli;
use serde::{ser::SerializeStruct,
            Serialize,
            Serializer};
use std::{fs::File,
          io::{BufRead,
               BufReader},
          path::{Path,
                 PathBuf},
          result};
use time::Timespec;

static LOGKEY: &str = "SV";

#[derive(Debug)]
pub struct Supervisor {
    service_group: ServiceGroup,
    state:         ProcessState,
    // TODO (CM): make this private
    pub state_entered: Timespec,
    pid: Option<Pid>,
    /// If the Supervisor is being run with the PIDS_FROM_LAUNCHER
    /// feature enabled, this will be `None`, otherwise it will be
    /// `Some(path)`. Client code should use the `Some`/`None` status
    /// of this field as an indicator of which mode the Supervisor is
    /// running in.
    pid_file: Option<PathBuf>,
}

impl Supervisor {
    /// Create a new instance for `service_group`.
    ///
    /// The `pid_source` governs how we determine the PID of the
    /// supervised process. Once the PIDS_FROM_LAUNCHER feature flag
    /// goes away, this can be removed.
    pub fn new(service_group: &ServiceGroup, pid_source: ServicePidSource) -> Supervisor {
        let pid_file = match pid_source {
            ServicePidSource::Files => Some(fs::svc_pid_file(service_group.service())),
            ServicePidSource::Launcher => None,
        };
        Supervisor { service_group: service_group.clone(),
                     state: ProcessState::Down,
                     state_entered: time::get_time(),
                     pid: None,
                     pid_file }
    }

    /// Check if the child process is running
    pub fn check_process(&mut self, launcher: &LauncherCli) -> bool {
        self.pid = self.pid
                       .or_else(|| {
                           if let Some(ref pid_file) = self.pid_file {
                               read_pid(pid_file)
                           } else {
                               match launcher.pid_of(&self.service_group.to_string()) {
                                   Ok(maybe_pid) => maybe_pid,
                                   Err(e) => {
                                       error!("Error getting pid from launcher: {:?}", e);
                                       None
                                   }
                               }
                           }
                       })
                       .and_then(|pid| {
                           if process::is_alive(pid) {
                               Some(pid)
                           } else {
                               debug!("Could not find a live process with PID: {:?}", pid);
                               None
                           }
                       });

        if self.pid.is_some() {
            self.change_state(ProcessState::Up);
        } else {
            self.change_state(ProcessState::Down);
            if let Some(ref pid_file) = self.pid_file {
                Self::cleanup_pidfile(pid_file.clone());
            }
        }

        self.pid.is_some()
    }

    // NOTE: the &self argument is only used to get access to
    // self.service_group, and even then only for Linux :/
    #[cfg(unix)]
    fn user_info(&self, pkg: &Pkg) -> Result<UserInfo> {
        if users::can_run_services_as_svc_user() {
            // We have the ability to run services as a user / group other
            // than ourselves, so they better exist
            let uid = users::get_uid_by_name(&pkg.svc_user).ok_or_else(|| {
                                                               Error::UserNotFound(pkg.svc_user
                                                                                      .to_string())
                                                           })?;
            let gid = users::get_gid_by_name(&pkg.svc_group).ok_or_else(|| {
                                                                Error::GroupNotFound(pkg.svc_group
                                                                                  .to_string())
                                                            })?;

            Ok(UserInfo { username:  Some(pkg.svc_user.clone()),
                          uid:       Some(uid),
                          groupname: Some(pkg.svc_group.clone()),
                          gid:       Some(gid), })
        } else {
            // We DO NOT have the ability to run as other users!  Also
            // note that we legitimately may not have a username or
            // groupname.
            let username = users::get_effective_username();
            let uid = users::get_effective_uid();
            let groupname = users::get_effective_groupname();
            let gid = users::get_effective_gid();

            let name_for_logging = username.clone()
                                           .unwrap_or_else(|| format!("anonymous [UID={}]", uid));
            outputln!(preamble self.service_group, "Current user ({}) lacks sufficient capabilites to \
                run services as a different user; running as self!", name_for_logging);

            Ok(UserInfo { username,
                          uid: Some(uid),
                          groupname,
                          gid: Some(gid) })
        }
    }

    #[cfg(windows)]
    fn user_info(&self, pkg: &Pkg) -> Result<UserInfo> {
        // Windows only really has usernames, not groups and other
        // IDs.
        //
        // Note that the Windows Supervisor does not yet have a
        // corresponding "non-root" behavior, as the Linux version
        // does; services run as the service user.
        Ok(UserInfo { username: Some(pkg.svc_user.clone()),
                      ..Default::default() })
    }

    pub fn start(&mut self,
                 pkg: &Pkg,
                 group: &ServiceGroup,
                 launcher: &LauncherCli,
                 svc_password: Option<&str>)
                 -> Result<()> {
        let user_info = self.user_info(&pkg)?;
        outputln!(preamble self.service_group,
                  "Starting service as user={}, group={}",
                  user_info.username.as_ref().map_or("<anonymous>", String::as_str),
                  user_info.groupname.as_ref().map_or("<anonymous>", String::as_str)
        );

        // In the interests of having as little logic in the Launcher
        // as possible, and to support cloud-native uses of the
        // Supervisor, in which the user running the Supervisor
        // doesn't necessarily have a username (or groupname), we only
        // pass the Launcher the bare minimum it needs to launch a
        // service.
        //
        // For Linux, that amounts to the UID and GID to run the
        // process as.
        //
        // For Windows, it's the name of the service user (no
        // "non-root" behavior there, yet).
        //
        // To support backwards compatibility, however, we must still
        // pass along values for the username and groupname; older
        // Launcher versions on Linux (and current Windows versions)
        // will use these, while newer versions will prefer the UID
        // and GID, ignoring the names.
        let pid = launcher.spawn(&group,
                                 &pkg.svc_run,
                                 user_info,
                                 svc_password, // Windows optional
                                 (*pkg.env).clone())?;
        if pid == 0 {
            warn!(target: "pidfile_tracing", "Spawned service for {} has a PID of 0!", group);
        }
        self.pid = Some(pid);
        // Only create a pidfile if we're actually using them.
        if let Some(ref pid_file) = self.pid_file {
            self.create_pidfile(pid_file)?;
        }
        self.change_state(ProcessState::Up);
        Ok(())
    }

    pub fn status(&self) -> (bool, String) {
        let status = format!("{}: {} for {}",
                             self.service_group,
                             self.state,
                             time::get_time() - self.state_entered);
        let healthy = match self.state {
            ProcessState::Up => true,
            ProcessState::Down => false,
        };
        (healthy, status)
    }

    /// Returns a future that stops a service asynchronously.
    pub fn stop(&self, shutdown_config: ShutdownConfig) -> impl Future<Item = (), Error = Error> {
        let service_group = self.service_group.clone();

        if let Some(pid) = self.pid {
            let pid_file = self.pid_file.clone();
            if pid == 0 {
                warn!(target: "pidfile_tracing", "Cowardly refusing to stop {}, because we think it has a PID of 0, which makes no sense",
                      service_group);
                return future::Either::B(future::ok(()));
            }

            future::Either::A(terminator::terminate_service(pid, service_group, shutdown_config).and_then(
                |_shutdown_method| {
                    // Only delete the pidfile if we're actually using one.
                    if let Some(pid_file) = pid_file {
                        Self::cleanup_pidfile(pid_file);
                    }
                    Ok(())
                },
            ))
        } else {
            // Not quite sure how we'd get down here without a PID...

            // TODO (CM): when this pidfile tracing bit has been
            // cleared up, remove this logging target; it was added
            // just to help with debugging. The overall logging
            // message can stay, however.
            warn!(target: "pidfile_tracing", "Cowardly refusing to stop {}, because we mysteriously have no PID!", service_group);
            future::Either::B(future::ok(()))
        }
    }

    /// Create a PID file for a running service
    fn create_pidfile(&self, pid_file: &PathBuf) -> Result<()> {
        if let Some(pid) = self.pid {
            // TODO (CM): when this pidfile tracing bit has been
            // cleared up, remove this logging target; it was added
            // just to help with debugging. The overall logging
            // message can stay, however.
            debug!(target: "pidfile_tracing", "Creating PID file for child {} -> {}",
                   pid_file.display(),
                   pid);
            fs::atomic_write(pid_file, pid.to_string())?;
        }

        Ok(())
    }

    fn cleanup_pidfile(pid_file: PathBuf) {
        // TODO (CM): when this pidfile tracing bit has been cleared
        // up, remove these logging targets; they were added just to
        // help with debugging. The overall logging messages can stay,
        // however.
        debug!(target: "pidfile_tracing", "Attempting to clean up pid file {}", pid_file.display());
        match std::fs::remove_file(pid_file) {
            Ok(_) => debug!(target: "pidfile_tracing", "Removed pid file"),
            Err(e) => {
                debug!(target: "pidfile_tracing", "Error removing pid file: {}, continuing", e)
            }
        }
    }

    fn change_state(&mut self, state: ProcessState) {
        if self.state == state {
            return;
        }
        self.state = state;
        self.state_entered = time::get_time();
    }
}

impl Serialize for Supervisor {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("supervisor", 5)?;
        strukt.serialize_field("pid", &self.pid)?;
        strukt.serialize_field("state", &self.state)?;
        strukt.serialize_field("state_entered", &self.state_entered.sec)?;
        strukt.end()
    }
}

fn read_pid<T>(pid_file: T) -> Option<Pid>
    where T: AsRef<Path>
{
    // TODO (CM): when this pidfile tracing bit has been cleared
    // up, remove these logging targets; they were added just to
    // help with debugging. The overall logging messages can stay,
    // however.
    let p = pid_file.as_ref();

    match File::open(p) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().next() {
                Some(Ok(line)) => {
                    match line.parse::<Pid>() {
                        Ok(pid) if pid == 0 => {
                            error!(target: "pidfile_tracing", "Read PID of 0 from {}!", p.display());
                            // Treat this the same as a corrupt pid
                            // file, because that's basically what it
                            // is. A PID of 0 effectively means the
                            // Supervisor thinks it's supervising
                            // itself. This *should* be an impossible situation.
                            None
                        }
                        Ok(pid) => Some(pid),
                        Err(e) => {
                            error!(target: "pidfile_tracing", "Unable to parse contents of PID file: {}; {:?}", p.display(), e);
                            None
                        }
                    }
                }
                _ => {
                    error!(target: "pidfile_tracing", "Unable to read a line of PID file: {}", p.display());
                    None
                }
            }
        }
        Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(_) => {
            error!(target: "pidfile_tracing", "Error reading PID file: {}", p.display());
            None
        }
    }
}
