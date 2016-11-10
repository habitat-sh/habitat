// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

pub mod logger;
pub mod workspace;
pub mod postprocessor;

pub use protocol::jobsrv::JobState;

use std::ffi::OsString;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};

use hab_core::package::archive::PackageArchive;
use hab_core::package::install::PackageInstall;
use hab_core::package::PackageIdent;
use hab_net::server::ZMQ_CONTEXT;
use protobuf::{parse_from_bytes, Message};
use protocol::jobsrv as proto;
use zmq;

use self::logger::Logger;
use self::postprocessor::PostProcessor;
use self::workspace::Workspace;
use config::Config;
use error::{Error, Result};
use vcs;

/// In-memory zmq address of Job RunnerMgr
const INPROC_ADDR: &'static str = "inproc://runner";
/// Protocol message to indicate the Job Runner has received a work request
const WORK_ACK: &'static str = "A";
/// Protocol message to indicate the Job Runner has completed a work request
const WORK_COMPLETE: &'static str = "C";

lazy_static! {
    // JW TODO: expose public API functions in the core crate to check if the Rust proccess which
    // is currently executing is, itself, packaged by Habitat. If so, then we should expose another
    // public API function to load the PackageInstall of a dep for the given `origin`/`name`
    // combination. If we can't, then we should fall back to the latest of core/hab-studio because
    // that means we're just in a dev shell.
    static ref STUDIO_PKG: PackageIdent = PackageIdent::from_str("core/hab-studio").unwrap();
}

#[derive(Debug)]
pub struct Job(proto::Job);

impl Job {
    pub fn new(job: proto::Job) -> Self {
        Job(job)
    }

    pub fn vcs(&self) -> &vcs::RemoteSource {
        if self.0.get_project().has_git() {
            return self.0.get_project().get_git();
        }
        unreachable!("unknown vcs associated with job's project");
    }

    pub fn origin(&self) -> &str {
        let items = self.0.get_project().get_id().split("/").collect::<Vec<&str>>();
        assert!(items.len() == 2,
                format!("Invalid project identifier - {}",
                        self.0.get_project().get_id()));
        items[0]
    }
}

impl Deref for Job {
    type Target = proto::Job;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Job {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Runner {
    workspace: Workspace,
    auth_token: String,
    logger: Option<Logger>,
}

impl Runner {
    pub fn new(job: Job, config: &Config) -> Self {
        Runner {
            auth_token: config.auth_token.clone(),
            workspace: Workspace::new(config.data_path.clone(), job),
            logger: None,
        }
    }

    pub fn job(&self) -> &Job {
        &self.workspace.job
    }

    pub fn job_mut(&mut self) -> &mut Job {
        &mut self.workspace.job
    }

    pub fn logger(&mut self) -> &mut Logger {
        self.logger.as_mut().expect("logger not initialized")
    }

    pub fn run(mut self) -> Job {
        if let Some(err) = self.setup().err() {
            error!("WORKSPACE SETUP ERR={:?}", err);
            return self.fail();
        }
        // JW TODO: How are we going to get the secret keys for this thing?
        if let Some(err) = self.job().vcs().clone(&self.workspace.src()).err() {
            error!("CLONE ERROR={}", err);
            return self.fail();
        }
        let mut archive = match self.build() {
            Ok(archive) => archive,
            Err(err) => {
                error!("STUDIO ERR={}", err);
                return self.fail();
            }
        };

        let mut post_processor = PostProcessor::new(&self.workspace);

        if !post_processor.run(&mut archive, &self.auth_token) {
            // JW TODO: We should shelve the built artifacts and allow a retry on post-processing.
            // If the job is killed then we can kill the shelved artifacts.
            return self.fail();
        }

        if let Some(err) = fs::remove_dir_all(self.workspace.out()).err() {
            error!("unable to remove out directory ({}), ERR={:?}",
                   self.workspace.out().display(),
                   err)
        }
        self.complete()
    }

    fn build(&mut self) -> Result<PackageArchive> {
        let args = vec![OsString::from("-s"),
                        OsString::from(self.workspace.src()),
                        OsString::from("-r"),
                        OsString::from(self.workspace.studio()),
                        OsString::from("-k"),
                        OsString::from(self.job().origin()),
                        OsString::from("build"),
                        OsString::from(Path::new(self.job().get_project().get_plan_path())
                            .parent()
                            .unwrap())];
        let command = studio_cmd();
        debug!("building, cmd={:?}, args={:?}", command, args);
        let mut child = Command::new(command)
            .args(&args)
            .env_clear()
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn child");
        self.logger().pipe(&mut child);
        let exit_status = child.wait().expect("failed to wait on child");
        debug!("build complete, status={:?}", exit_status);
        if exit_status.success() {
            try!(fs::rename(self.workspace.src().join("results"), self.workspace.out()));
            self.workspace.last_built()
        } else {
            Err(Error::BuildFailure(exit_status.code().unwrap_or(-1)))
        }
    }

    fn complete(mut self) -> Job {
        self.teardown().err().map(|e| error!("{}", e));
        self.workspace.job.set_state(JobState::Complete);
        self.workspace.job
    }

    fn fail(mut self) -> Job {
        self.teardown().err().map(|e| error!("{}", e));
        self.workspace.job.set_state(JobState::Failed);
        self.workspace.job
    }

    fn post_process(&self, archive: &mut PackageArchive) -> bool {
        // JW TODO: In the future we'll support multiple and configurable post processors, but for
        // now let's just publish to the public depot
        //
        // Things to solve right now
        // * Where do we get the token for authentication?
        //      * Should the workers ask for a lease from the JobSrv?
        let client =
            depot_client::Client::new(&hab_core::url::default_depot_url(), PRODUCT, VERSION, None)
                .unwrap();
        if let Some(err) = client.x_put_package(archive, &self.auth_token).err() {
            error!("post processing error, ERR={:?}", err);
        }
        if let Some(err) = fs::remove_dir_all(self.workspace.out()).err() {
            error!("unable to remove out directory ({}), ERR={:?}",
                   self.workspace.out().display(),
                   err)
        }
        true
    }

    fn setup(&mut self) -> Result<()> {
        if let Some(err) = fs::create_dir_all(self.workspace.src()).err() {
            return Err(Error::WorkspaceSetup(format!("{}", self.workspace.src().display()), err));
        }
        self.logger = Some(Logger::init(&self.workspace));
        Ok(())
    }

    fn teardown(&mut self) -> Result<()> {
        let args = vec![OsString::from("-s"),
                        OsString::from(self.workspace.src()),
                        OsString::from("-r"),
                        OsString::from(self.workspace.studio()),
                        OsString::from("rm"),
                        OsString::from(Path::new(self.job().get_project().get_plan_path())
                            .parent()
                            .unwrap())];
        let command = studio_cmd();
        debug!("removing studio, cmd={:?}, args={:?}", command, args);
        let mut child = Command::new(command)
            .args(&args)
            .env_clear()
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn child");
        self.logger().pipe(&mut child);
        let exit_status = child.wait().expect("failed to wait on child");
        debug!("studio removal complete, status={:?}", exit_status);
        if exit_status.success() {
            if let Some(err) = fs::remove_dir_all(self.workspace.src()).err() {
                return Err(Error::WorkspaceTeardown(format!("{}", self.workspace.src().display()),
                                                    err));
            }
            Ok(())
        } else {
            Err(Error::BuildFailure(exit_status.code().unwrap_or(-1)))
        }
    }
}

/// Client for sending and receiving messages to and from the Job Runner
pub struct RunnerCli {
    sock: zmq::Socket,
    msg: zmq::Message,
}

impl RunnerCli {
    /// Create a new Job Runner client
    pub fn new() -> Self {
        let sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
        RunnerCli {
            sock: sock,
            msg: zmq::Message::new().unwrap(),
        }
    }

    /// Return a poll item used in `zmq::poll` for awaiting messages on multiple sockets
    pub fn as_poll_item<'a>(&'a self, events: i16) -> zmq::PollItem<'a> {
        self.sock.as_poll_item(events)
    }

    /// Connect to the Job Runner
    pub fn connect(&mut self) -> Result<()> {
        try!(self.sock.connect(INPROC_ADDR));
        Ok(())
    }

    /// Wait until client receives a work received acknowledgement by the Runner and return
    /// the assigned JobID.
    pub fn recv_ack(&mut self) -> Result<&zmq::Message> {
        try!(self.sock.recv(&mut self.msg, 0));
        if Some(WORK_ACK) != self.msg.as_str() {
            unreachable!("wk:run:1, received unexpected response from runner");
        }
        try!(self.sock.recv(&mut self.msg, 0));
        Ok(&self.msg)
    }

    /// Wait until client receives a work complete message by the Runner and return an encoded
    /// representation of the job.
    pub fn recv_complete(&mut self) -> Result<&zmq::Message> {
        try!(self.sock.recv(&mut self.msg, 0));
        if Some(WORK_COMPLETE) != self.msg.as_str() {
            unreachable!("wk:run:2, received unexpected response from runner");
        }
        try!(self.sock.recv(&mut self.msg, 0));
        Ok(&self.msg)
    }

    /// Send a message to the Job Runner
    pub fn send(&mut self, msg: &zmq::Message) -> Result<()> {
        try!(self.sock.send(&*msg, 0));
        Ok(())
    }
}

/// Receives work notifications from a `RunnerCli` and performs long-running tasks in a
/// separate thread.
pub struct RunnerMgr {
    sock: zmq::Socket,
    msg: zmq::Message,
    config: Arc<RwLock<Config>>,
}

impl RunnerMgr {
    /// Start the Job Runner
    pub fn start(config: Arc<RwLock<Config>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::Builder::new()
            .name("runner".to_string())
            .spawn(move || {
                let mut runner = Self::new(config).unwrap();
                runner.run(tx).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("runner thread startup error, err={}", e),
        }
    }

    fn new(config: Arc<RwLock<Config>>) -> Result<Self> {
        let sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        Ok(RunnerMgr {
            sock: sock,
            msg: zmq::Message::new().unwrap(),
            config: config,
        })
    }

    // Main loop for server
    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        try!(self.sock.bind(INPROC_ADDR));
        rz.send(()).unwrap();
        loop {
            let job = try!(self.recv_job());
            try!(self.send_ack(&job));
            try!(self.execute_job(job));
        }
    }

    fn execute_job(&mut self, job: Job) -> Result<()> {
        let runner = {
            Runner::new(job, &self.config.read().unwrap())
        };
        debug!("executing work, job={:?}", runner.job());
        let job = runner.run();
        self.send_complete(&job)
    }

    fn recv_job(&mut self) -> Result<Job> {
        try!(self.sock.recv(&mut self.msg, 0));
        let job: proto::Job = parse_from_bytes(&self.msg).unwrap();
        Ok(Job::new(job))
    }

    fn send_ack(&mut self, job: &Job) -> Result<()> {
        debug!("received work, job={:?}", job);
        try!(self.sock.send_str(WORK_ACK, zmq::SNDMORE));
        try!(self.sock.send(&*job.write_to_bytes().unwrap(), 0));
        Ok(())
    }

    fn send_complete(&mut self, job: &Job) -> Result<()> {
        debug!("work complete, job={:?}", job);
        try!(self.sock.send_str(WORK_COMPLETE, zmq::SNDMORE));
        try!(self.sock.send(&*job.write_to_bytes().unwrap(), 0));
        Ok(())
    }
}

fn studio_cmd() -> String {
    match PackageInstall::load(&STUDIO_PKG, None) {
        Ok(package) => format!("{}/hab-studio", package.paths().unwrap()[0].display()),
        Err(_) => {
            panic!("core/hab-studio not found! This should be available as it is a runtime \
                    dependency in the worker's plan.sh and also present in our dev Dockerfile")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{jobsrv, vault};

    #[test]
    fn extract_origin_from_job() {
        let mut inner = jobsrv::Job::new();
        let mut project = vault::Project::new();
        project.set_id("core/nginx".to_string());
        inner.set_project(project);
        let job = Job::new(inner);
        assert_eq!(job.origin(), "core");
    }
}
