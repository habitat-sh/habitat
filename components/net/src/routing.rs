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

//! Contains types and functions for sending and receiving messages to and from a message broker
//! connected to one or more `RouteSrv`. All messages are routed through a `RouteSrv` and forwarded
//! to the appropriate receiver of a message.

use std::net;
use std::result;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use fnv::FnvHasher;
use protobuf::{self, parse_from_bytes, Message};
use protocol::{self, Routable, RouteKey};
use protocol::net::{ErrCode, NetError};
use zmq;

use config::ToAddrString;
use error::Result;
use server::ZMQ_CONTEXT;

pub type RouteResult<T> = result::Result<T, NetError>;

/// Time to wait before timing out a message receive for a `BrokerConn`.
pub const RECV_TIMEOUT_MS: i32 = 5_000;
/// Time to wait before timing out a message send for a `Broker` to a router.
pub const SEND_TIMEOUT_MS: i32 = 5_000;
// ZeroMQ address for the application's Broker's queue.
const ROUTE_INPROC_ADDR: &'static str = "inproc://route-broker";

/// Client connection for sending and receiving messages to and from the service cluster through
/// a running `Broker`.
pub struct BrokerConn {
    sock: zmq::Socket,
    hasher: FnvHasher,
}

impl BrokerConn {
    /// Create a new `BrokerConn`
    ///
    /// # Errors
    ///
    /// * Socket could not be created
    /// * Socket could not be configured
    pub fn new() -> Result<Self> {
        let socket = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::REQ));
        try!(socket.set_rcvtimeo(RECV_TIMEOUT_MS));
        try!(socket.set_sndtimeo(SEND_TIMEOUT_MS));
        try!(socket.set_immediate(true));
        Ok(BrokerConn {
            sock: socket,
            hasher: FnvHasher::default(),
        })
    }

    /// Connect to a running `Broker` with the given ZeroMQ address.
    ///
    /// # Errors
    ///
    /// * A connection cannot be established to a socket at the given address
    pub fn connect(&mut self, addr: &str) -> Result<()> {
        try!(self.sock.connect(addr));
        Ok(())
    }

    /// Routes a message to the connected broker, through a router, and to appropriate service,
    /// waits for a response, and then parses and returns the value of the response.
    ///
    /// # Errors
    ///
    /// * One or more message frames cannot be sent to the Broker's queue
    ///
    /// # Panics
    ///
    /// * Could not serialize message
    pub fn route<M: Routable, R: protobuf::MessageStatic>(&mut self, msg: &M) -> RouteResult<R> {
        if self.route_async(msg).is_err() {
            return Err(protocol::net::err(ErrCode::ZMQ, "net:route:1"));
        }
        match self.recv() {
            Ok(rep) => {
                if rep.get_message_id() == "NetError" {
                    let err = parse_from_bytes(rep.get_body()).unwrap();
                    return Err(err);
                }
                match parse_from_bytes::<R>(rep.get_body()) {
                    Ok(entity) => Ok(entity),
                    Err(_) => {
                        unreachable!("net:route, unexpected response, message_id={}",
                                     rep.get_message_id())
                    }
                }
            }
            Err(_) => Err(protocol::net::err(ErrCode::ZMQ, "net:route:3")),
        }
    }

    /// Asynchronously routes a message to the connected broker, through a router, and to
    /// appropriate service.
    ///
    /// # Errors
    ///
    /// * One or more message frames cannot be sent to the Broker's queue
    ///
    /// # Panics
    ///
    /// * Could not serialize message
    pub fn route_async<M: Routable>(&mut self, msg: &M) -> Result<()> {
        let route_hash = msg.route_key().map(|key| key.hash(&mut self.hasher));
        let req = protocol::Message::new(msg).routing(route_hash).build();
        let bytes = req.write_to_bytes().unwrap();
        try!(self.sock.send_str("RQ", zmq::SNDMORE));
        try!(self.sock.send(&bytes, 0));
        Ok(())
    }

    /// Receives a message from the connected broker. This function will block the calling thread
    /// until a message is received or a timeout occurs.
    ///
    /// # Errors
    ///
    /// * `Broker` Queue became unavailable
    /// * Message was not received within the timeout
    /// * Received an unparsable message
    pub fn recv(&mut self) -> Result<protocol::net::Msg> {
        let envelope = try!(self.sock.recv_msg(0));
        let msg: protocol::net::Msg = try!(parse_from_bytes(&envelope));
        Ok(msg)
    }
}

/// A messaging Broker for proxying messages from clients to one or more `RouteSrv` and vice versa.
pub struct Broker {
    client_sock: zmq::Socket,
    router_sock: zmq::Socket,
}

impl Broker {
    /// Create a new `Broker`
    ///
    /// # Errors
    ///
    /// * A socket cannot be created within the given `zmq::Context`
    /// * A socket cannot be configured
    ///
    /// # Panics
    ///
    /// * Could not read `zmq::Context` due to deadlock or poisoning
    fn new(net_ident: String) -> Result<Self> {
        let fe = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::ROUTER));
        let be = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        try!(fe.set_identity(net_ident.as_bytes()));
        try!(be.set_rcvtimeo(RECV_TIMEOUT_MS));
        try!(be.set_sndtimeo(SEND_TIMEOUT_MS));
        try!(be.set_immediate(true));
        Ok(Broker {
            client_sock: fe,
            router_sock: be,
        })
    }

    /// Helper function for creating a new `BrokerConn` and connecting to the application's `Broker`
    ///
    /// # Errors
    ///
    /// * Could not connect to `Broker`
    /// * Could not create socket
    ///
    /// # Panics
    ///
    /// * Could not read `zmq::Context` due to deadlock or poisoning
    pub fn connect() -> Result<BrokerConn> {
        let mut conn = try!(BrokerConn::new());
        try!(conn.connect(ROUTE_INPROC_ADDR));
        Ok(conn)
    }

    /// Create a new `Broker` and run it in a separate thread. This function will block the calling
    /// thread until the new broker has successfully started.
    ///
    /// # Panics
    ///
    /// * Broker crashed during startup
    pub fn run(net_ident: String, routers: &Vec<net::SocketAddrV4>) -> JoinHandle<()> {
        let (tx, rx) = mpsc::sync_channel(1);
        let addrs = routers.iter().map(|a| a.to_addr_string()).collect();
        let handle = thread::Builder::new()
            .name("router-broker".to_string())
            .spawn(move || {
                let mut broker = Self::new(net_ident).unwrap();
                broker.start(tx, addrs).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => handle,
            Err(e) => panic!("router-broker thread startup error, err={}", e),
        }
    }

    // Main loop for `Broker`.
    //
    // Binds front-end socket to ZeroMQ inproc address and connects to all routers. Sends a message
    // back to the caller over the given rendezvous channel to signal when ready.
    fn start(&mut self, rz: mpsc::SyncSender<()>, routers: Vec<String>) -> Result<()> {
        try!(self.client_sock.bind(ROUTE_INPROC_ADDR));
        for addr in routers {
            try!(self.router_sock.connect(&addr));
        }
        rz.send(()).unwrap();
        try!(zmq::proxy(&mut self.client_sock, &mut self.router_sock));
        Ok(())
    }
}
