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

use std::path::PathBuf;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use hab_net::server::ZMQ_CONTEXT;
use protobuf::{parse_from_bytes, Message};
use protocol::jobsrv as proto;
use zmq;

use error::{Error, Result};
use studio;
use vcs;

/// In-memory zmq address of Job Runner
const INPROC_ADDR: &'static str = "inproc://runner";
/// Protocol message to indicate the Job Runner has received a work request
const WORK_ACK: &'static str = "A";
/// Protocol message to indicate the Job Runner has completed a work request
const WORK_COMPLETE: &'static str = "C";

pub struct Runner {
    pub job: proto::Job,
    pub workspace: PathBuf,
}

impl Runner {
    pub fn new(job: proto::Job) -> Self {
        // JW TODO: this needs to be cleaned or something maybe? Stored somewhere?
        // Would be pretty awesome if the workspace could be shelved into S3 and then re-mounted
        // during a run.
        let workspace = PathBuf::from("/tmp/workspace");
        Runner {
            job: job,
            workspace: workspace,
        }
    }

    pub fn run(&mut self) -> () {
        // the studio should be cloning the workspace I think
        if let Some(err) = vcs::clone(&self.job, &self.workspace).err() {
            println!("CLONE ERROR={:?}", err);
            return self.fail();
        }
        if let Some(err) = studio::build(&self.job, &self.workspace).err() {
            println!("BUILD ERROR={:?}", err);
            return self.fail();
        }
        self.complete()
    }

    fn complete(&mut self) -> () {
        // clean workspace?
        self.job.set_state(proto::JobState::Complete);
    }

    fn fail(&mut self) -> () {
        // clean workspace?
        self.job.set_state(proto::JobState::Failed);
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
}

impl RunnerMgr {
    /// Start the Job Runner
    pub fn start() -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::Builder::new()
            .name("runner".to_string())
            .spawn(move || {
                let mut runner = Self::new().unwrap();
                runner.run(tx).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("runner thread startup error, err={}", e),
        }
    }

    fn new() -> Result<Self> {
        let sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        Ok(RunnerMgr {
            sock: sock,
            msg: zmq::Message::new().unwrap(),
        })
    }

    // Main loop for server
    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        try!(self.sock.bind(INPROC_ADDR));
        rz.send(()).unwrap();
        loop {
            let mut job: proto::Job = try!(self.recv_job());
            try!(self.send_ack(&job));
            try!(self.execute_job(job));
        }
        Ok(())
    }

    fn execute_job(&mut self, job: proto::Job) -> Result<()> {
        let mut runner = Runner::new(job);
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
