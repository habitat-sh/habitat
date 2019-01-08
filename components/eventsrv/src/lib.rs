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
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use habitat_core as core;
use habitat_eventsrv_protocol as protocol;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

pub mod config;
pub mod error;

use std::collections::HashMap;
use std::collections::HashSet;

use crate::protocol::EventEnvelope;
use protobuf::parse_from_bytes;
use zmq::{Context, PULL, XPUB};

/// Proxies messages coming into `frontend_port` out through
/// `backend_port`, caching recent messages for new subscribers.
///
/// Event publishers should connect a ZMQ `PUSH` socket to
/// `frontend_port` and send `EventEnvelope` protobuf messages to
/// it. Publishers can connect to multiple such proxy processes;
/// messages will be fairly-dealt to all connected proxies.
///
/// Event subscribers should connect a ZMQ `SUB` socket to
/// `backend_port` to receive `EventEnvelope` protobuf
/// messages. Subscribers should connect to all available proxy
/// processes to ensure they receive all event messages.
///
/// Upon connection, subscribers will receive, from each connected
/// proxy, the most recent messages from each service and from each
/// ring member. Subscribers are responsible for sorting the messages
/// received by timestamp.
///
/// # Panics
///
/// If either `frontend_port` or `backend_port` cannot be bound to
/// sockets (e.g., they're already in use), the thread will panic.
pub fn proxy(frontend_port: u16, backend_port: u16) {
    let ctx = Context::new();

    let pull_sock = ctx.socket(PULL).unwrap();
    let pull_bind = format!("tcp://*:{}", frontend_port);
    if let Err(e) = pull_sock.bind(&pull_bind) {
        panic!("Could not bind socket to port {}: {:?}", frontend_port, e);
    }

    let xpub_sock = ctx.socket(XPUB).unwrap();
    let xpub_bind = format!("tcp://*:{}", backend_port);
    if let Err(e) = xpub_sock.bind(&xpub_bind) {
        panic!("Could not bind socket to port {}: {:?}", backend_port, e);
    }

    // We'll cache the most recent messages from each service and each
    // ring member. When new subscribers connect, we can send them
    // this "snapshot" of current activity.
    let mut service_cache = HashMap::new();
    let mut member_cache = HashMap::new();

    let mut poll_items = [
        pull_sock.as_poll_item(zmq::POLLIN),
        xpub_sock.as_poll_item(zmq::POLLIN),
    ];

    loop {
        // A timeout of -1 says to wait indefinitely until a message comes
        if let Err(e) = zmq::poll(&mut poll_items, -1) {
            panic!("Error!: {}", e)
        }

        if poll_items[0].is_readable() {
            // An event was published!

            let bytes = pull_sock.recv_bytes(0).unwrap();
            let event = parse_from_bytes::<EventEnvelope>(&bytes).unwrap();
            let member_id = event.get_member_id().to_string();
            let service = event.get_service().to_string();

            if service.is_empty() {
                warn!("missing service: {:?}", event);
                continue;
            }

            // Store the bytes of the message in the cache. For the
            // service cache, we also record the member ID, and vice
            // versa for the member cache; these data will be used for
            // deduplication of messages being sent to new subscribers.
            service_cache.insert(service.clone(), (member_id.clone(), bytes.clone()));
            member_cache.insert(member_id, (service, bytes.clone()));
            xpub_sock.send(&bytes, 0).unwrap();
        }

        if poll_items[1].is_readable() {
            // A subscriber connected; let's ensure they've got a
            // snapshot of what's currently happening.

            // Event is one byte 0=unsub or 1=sub, followed by topic
            let event = xpub_sock.recv_bytes(0).unwrap();
            if event[0] == 1 {
                // The subscriber has subscribed. Send all unique
                // cached messages to it.
                //
                // First we'll send all the latest messages from the
                // services, keeping track of which ring members those
                // were from. Then, we'll send the latest messages
                // from the members, but only if we didn't just send a
                // service message from them. This prevents us from
                // sending the same message twice.

                let mut members_encountered = HashSet::new();

                for (_, &(ref member_id, ref message)) in &service_cache {
                    members_encountered.insert(member_id);
                    xpub_sock.send(&message, 0).unwrap();
                }
                for (member_id, &(_, ref message)) in &member_cache {
                    if !(members_encountered.contains(member_id)) {
                        xpub_sock.send(&message, 0).unwrap();
                    }
                }
            }
        }
    }
}
