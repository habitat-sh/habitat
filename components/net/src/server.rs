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

use std::cell::UnsafeCell;
use std::error;
use std::result;
use std::sync::{Arc, RwLock};

use core::os::process;
use fnv::FnvHasher;
use protobuf::{self, parse_from_bytes};
use protobuf::core::Message as ProtoBufMessage;
use protocol::{self, Routable, RouteKey};
use time;
use zmq;

use config::{self, RouterCfg, Shards, ToAddrString};
use error::{Error, Result};

const PING_INTERVAL: i64 = 2000;
const SERVER_TTL: i64 = 6000;
const MAX_HOPS: usize = 8;

lazy_static! {
    /// A threadsafe shared ZMQ context for consuming services.
    ///
    /// You probably want to use this context to create new ZMQ sockets unless you *do not* want to
    /// connect them together using an in-proc queue.
    pub static ref ZMQ_CONTEXT: Box<ServerContext> = {
        let ctx = ServerContext::new();
        Box::new(ctx)
    };
}

/// This is a wrapper to provide interior mutability of an underlying `zmq::Context` and allows
/// for sharing/sending of a `zmq::Context` between threads.
pub struct ServerContext(UnsafeCell<zmq::Context>);

impl ServerContext {
    pub fn new() -> Self {
        ServerContext(UnsafeCell::new(zmq::Context::new()))
    }

    pub fn as_mut(&self) -> &mut zmq::Context {
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl Send for ServerContext {}
unsafe impl Sync for ServerContext {}

pub struct Envelope {
    pub msg: protocol::net::Msg,
    hops: Vec<zmq::Message>,
    started: bool,
}

impl Envelope {
    pub fn new(hops: Vec<zmq::Message>, msg: protocol::net::Msg) -> Self {
        let mut env = Envelope::default();
        env.hops = hops;
        env.msg = msg;
        env
    }

    pub fn add_hop(&mut self, hop: zmq::Message) -> Result<()> {
        if self.max_hops() {
            return Err(Error::MaxHops);
        }
        self.hops.push(hop);
        Ok(())
    }

    pub fn body(&self) -> &[u8] {
        self.msg.get_body()
    }

    pub fn hops(&self) -> &Vec<zmq::Message> {
        &self.hops
    }

    pub fn max_hops(&self) -> bool {
        self.hops.len() >= MAX_HOPS
    }

    pub fn message_id(&self) -> &str {
        self.msg.get_message_id()
    }

    pub fn route_info(&self) -> &protocol::net::RouteInfo {
        self.msg.get_route_info()
    }

    pub fn protocol(&self) -> protocol::net::Protocol {
        self.msg.get_route_info().get_protocol()
    }

    pub fn reply<M: ProtoBufMessage>(&mut self, sock: &mut zmq::Socket, msg: &M) -> Result<()> {
        try!(self.send_header(sock));
        let rep = protocol::Message::new(msg).build();
        try!(sock.send(&rep.write_to_bytes().unwrap(), zmq::SNDMORE));
        Ok(())
    }

    pub fn reply_complete<M: ProtoBufMessage>(
        &mut self,
        sock: &mut zmq::Socket,
        msg: &M,
    ) -> Result<()> {
        try!(self.send_header(sock));
        let rep = protocol::Message::new(msg).build();
        let bytes = try!(rep.write_to_bytes());
        try!(sock.send(&bytes, 0));
        Ok(())
    }

    pub fn parse_msg<M: protobuf::MessageStatic>(&self) -> Result<M> {
        let msg: M = try!(parse_from_bytes(&self.body()));
        Ok(msg)
    }

    pub fn reset(&mut self) {
        self.started = false;
        self.hops.clear();
        self.msg = protocol::net::Msg::new();
    }

    fn send_header(&mut self, sock: &mut zmq::Socket) -> Result<()> {
        if !self.started {
            for hop in self.hops.iter() {
                sock.send(hop, zmq::SNDMORE).unwrap();
            }
            sock.send(&[], zmq::SNDMORE).unwrap();
            sock.send_str("RP", zmq::SNDMORE).unwrap();
            self.started = true;
        }
        Ok(())
    }
}

impl Default for Envelope {
    fn default() -> Envelope {
        Envelope {
            msg: protocol::net::Msg::new(),
            hops: Vec::with_capacity(MAX_HOPS),
            started: false,
        }
    }
}

pub trait Application {
    type Error: error::Error;

    fn run(&mut self) -> result::Result<(), Self::Error>;
}

pub trait NetIdent {
    fn component() -> Option<&'static str> {
        None
    }

    fn net_ident() -> String {
        let hostname = super::hostname().unwrap();
        let pid = process::current_pid();
        if let Some(component) = Self::component() {
            format!("{}#{}@{}", component, pid, hostname)
        } else {
            format!("{}@{}", pid, hostname)
        }
    }
}

pub trait Service: NetIdent {
    type Application: Application;
    type Config: config::RouterCfg + config::Shards;
    type Error: error::Error + From<Error> + From<zmq::Error>;

    fn protocol() -> protocol::net::Protocol;

    fn config(&self) -> &Arc<RwLock<Self::Config>>;

    fn conn(&self) -> &RouteConn;
    fn conn_mut(&mut self) -> &mut RouteConn;

    fn connect(&mut self) -> result::Result<(), Self::Error> {
        let mut reg = protocol::routesrv::Registration::new();
        reg.set_protocol(Self::protocol());
        reg.set_endpoint(Self::net_ident());
        let (hb_addrs, addrs) = {
            let cfg = self.config().read().unwrap();
            reg.set_shards(cfg.shards().clone());
            let hb_addrs: Vec<String> = cfg.route_addrs()
                .iter()
                .map(|f| format!("tcp://{}:{}", f.host, f.heartbeat))
                .collect();
            let addrs: Vec<String> = cfg.route_addrs()
                .iter()
                .map(|f| f.to_addr_string())
                .collect();
            (hb_addrs, addrs)
        };
        for addr in &hb_addrs {
            println!("Connecting to {:?}...", addr);
            try!(self.conn_mut().register(&addr));
        }
        let mut ready = 0;
        let mut rt = try!(zmq::Message::new());
        let mut hb = try!(zmq::Message::new());
        while ready < hb_addrs.len() {
            try!(self.conn_mut().heartbeat.recv(&mut rt, 0));
            try!(self.conn_mut().heartbeat.recv(&mut hb, 0));
            debug!("received reg request, {:?}", hb.as_str());
            try!(self.conn_mut().heartbeat.send_str("R", zmq::SNDMORE));
            try!(self.conn_mut().heartbeat.send(
                &reg.write_to_bytes().unwrap(),
                0,
            ));
            try!(self.conn_mut().heartbeat.recv(&mut hb, 0));
            ready += 1;
        }
        for addr in addrs {
            try!(self.conn_mut().connect(&addr));
        }
        println!("Connected");
        Ok(())
    }
}

#[derive(Eq, Hash)]
pub struct ServerReg {
    /// Server identifier
    pub endpoint: String,
    /// True if known to be alive
    pub alive: bool,
    /// Next ping at this time
    pub ping_at: i64,
    /// Connection expires at this time
    pub expires: i64,
}

impl ServerReg {
    pub fn new(endpoint: String) -> Self {
        let now_ms = Self::clock_time();
        ServerReg {
            endpoint: endpoint,
            alive: false,
            ping_at: now_ms + PING_INTERVAL,
            expires: now_ms + SERVER_TTL,
        }
    }

    pub fn clock_time() -> i64 {
        let timespec = time::get_time();
        (timespec.sec as i64 * 1000) + (timespec.nsec as i64 / 1000 / 1000)
    }

    pub fn ping(&mut self, socket: &mut zmq::Socket) -> Result<()> {
        let now_ms = Self::clock_time();
        if now_ms >= self.ping_at {
            let ping = protocol::net::Ping::new();
            let req = protocol::Message::new(&ping).build();
            let bytes = try!(req.write_to_bytes());
            try!(socket.send(&bytes, 0));
            self.ping_at = Self::clock_time() + PING_INTERVAL;
        }
        Ok(())
    }
}

impl PartialEq for ServerReg {
    fn eq(&self, other: &ServerReg) -> bool {
        if self.endpoint != other.endpoint {
            return false;
        }
        true
    }
}

pub struct RouteConn {
    pub ident: String,
    pub socket: zmq::Socket,
    pub heartbeat: zmq::Socket,
}

impl RouteConn {
    pub fn new(ident: String, context: &mut zmq::Context) -> Result<Self> {
        let socket = try!(context.socket(zmq::DEALER));
        let heartbeat = try!(context.socket(zmq::DEALER));
        try!(socket.set_identity(ident.as_bytes()));
        try!(heartbeat.set_identity(format!("hb#{}", ident).as_bytes()));
        try!(heartbeat.set_probe_router(true));
        Ok(RouteConn {
            ident: ident,
            socket: socket,
            heartbeat: heartbeat,
        })
    }

    pub fn connect(&mut self, addr: &str) -> Result<()> {
        try!(self.socket.connect(addr));
        Ok(())
    }

    pub fn register(&mut self, addr: &str) -> Result<()> {
        try!(self.heartbeat.connect(addr));
        Ok(())
    }

    pub fn recv(&mut self, flags: i32) -> Result<protocol::net::Msg> {
        let envelope = try!(self.socket.recv_msg(flags));
        let msg: protocol::net::Msg = parse_from_bytes(&envelope).unwrap();
        Ok(msg)
    }

    pub fn route<M: Routable>(&mut self, msg: &M) -> Result<()> {
        let route_hash = msg.route_key().map(
            |key| key.hash(&mut FnvHasher::default()),
        );
        let req = protocol::Message::new(msg).routing(route_hash).build();
        let bytes = try!(req.write_to_bytes());
        try!(self.socket.send(&bytes, 0));
        Ok(())
    }
}
