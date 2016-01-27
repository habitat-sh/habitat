// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! The Gossip Client.
//!
//! This module takes a `UtpSocket`, and lets you send and receive messages with it. Messages are
//! encoded with msgpack.

use msgpack::{Encoder, Decoder};
use rustc_serialize::{Encodable, Decodable};
use utp::UtpSocket;

use std::net::ToSocketAddrs;

use error::BldrResult;
use gossip::message::{BUFFER_SIZE, Message};

//static LOGKEY: &'static str = "GC";

/// A Gossip Client.
pub struct Client {
    socket: UtpSocket,
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
    /// * If we cannot receive a pong
    pub fn ping(&mut self) -> BldrResult<()> {
        try!(self.send_message(Message::Ping));
        let msg = try!(self.recv_message());
        match msg {
            Message::Pong => debug!("Gossip is alive - Ping successful"),
            _ => unreachable!(),
        }
        Ok(())
    }

    /// Send a pong.
    ///
    /// # Errors
    ///
    /// * If we cannot send a pong
    pub fn pong(&mut self) -> BldrResult<()> {
        try!(self.send_message(Message::Pong));
        Ok(())
    }

    /// Receives a message.
    ///
    /// # Errors
    ///
    /// * We cannot receive the data from the socket
    /// * We cannot decode the data into a `gossip::message::Message`
    pub fn recv_message(&mut self) -> BldrResult<Message> {
        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        let (amt, src) = try!(self.socket.recv_from(&mut buf));

        let mut decoder = Decoder::new(&buf[0..amt]);
        let msg: Message = try!(Decodable::decode(&mut decoder));
        debug!("Received message ({:?}): {:?}", src, msg);
        Ok(msg)
    }

    /// Send a message.
    ///
    /// # Errors
    ///
    /// * We cannot encode the `Message`
    /// * We fail to send the encoded buffer to the remote
    pub fn send_message(&mut self, msg: Message) -> BldrResult<()> {
        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        try!(msg.encode(&mut Encoder::new(&mut &mut buf)));
        try!(self.socket.send_to(&buf[..]));
        debug!("Sent message: {:?}", msg);
        Ok(())
    }
}
