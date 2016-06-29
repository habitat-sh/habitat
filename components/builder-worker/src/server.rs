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

use std::sync::{Arc, RwLock};

use hab_net::server::{NetIdent, ZMQ_CONTEXT};
use protobuf::{parse_from_bytes, Message};
use protocol;
use zmq;

use config::Config;
use error::Result;
use heartbeat::{HeartbeatCli, HeartbeatMgr};
use runner::{RunnerCli, RunnerMgr};

enum State {
    Ready,
    Busy,
}

impl Default for State {
    fn default() -> State {
        State::Ready
    }
}

pub struct Server {
    config: Arc<RwLock<Config>>,
    fe_sock: zmq::Socket,
    hb_cli: HeartbeatCli,
    runner_cli: RunnerCli,
    state: State,
    msg: zmq::Message,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let fe_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        let hb_cli = HeartbeatCli::new();
        let runner_cli = RunnerCli::new();
        try!(fe_sock.set_identity(Self::net_ident().as_bytes()));
        Ok(Server {
            config: Arc::new(RwLock::new(config)),
            fe_sock: fe_sock,
            hb_cli: hb_cli,
            runner_cli: runner_cli,
            state: State::default(),
            msg: try!(zmq::Message::new()),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let cfg1 = self.config.clone();
        let heartbeat = try!(HeartbeatMgr::start(cfg1));
        let runner = try!(RunnerMgr::start());
        try!(self.hb_cli.connect());
        try!(self.runner_cli.connect());

        {
            let cfg = self.config.read().unwrap();
            for (_, queue) in cfg.jobsrv_addrs() {
                println!("Connecting to job queue, {}", queue);
                try!(self.fe_sock.connect(&queue));
            }
        }

        let mut fe_msg = false;
        let mut runner_msg = false;
        let mut reply = protocol::jobsrv::Job::new();
        loop {
            {
                let mut items = [self.fe_sock.as_poll_item(1), self.runner_cli.as_poll_item(1)];
                try!(zmq::poll(&mut items, -1));
                if items[0].get_revents() & zmq::POLLIN > 0 {
                    fe_msg = true;
                }
                if items[1].get_revents() & zmq::POLLIN > 0 {
                    runner_msg = true;
                }
            }
            if runner_msg {
                let job = try!(self.runner_cli.recv_complete());
                try!(self.fe_sock.send(&*job, 0));
                try!(self.set_ready());
                runner_msg = false;
            }
            if fe_msg {
                try!(self.fe_sock.recv(&mut self.msg, 0));
                try!(self.fe_sock.recv(&mut self.msg, 0));
                match self.state {
                    State::Ready => {
                        try!(self.runner_cli.send(&self.msg));
                        let job_id: u64 = try!(self.runner_cli.recv_ack());
                        reply.set_id(job_id);
                        reply.set_state(protocol::jobsrv::JobState::Processing);
                        try!(self.set_busy());
                        try!(self.fe_sock.send(&try!(reply.write_to_bytes()), 0));
                    }
                    State::Busy => {
                        reply = parse_from_bytes(&self.msg).unwrap();
                        reply.set_state(protocol::jobsrv::JobState::Rejected);
                        let bytes = try!(reply.write_to_bytes());
                        try!(self.fe_sock.send(&bytes, 0));
                    }
                }
                fe_msg = false;
            }
        }
        heartbeat.join().unwrap();
        runner.join().unwrap();
        Ok(())
    }

    fn set_busy(&mut self) -> Result<()> {
        try!(self.hb_cli.set_busy());
        self.state = State::Busy;
        Ok(())
    }

    fn set_ready(&mut self) -> Result<()> {
        try!(self.hb_cli.set_ready());
        self.state = State::Ready;
        Ok(())
    }
}

impl NetIdent for Server {}

impl Drop for Server {
    fn drop(&mut self) {
        self.fe_sock.close().unwrap();
    }
}

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
