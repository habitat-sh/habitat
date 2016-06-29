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

use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use hab_net::server::ZMQ_CONTEXT;
use protobuf::{parse_from_bytes, Message};
use protocol::jobsrv as proto;
use zmq;

use error::Result;

/// In-memory zmq address of Job Runner
const INPROC_ADDR: &'static str = "inproc://runner";
/// Protocol message to indicate the Job Runner has received a work request
const WORK_ACK: &'static str = "A";
/// Protocol message to indicate the Job Runner has completed a work request
const WORK_COMPLETE: &'static str = "C";

/// Client for sending and receiving messages from a Job Runner
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
    pub fn recv_ack(&mut self) -> Result<u64> {
        try!(self.sock.recv(&mut self.msg, 0));
        if Some(WORK_ACK) != self.msg.as_str() {
            unreachable!("runcli:1, received unexpected response from runner");
        }
        try!(self.sock.recv(&mut self.msg, 0));
        let job_id = self.msg.as_str().unwrap().parse().unwrap();
        Ok(job_id)
    }

    /// Wait until client receives a work complete message by the Runner and return an encoded
    /// representation of the job.
    pub fn recv_complete(&mut self) -> Result<Vec<u8>> {
        try!(self.sock.recv(&mut self.msg, 0));
        if Some(WORK_COMPLETE) != self.msg.as_str() {
            unreachable!("runcli:2, received unexpected response from runner");
        }
        try!(self.sock.recv(&mut self.msg, 0));
        Ok(self.msg.to_vec())
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
    fn new() -> Result<Self> {
        let sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        Ok(RunnerMgr {
            sock: sock,
            msg: zmq::Message::new().unwrap(),
        })
    }

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

    // Main loop for server
    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        try!(self.sock.bind(INPROC_ADDR));
        rz.send(()).unwrap();
        loop {
            let mut job: proto::Job = try!(self.recv_job());
            try!(self.send_ack(&job));
            try!(self.execute_job(&mut job));
            try!(self.send_complete(&job));
        }
        Ok(())
    }

    fn execute_job(&mut self, job: &mut proto::Job) -> Result<()> {
        debug!("executing work, job={:?}", job);
        // check source and delegate to runner
        // retrieve source
        // enter studio and build plan
        // publish work
        job.set_state(proto::JobState::Complete);
        Ok(())
    }

    fn recv_job(&mut self) -> Result<proto::Job> {
        try!(self.sock.recv(&mut self.msg, 0));
        let job: proto::Job = parse_from_bytes(&self.msg).unwrap();
        Ok(job)
    }

    fn send_ack(&mut self, job: &proto::Job) -> Result<()> {
        debug!("received work, job={:?}", job);
        try!(self.sock.send_str(WORK_ACK, zmq::SNDMORE));
        try!(self.sock.send_str(&job.get_id().to_string(), 0));
        Ok(())
    }

    fn send_complete(&mut self, job: &proto::Job) -> Result<()> {
        debug!("work complete, job={:?}", job);
        try!(self.sock.send_str(WORK_COMPLETE, zmq::SNDMORE));
        try!(self.sock.send(&job.write_to_bytes().unwrap(), 0));
        Ok(())
    }
}
