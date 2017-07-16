// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

pub use hab_net::conn::{route, route_reply, wait_recv, ConnErr, ConnEvent};
use protobuf;
use protocol::Message;
use zmq;

use config::Config;

pub struct SrvConn {
    socket: zmq::Socket,
    recv_buf: zmq::Message,
}

impl SrvConn {
    pub fn new(context: &mut zmq::Context, cfg: &Config) -> Result<Self, ConnErr> {
        let socket = context.socket(zmq::ROUTER)?;
        socket.set_router_mandatory(true)?;
        socket.set_probe_router(true)?;
        socket.set_identity(cfg.addr().as_bytes())?;
        Ok(SrvConn {
            socket: socket,
            recv_buf: zmq::Message::new()?,
        })
    }

    pub fn bind<T>(&self, addr: T) -> Result<(), ConnErr>
    where
        T: AsRef<str>,
    {
        self.socket.bind(addr.as_ref()).map_err(ConnErr::Socket)
    }

    pub fn forward(&self, message: &mut Message, destination: Vec<u8>) -> Result<(), ConnErr> {
        if message.route_info().is_none() {
            return Err(ConnErr::NoRouteInfo);
        }
        message.identities.insert(0, destination);
        route(&self.socket, message)
    }

    pub fn forward_reply(&self, message: &mut Message) -> Result<(), ConnErr> {
        if message.route_info().is_none() {
            return Err(ConnErr::NoRouteInfo);
        }
        if message.txn().is_none() {
            return Err(ConnErr::NoTxn);
        }
        if !message.completed_txn() {
            return Err(ConnErr::TxnNotComplete);
        }
        route(&self.socket, message)
    }

    pub fn route_reply<T>(&self, message: &mut Message, reply: &T) -> Result<(), ConnErr>
    where
        T: protobuf::Message,
    {
        route_reply(&self.socket, message, reply)
    }

    pub fn wait_recv(&mut self, message: &mut Message, timeout: i64) -> Result<ConnEvent, ConnErr> {
        wait_recv(&self.socket, message, &mut self.recv_buf, timeout)
    }
}
