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
/// The supervisor is responsible for running any services we are asked to start. It handles
/// spawning the new process, watching for failure, and ensuring the service is either up or down.
/// If the process dies, the supervisor will restart it.

use std;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Child;
use std::result;
use std::sync::{Arc, RwLock};
use std::thread;

use hcore::os::process::{HabChild, ExitStatusExt};
use hcore::package::PackageInstall;
use hcore::service::ServiceGroup;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use time::SteadyTime;

use error::{Result, Error};
use fs;
use util;

const PIDFILE_NAME: &'static str = "PID";
static LOGKEY: &'static str = "SV";

#[derive(Debug, Deserialize, Serialize)]
pub enum ProcessState {
    Down,
    Up,
    Start,
    Restart,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match self {
            &ProcessState::Down => "down",
            &ProcessState::Up => "up",
            &ProcessState::Start => "start",
            &ProcessState::Restart => "restart",
        };
        write!(f, "{}", state)
    }
}

/// Additional params used to start the Supervisor.
/// These params are outside the scope of what is in
/// Supervisor.package.ident, and aren't runtime params that are stored
/// in the top-level Supervisor struct (such as PID etc)
#[derive(Debug, Deserialize, Serialize)]
pub struct RuntimeConfig {
    pub svc_user: String,
    pub svc_group: String,
}

impl RuntimeConfig {
    pub fn new(svc_user: String, svc_group: String) -> RuntimeConfig {
        RuntimeConfig {
            svc_user: svc_user,
            svc_group: svc_group,
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> RuntimeConfig {
        RuntimeConfig {
            svc_user: util::users::DEFAULT_USER.to_string(),
            svc_group: util::users::DEFAULT_GROUP.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Supervisor {
    pub child: Option<HabChild>,
    pub package: Arc<RwLock<PackageInstall>>,
    pub preamble: String,
    pub state: ProcessState,
    pub state_entered: SteadyTime,
    pub has_started: bool,
    pub runtime_config: RuntimeConfig,
}

impl Supervisor {
    pub fn new(package: Arc<RwLock<PackageInstall>>,
               service_group: &ServiceGroup,
               runtime_config: RuntimeConfig)
               -> Supervisor {
        Supervisor {
            child: None,
            package: package,
            preamble: format!("{}", service_group),
            state: ProcessState::Down,
            state_entered: SteadyTime::now(),
            has_started: false,
            runtime_config: runtime_config,
        }
    }

    fn enter_state(&mut self, state: ProcessState) {
        self.state = state;
        self.state_entered = SteadyTime::now();
    }

    pub fn status(&self) -> (bool, String) {
        let status = format!("{}: {} for {}",
                             self.preamble,
                             self.state,
                             SteadyTime::now() - self.state_entered);
        let healthy = match self.state {
            ProcessState::Up | ProcessState::Start | ProcessState::Restart => true,
            ProcessState::Down => false,
        };
        (healthy, status)
    }

    pub fn start(&mut self) -> Result<()> {
        if self.child.is_none() {
            outputln!(preamble self.preamble,
                      "Starting process as user={}, group={}",
                      &self.runtime_config.svc_user,
                      &self.runtime_config.svc_group);
            self.enter_state(ProcessState::Start);
            let mut child = try!(util::create_command(self.run_cmd(),
                                                      &self.runtime_config.svc_user,
                                                      &self.runtime_config.svc_group)
                .spawn());

            let hab_child = try!(HabChild::from(&mut child));
            self.child = Some(hab_child);
            try!(self.create_pidfile());
            let package_name = self.preamble.clone();
            try!(thread::Builder::new()
                .name(String::from("sup-service-read"))
                .spawn(move || -> Result<()> { child_reader(&mut child, package_name) }));
            self.enter_state(ProcessState::Up);
            self.has_started = true;
        } else {
            outputln!(preamble & self.preamble, "Already started");
        }
        Ok(())
    }

    /// Send a SIGTERM to a process, wait 8 seconds, then send SIGKILL
    pub fn stop(&mut self) -> Result<()> {
        match self.child {
            Some(ref mut child) => {
                outputln!(preamble & self.preamble, "Stopping...");
                let shutdown = try!(child.kill());
                outputln!("{} - Shutdown method: {}", self.preamble, shutdown);
            }
            None => {}
        };
        self.check_process();
        Ok(())
    }

    pub fn is_up(&self) -> bool {
        if let ProcessState::Up = self.state {
            true
        } else {
            false
        }
    }

    pub fn is_down(&self) -> bool {
        if let ProcessState::Down = self.state {
            true
        } else {
            false
        }
    }

    pub fn down(&mut self) -> Result<()> {
        self.enter_state(ProcessState::Down);
        try!(self.stop());
        self.cleanup_pidfile();
        Ok(())
    }

    pub fn restart(&mut self) -> Result<()> {
        self.enter_state(ProcessState::Restart);
        try!(self.stop());
        try!(self.start());
        Ok(())
    }

    /// if the child process exists, check it's status via waitpid().
    pub fn check_process(&mut self) {
        let changed = match self.child {
            None => false,
            Some(ref mut child) => {
                match child.status() {
                    Ok(ref status) if status.no_status() => false,
                    Ok(ref status) => {
                        if status.code().is_some() {
                            outputln!("{} - process {} died with exit code {}",
                                      self.preamble,
                                      child.id(),
                                      status.code().unwrap());
                        } else if status.signal().is_some() {
                            outputln!("{} - process {} died with signal {}",
                                      self.preamble,
                                      child.id(),
                                      status.signal().unwrap());
                        }
                        true
                    }
                    Err(e) => {
                        debug!("Error checking process status: {}, continuing", e);
                        false
                    }
                }
            }
        };
        if changed {
            match self.state {
                ProcessState::Up | ProcessState::Start | ProcessState::Restart => {
                    outputln!("{} - Service exited", self.preamble);
                    self.child = None;
                }
                ProcessState::Down => {
                    self.enter_state(ProcessState::Down);
                    self.child = None;
                }
            }
        }
    }

    pub fn run_cmd(&self) -> PathBuf {
        self.service_dir().join("run")
    }

    pub fn service_dir(&self) -> PathBuf {
        fs::svc_path(&self.package.read().expect("Package lock poisoned").ident().name)
    }

    pub fn pid_file(&self) -> PathBuf {
        self.service_dir().join(PIDFILE_NAME)
    }

    /// Create a pid file for a package
    /// The existence of this file does not guarantee that a
    /// process exists at the PID contained within.
    pub fn create_pidfile(&self) -> Result<()> {
        match self.child {
            Some(ref child) => {
                let pid_file = self.pid_file();
                let ref pid = child.id();
                debug!("Creating PID file for child {} -> {:?}",
                       pid_file.display(),
                       pid);
                let mut f = try!(File::create(pid_file));
                try!(write!(f, "{}", pid));
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Remove a pidfile for this package if it exists.
    /// Do NOT fail if there is an error removing the PIDFILE
    pub fn cleanup_pidfile(&self) {
        let pid_file = self.pid_file();
        debug!("Attempting to clean up pid file {}", &pid_file.display());
        match std::fs::remove_file(pid_file) {
            Ok(_) => {
                debug!("Removed pid file");
            }
            Err(e) => {
                debug!("Error removing pidfile: {}, continuing", e);
            }
        };
    }

    /// attempt to read the pidfile for this package.
    /// If the pidfile does not exist, then return None,
    /// otherwise, return Some(pid, uptime_seconds).
    pub fn read_pidfile(&self) -> Result<Option<u32>> {
        let pid_file = self.pid_file();
        debug!("Reading pidfile {}", &pid_file.display());

        let mut f = try!(File::open(pid_file));
        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));
        debug!("pidfile contents = {}", contents);
        let pid = match contents.parse::<u32>() {
            Ok(pid) => pid,
            Err(e) => {
                debug!("Error reading pidfile: {}", e);
                return Err(sup_error!(Error::InvalidPidFile));
            }
        };
        Ok(Some(pid))
    }
}

impl Serialize for Supervisor {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let pid = match self.child {
            Some(ref child) => Some(child.id()),
            None => None,
        };
        let mut strukt = try!(serializer.serialize_struct("supervisor", 7));
        try!(strukt.serialize_field("pid", &pid));
        try!(strukt.serialize_field("package",
                                    &self.package
                                        .read()
                                        .expect("Package lock poisoned")
                                        .to_string()));
        try!(strukt.serialize_field("preamble", &self.preamble));
        try!(strukt.serialize_field("state", &self.state));
        try!(strukt.serialize_field("state_entered", &self.state_entered.to_string()));
        try!(strukt.serialize_field("started", &self.has_started));
        try!(strukt.serialize_field("runtime_config", &self.runtime_config));
        strukt.end()
    }
}

impl Drop for Supervisor {
    fn drop(&mut self) {
        let _ = self.cleanup_pidfile();
    }
}

/// Consume output from a child process until EOF, then finish
fn child_reader(child: &mut Child, package_name: String) -> Result<()> {
    let c_stdout = match child.stdout {
        Some(ref mut s) => s,
        None => return Err(sup_error!(Error::UnpackFailed)),
    };

    let mut reader = BufReader::new(c_stdout);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).unwrap() > 0 {
        let mut line = output_format!(preamble &package_name, logkey "O");
        line.push_str(&buffer);
        print!("{}", line);
        buffer.clear();
    }
    debug!("child_reader exiting");
    Ok(())
}
