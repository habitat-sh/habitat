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

//! Listening server for receiving client connections which speak SrvProtocol.
//!
//! The server runs in a separate thread and dispatches operational commands to the main thread
//! over an unbounded mpsc channel, `MgrSender`, to `MgrReceiver`. These commands are wrapped in
//! a [`ctl_gateway.CtlRequest`] if they are transactional.
//!
//! Replies to transactional messages are sent back to the CtlGateway thread over an unbounded
//! mpsc channel, [`CtlSender`], to [`CtlReceiver`]. A new mpsc pair is created for each
//! transactional request where the sending half is given to a [`ctl_gateway.CtlRequest`].

use std::cell::RefCell;
use std::error;
use std::fmt;
use std::io;
use std::net::SocketAddr;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::hcore::crypto;
use crate::protocol;
use crate::protocol::codec::*;
use crate::protocol::net::{self, ErrCode, NetErr, NetResult};
use futures::future::{self, Either};
use futures::prelude::*;
use futures::sync::mpsc;
use prost;
use tokio::net::TcpListener;
use tokio_codec::Framed;
use tokio_core::reactor;

use super::{CtlRequest, REQ_TIMEOUT};
use crate::manager::{commands, ManagerState};

/// Sending half of an mpsc unbounded channel used for sending replies for a transactional message
/// from the main thread back to the CtlGateway. This half is stored in a
/// [`ctl_gateway.CtlRequest`] in the main thread.
pub type CtlSender = mpsc::UnboundedSender<SrvMessage>;
/// Receiving half of an mpsc unbounded channel used for sending replies for a transactional
/// message from the main thread back to the CtlGateway. This half is stored in the CtlGateway on
/// it's thread.
pub type CtlReceiver = mpsc::UnboundedReceiver<SrvMessage>;
/// Sender from the CtlGateway to the Manager to dispatch control commands for clients.
pub type MgrSender = mpsc::UnboundedSender<CtlCommand>;
/// Receiver on the Manager for the sender on the CtlGateway to receive control commands.
pub type MgrReceiver = mpsc::UnboundedReceiver<CtlCommand>;

#[derive(Debug)]
pub enum HandlerError {
    Decode(prost::DecodeError),
    Io(io::Error),
    NetErr(NetErr),
    SendError(mpsc::SendError<CtlCommand>),
}

impl error::Error for HandlerError {
    fn description(&self) -> &str {
        match *self {
            HandlerError::Decode(ref err) => err.description(),
            HandlerError::Io(ref err) => err.description(),
            HandlerError::NetErr(ref err) => err.description(),
            HandlerError::SendError(ref err) => err.description(),
        }
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match *self {
            HandlerError::Decode(ref err) => format!("{}", err),
            HandlerError::Io(ref err) => format!("{}", err),
            HandlerError::NetErr(ref err) => format!("{}", err),
            HandlerError::SendError(ref err) => format!("{}", err),
        };
        write!(f, "{}", content)
    }
}

impl From<NetErr> for HandlerError {
    fn from(err: NetErr) -> Self {
        HandlerError::NetErr(err)
    }
}

impl From<io::Error> for HandlerError {
    fn from(err: io::Error) -> Self {
        HandlerError::Io(err)
    }
}

impl From<prost::DecodeError> for HandlerError {
    fn from(err: prost::DecodeError) -> Self {
        HandlerError::Decode(err)
    }
}

impl From<mpsc::SendError<CtlCommand>> for HandlerError {
    fn from(err: mpsc::SendError<CtlCommand>) -> Self {
        HandlerError::SendError(err)
    }
}

/// A wrapper around a [`ctl_gateway.CtlRequest`] and a closure for the main thread to execute.
pub struct CtlCommand {
    pub req: CtlRequest,
    // JW: This needs to be an `FnOnce<Box>` and not an `Fn<Box>` but right now there is no support
    // for boxing an FnOnce in stable Rust. There is a new type called `FnBox` which exists only on
    // nightly right now which accomplishes this but it won't stabilize because the Rust core team
    // feels that they should just get `Box<FnOnce>` working. We'll need to clone the `CtlRequest`
    // argument passed to this closure until `FnOnce<Box>` stabilizes.
    //
    // https://github.com/rust-lang/rust/issues/28796
    fun: Box<dyn Fn(&ManagerState, &mut CtlRequest) -> NetResult<()> + Send>,
}

impl CtlCommand {
    /// Create a new CtlCommand from the given CtlSender, transaction, and closure to execute.
    pub fn new<F>(tx: Option<CtlSender>, txn: Option<SrvTxn>, fun: F) -> Self
    where
        F: Fn(&ManagerState, &mut CtlRequest) -> NetResult<()> + Send + 'static,
    {
        CtlCommand {
            fun: Box::new(fun),
            req: CtlRequest::new(tx, txn),
        }
    }

    /// Run the contained closure with the given [`manager.ManagerState`].
    pub fn run(&mut self, state: &ManagerState) -> NetResult<()> {
        (self.fun)(state, &mut self.req)
    }
}

/// Server's client representation. Each new connection will allocate a new Client.
struct Client {
    handle: reactor::Handle,
    state: Rc<RefCell<SrvState>>,
}

impl Client {
    /// Serve the client from the given framed socket stream.
    pub fn serve(self, socket: SrvStream) -> Box<dyn Future<Item = (), Error = HandlerError>> {
        let mgr_tx = self.state.borrow().mgr_tx.clone();
        Box::new(
            self.handshake(socket)
                .and_then(|socket| SrvHandler::new(socket, mgr_tx)),
        )
    }

    /// Initiate a handshake with the connected client before allowing future requests. A failed
    /// handshake will close the connection.
    fn handshake(
        &self,
        socket: SrvStream,
    ) -> Box<dyn Future<Item = SrvStream, Error = HandlerError>> {
        let secret_key = self.state.borrow().secret_key.to_string();
        let handshake = socket
            .into_future()
            .map_err(|(err, _)| HandlerError::from(err))
            .and_then(move |(m, io)| {
                m.map_or_else(
                    || {
                        Err(HandlerError::from(io::Error::from(
                            io::ErrorKind::UnexpectedEof,
                        )))
                    },
                    move |m| {
                        if m.message_id() != "Handshake" {
                            debug!("No handshake");
                            return Err(HandlerError::from(io::Error::from(
                                io::ErrorKind::ConnectionAborted,
                            )));
                        }
                        if !m.is_transaction() {
                            return Err(HandlerError::from(io::Error::from(
                                io::ErrorKind::ConnectionAborted,
                            )));
                        }
                        match m.parse::<protocol::ctl::Handshake>() {
                            Ok(decoded) => {
                                trace!("Received handshake, {:?}", decoded);
                                let decoded_key = decoded.secret_key.unwrap_or_default();
                                Ok((m, crypto::secure_eq(decoded_key, secret_key), io))
                            }
                            Err(err) => {
                                warn!("Handshake error, {:?}", err);
                                Err(HandlerError::from(io::Error::from(
                                    io::ErrorKind::ConnectionAborted,
                                )))
                            }
                        }
                    },
                )
            })
            .and_then(|(msg, success, socket)| {
                let mut reply = if success {
                    SrvMessage::from(net::ok())
                } else {
                    SrvMessage::from(net::err(ErrCode::Unauthorized, "secret key mismatch"))
                };
                reply.reply_for(msg.transaction().unwrap(), true);
                socket
                    .send(reply)
                    .map_err(HandlerError::from)
                    .and_then(move |io| Ok((io, success)))
            });
        Box::new(
            handshake
                .select2(self.timeout(REQ_TIMEOUT))
                .then(|res| match res {
                    Ok(Either::A(((io, true), _to))) => future::ok(io),
                    Ok(Either::A(((_, false), _to))) => future::err(HandlerError::from(
                        io::Error::new(io::ErrorKind::ConnectionAborted, "handshake failed"),
                    )),
                    Ok(Either::B((_to, _hs))) => future::err(HandlerError::from(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "client timed out",
                    ))),
                    Err(Either::A((err, _))) => future::err(HandlerError::from(err)),
                    Err(Either::B((err, _))) => future::err(HandlerError::from(err)),
                }),
        )
    }

    /// Generate a new timeout future with the given duration in milliseconds.
    fn timeout(&self, millis: u64) -> reactor::Timeout {
        reactor::Timeout::new(Duration::from_millis(millis), &self.handle)
            .expect("failed to generate timeout future")
    }
}

/// A `Future` that will resolve into a stream of one or more `SrvMessage` replies.
#[must_use = "futures do nothing unless polled"]
struct SrvHandler {
    io: SrvStream,
    state: SrvHandlerState,
    mgr_tx: MgrSender,
    rx: CtlReceiver,
    tx: CtlSender,
}

impl SrvHandler {
    fn new(io: SrvStream, mgr_tx: MgrSender) -> Self {
        let (tx, rx) = mpsc::unbounded();
        SrvHandler {
            io: io,
            state: SrvHandlerState::Receiving,
            mgr_tx: mgr_tx,
            rx: rx,
            tx: tx,
        }
    }
}

impl Future for SrvHandler {
    type Item = ();
    type Error = HandlerError;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        loop {
            match self.state {
                SrvHandlerState::Receiving => match try_ready!(self.io.poll()) {
                    Some(msg) => {
                        trace!("OnMessage, {}", msg.message_id());
                        let cmd = match msg.message_id() {
                            "SvcGetDefaultCfg" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcGetDefaultCfg>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| commands::service_cfg(state, req, m.clone()),
                                )
                            }
                            "SvcFilePut" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcFilePut>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| {
                                        commands::service_file_put(state, req, m.clone())
                                    },
                                )
                            }
                            "SvcSetCfg" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcSetCfg>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| {
                                        commands::service_cfg_set(state, req, m.clone())
                                    },
                                )
                            }
                            "SvcValidateCfg" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcValidateCfg>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| {
                                        commands::service_cfg_validate(state, req, m.clone())
                                    },
                                )
                            }
                            "SvcLoad" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcLoad>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| commands::service_load(state, req, m.clone()),
                                )
                            }
                            "SvcUnload" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcUnload>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| {
                                        commands::service_unload(state, req, m.clone())
                                    },
                                )
                            }
                            "SvcStart" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcStart>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| {
                                        commands::service_start(state, req, m.clone())
                                    },
                                )
                            }
                            "SvcStop" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcStop>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| commands::service_stop(state, req, m.clone()),
                                )
                            }
                            "SvcStatus" => {
                                let m = msg
                                    .parse::<protocol::ctl::SvcStatus>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| {
                                        commands::service_status(state, req, m.clone())
                                    },
                                )
                            }
                            "SupDepart" => {
                                let m = msg
                                    .parse::<protocol::ctl::SupDepart>()
                                    .map_err(HandlerError::from)?;
                                CtlCommand::new(
                                    Some(self.tx.clone()),
                                    msg.transaction(),
                                    move |state, req| {
                                        commands::supervisor_depart(state, req, m.clone())
                                    },
                                )
                            }
                            _ => {
                                warn!("Unhandled message, {}", msg.message_id());
                                break;
                            }
                        };
                        match self.mgr_tx.start_send(cmd) {
                            Ok(AsyncSink::Ready) => {
                                self.state = SrvHandlerState::Sending;
                                continue;
                            }
                            Ok(AsyncSink::NotReady(_)) => return Ok(Async::NotReady),
                            Err(err) => {
                                warn!("ManagerReceiver err, {:?}", err);
                                return Err(HandlerError::from(err));
                            }
                        }
                    }
                    None => break,
                },
                SrvHandlerState::Sending => match self.rx.poll() {
                    Ok(Async::Ready(Some(msg))) => {
                        trace!("MgrSender -> SrvHandler, {:?}", msg);
                        if msg.is_complete() {
                            self.state = SrvHandlerState::Sent;
                        }
                        try_nb!(self.io.start_send(msg));
                        try_ready!(self.io.poll_complete());
                        continue;
                    }
                    Ok(Async::Ready(None)) => self.state = SrvHandlerState::Sent,
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Err(()) => break,
                },
                SrvHandlerState::Sent => {
                    trace!("OnMessage complete");
                    break;
                }
            }
        }
        Ok(Async::Ready(()))
    }
}

enum SrvHandlerState {
    /// Handler is Receiving/Waiting for message from client.
    Receiving,
    /// Handler has sent a request to the Manager and is streaming replies back to the client
    /// socket.
    Sending,
    /// All messages have been sent to the client and the Handler is now flushing the connection.
    Sent,
}

struct SrvState {
    secret_key: String,
    mgr_tx: MgrSender,
}

/// Start a new thread which will run the CtlGateway server.
///
/// New connections will be authenticated using `secret_key`. Messages from the main thread
/// will be sent over the channel `mgr_tx`.
pub fn run(listen_addr: SocketAddr, secret_key: String, mgr_tx: MgrSender) {
    thread::Builder::new()
        .name("ctl-gateway".to_string())
        .spawn(move || {
            let mut core = reactor::Core::new().unwrap();
            let handle = core.handle();
            let listener = TcpListener::bind(&listen_addr).unwrap();
            let state = SrvState {
                secret_key: secret_key,
                mgr_tx: mgr_tx,
            };
            let state = Rc::new(RefCell::new(state));
            let clients = listener.incoming().map(|socket| {
                let addr = socket.peer_addr().unwrap();
                let io = Framed::new(socket, SrvCodec::new());
                (
                    Client {
                        handle: handle.clone(),
                        state: state.clone(),
                    }
                    .serve(io),
                    addr,
                )
            });
            let server = clients.for_each(|(client, addr)| {
                handle.spawn(client.then(move |res| {
                    debug!("DISCONNECTED from {:?} with result {:?}", addr, res);
                    future::ok(())
                }));
                Ok(())
            });
            core.run(server)
        })
        .expect("ctl-gateway thread start failure");
}
