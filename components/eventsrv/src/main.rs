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

extern crate habitat_eventsrv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate zmq;
extern crate protobuf;

mod message;

use zmq::{Context, PULL, PUSH};
use protobuf::parse_from_bytes;

use message::event::EventEnvelope;

fn main() {
    let ctx = Context::new();
    let pull_sock = ctx.socket(PULL).unwrap();
    let push_sock = ctx.socket(PUSH).unwrap();
    assert!(pull_sock.connect("tcp://127.0.0.1:34567").is_ok());
    assert!(push_sock.connect("tcp://127.0.0.1:45678").is_ok());

    loop {
        match pull_sock.recv_bytes(0) {
            Ok(bytes) => {
                let event = parse_from_bytes::<EventEnvelope>(&bytes).unwrap();
                let member_id = event.get_member_id();
                let timestamp = event.get_timestamp();

                println!("Timestamp {}", timestamp);
                println!("Member ID {}\n", member_id);
                push_sock.send(&bytes, 0).unwrap();
            }
            Err(e) => panic!("zeromq socket error: {:?}", e),
        }
    }
}
