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

use std::sync::{mpsc, Arc, RwLock};
use std::time::Duration;
use std::thread::{self, JoinHandle};

use hab_net::server::{NetIdent, ZMQ_CONTEXT};
use protobuf::{parse_from_bytes, Message};
use protocol;
use zmq;

use config::Config;
use error::Result;
use runner::{RunnerCli, RunnerMgr};

const HEARTBEAT_MS: i64 = 30_000;
const HB_INPROC_ADDR: &'static str = "inproc://heartbeat";
const HB_CMD_PULSE: &'static str = "R";
const HB_CMD_PAUSE: &'static str = "P";

#[cfg(target_os = "linux")]
fn worker_os() -> protocol::jobsrv::Os {
    protocol::jobsrv::Os::Linux
}

#[cfg(target_os = "windows")]
fn worker_os() -> protocol::jobsrv::Os {
    protocol::jobsrv::Os::Windows
}

#[cfg(target_os = "macos")]
fn worker_os() -> protocol::jobsrv::Os {
    protocol::jobsrv::Os::Darwin
}

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
    hb_conn: zmq::Socket,
    runner_cli: RunnerCli,
    state: State,
    msg: zmq::Message,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let fe_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        let hb_conn = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::REQ));
        let runner_cli = RunnerCli::new();
        try!(fe_sock.set_identity(Self::net_ident().as_bytes()));
        Ok(Server {
            config: Arc::new(RwLock::new(config)),
            fe_sock: fe_sock,
            hb_conn: hb_conn,
            runner_cli: runner_cli,
            state: State::default(),
            msg: try!(zmq::Message::new()),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let cfg1 = self.config.clone();
        let heartbeat = try!(Heartbeat::start(cfg1));
        let runner = try!(RunnerMgr::start());
        try!(self.hb_conn.connect(HB_INPROC_ADDR));
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
        try!(self.hb_conn.send_str(PulseState::Pause.as_ref(), 0));
        try!(self.hb_conn.recv(&mut self.msg, 0));
        self.state = State::Busy;
        Ok(())
    }

    fn set_ready(&mut self) -> Result<()> {
        try!(self.hb_conn.send_str(PulseState::Pulse.as_ref(), 0));
        try!(self.hb_conn.recv(&mut self.msg, 0));
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

#[derive(PartialEq)]
enum PulseState {
    Pause,
    Pulse,
}

impl AsRef<str> for PulseState {
    fn as_ref(&self) -> &str {
        match *self {
            PulseState::Pause => HB_CMD_PAUSE,
            PulseState::Pulse => HB_CMD_PULSE,
        }
    }
}

impl Default for PulseState {
    fn default() -> PulseState {
        PulseState::Pulse
    }
}

struct Heartbeat {
    state: PulseState,
    config: Arc<RwLock<Config>>,
    pub_sock: zmq::Socket,
    be_sock: zmq::Socket,
    reg: protocol::jobsrv::Heartbeat,
}

impl Heartbeat {
    fn new(config: Arc<RwLock<Config>>) -> Result<Self> {
        let pub_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::PUB));
        let be_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::REP));
        try!(pub_sock.set_immediate(true));
        try!(pub_sock.set_sndhwm(1));
        try!(pub_sock.set_linger(0));
        let mut reg = protocol::jobsrv::Heartbeat::new();
        reg.set_endpoint(Server::net_ident());
        reg.set_os(worker_os());
        reg.set_state(protocol::jobsrv::WorkerState::Ready);
        Ok(Heartbeat {
            state: PulseState::default(),
            config: config,
            pub_sock: pub_sock,
            be_sock: be_sock,
            reg: reg,
        })
    }

    pub fn start(config: Arc<RwLock<Config>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::Builder::new()
            .name("heartbeat".to_string())
            .spawn(move || {
                let mut heartbeat = Self::new(config).unwrap();
                heartbeat.run(tx).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("heartbeat thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        {
            let cfg = self.config.read().unwrap();
            for (hb, _) in cfg.jobsrv_addrs() {
                println!("Connecting to heartbeat, {}", hb);
                try!(self.pub_sock.connect(&hb));
            }
        }
        try!(self.be_sock.bind(HB_INPROC_ADDR));
        rz.send(()).unwrap();
        // Needed for connections to establish. Wow zmq. Good thing we need to use a splay anyway.
        thread::sleep(Duration::from_millis(100));
        let mut be_sock = false;
        let mut msg = try!(zmq::Message::new());
        loop {
            if self.state == PulseState::Pulse {
                try!(self.pulse());
            }
            {
                let mut items = [self.be_sock.as_poll_item(1)];
                // Poll until timeout or message is received. Checking for the zmq::POLLIN flag on
                // a poll item's revents will let you know if you have received a message or not
                // on that socket.
                try!(zmq::poll(&mut items, HEARTBEAT_MS));
                if items[0].get_revents() & zmq::POLLIN > 0 {
                    be_sock = true;
                }
            }
            if be_sock {
                try!(self.be_sock.recv(&mut msg, 0));
                match msg.as_str() {
                    Some(HB_CMD_PAUSE) => {
                        self.pause();
                        try!(self.be_sock.send(&[], 0));
                    }
                    Some(HB_CMD_PULSE) => {
                        self.resume();
                        try!(self.be_sock.send(&[], 0));
                    }
                    _ => (),
                }
                be_sock = false;
            }
        }
        Ok(())
    }

    fn pause(&mut self) {
        debug!("heartbeat paused");
        self.reg.set_state(protocol::jobsrv::WorkerState::Busy);
        self.state = PulseState::Pause;
    }

    fn resume(&mut self) {
        debug!("heartbeat resumed");
        self.reg.set_state(protocol::jobsrv::WorkerState::Ready);
        self.state = PulseState::Pulse;
    }

    fn pulse(&mut self) -> Result<()> {
        debug!("heartbeat pulsed");
        try!(self.pub_sock.send(&self.reg.write_to_bytes().unwrap(), 0));
        Ok(())
    }
}

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
