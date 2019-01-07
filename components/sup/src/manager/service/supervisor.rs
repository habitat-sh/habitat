// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

/// Supervise a service.
///
/// The Supervisor is responsible for running any services we are asked to start. It handles
/// spawning the new process, watching for failure, and ensuring the service is either up or down.
/// If the process dies, the Supervisor will restart it.
use std;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::result;

use crate::hcore::os::process::{self, Pid};
#[cfg(unix)]
use crate::hcore::os::users;
use crate::hcore::service::ServiceGroup;
use crate::launcher_client::LauncherCli;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use time::{self, Timespec};

use super::ProcessState;
use super::ShutdownReason;
use crate::error::{Error, Result};
use crate::fs;
use crate::manager::service::Pkg;
#[cfg(unix)]
use crate::sys::abilities;

static LOGKEY: &'static str = "SV";

/// Bundles up information about the user and group that a supervised
/// service should be run as. If the Supervisor itself is running with
/// root-like permissions, then these will be for `SVC_USER` and
/// `SVC_GROUP` for a service. If not, it will be for the user the
/// Supervisor itself is running as.
///
/// On Windows, all but `username` will be `None`. On Linux,
/// `username` and `groupname` may legitimately be `None`, but `uid`
/// and `gid` should always be `Some`.
#[derive(Debug, Default)]
struct UserInfo {
    username: Option<String>,
    uid: Option<u32>,
    groupname: Option<String>,
    gid: Option<u32>,
}

#[derive(Debug)]
pub struct Supervisor {
    pub preamble: String,
    pub state: ProcessState,
    pub state_entered: Timespec,
    pid: Option<Pid>,
    pid_file: PathBuf,
}

impl Supervisor {
    pub fn new(service_group: &ServiceGroup) -> Supervisor {
        Supervisor {
            preamble: service_group.to_string(),
            state: ProcessState::Down,
            state_entered: time::get_time(),
            pid: None,
            pid_file: fs::svc_pid_file(service_group.service()),
        }
    }

    /// Check if the child process is running
    pub fn check_process(&mut self) -> bool {
        let pid = match self.pid {
            Some(pid) => Some(pid),
            None => {
                if self.pid_file.exists() {
                    Some(read_pid(&self.pid_file).unwrap())
                } else {
                    None
                }
            }
        };
        if let Some(pid) = pid {
            if process::is_alive(pid) {
                self.change_state(ProcessState::Up);
                self.pid = Some(pid);
                return true;
            }
        }
        debug!("Could not find a live process with pid {:?}", self.pid);
        self.change_state(ProcessState::Down);
        self.cleanup_pidfile();
        self.pid = None;
        false
    }

    // NOTE: the &self argument is only used to get access to
    // self.preamble, and even then only for Linux :/
    #[cfg(unix)]
    fn user_info(&self, pkg: &Pkg) -> Result<UserInfo> {
        if abilities::can_run_services_as_svc_user() {
            // We have the ability to run services as a user / group other
            // than ourselves, so they better exist
            let uid = users::get_uid_by_name(&pkg.svc_user)
                .ok_or(sup_error!(Error::UserNotFound(pkg.svc_user.to_string(),)))?;
            let gid = users::get_gid_by_name(&pkg.svc_group)
                .ok_or(sup_error!(Error::GroupNotFound(pkg.svc_group.to_string(),)))?;

            Ok(UserInfo {
                username: Some(pkg.svc_user.clone()),
                uid: Some(uid),
                groupname: Some(pkg.svc_group.clone()),
                gid: Some(gid),
            })
        } else {
            // We DO NOT have the ability to run as other users!  Also
            // note that we legitimately may not have a username or
            // groupname.
            let username = users::get_effective_username();
            let uid = users::get_effective_uid();
            let groupname = users::get_effective_groupname();
            let gid = users::get_effective_gid();

            let name_for_logging = username
                .as_ref()
                .map(|name| name.clone())
                .unwrap_or_else(|| format!("anonymous [UID={}]", uid));
            outputln!(preamble self.preamble, "Current user ({}) lacks sufficient capabilites to \
                run services as a different user; running as self!", name_for_logging);

            Ok(UserInfo {
                username: username,
                uid: Some(uid),
                groupname: groupname,
                gid: Some(gid),
            })
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
        Ok(UserInfo {
            username: Some(pkg.svc_user.clone()),
            ..Default::default()
        })
    }

    pub fn start<T>(
        &mut self,
        pkg: &Pkg,
        group: &ServiceGroup,
        launcher: &LauncherCli,
        svc_password: Option<T>,
    ) -> Result<()>
    where
        T: ToString,
    {
        let UserInfo {
            username: service_user,
            uid: service_user_id,
            groupname: service_group,
            gid: service_group_id,
        } = self.user_info(&pkg)?;

        outputln!(preamble self.preamble,
                  "Starting service as user={}, group={}",
                  service_user.as_ref().map_or("<anonymous>", |s| s.as_str()),
                  service_group.as_ref().map_or("<anonymous>", |s| s.as_str())
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
        let pid = launcher.spawn(
            group.to_string(),
            &pkg.svc_run,
            service_user,     // Windows required, Linux optional
            service_group,    // Linux optional
            service_user_id,  // Linux preferred
            service_group_id, // Linux preferred
            svc_password,     // Windows optional
            (*pkg.env).clone(),
        )?;
        self.pid = Some(pid);
        self.create_pidfile()?;
        self.change_state(ProcessState::Up);
        Ok(())
    }

    pub fn status(&self) -> (bool, String) {
        let status = format!(
            "{}: {} for {}",
            self.preamble,
            self.state,
            time::get_time() - self.state_entered
        );
        let healthy = match self.state {
            ProcessState::Up => true,
            ProcessState::Down => false,
        };
        (healthy, status)
    }

    pub fn stop(&mut self, launcher: &LauncherCli, cause: ShutdownReason) -> Result<()> {
        if self.pid.is_none() {
            return Ok(());
        }
        if let ShutdownReason::LauncherStopping = cause {
            // sending any cmds to launcher will block while it is shutting down
            // we'll avoid this knowing that launcher will gratuitously kill off
            // all services as part of its shutdown routine
        } else {
            launcher.terminate(self.pid.unwrap())?;
        }
        self.cleanup_pidfile();
        self.change_state(ProcessState::Down);
        Ok(())
    }

    pub fn restart<T>(
        &mut self,
        pkg: &Pkg,
        group: &ServiceGroup,
        launcher: &LauncherCli,
        svc_password: Option<T>,
    ) -> Result<()>
    where
        T: ToString,
    {
        match self.pid {
            Some(pid) => match launcher.restart(pid) {
                Ok(pid) => {
                    self.pid = Some(pid);
                    self.create_pidfile()?;
                    self.change_state(ProcessState::Up);
                    Ok(())
                }
                Err(err) => {
                    self.cleanup_pidfile();
                    self.change_state(ProcessState::Down);
                    Err(sup_error!(Error::Launcher(err)))
                }
            },
            None => self.start(pkg, group, launcher, svc_password),
        }
    }

    /// Create a PID file for a running service
    fn create_pidfile(&mut self) -> Result<()> {
        match self.pid {
            Some(pid) => {
                debug!(
                    "Creating PID file for child {} -> {:?}",
                    self.pid_file.display(),
                    pid
                );
                let mut f = File::create(&self.pid_file)?;
                write!(f, "{}", pid)?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Remove a pidfile for this package if it exists.
    /// Do NOT fail if there is an error removing the PIDFILE
    fn cleanup_pidfile(&mut self) {
        debug!(
            "Attempting to clean up pid file {}",
            self.pid_file.display()
        );
        match std::fs::remove_file(&self.pid_file) {
            Ok(_) => debug!("Removed pid file"),
            Err(e) => debug!("Error removing pidfile: {}, continuing", e),
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
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("supervisor", 5)?;
        strukt.serialize_field("pid", &self.pid)?;
        strukt.serialize_field("state", &self.state)?;
        strukt.serialize_field("state_entered", &self.state_entered.sec)?;
        strukt.end()
    }
}

fn read_pid<T>(pid_file: T) -> Result<Pid>
where
    T: AsRef<Path>,
{
    match File::open(pid_file.as_ref()) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().next() {
                Some(Ok(line)) => match line.parse::<Pid>() {
                    Ok(pid) => Ok(pid),
                    Err(_) => Err(sup_error!(Error::PidFileCorrupt(
                        pid_file.as_ref().to_path_buf()
                    ))),
                },
                _ => Err(sup_error!(Error::PidFileCorrupt(
                    pid_file.as_ref().to_path_buf()
                ))),
            }
        }
        Err(err) => Err(sup_error!(Error::PidFileIO(
            pid_file.as_ref().to_path_buf(),
            err
        ))),
    }
}
