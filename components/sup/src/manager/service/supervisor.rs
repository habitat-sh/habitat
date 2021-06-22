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
use habitat_common::{outputln,
                     templating::package::Pkg,
                     types::UserInfo};
#[cfg(unix)]
use habitat_core::os::users;
use habitat_core::{fs,
                   fs::{AtomicWriter,
                        Permissions},
                   os::process::{self,
                                 Pid},
                   service::ServiceGroup};
#[cfg(windows)]
use habitat_launcher_client::Error as launcher_error;
use habitat_launcher_client::LauncherCli;
#[cfg(windows)]
use habitat_launcher_protocol::{self as protocol,
                                Error as launcher_protocol_error};
use serde::{ser::SerializeStruct,
            Serialize,
            Serializer};
#[cfg(windows)]
use std::env;
use std::{fs::File,
          io::{BufRead,
               BufReader,
               Write},
          path::{Path,
                 PathBuf},
          result,
          time::{Duration,
                 SystemTime}};

static LOGKEY: &str = "SV";

// We only set PID file permissions on Unix-like systems. On Windows,
// the file will inherit the permissions of the parent directory. In
// this case, the parent directory should already allow broad reading
// of the PID file.
#[cfg(windows)]
const PIDFILE_PERMISSIONS: Permissions = Permissions::Standard;
#[cfg(not(windows))]
const PIDFILE_PERMISSIONS: Permissions = Permissions::Explicit(0o644);

#[derive(Debug)]
pub struct Supervisor {
    service_group: ServiceGroup,
    state:         ProcessState,
    pid:           Option<Pid>,
    /// The time at which the Supervisor's state changed. Absolute
    /// precision is not necessary, but being able to get the seconds
    /// since the UNIX epoch is.
    state_entered: SystemTime,
    /// If the Supervisor is being run with an newer Launcher that can
    /// provide service PIDs, this will be
    /// `ServicePidSource::Launcher`; otherwise it will be
    /// `ServicePidSource::Files`. Client code should use this as an
    /// indicator of which mode the Supervisor is running in.
    pid_source:    ServicePidSource,
    /// Path at which the currently-running PID of this service is
    /// written to disk.
    ///
    /// If `pid_source` is `ServicePidSource::Files`,
    /// this will be where a restarting Supervisor figures out which
    /// processes it should continue monitoring.
    ///
    /// Regardless of the value of `pid_source`, the current PID will
    /// always be written to this path, for use by service hooks.
    pid_file:      PathBuf,
}

impl Supervisor {
    /// Create a new instance for `service_group`.
    ///
    /// The `pid_source` governs how we determine the PID of the
    /// supervised process. Once the we decide to no longer support
    /// the older Launchers that can't provide service PIDs, this can
    /// be removed.
    pub fn new(service_group: &ServiceGroup, pid_source: ServicePidSource) -> Supervisor {
        let pid_file = fs::svc_pid_file(service_group.service());
        Supervisor { service_group: service_group.clone(),
                     state: ProcessState::Down,
                     state_entered: SystemTime::now(),
                     pid_source,
                     pid: None,
                     pid_file }
    }

    /// Check if the child process is running
    pub fn check_process(&mut self, launcher: &LauncherCli) -> bool {
        self.pid = self.pid
                       .or_else(|| {
                           if self.pid_source == ServicePidSource::Files {
                               read_pid(&self.pid_file)
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
            Self::cleanup_pidfile(&self.pid_file);
        }

        self.pid.is_some()
    }

    // NOTE: the &self argument is only used to get access to
    // self.service_group, and even then only for Linux :/
    #[cfg(unix)]
    fn user_info(&self, pkg: &Pkg, _: &LauncherCli) -> Result<UserInfo> {
        if process::can_run_services_as_svc_user() {
            // We have the ability to run services as a user / group other
            // than ourselves, so they better exist
            let uid = users::get_uid_by_name(&pkg.svc_user)?.ok_or_else(|| {
                                                                Error::UserNotFound(pkg.svc_user
                                                                                       .to_string())
                                                            })?;
            let gid = users::get_gid_by_name(&pkg.svc_group)?.ok_or_else(|| {
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
            let username = users::get_effective_username()?;
            let uid = users::get_effective_uid();
            let groupname = users::get_effective_groupname()?;
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
    fn user_info(&self, pkg: &Pkg, launcher: &LauncherCli) -> Result<UserInfo> {
        // We have changed the implementation of get_current_username in core
        // to use a win32 call GetUserNameW instead of the USERNAME environment
        // variable. This introduces a problem if we are using an older launcher
        // because both might get the current user name and an older launcher would
        // get a different value if the current user is the SYSTEM user. The API
        // call returns 'system' but the environment variable would be the host
        // name followed by a dollar sign. So when an older launcher attempts to
        // spawn a service and the supervisor is telling it to use the 'system' user
        // name, the launcher will try to spawn the service using CreateProcessAsUserW
        // instead of CreateProcessW thinking it needs to spawn the process as a
        // different user. CreateProcessAsUserW will fail because it will attempt to
        // authenticate the system user to create its access token which cannot be done
        // with the system user.
        //
        // So we will check the version of the launcher. This version check is added
        // in the same set of commits that changed the current username inspection
        // implementation. So if the version handler is not supported, which it would
        // not be in an older launcher, we will convert the user name to the old style
        // system user using the host name.
        let user = {
            if pkg.svc_user == "system" {
                let legacy_user = env::var("COMPUTERNAME")?.to_lowercase() + "$";
                match launcher.version() {
                    // 14227 is the last unstable launcher version as of the writing
                    // of this comment.
                    Ok(v) if v > 14227 => pkg.svc_user.clone(),
                    Ok(_) => legacy_user,
                    Err(err @ launcher_error::Timeout) => {
                        error!("Timeout getting version from launcher: {:?}", err);
                        legacy_user
                    }
                    Err(launcher_error::Protocol(launcher_protocol_error::NetErr(err))) => {
                        match err.code {
                            protocol::ErrCode::UnknownMessage => {
                                error!("Unable to retrieve version from launcher: {:?}", err);
                                legacy_user
                            }
                            _ => return Err(Error::Launcher(launcher_error::Protocol(launcher_protocol_error::NetErr(err)))),
                        }
                    }
                    Err(err) => {
                        return Err(Error::Launcher(err));
                    }
                }
            } else {
                pkg.svc_user.clone()
            }
        };

        // Windows only really has usernames, not groups and other
        // IDs.
        //
        // Note that the Windows Supervisor does not yet have a
        // corresponding "non-root" behavior, as the Linux version
        // does; services run as the service user.
        Ok(UserInfo { username: Some(user),
                      ..Default::default() })
    }

    pub fn start(&mut self,
                 pkg: &Pkg,
                 group: &ServiceGroup,
                 launcher: &LauncherCli,
                 svc_password: Option<&str>)
                 -> Result<()> {
        let user_info = self.user_info(&pkg, launcher)?;
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
        self.create_pidfile(&self.pid_file)?;
        self.change_state(ProcessState::Up);
        Ok(())
    }

    /// Is the process up or down?
    pub fn status(&self) -> ProcessState { self.state }

    /// Returns a future that stops a service asynchronously.
    pub fn stop(&self, shutdown_config: ShutdownConfig) {
        let service_group = self.service_group.clone();

        if let Some(pid) = self.pid {
            if pid == 0 {
                warn!(target: "pidfile_tracing", "Cowardly refusing to stop {}, because we think it has a PID of 0, which makes no sense",
                      service_group);
            } else {
                tokio::spawn(async move {
                    if terminator::terminate_service(pid, service_group.clone(),
                        shutdown_config).await  .is_err()
                    {
                    error!(target: "pidfile_tracing", "Failed to to stop service {}", service_group);
                    };
                });
                Self::cleanup_pidfile(&self.pid_file);
            }
        } else {
            // Not quite sure how we'd get down here without a PID...

            // TODO (CM): when this pidfile tracing bit has been
            // cleared up, remove this logging target; it was added
            // just to help with debugging. The overall logging
            // message can stay, however.
            warn!(target: "pidfile_tracing", "Cowardly refusing to stop {}, because we mysteriously have no PID!", service_group);
        }
    }

    /// Create a PID file for a running service
    fn create_pidfile(&self, pid_file: &Path) -> Result<()> {
        if let Some(pid) = self.pid {
            // TODO (CM): when this pidfile tracing bit has been
            // cleared up, remove this logging target; it was added
            // just to help with debugging. The overall logging
            // message can stay, however.
            debug!(target: "pidfile_tracing", "Creating PID file for child {} -> {}",
                   pid_file.display(),
                   pid);
            let w = AtomicWriter::new_with_permissions(pid_file, PIDFILE_PERMISSIONS)?;
            w.with_writer(|f| f.write_all(pid.to_string().as_ref()))?;
        }

        Ok(())
    }

    fn cleanup_pidfile(pid_file: impl AsRef<Path>) {
        // TODO (CM): when this pidfile tracing bit has been cleared
        // up, remove these logging targets; they were added just to
        // help with debugging. The overall logging messages can stay,
        // however.
        debug!(target: "pidfile_tracing", "Attempting to clean up pid file {}", pid_file.as_ref().display());
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
        self.state_entered = SystemTime::now();
    }

    pub fn state_entered(&self) -> SystemTime { self.state_entered }

    /// Returns how long after the UNIX Epoch this Supervisor changed
    /// state.
    fn since_epoch(&self) -> Duration {
        self.state_entered
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("our time should ALWAYS be after the UNIX Epoch")
    }
}

// This is used to generate the output of the HTTP gateway
impl Serialize for Supervisor {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("supervisor", 5)?;
        strukt.serialize_field("pid", &self.pid)?;
        strukt.serialize_field("state", &self.state)?;
        strukt.serialize_field("state_entered", &self.since_epoch().as_secs())?;
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
