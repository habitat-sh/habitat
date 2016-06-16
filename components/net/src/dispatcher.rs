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

use std::fmt;
use std::result;
use std::sync::{mpsc, Arc, RwLock};

use protobuf::parse_from_bytes;
use zmq;

use server::Envelope;

pub type MessageHandler<T> = Fn(&mut Envelope) -> result::Result<(), T>;

/// Dispatchers connect to Message Queue Servers
pub trait Dispatcher: Sized + Send {
    type Config: Send + Sync;
    type Error: Send + From<zmq::Error> + fmt::Display;
    type State;

    fn message_queue() -> &'static str;

    // JW TODO: This should take something that impelements an "application config" trait
    fn new(config: Arc<RwLock<Self::Config>>) -> Self;

    fn context(&mut self) -> &mut zmq::Context;

    fn dispatch(message: &mut Envelope,
                socket: &mut zmq::Socket,
                state: &mut Self::State)
                -> result::Result<(), Self::Error>;

    fn init(&mut self) -> result::Result<(), Self::Error> {
        Ok(())
    }

    fn start(mut self, rz: mpsc::SyncSender<()>) -> result::Result<(), Self::Error> {
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
                    try!(Self::dispatch(&mut envelope, &mut sock, self.state()));
                }
                Err(e) => warn!("erorr parsing message, err={}", e),
            }
            envelope.reset();
        }
        try!(sock.close());
        Ok(())
    }

    fn state(&mut self) -> &mut Self::State;
}
