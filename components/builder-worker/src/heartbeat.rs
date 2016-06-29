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
use protobuf::Message;
use protocol::jobsrv as proto;
use zmq;

use config::Config;
use error::Result;
use server::Server;

/// Polling timeout for HeartbeatMgr
const HEARTBEAT_MS: i64 = 30_000;
/// In-memory zmq address for HeartbeatMgr
const INPROC_ADDR: &'static str = "inproc://heartbeat";
/// Protocol message to notify the HeartbeatMgr to begin pulsing
const CMD_PULSE: &'static str = "R";
/// Protocol message to notify the HeartbeatMgr to pause pulsing
const CMD_PAUSE: &'static str = "P";

#[cfg(target_os = "linux")]
fn worker_os() -> proto::Os {
    proto::Os::Linux
}

#[cfg(target_os = "windows")]
fn worker_os() -> proto::Os {
    proto::Os::Windows
}

#[cfg(target_os = "macos")]
fn worker_os() -> proto::Os {
    proto::Os::Darwin
}

#[derive(PartialEq)]
enum PulseState {
    Pause,
    Pulse,
}

impl AsRef<str> for PulseState {
    fn as_ref(&self) -> &str {
        match *self {
            PulseState::Pause => CMD_PAUSE,
            PulseState::Pulse => CMD_PULSE,
        }
    }
}

impl Default for PulseState {
    fn default() -> PulseState {
        PulseState::Pulse
    }
}

/// Client for sending and receiving messages to and from the HeartbeatMgr
pub struct HeartbeatCli {
    sock: zmq::Socket,
    msg: zmq::Message,
}

impl HeartbeatCli {
    /// Create a new HeartbeatMgr client
    pub fn new() -> Self {
        let sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::REQ).unwrap();
        HeartbeatCli {
            sock: sock,
            msg: zmq::Message::new().unwrap(),
        }
    }

    /// Connect to the `HeartbeatMgr`
    pub fn connect(&mut self) -> Result<()> {
        try!(self.sock.connect(INPROC_ADDR));
        Ok(())
    }

    /// Set the `HeartbeatMgr` state to busy
    pub fn set_busy(&mut self) -> Result<()> {
        try!(self.sock.send_str(PulseState::Pause.as_ref(), 0));
        try!(self.sock.recv(&mut self.msg, 0));
        Ok(())
    }

    /// Set the `HeartbeatMgr` state to ready
    pub fn set_ready(&mut self) -> Result<()> {
        try!(self.sock.send_str(PulseState::Pulse.as_ref(), 0));
        try!(self.sock.recv(&mut self.msg, 0));
        Ok(())
    }
}

/// Maintains and broadcasts health and state of the Worker server to consumers
pub struct HeartbeatMgr {
    /// Public socket for publishing worker state to consumers
    pub pub_sock: zmq::Socket,
    /// Internal socket for sending and receiving message to and from a `HearbeatCli`
    pub cli_sock: zmq::Socket,
    state: PulseState,
    config: Arc<RwLock<Config>>,
    reg: proto::Heartbeat,
    msg: zmq::Message,
}

impl HeartbeatMgr {
    /// Start the HeartbeatMgr
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

    fn new(config: Arc<RwLock<Config>>) -> Result<Self> {
        let pub_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::PUB));
        let cli_sock = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::REP));
        try!(pub_sock.set_immediate(true));
        try!(pub_sock.set_sndhwm(1));
        try!(pub_sock.set_linger(0));
        let mut reg = proto::Heartbeat::new();
        reg.set_endpoint(Server::net_ident());
        reg.set_os(worker_os());
        reg.set_state(proto::WorkerState::Ready);
        Ok(HeartbeatMgr {
            state: PulseState::default(),
            config: config,
            pub_sock: pub_sock,
            cli_sock: cli_sock,
            reg: reg,
            msg: zmq::Message::new().unwrap(),
        })
    }

    // Main loop for server
    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        {
            let cfg = self.config.read().unwrap();
            for (hb, _) in cfg.jobsrv_addrs() {
                println!("Connecting to heartbeat, {}", hb);
                try!(self.pub_sock.connect(&hb));
            }
        }
        try!(self.cli_sock.bind(INPROC_ADDR));
        rz.send(()).unwrap();
        // This hackey sleep is recommended and required by zmq for connections to establish
        thread::sleep(Duration::from_millis(100));
        let mut cli_sock_msg = false;
        loop {
            if self.state == PulseState::Pulse {
                try!(self.pulse());
            }
            {
                let mut items = [self.cli_sock.as_poll_item(1)];
                // Poll until timeout or message is received. Checking for the zmq::POLLIN flag on
                // a poll item's revents will let you know if you have received a message or not
                // on that socket.
                try!(zmq::poll(&mut items, HEARTBEAT_MS));
                if items[0].get_revents() & zmq::POLLIN > 0 {
                    cli_sock_msg = true;
                }
            }
            if cli_sock_msg {
                try!(self.recv_cmd());
                cli_sock_msg = false;
            }
        }
        Ok(())
    }

    // Set internal state to `PulseState::Pause` and notify client OK
    fn pause(&mut self) {
        debug!("heartbeat paused");
        self.reg.set_state(proto::WorkerState::Busy);
        self.state = PulseState::Pause;
        self.cli_sock.send(&[], 0).unwrap();
    }

    // Broadcast to subscribers the HeartbeatMgr health and state
    fn pulse(&mut self) -> Result<()> {
        debug!("heartbeat pulsed");
        try!(self.pub_sock.send(&self.reg.write_to_bytes().unwrap(), 0));
        Ok(())
    }

    // Wait receive for a command from a client
    fn recv_cmd(&mut self) -> Result<()> {
        try!(self.cli_sock.recv(&mut self.msg, 0));
        match self.msg.as_str() {
            Some(CMD_PAUSE) => self.pause(),
            Some(CMD_PULSE) => self.resume(),
            _ => unreachable!("wk:hb:1, received unexpected message from client"),
        }
        Ok(())
    }

    // Set internal state to `PulseState::Pulse` and notify client OK
    fn resume(&mut self) {
        debug!("heartbeat resumed");
        self.reg.set_state(proto::WorkerState::Ready);
        self.state = PulseState::Pulse;
        self.cli_sock.send(&[], 0).unwrap();
    }
}
