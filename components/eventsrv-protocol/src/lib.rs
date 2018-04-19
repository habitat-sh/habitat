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

extern crate protobuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod message;

use std::net::{IpAddr, Ipv4Addr};
pub use message::event::*;

pub const DEFAULT_CONSUMER_PORT: u16 = 9689;
pub const DEFAULT_PRODUCER_PORT: u16 = 9688;

/// Configuration structure for connecting to an EventSrv
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct EventSrvAddr {
    /// Listening address of EventSrv
    pub host: IpAddr,
    /// Listening port of consumer socket
    pub consumer_port: u16,
    /// Listening port of producer socket
    pub producer_port: u16,
}

impl EventSrvAddr {
    pub fn to_consumer_addr(&self) -> String {
        format!("tcp://{}:{}", self.host, self.consumer_port)
    }

    pub fn to_producer_addr(&self) -> String {
        format!("tcp://{}:{}", self.host, self.producer_port)
    }
}

impl Default for EventSrvAddr {
    fn default() -> Self {
        EventSrvAddr {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            consumer_port: DEFAULT_CONSUMER_PORT,
            producer_port: DEFAULT_PRODUCER_PORT,
        }
    }
}
