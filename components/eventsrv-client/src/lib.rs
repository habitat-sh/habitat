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
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate env_logger;
extern crate habitat_eventsrv_protocol as protocol;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate time;
extern crate zmq;

pub mod message;

use zmq::{Context, PUSH, Socket};
use protobuf::Message;
use protocol::EventEnvelope;

pub struct EventSrvClient {
    ports: Vec<String>,
    socket: Socket,
}

impl EventSrvClient {
    pub fn new(ports: Vec<String>) -> Self {
        let ctx = Context::new();
        let socket = ctx.socket(PUSH).expect(
            "unable to create EventSrvClient push socket",
        );

        // We want to intentionally set the high water mark for this socket to a low number. In the
        // event that one of our eventsrv processes crashes, this provides two benefits: it reduces
        // the number of message frames that get backed up and it also reduces the impact those
        // stale messages have when the dead process comes back and those messages get sent
        // through.
        socket.set_sndhwm(2).unwrap();

        EventSrvClient {
            ports: ports,
            socket: socket,
        }
    }

    pub fn connect(&self) {
        for p in &self.ports {
            let push_connect = format!("tcp://localhost:{}", p);
            debug!("EventSrvClient connecting to {}", push_connect);
            assert!(self.socket.connect(&push_connect).is_ok());
        }
    }

    pub fn send(&self, mut event: EventEnvelope) {
        let timestamp = self.current_time();
        event.set_timestamp(timestamp);
        self.socket
            .send(event.write_to_bytes().unwrap().as_slice(), 0)
            .unwrap();
    }

    fn current_time(&self) -> u64 {
        let timespec = time::get_time();
        let sec: u64 = timespec.sec as u64 * 1000;
        let nsec: u64 = timespec.nsec as u64 / 1000 / 1000;
        sec + nsec
    }
}
