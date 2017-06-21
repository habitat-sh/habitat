// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

pub mod prelude;

use std::default::Default;
use std::fmt;
use std::result;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::SyncSender;

use protobuf::parse_from_bytes;
use zmq;

use config::DispatcherCfg;
use server::Envelope;

/// Function signature for dispatch handlers.
pub type MessageHandler<T> = Fn(&mut Envelope) -> Result<(), T>;

/// Apply to a struct containing worker state that will be passed as a mutable reference on each
/// call of `dispatch()` to an implementer of `Dispatcher`.
pub trait DispatcherState {
    fn is_initialized(&self) -> bool;
}

/// Dispatchers connect to Message Queue Servers
pub trait Dispatcher: Sized + Send {
    type Config: Send + Sync + DispatcherCfg;
    type Error: Send + From<zmq::Error> + fmt::Display;
    type InitState: Clone + Send + Into<Self::State>;
    type State: DispatcherState;

    fn dispatch(
        message: &mut Envelope,
        socket: &mut zmq::Socket,
        state: &mut Self::State,
    ) -> result::Result<(), Self::Error>;

    /// Address to the ZeroMQ message queue that dispatcher workers will consume from.
    fn message_queue() -> &'static str;

    fn new(config: Arc<RwLock<Self::Config>>) -> Self;

    fn context(&mut self) -> &mut zmq::Context;

    /// Callback to perform dispatcher initialization.
    ///
    /// The default implementation will take your initial state and convert it into the actual
    /// state of the worker. Override this function if you need to perform additional steps to
    /// initialize your worker state.
    fn init(&mut self, init_state: Self::InitState) -> result::Result<Self::State, Self::Error> {
        Ok(init_state.into())
    }

    fn start(mut self, rz: SyncSender<()>, mut state: Self::State) -> Result<(), Self::Error> {
        // Debug assert because it's only necessary to perform this check during development. It
        // isn't possible for the state to not be initialized at runtime unless the developer
        // wrongfully implements the `init()` callback or omits an override implementation where
        // the default implementation isn't enough to initialize the dispatcher's state.
        debug_assert!(state.is_initialized(), "Dispatcher state not initialized!");
        let mut raw = zmq::Message::new().unwrap();
        let mut sock = self.context().socket(zmq::DEALER).unwrap();
        let mut envelope = Envelope::default();
        try!(sock.connect(Self::message_queue()));
        rz.send(()).unwrap();
        'recv: loop {
            'hops: loop {
                let hop = try!(sock.recv_msg(0));
                if hop.len() == 0 {
                    break;
                }
                if envelope.add_hop(hop).is_err() {
                    warn!("drop message, too many hops");
                    envelope.reset();
                    break 'recv;
                }
            }
            try!(sock.recv(&mut raw, 0));
            match parse_from_bytes(&raw) {
                Ok(msg) => {
                    debug!("OnMessage, {:?}", &msg);
                    envelope.msg = msg;
                    if let Some(err) = Self::dispatch(&mut envelope, &mut sock, &mut state).err() {
                        warn!("dispatch error, {}", err);
                    }
                }
                Err(e) => warn!("OnMessage bad message, {}", e),
            }
            envelope.reset();
        }
        Ok(())
    }
}
