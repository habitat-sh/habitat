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

use habitat_eventsrv_protocol as protocol;

pub mod message;

use crate::protocol::EventEnvelope;
pub use crate::protocol::EventSrvAddr;
use protobuf::Message;

pub struct EventSrvClient(zmq::Socket);

impl EventSrvClient {
    pub fn new() -> Self {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::PUSH).unwrap();
        // We want to intentionally set the high water mark for this socket to a low number. In the
        // event that one of our eventsrv processes crashes, this provides two benefits: it reduces
        // the number of message frames that get backed up and it also reduces the impact those
        // stale messages have when the dead process comes back and those messages get sent
        // through.
        socket.set_sndhwm(2).unwrap();
        EventSrvClient(socket)
    }

    pub fn connect(&self, addr: &EventSrvAddr) {
        self.0.connect(&addr.to_producer_addr()).unwrap();
    }

    pub fn send(&self, event: &mut EventEnvelope) {
        let timestamp = self.current_time();
        event.set_timestamp(timestamp);
        self.0
            .send(event.write_to_bytes().unwrap().as_slice(), 0)
            .unwrap();
    }

    /// Returns the current time as milliseconds since the Epoch
    /// (1970-01-01T00:00:00Z).
    fn current_time(&self) -> u64 {
        let timespec = time::get_time();
        let sec: u64 = timespec.sec as u64 * 1000;
        let nsec: u64 = timespec.nsec as u64 / 1000 / 1000;
        sec + nsec
    }
}
