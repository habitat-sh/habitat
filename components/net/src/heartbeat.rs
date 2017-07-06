use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use protobuf::Message;
use protocol::net as proto;

use error::Result;
use server::{NetIdent, ZMQ_CONTEXT};
use zmq;

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
