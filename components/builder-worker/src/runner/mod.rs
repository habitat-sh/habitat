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

use std::ffi::OsString;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};

use depot_client;
use hab_core::package::archive::PackageArchive;
use hab_core::package::install::PackageInstall;
use hab_core::package::PackageIdent;
use hab_net::server::ZMQ_CONTEXT;
use protobuf::{parse_from_bytes, Message};
use protocol::jobsrv as proto;
use zmq;

use self::logger::Logger;
use self::workspace::Workspace;
use config::Config;
use error::{Error, Result};
use vcs;
use {PRODUCT, VERSION};

/// In-memory zmq address of Job RunnerMgr
const INPROC_ADDR: &'static str = "inproc://runner";
/// Protocol message to indicate the Job Runner has received a work request
const WORK_ACK: &'static str = "A";
/// Protocol message to indicate the Job Runner has completed a work request
const WORK_COMPLETE: &'static str = "C";

lazy_static! {
    static ref STUDIO_PKG: PackageIdent = PackageIdent::from_str("core/hab-studio").unwrap();
}

pub struct Runner {
    pub job: proto::Job,
    auth_token: String,
    workspace: Workspace,
}

impl Runner {
    pub fn new(job: proto::Job, config: &Config) -> Self {
        Runner {
            auth_token: config.auth_token.clone(),
            workspace: Workspace::new(config.data_path.clone(), &job),
            job: job,
        }
    }

    pub fn run(&mut self) -> () {
        if let Some(err) = self.workspace.setup().err() {
            error!("WORKSPACE SETUP ERR={:?}", err);
            return self.fail();
        }
        let mut logger = logger::Logger::new(&self.workspace);
        // JW TODO: How are we going to get the secret keys for this thing?
        if let Some(err) = vcs::clone(&self.job, &self.workspace.src()).err() {
            error!("CLONE ERROR={}", err);
            return self.fail();
        }
        let mut archive = match self.build(&mut logger) {
            Ok(archive) => archive,
            Err(err) => {
                error!("STUDIO ERR={}", err);
                return self.fail();
            }
        };
        if !self.post_process(&mut archive) {
            // JW TODO: We should shelve the built artifacts and allow a retry on post-processing.
            // If the job is killed then we can kill the shelved artifacts.
            return self.fail();
        }
        self.complete()
    }

    pub fn build(&self, logger: &mut Logger) -> Result<PackageArchive> {
        let args = vec![OsString::from("-s"),
                        OsString::from(self.workspace.src()),
                        OsString::from("-r"),
                        OsString::from(self.workspace.studio()),
                        OsString::from("build"),
                        OsString::from("-R"),
                        OsString::from(Path::new(self.job.get_project().get_plan_path())
                            .parent()
                            .unwrap())];
        let command = match PackageInstall::load(&STUDIO_PKG, None) {
            Ok(package) => format!("{}/hab-studio", package.paths().unwrap()[0].display()),
            Err(_) => panic!("should install core/hab-studio because it's missing?"),
        };
        debug!("building, cmd={:?}, args={:?}", command, args);
        let mut child = Command::new(command)
            .args(&args)
            .env_clear()
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn child");
        if let Some(ref mut stdout) = child.stdout {
            for line in BufReader::new(stdout).lines() {
                let mut l: String = line.unwrap();
                let eol: &str = "\n";
                l = l + eol;
                logger.log(l.as_bytes());
            }
        }
        let exit_status = child.wait().expect("failed to wait on child");
        println!("DONE: {:?}", exit_status);
        if exit_status.success() {
            // actually figure out where the thing that was built came from by reading the results
            // of last_build.env
            let archive = PackageArchive::new(self.workspace.src().join("results"));
            Ok(archive)
        } else {
            Err(Error::BuildFailure(exit_status.code().unwrap_or(-1)))
        }
    }

    fn complete(&mut self) -> () {
        self.workspace.teardown().err().map(|e| error!("{}", e));
        self.job.set_state(proto::JobState::Complete);
    }

    fn fail(&mut self) -> () {
        self.job.set_state(proto::JobState::Failed);
    }

    fn post_process(&self, archive: &mut PackageArchive) -> bool {
        // JW TODO: In the future we'll support multiple and configurable post processors, but for
        // now let's just publish to the public depot
        //
        // Things to solve right now
        // * Where do we get the token for authentication?
        //      * Should the workers ask for a lease from the JobSrv?
        let client =
            depot_client::Client::new("https://app.habitat.sh/v1/depot", PRODUCT, VERSION, None)
                .unwrap();
        client.x_put_package(archive, &self.auth_token).unwrap();
        true
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
            let job: proto::Job = try!(self.recv_job());
            try!(self.send_ack(&job));
            try!(self.execute_job(job));
        }
    }

    fn execute_job(&mut self, job: proto::Job) -> Result<()> {
        let mut runner = {
            Runner::new(job, &self.config.read().unwrap())
        };
        debug!("executing work, job={:?}", runner.job);
        runner.run();
        self.send_complete(&runner.job)
    }

    fn recv_job(&mut self) -> Result<proto::Job> {
        try!(self.sock.recv(&mut self.msg, 0));
        let job: proto::Job = parse_from_bytes(&self.msg).unwrap();
        Ok(job)
    }

    fn send_ack(&mut self, job: &proto::Job) -> Result<()> {
        debug!("received work, job={:?}", job);
        try!(self.sock.send_str(WORK_ACK, zmq::SNDMORE));
        try!(self.sock.send(&job.write_to_bytes().unwrap(), 0));
        Ok(())
    }

    fn send_complete(&mut self, job: &proto::Job) -> Result<()> {
        debug!("work complete, job={:?}", job);
        try!(self.sock.send_str(WORK_COMPLETE, zmq::SNDMORE));
        try!(self.sock.send(&job.write_to_bytes().unwrap(), 0));
        Ok(())
    }
}
