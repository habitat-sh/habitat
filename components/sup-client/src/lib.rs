// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Client for connecting and communicating with a server listener which speaks SrvProtocol.
//!
//! Currently all functions will block and wait for a response from the remote. This is likely to
//! change to a futures based implementation as our needs increase.
//!
//! # RPC Call Example
//!
//! ```ignore
//! let conn = SrvClient::connect(&listen_addr, secret_key).wait()?;
//! let msg = protocols::ctl::ServiceGetDefaultCfg::new();
//! conn.call(msg).for_each(|reply| {
//!     match reply.message_id() {
//!         "ServiceCfg" => {
//!             let m = reply.parse::<protocols::types::ServiceCfg>().unwrap();
//!             println!("{}", m.get_default());
//!         }
//!         "NetErr" => {
//!             let m = reply.parse::<protocols::net::NetErr>().unwrap();
//!             println!("{}", m);
//!         }
//!         _ => (),
//!     }
//!     Ok(())
//! })
//! ```

#[macro_use]
extern crate futures;
extern crate habitat_sup_protocol as protocol;
#[macro_use]
extern crate log;
extern crate prost;
extern crate tokio;
extern crate tokio_codec;

use std::error;
use std::fmt;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;

use futures::prelude::*;
use futures::sink;
use protocol::codec::*;
use protocol::net::NetErr;
use tokio::net::TcpStream;
use tokio_codec::Framed;

pub type SrvSend = sink::Send<SrvStream>;

/// Error types returned by a [`SrvClient`].
#[derive(Debug)]
pub enum SrvClientError {
    /// The remote server unexpectedly closed the connection.
    ConnectionClosed,
    /// Unable to locate a secret key on disk.
    CtlSecretNotFound(PathBuf),
    /// Decoding a message from the remote failed.
    Decode(prost::DecodeError),
    /// An Os level IO error occurred.
    Io(io::Error),
    /// An RPC call to the remote was received but failed.
    NetErr(NetErr),
}

impl error::Error for SrvClientError {
    fn description(&self) -> &str {
        match *self {
            SrvClientError::ConnectionClosed => "Connection closed",
            SrvClientError::CtlSecretNotFound(_) => "Ctl secret key not found",
            SrvClientError::Decode(ref err) => err.description(),
            SrvClientError::Io(ref err) => err.description(),
            SrvClientError::NetErr(ref err) => err.description(),
        }
    }
}

impl fmt::Display for SrvClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match *self {
            SrvClientError::ConnectionClosed => format!(
                "Connection closed.\n\n\
                The remote Supervisor unexpectedly closed the connection. If this occurs\n\
                consistently it may be a symptom of a protocol mismatch. In that case\n\
                the client likely has new features, requiring an upgrade of the Supervisor\n\
                in order for it to respond appropriately to this request type.",
            ),
            SrvClientError::CtlSecretNotFound(ref path) => format!(
                "No Supervisor CtlGateway secret set in `cli.toml` or found at {}. Run \
                 `hab setup` or run the Supervisor for the first time before attempting to \
                 command the Supervisor.",
                path.display()
            ),
            SrvClientError::Decode(ref err) => format!("{}", err),
            SrvClientError::Io(ref err) => format!(
                "Unable to contact the Supervisor.\n\n\
                If the Supervisor you are contacting is local, this probably means it is not running. You can run a Supervisor in the foreground with:\n\n\
                   hab sup run\n\n\
                Or try restarting the Supervisor through your operating system's init process or Windows service.\n\n\
                Original error is:\n\n\
                   {}",
                err
            ),
            SrvClientError::NetErr(ref err) => format!("{}", err),
        };
        write!(f, "{}", content)
    }
}

impl From<NetErr> for SrvClientError {
    fn from(err: NetErr) -> Self {
        SrvClientError::NetErr(err)
    }
}

impl From<io::Error> for SrvClientError {
    fn from(err: io::Error) -> Self {
        SrvClientError::Io(err)
    }
}

impl From<prost::DecodeError> for SrvClientError {
    fn from(err: prost::DecodeError) -> Self {
        SrvClientError::Decode(err)
    }
}

/// Client for connecting and communicating with a server listener which speaks SrvProtocol.
///
/// See module doc for usage.
pub struct SrvClient {
    /// Sending and receiving framed socket pair for connecting and communicating to a remote
    /// server.
    socket: SrvStream,
    /// Transaction ID counter.
    ///
    /// Useful for determining the next transaction ID in sequence embed in
    /// the next transactional message sent.
    current_txn: SrvTxn,
}

impl SrvClient {
    /// Connect to the given remote server and authenticate with the given secret_key.
    pub fn connect<T>(
        addr: &SocketAddr,
        secret_key: T,
    ) -> Box<Future<Item = SrvClient, Error = SrvClientError> + 'static>
    where
        T: ToString,
    {
        let secret_key = secret_key.to_string();
        let conn = TcpStream::connect(addr)
            .map_err(SrvClientError::from)
            .and_then(move |socket| {
                let client = Self::new(socket, None);
                let mut request = protocol::ctl::Handshake::default();
                request.secret_key = Some(secret_key);
                client
                    .call(request)
                    .into_future()
                    .map_err(|(err, _)| err)
                    .and_then(move |(m, io)| {
                        m.map_or_else(
                            || Err(SrvClientError::ConnectionClosed),
                            move |m| {
                                m.try_ok()
                                    .map_err(SrvClientError::from)
                                    .and_then(|()| Ok(io.into_inner()))
                            },
                        )
                    })
            });
        Box::new(conn)
    }

    pub fn read_secret_key() -> Result<String, SrvClientError> {
        let mut buf = String::new();
        protocol::read_secret_key(protocol::sup_root(None::<String>), &mut buf)
            .map_err(SrvClientError::from)?;
        Ok(buf)
    }

    fn new(socket: TcpStream, current_txn: Option<SrvTxn>) -> Self {
        SrvClient {
            socket: Framed::new(socket, SrvCodec::new()),
            current_txn: current_txn.unwrap_or_default(),
        }
    }

    /// Send a transactional request to the connected server. The returned `SrvReply` is a Stream
    /// containing one or more `SrvMessage` responses for the given request.
    pub fn call<T>(mut self, request: T) -> SrvReply
    where
        T: Into<SrvMessage> + fmt::Debug,
    {
        self.current_txn.increment();
        let mut msg: SrvMessage = request.into();
        msg.set_transaction(self.current_txn);
        trace!("Sending SrvMessage -> {:?}", msg);
        SrvReply::new(self.socket.send(msg), self.current_txn)
    }

    /// Send a non-transactional request to the connected server.
    pub fn cast<T>(self, request: T) -> SrvSend
    where
        T: Into<SrvMessage> + fmt::Debug,
    {
        let message: SrvMessage = request.into();
        self.socket.send(message)
    }
}

/// A `Future` that will resolve into a stream of one or more `SrvMessage` replies.
#[must_use = "futures do nothing unless polled"]
pub struct SrvReply {
    io: sink::Send<SrvStream>,
    state: SrvReplyState,
    txn_id: SrvTxn,
}

impl SrvReply {
    fn new(io: sink::Send<SrvStream>, txn_id: SrvTxn) -> Self {
        SrvReply {
            io: io,
            state: SrvReplyState::Sending,
            txn_id: txn_id,
        }
    }

    /// Consume the `SrvReply` and return the contained `SrvClient`.
    pub fn into_inner(self) -> SrvClient {
        match self.state {
            SrvReplyState::Receiving(io, true) => {
                SrvClient::new(io.into_inner(), Some(self.txn_id))
            }
            _ => panic!("into_inner called before complete"),
        }
    }
}

impl Stream for SrvReply {
    type Item = SrvMessage;
    type Error = SrvClientError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.state {
                SrvReplyState::Sending => {
                    let io = try_ready!(self.io.poll());
                    self.state = SrvReplyState::Receiving(io, false);
                    continue;
                }
                SrvReplyState::Receiving(_, true) => return Ok(Async::Ready(None)),
                SrvReplyState::Receiving(ref mut io, ref mut complete) => {
                    match try_ready!(io.poll()) {
                        Some(msg) => {
                            *complete = msg.is_complete();
                            return Ok(Async::Ready(Some(msg)));
                        }
                        None => return Err(SrvClientError::ConnectionClosed),
                    }
                }
            }
        }
    }
}

enum SrvReplyState {
    /// Request is sending.
    Sending,
    /// Request is sent and awaiting message(s). Receiving is complete when the bool at `self.1`
    /// is true.
    Receiving(SrvStream, bool),
}
