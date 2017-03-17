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

// NOTE: The sole purpose of this subscriber is testing and debugging. It's not
// required for normal operation.

extern crate habitat_eventsrv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate zmq;
extern crate protobuf;

mod message;

use zmq::{Context, PULL};
use protobuf::parse_from_bytes;

use message::event::EventEnvelope;

fn main() {
    let ctx = Context::new();
    let socket = ctx.socket(PULL).unwrap();
    assert!(socket.bind("tcp://*:45678").is_ok());

    loop {
        match socket.recv_bytes(0) {
            Ok(bytes) => {
                let event = parse_from_bytes::<EventEnvelope>(&bytes).unwrap();
                let received_payload = String::from_utf8(event.get_payload().to_vec()).unwrap();
                let member_id = event.get_member_id();
                let timestamp = event.get_timestamp();

                println!("Timestamp {}", timestamp);
                println!("Member ID {}\n", member_id);
                debug!("{}\n", received_payload);
            }
            Err(e) => panic!("zeromq socket error: {:?}", e),
        }
    }
}
