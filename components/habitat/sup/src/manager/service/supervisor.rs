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
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use hcore::os::process::{self, Pid};
use std::result;

use hcore::service::ServiceGroup;
use launcher_client::LauncherCli;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use time::{self, Timespec};

use error::{Result, Error};
use fs;
use manager::service::Pkg;

static LOGKEY: &'static str = "SV";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ProcessState {
    Down,
    Up,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match *self {
            ProcessState::Down => "down",
            ProcessState::Up => "up",
        };
        write!(f, "{}", state)
    }
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
        outputln!(preamble self.preamble,
            "Starting service as user={}, group={}", &pkg.svc_user, &pkg.svc_group);
        let pid = launcher.spawn(
            group.to_string(),
            &pkg.svc_run,
            &pkg.svc_user,
            &pkg.svc_group,
            svc_password,
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

    pub fn stop(&mut self, launcher: &LauncherCli) -> Result<()> {
        if self.pid.is_none() {
            return Ok(());
        }
        launcher.terminate(self.pid.unwrap())?;
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
            Some(pid) => {
                match launcher.restart(pid) {
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
                }
            }
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
        strukt.serialize_field(
            "state_entered",
            &self.state_entered.sec,
        )?;
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
                Some(Ok(line)) => {
                    match line.parse::<Pid>() {
                        Ok(pid) => Ok(pid),
                        Err(_) => Err(sup_error!(
                            Error::PidFileCorrupt(pid_file.as_ref().to_path_buf())
                        )),
                    }
                }
                _ => Err(sup_error!(
                    Error::PidFileCorrupt(pid_file.as_ref().to_path_buf())
                )),
            }
        }
        Err(err) => Err(sup_error!(
            Error::PidFileIO(pid_file.as_ref().to_path_buf(), err)
        )),
    }
}
