// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! The Gossip Client.
//!
//! This module takes a `UtpSocket`, and lets you send and receive messages with it. Messages are
//! encoded with json.
//!

use rustc_serialize::json;
use utp::UtpSocket;

use std::net::ToSocketAddrs;
use std::str;

use error::BldrResult;
use gossip::rumor::{Protocol, Peer, RumorList};

pub const BUFFER_SIZE: usize = 10000;

/// A Gossip Client.
pub struct Client {
    pub socket: UtpSocket,
}

impl Client {
    /// Create a new client from anything that can become a `SocketAddr`.
    ///
    /// # Errors
    ///
    /// * If we cannot connect the UTP socket
    pub fn new<A: ToSocketAddrs>(dst: A) -> BldrResult<Client> {
        let socket = try!(UtpSocket::connect(dst));
        Ok(Client { socket: socket })
    }

    /// Create a new client from a `UtpSocket`
    pub fn from_socket(socket: UtpSocket) -> Client {
        Client { socket: socket }
    }

    /// Send a ping.
    ///
    /// # Errors
    ///
    /// * If we cannot send a ping
    pub fn ping(&mut self, my_peer: Peer, rumors_for_remote: RumorList) -> BldrResult<()> {
        try!(self.send_message(Protocol::Ping(my_peer, rumors_for_remote)));
        Ok(())
    }

    /// Send a pingreq.
    ///
    /// # Errors
    ///
    /// * If we cannot send a pingreq
    pub fn pingreq(&mut self, through_peer: Peer, rumors_for_remote: RumorList) -> BldrResult<()> {
        try!(self.send_message(Protocol::PingReq(through_peer, rumors_for_remote)));
        Ok(())
    }

    /// Send a Ack.
    ///
    /// # Errors
    ///
    /// * If we cannot send a Ack
    pub fn ack(&mut self, my_peer: Peer, rumors_for_remote: RumorList) -> BldrResult<()> {
        try!(self.send_message(Protocol::Ack(my_peer, rumors_for_remote)));
        Ok(())
    }

    /// Receives a message.
    ///
    /// # Errors
    ///
    /// * We cannot receive the data from the socket
    /// * We cannot decode the data into a `gossip::message::Protocol`
    pub fn recv_message(&mut self) -> BldrResult<Protocol> {
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

        let msg: Protocol = try!(json::decode(&json_str));
        Ok(msg)
    }

    /// Send a message.
    ///
    /// # Errors
    ///
    /// * We cannot encode the `Message`
    /// * We fail to send the encoded buffer to the remote
    pub fn send_message(&mut self, msg: Protocol) -> BldrResult<()> {
        let encoded = try!(json::encode(&msg));
        debug!("Encoded message {:#?}", encoded);
        try!(self.socket.send_to(encoded.as_bytes()));
        debug!("Sent protocol: {:?}", msg);
        Ok(())
    }
}
