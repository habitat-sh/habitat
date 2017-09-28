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

//! # Example Application
//!
//! ```rust,no_run
//! extern crate habitat_builder_protocol as protocol;
//! extern crate habitat_net;
//! #[macro_use]
//! extern crate lazy_static;
//! #[macro_use]
//! extern crate log;
//!
//! use std::process;
//! use habitat_net::app::prelude::*;
//! // Use the protocol that your server implements here. For an example, we'll use sessionsrv. You
//! // can create your own protocol in `components/builder-protocol`.
//! use protocol::sessionsrv::*;
//!
//! pub mod config {
//!     use habitat_net::app::config::*;
//!
//!     #[derive(Default)]
//!     pub struct SrvConfig {
//!         pub routers: Vec<RouterAddr>,
//!         pub shards: Vec<ShardId>,
//!         pub worker_threads: usize,
//!     }
//!
//!     impl AppCfg for SrvConfig {
//!         fn route_addrs(&self) -> &[RouterAddr] {
//!             self.routers.as_slice()
//!         }
//!
//!         fn shards(&self) -> Option<&[ShardId]> {
//!             Some(self.shards.as_slice())
//!         }
//!
//!         fn worker_count(&self) -> usize {
//!             self.worker_threads
//!         }
//!     }
//! }
//!
//! pub mod error {
//!     use std::error;
//!     use std::fmt;
//!
//!     use habitat_net;
//!     use protocol;
//!
//!     pub type SrvResult<T> = Result<T, SrvError>;
//!
//!     #[derive(Debug)]
//!     pub enum SrvError {
//!         ConnErr(habitat_net::conn::ConnErr),
//!         MomsSpaghetti,
//!         Protocol(protocol::ProtocolError),
//!     }
//!
//!     impl fmt::Display for SrvError {
//!         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!             match *self {
//!                 SrvError::ConnErr(ref e) => write!(f, "{}", e),
//!                 SrvError::MomsSpaghetti => write!(f, "knees weak, arms are heavy"),
//!                 SrvError::Protocol(ref e) => write!(f, "{}", e),
//!             }
//!         }
//!     }
//!
//!     impl error::Error for SrvError {
//!         fn description(&self) -> &str {
//!             match *self {
//!                 SrvError::ConnErr(ref err) => err.description(),
//!                 SrvError::MomsSpaghetti => "vomit on my sweater already",
//!                 SrvError::Protocol(ref err) => err.description(),
//!             }
//!         }
//!     }
//!
//!     impl From<habitat_net::conn::ConnErr> for SrvError {
//!         fn from(err: habitat_net::conn::ConnErr) -> Self {
//!             SrvError::ConnErr(err)
//!         }
//!     }
//!
//!     impl From<protocol::ProtocolError> for SrvError {
//!         fn from(err: protocol::ProtocolError) -> Self {
//!             SrvError::Protocol(err)
//!         }
//!     }
//! }
//!
//! mod handlers {
//!     use habitat_net::app::prelude::*;
//!     use protocol::sessionsrv::AccountGet;
//!
//!     use super::SrvState;
//!     use error::SrvResult;
//!
//!     pub fn account_get(
//!         req: &mut Message,
//!         conn: &mut RouteConn,
//!         state: &mut SrvState,
//!     ) -> SrvResult<()> {
//!         let msg = req.parse::<AccountGet>()?;
//!         conn.route_reply(req, &NetOk::new())?;
//!         Ok(())
//!     }
//! }
//!
//! use config::SrvConfig;
//! use error::{SrvError, SrvResult};
//!
//! lazy_static! {
//!     static ref DISPATCH_TABLE: DispatchTable<MySrv> = {
//!         let mut map = DispatchTable::new();
//!         // Register each protocol message and map it to a handler function. For an example, we
//!         // will use a sessionsrv protocol message for an example here
//!         map.register(AccountGet::descriptor_static(None), handlers::account_get);
//!         map
//!     };
//! }
//!
//! #[derive(Clone, Default)]
//! pub struct SrvState;
//! impl AppState for SrvState {
//!     type Config = SrvConfig;
//!     type Error = error::SrvError;
//!     type InitState = Self;
//!
//!     fn build(_config: &Self::Config, init_state: Self::InitState) -> SrvResult<Self> {
//!         Ok(init_state)
//!     }
//! }
//!
//! struct MySrv;
//! impl Dispatcher for MySrv {
//!     const APP_NAME: &'static str = "my-srv";
//!     // Define your protocol in `components/builder-protocol/protocols/net.proto`. For an
//!     // example, we will use the SessionSrv protocol
//!     const PROTOCOL: Protocol = Protocol::SessionSrv;
//!
//!     type Error = error::SrvError;
//!     type State = SrvState;
//!
//!     fn app_init(
//!         config: &<Self::State as AppState>::Config,
//!         router_pipe: Arc<String>,
//!     ) -> SrvResult<<Self::State as AppState>::InitState> {
//!         Ok(SrvState::default())
//!     }
//!
//!     fn dispatch_table() -> &'static DispatchTable<Self> {
//!         &DISPATCH_TABLE
//!     }
//! }
//!
//! fn main() {
//!     let config = config::SrvConfig::default();
//!     if let Err(err) = app_start::<MySrv>(config) {
//!         error!("{}", err);
//!         process::exit(1);
//!     }
//! }
//! ```

pub mod config;
pub mod error;
pub mod prelude;
mod dispatcher;

use std;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Arc;

use core::os::signals;
use protocol::{self, Protocol};
use protocol::routesrv;
use uuid::Uuid;
use zmq;

use self::config::AppCfg;
use self::error::{AppError, AppResult};
use self::dispatcher::{Dispatcher, DispatcherPool};
use conn::{self, ConnErr, ConnEvent};
use error::{ErrCode, NetError};
use socket::{self, DEFAULT_CONTEXT, ToAddrString};
use time;

/// Coordination signals for the Application's main thread.
enum RecvEvent {
    /// Signals which sockets have pending messages to be processed.
    ///
    /// The containing tuple consists of 3 elements marking which sockets have pending messages.
    ///     * `0` - Incoming message from Router
    ///     * `1` - Outgoing reply from Dispatcher
    ///     * `2` - Outgoing request from Dispatcher
    OnMessage((bool, bool, bool)),
    /// Signals that the server is shutting down.
    Shutdown,
    /// Signals that no message events were received in the allotted time.
    Timeout,
}

/// Apply to a struct containing worker state that will be passed as a mutable reference on each
/// call of `dispatch()` to an implementer of `Dispatcher`.
pub trait AppState: Send + Sized {
    type Config: AppCfg;
    type Error: std::error::Error;
    type InitState: Clone + Send;

    /// Callback to perform dispatcher initialization.
    ///
    /// The default implementation will take your initial state and convert it into the actual
    /// state of the worker. Override this function if you need to perform additional steps to
    /// initialize your worker state.
    fn build(&Self::Config, init_state: Self::InitState) -> Result<Self, Self::Error>;
}

struct Application<T>
where
    T: Dispatcher,
{
    /// Message buffer for server to RouterSrv Heartbeat.
    heartbeat: protocol::Message,
    /// Message buffer for reading complete protocol messages from Sockets.
    msg_buf: protocol::Message,
    /// Time in milliseconds when the main thread should send a heartbeat to all RouteSrvs by.
    next_heartbeat: i64,
    /// InProc Pipe containing outgoing replies from Dispatcher to Router.
    pipe_in: zmq::Socket,
    /// InProc Pipe containing outgoing requests from Dispatcher to Router.
    pipe_out: zmq::Socket,
    /// Internal message buffer used for proxying messages between Router and Dispatcher sockets.
    recv_buf: zmq::Message,
    /// Message buffer for server to RouteSrv Registration.
    registration: protocol::Message,
    /// Network Socket connecting to RouteSrv(s).
    router_sock: zmq::Socket,
    /// Set of RouteSrv's connections.
    routers: HashSet<Vec<u8>>,
    marker: PhantomData<T>,
}

impl<T> Application<T>
where
    T: Dispatcher,
{
    fn new(config: &<T::State as AppState>::Config) -> AppResult<Self, T::Error> {
        let router_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::ROUTER)?;
        router_sock.set_identity(socket::srv_ident().as_bytes())?;
        router_sock.set_probe_router(true)?;
        router_sock.set_immediate(true)?;
        router_sock.set_router_mandatory(true)?;
        let pipe_out = (**DEFAULT_CONTEXT).as_mut().socket(zmq::ROUTER).unwrap();
        pipe_out.set_immediate(true)?;
        let pipe_in = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
        let mut registration = routesrv::Registration::new();
        registration.set_protocol(T::PROTOCOL);
        if let Some(ref shards) = config.shards() {
            registration.set_shards(shards.to_vec());
        }
        Ok(Application {
            heartbeat: protocol::Message::build(&routesrv::Heartbeat::new())?,
            msg_buf: protocol::Message::default(),
            next_heartbeat: next_heartbeat(),
            pipe_out: pipe_out,
            pipe_in: pipe_in,
            recv_buf: zmq::Message::new()?,
            registration: protocol::Message::build(&registration)?,
            router_sock: router_sock,
            routers: HashSet::default(),
            marker: PhantomData,
        })
    }

    /// Forward a request originating in a Dispatcher to a randomly selected active RouteSrv.
    fn forward_request(&mut self) -> AppResult<(), T::Error> {
        {
            let addr = self.select_router()?;
            self.router_sock.send(addr, zmq::SNDMORE).map_err(
                ConnErr::Socket,
            )?;
        }
        proxy_message::<T>(
            &mut self.pipe_out,
            &mut self.router_sock,
            &mut self.recv_buf,
        )
    }

    /// Handle incoming server connect messages.
    fn handle_connect(&mut self) -> AppResult<(), T::Error> {
        debug!("handle-connect, {:?}", self.msg_buf.sender_str().unwrap());
        match conn::send_to(
            &self.router_sock,
            &self.registration,
            self.msg_buf.sender().unwrap(),
        ) {
            Ok(()) => {
                self.routers.insert(self.msg_buf.sender().unwrap().to_vec());
                Ok(())
            }
            Err(ConnErr::HostUnreachable) => Ok(()),
            Err(err) => Err(AppError::from(err)),
        }
    }

    /// Handle incoming protocol messages.
    ///
    /// Messages tagged with the `RouteSrv` protocol will be handled by the application itself
    /// while all other messages are handled by the `DispatcherPool`.
    fn handle_message(&mut self) -> AppResult<(), T::Error> {
        debug!("handle-message, {:?}", self.msg_buf);
        match self.msg_buf.route_info().map(|r| r.protocol()) {
            Some(Protocol::RouteSrv) => {
                if self.msg_buf.message_id() == NetError::message_id() {
                    let err = NetError::parse(&self.msg_buf).unwrap();
                    match err.code() {
                        ErrCode::REG_CONFLICT => {
                            error!("{}, retrying registration to RouteSrv", err);
                        }
                        ErrCode::REG_NOT_FOUND => {
                            match conn::send_to(
                                &self.router_sock,
                                &self.registration,
                                self.msg_buf.sender().unwrap(),
                            ) {
                                Ok(()) |
                                Err(ConnErr::HostUnreachable) => (),
                                Err(err) => return Err(AppError::from(err)),
                            }
                        }
                        _ => error!("{}, unhandled error from RouteSrv", err),
                    }
                } else {
                    warn!(
                        "handle-message, received unknown message from RouteSrv, {}",
                        self.msg_buf.message_id()
                    );
                }
            }
            Some(Protocol::Net) => warn!("handle-message, received Net protocol message"),
            None => warn!("handle-message, no route-info"),
            Some(_) => {
                if self.msg_buf.completed_txn() {
                    self.msg_buf.identities.remove(0);
                    conn::route(&self.pipe_out, &self.msg_buf)?;
                } else {
                    conn::route(&self.pipe_in, &self.msg_buf)?;
                }
            }
        }
        Ok(())
    }

    fn run(mut self, config: <T::State as AppState>::Config) -> AppResult<(), T::Error> {
        signals::init();
        for addr in config.route_addrs() {
            let addr = addr.to_addr_string();
            self.router_sock.connect(&addr)?;
        }
        let mut drop_buf: Vec<Vec<u8>> = Vec::with_capacity(config.route_addrs().len());
        let pipe_in = Arc::new(format!("inproc://net.dispatcher.in.{}", Uuid::new_v4()));
        let pipe_out = Arc::new(format!("inproc://net.dispatcher.out.{}", Uuid::new_v4()));
        self.pipe_in.bind(&*pipe_in)?;
        self.pipe_out.bind(&*pipe_out)?;
        let state = T::app_init(&config, pipe_out.clone()).map_err(
            AppError::Init,
        )?;
        DispatcherPool::<T>::new(pipe_in, pipe_out, config, state).run();
        info!("{} is ready to go.", T::APP_NAME);
        loop {
            self.msg_buf.reset();
            trace!("waiting for message");
            match self.wait_recv() {
                RecvEvent::OnMessage((router, pipe_in, pipe_out)) => {
                    trace!(
                        "received messages, router={}, pipe-in={}, pipe-out={}",
                        router,
                        pipe_in,
                        pipe_out
                    );
                    // Handle completed work before new work
                    if pipe_in {
                        trace!("OnReply, dispatcher->router");
                        proxy_message::<T>(
                            &mut self.pipe_in,
                            &mut self.router_sock,
                            &mut self.recv_buf,
                        )?;
                    }
                    if pipe_out {
                        trace!("OnRequest, dispatcher->router");
                        self.forward_request()?;
                    }
                    if router {
                        match conn::socket_read(
                            &self.router_sock,
                            &mut self.msg_buf,
                            &mut self.recv_buf,
                        ) {
                            Ok(ConnEvent::OnConnect) => self.handle_connect()?,
                            Ok(ConnEvent::OnMessage) => self.handle_message()?,
                            Err(err) => return Err(AppError::from(err)),
                        }
                    }
                }
                RecvEvent::Timeout => {
                    trace!("recv timeout");
                    self.next_heartbeat = next_heartbeat();
                    for addr in self.routers.iter() {
                        trace!("sending heartbeat to {:?}", addr);
                        match conn::send_to(&self.router_sock, &self.heartbeat, addr) {
                            Ok(()) => (),
                            Err(ConnErr::HostUnreachable) => {
                                trace!("router went away, {:?}", addr);
                                drop_buf.push(addr.to_vec());
                            }
                            Err(err) => return Err(AppError::from(err)),
                        }
                    }
                    for addr in drop_buf.iter() {
                        self.routers.remove(addr);
                    }
                    drop_buf.clear();
                }
                RecvEvent::Shutdown => {
                    info!("received shutdown signal, shutting down...");
                    let disconnect = protocol::Message::build(&routesrv::Disconnect::new())?;
                    for addr in self.routers.iter() {
                        trace!("sending disconnect to {:?}", addr);
                        conn::send_to(&self.router_sock, &disconnect, &addr)?;
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    /// Randomly select a RouteSrv from the active peers to route a request to.
    fn select_router(&self) -> AppResult<&[u8], T::Error> {
        // JW TODO: Select a random RouteSrv to send to
        self.routers.iter().last().map(Vec::as_slice).ok_or(
            AppError::NoRouter,
        )
    }

    /// Wait for incoming messages from RouteSrv(s) and Dispatchers and return a `RecvEvent` when
    /// a message is received, a timeout occurs, or the server is shutting down.
    fn wait_recv(&self) -> RecvEvent {
        let mut items = [
            self.router_sock.as_poll_item(zmq::POLLIN),
            self.pipe_in.as_poll_item(zmq::POLLIN),
            self.pipe_out.as_poll_item(zmq::POLLIN),
        ];
        match conn::socket_poll(&mut items, self.wait_timeout()) {
            Ok(count) => trace!("application received '{}' POLLIN events", count),
            Err(ConnErr::Timeout) => return RecvEvent::Timeout,
            Err(ConnErr::Shutdown(_)) => return RecvEvent::Shutdown,
            Err(err) => {
                error!("Error while waiting for socket events, {}", err);
                return RecvEvent::Shutdown;
            }
        }
        RecvEvent::OnMessage((
            items[0].is_readable(),
            items[1].is_readable(),
            items[2].is_readable(),
        ))
    }

    /// A tickless timer for determining how long to wait between each server tick. This value is
    /// variable depending upon when the next heartbeat is expected to occur.
    fn wait_timeout(&self) -> i64 {
        let time = self.next_heartbeat - time::clock_time();
        if time.is_negative() { 0 } else { time }
    }
}

pub fn start<T>(cfg: <T::State as AppState>::Config) -> AppResult<(), T::Error>
where
    T: Dispatcher,
{
    let app = Application::<T>::new(&cfg)?;
    app.run(cfg)
}

fn next_heartbeat() -> i64 {
    time::clock_time() + routesrv::PING_INTERVAL_MS
}

/// Proxy messages from one socket to another.
fn proxy_message<T>(
    source: &mut zmq::Socket,
    destination: &mut zmq::Socket,
    buf: &mut zmq::Message,
) -> AppResult<(), T::Error>
where
    T: Dispatcher,
{
    loop {
        match source.recv(buf, 0) {
            Ok(()) => {
                trace!("proxy-message, {:?}", buf);
                let flags = if buf.get_more() { zmq::SNDMORE } else { 0 };
                destination.send(&*buf, flags).map_err(ConnErr::Socket)?;
            }
            Err(err) => return Err(AppError::from(ConnErr::Socket(err))),
        }
        if !buf.get_more() {
            break;
        }
    }
    Ok(())
}
