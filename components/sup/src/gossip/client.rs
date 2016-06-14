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

//! The Gossip Client.
//!
//! This module takes a `UtpSocket`, and lets you send and receive messages with it. Messages are
//! encoded with json.
//!

use std::net::ToSocketAddrs;
use std::str;

use common::wire_message::WireMessage;
use hcore::crypto::SymKey;
use rustc_serialize::json;
use utp::UtpSocket;

use error::Result;
use gossip::rumor::{Protocol, Peer, RumorList};

pub const BUFFER_SIZE: usize = 10000;

/// A Gossip Client.
pub struct Client<'a> {
    pub socket: UtpSocket,
    ring_key: Option<&'a SymKey>,
}

impl<'a> Client<'a> {
    /// Create a new client from anything that can become a `SocketAddr`.
    ///
    /// # Errors
    ///
    /// * If we cannot connect the UTP socket
    pub fn new<A: ToSocketAddrs>(dst: A, ring_key: Option<&'a SymKey>) -> Result<Client> {
        let socket = try!(UtpSocket::connect(dst));
        Ok(Client {
            socket: socket,
            ring_key: ring_key,
        })
    }

    /// Create a new client from a `UtpSocket`
    pub fn from_socket(socket: UtpSocket, ring_key: Option<&'a SymKey>) -> Client {
        Client {
            socket: socket,
            ring_key: ring_key,
        }
    }

    /// Send a ping.
    ///
    /// # Errors
    ///
    /// * If we cannot send a ping
    pub fn ping(&mut self, my_peer: Peer, rumors_for_remote: RumorList) -> Result<()> {
        try!(self.send_message(Protocol::Ping(my_peer, rumors_for_remote)));
        Ok(())
    }

    /// Send a pingreq.
    ///
    /// # Errors
    ///
    /// * If we cannot send a pingreq
    pub fn pingreq(&mut self, through_peer: Peer, rumors_for_remote: RumorList) -> Result<()> {
        try!(self.send_message(Protocol::PingReq(through_peer, rumors_for_remote)));
        Ok(())
    }

    /// Send a Ack.
    ///
    /// # Errors
    ///
    /// * If we cannot send a Ack
    pub fn ack(&mut self, my_peer: Peer, rumors_for_remote: RumorList) -> Result<()> {
        try!(self.send_message(Protocol::Ack(my_peer, rumors_for_remote)));
        Ok(())
    }

    pub fn inject(&mut self, rumors_for_remote: RumorList) -> Result<()> {
        try!(self.send_message(Protocol::Inject(rumors_for_remote)));
        Ok(())
    }

    /// Receives a message.
    ///
    /// # Errors
    ///
    /// * We cannot receive the data from the socket
    /// * We cannot decode the data into a `gossip::message::Protocol`
    pub fn recv_message(&mut self) -> Result<Protocol> {
        let mut buf = [0u8; BUFFER_SIZE];
        let mut json_str = String::new();
        let mut keep_reading_buffer = true;

        while keep_reading_buffer {
            let (amt, _src) = try!(self.socket.recv_from(&mut buf));
            match amt {
                0 => keep_reading_buffer = false,
                amt => {
                    let partial_str = try!(str::from_utf8(&buf[..amt]));
                    json_str.push_str(partial_str);
                }
            }
        }

        debug!("Received protocol ({:?}): {}",
               self.socket.peer_addr(),
               json_str);

        let wire_msg: WireMessage = try!(json::decode(&json_str));
        Ok(try!(wire_msg.msg(self.ring_key)))
    }

    /// Send a message.
    ///
    /// # Errors
    ///
    /// * We cannot encode the `Message`
    /// * We fail to send the encoded buffer to the remote
    pub fn send_message(&mut self, msg: Protocol) -> Result<()> {
        let encoded = {
            let wire_msg = match self.ring_key.as_ref() {
                Some(key) => try!(WireMessage::encrypted(&msg, &key)),
                None => try!(WireMessage::plain(&msg)),
            };
            try!(json::encode(&wire_msg))
        };
        debug!("Encoded message {:#?}", encoded);
        try!(self.socket.send_to(encoded.as_bytes()));
        debug!("Sent protocol: {:?}", msg);
        Ok(())
    }
}
