//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! The Gossip Server.
//!
//! This module listens for gossip requests, and handles routing messages.

use std::net::SocketAddr;
use std::thread;
use std::net;

use msgpack::Decoder;
use rustc_serialize::Decodable;
use utp::{UtpListener, UtpSocket};
use wonder::actor;
use wonder::actor::{GenServer, ActorSender, HandleResult, InitResult, StopReason};

use gossip::client::Client;
use gossip::message::{BUFFER_SIZE, Message};
use error::{BldrResult, BldrError};

static LOGKEY: &'static str = "GS";

/// A gossip server
pub struct Server {
    binding: String,
}

impl Server {
    /// Create a new Server; takes a string, which will be passed directly to `UtpListener`; it
    /// needs to deref to a SocketAddr.
    pub fn new(binding: String) -> Server {
        Server { binding: binding }
    }

    /// Listens for incoming UTP requests, and spawns a thread to handle each.
    ///
    /// # Errors
    ///
    /// * If we can't bind the address the server was started with
    pub fn listen(&mut self) -> BldrResult<()> {
        let bind: &str = self.binding.as_ref();
        let listener = try!(UtpListener::bind(bind));
        for connection in listener.incoming() {
            match connection {
                Ok((socket, src)) => {
                    debug!("Inbound connection from {:?}", src);
                    thread::spawn(move || receive(socket, src));
                }
                _ => {}
            }
        }
        Ok(())
    }
}


/// The internal receive function. It turns the raw socket into a `gossip::Client`, then blocks on
/// receiving a message.
///
/// # Errors
///
/// * We fail to receive a message
/// * We fail to decode the message into a gossip::Message
/// * We fail to transmit a response (depending on the message)
fn receive(mut socket: UtpSocket, src: net::SocketAddr) -> BldrResult<Message> {
    let mut client = Client::from_socket(socket);
    let msg = try!(client.recv_message());

    match msg {
        Message::Ping => {
            try!(client.pong());
        }
        Message::Pong => {
            debug!("Pong from {:?}", src);
        }
    }
    Ok(msg)
}

/// Messages for the ServerActor
#[derive(Debug)]
pub enum ServerActorMessage {
    Ok,
    Stop,
}

/// A simple ServerActor
#[derive(Debug)]
pub struct ServerActor;

/// The state of our ServerActor
#[derive(Debug)]
pub struct ServerState {
    /// A string that becomes a valid listener description.
    pub listen: String,
}

impl GenServer for ServerActor {
    type T = ServerActorMessage;
    type S = ServerState;
    type E = BldrError;

    /// Set up the underlying server
    fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
        let mut server = Server::new(state.listen.clone());
        thread::spawn(move || server.listen());
        Ok(None)
    }

    /// Respond to messages, after checking for new data from etcd.
    fn handle_call(&self,
                   message: Self::T,
                   _caller: &ActorSender<Self::T>,
                   _me: &ActorSender<Self::T>,
                   state: &mut Self::S)
                   -> HandleResult<Self::T> {
        match message {
            ServerActorMessage::Stop => {
                HandleResult::Stop(StopReason::Normal, Some(ServerActorMessage::Ok))
            }
            ServerActorMessage::Ok =>
                HandleResult::Stop(StopReason::Fatal(format!("You don't send me Ok! I send YOU \
                                                              Ok!")),
                                   Some(ServerActorMessage::Ok)),
        }
    }
}
