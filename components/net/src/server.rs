// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::error;
use std::marker::PhantomData;
use std::net;
use std::result;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Duration;

use fnv::FnvHasher;
use libc;
use protobuf::{self, parse_from_bytes};
use protobuf::core::Message as ProtoBufMessage;
use protocol::{self, Routable, RouteKey};
use time;
use zmq;

use config::{self, RouteAddrs, Shards};
use error::{Error, Result};

const PING_INTERVAL: i64 = 2000;
const SERVER_TTL: i64 = 6000;
const MAX_HOPS: usize = 8;

pub trait ToAddrString {
    fn to_addr_string(&self) -> String;
}

impl ToAddrString for net::SocketAddrV4 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

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

    pub fn reply_complete<M: ProtoBufMessage>(&mut self,
                                              sock: &mut zmq::Socket,
                                              msg: &M)
                                              -> Result<()> {
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
        let pid = unsafe { libc::getpid() };
        if let Some(component) = Self::component() {
            format!("{}#{}@{}", component, pid, hostname)
        } else {
            format!("{}@{}", pid, hostname)
        }
    }
}

pub trait Service: NetIdent {
    type Application: Application;
    type Config: config::RouteAddrs + config::Shards;
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
                .map(|f| format!("tcp://{}:{}", f.ip(), cfg.heartbeat_port()))
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
            try!(self.conn_mut().heartbeat.send(&reg.write_to_bytes().unwrap(), 0));
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
    hasher: FnvHasher,
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
            hasher: FnvHasher::default(),
        })
    }

    pub fn close(&mut self) -> Result<()> {
        try!(self.socket.close());
        Ok(())
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
        let route_hash = msg.route_key().map(|key| key.hash(&mut self.hasher));
        let req = protocol::Message::new(msg).routing(route_hash).build();
        let bytes = try!(req.write_to_bytes());
        try!(self.socket.send(&bytes, 0));
        Ok(())
    }
}

impl Drop for RouteConn {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

pub struct Supervisor<T>
    where T: Supervisable
{
    context: Arc<RwLock<zmq::Context>>,
    config: Arc<RwLock<T::Config>>,
    workers: Vec<mpsc::Receiver<()>>,
    _marker: PhantomData<T>,
}

impl<T> Supervisor<T>
    where T: Supervisable + 'static
{
    pub fn new(ctx: Arc<RwLock<zmq::Context>>, config: Arc<RwLock<T::Config>>) -> Self {
        Supervisor {
            context: ctx,
            config: config,
            workers: vec![],
            _marker: PhantomData,
        }
    }

    pub fn start(mut self, addr: &str, worker_count: usize) -> super::Result<()> {
        try!(self.init(addr, worker_count));
        debug!("Supervisor ready");
        self.run(addr.to_string(), worker_count)
    }

    fn init(&mut self, addr: &str, worker_count: usize) -> super::Result<()> {
        for _i in 0..worker_count {
            let rx = try!(self.spawn_worker(addr.to_string()));
            self.workers.push(rx);
        }
        let mut success = 0;
        while success != worker_count {
            match self.workers[success].recv() {
                Ok(()) => {
                    debug!("Worker {} ready", success);
                    success += 1;
                }
                Err(_) => debug!("Worker {} failed to start", success),
            }
        }
        Ok(())
    }

    fn run(mut self, addr: String, worker_count: usize) -> super::Result<()> {
        thread::spawn(move || {
            loop {
                for i in 0..worker_count {
                    match self.workers[i].try_recv() {
                        Err(mpsc::TryRecvError::Disconnected) => {
                            println!("Worker {} restarting...", i);
                            let rx = self.spawn_worker(addr.clone()).unwrap();
                            match rx.recv() {
                                Ok(()) => self.workers[i] = rx,
                                Err(_) => {
                                    println!("Worker {} failed restart!", i);
                                    self.workers.remove(i);
                                }
                            }
                        }
                        Ok(msg) => println!("Worker {} sent unexpected msg: {:?}", i, msg),
                        Err(mpsc::TryRecvError::Empty) => continue,
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
        Ok(())
    }

    fn spawn_worker(&self, addr: String) -> super::Result<mpsc::Receiver<()>> {
        let cfg = self.config.clone();
        let (tx, rx) = mpsc::sync_channel(1);
        let worker = T::new(&mut self.context.write().unwrap(), cfg);
        thread::spawn(move || worker.start(addr, tx));
        Ok(rx)
    }
}

pub trait Supervisable: Sized + Send {
    type Config: Send + Sync;
    type Error: Send + From<zmq::Error>;

    fn new(context: &mut zmq::Context, config: Arc<RwLock<Self::Config>>) -> Self;

    fn init(&mut self) -> result::Result<(), Self::Error> {
        Ok(())
    }

    fn on_message(&mut self, envelope: &mut Envelope) -> result::Result<(), Self::Error>;

    fn socket(&mut self) -> &mut zmq::Socket;

    #[allow(unused_assignments)]
    fn start(mut self,
             be_addr: String,
             rz: mpsc::SyncSender<()>)
             -> result::Result<(), Self::Error> {
        try!(self.init());
        try!(self.socket().connect(&be_addr));
        rz.send(()).unwrap();
        let mut raw = zmq::Message::new().unwrap();
        let mut envelope = Envelope::default();
        'recv: loop {
            'hops: loop {
                let hop = try!(self.socket().recv_msg(0));
                if hop.len() == 0 {
                    break;
                }
                if envelope.add_hop(hop).is_err() {
                    warn!("drop message, too many hops");
                    envelope.reset();
                    break 'recv;
                }
            }
            try!(self.socket().recv(&mut raw, 0));
            match parse_from_bytes(&raw) {
                Ok(msg) => {
                    envelope.msg = msg;
                    try!(self.on_message(&mut envelope));
                }
                Err(e) => warn!("erorr parsing message, err={:?}", e),
            }
            envelope.reset();
        }
        Ok(())
    }
}
