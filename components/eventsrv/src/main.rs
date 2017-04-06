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

extern crate env_logger;
extern crate habitat_eventsrv;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate zmq;

mod message;

use message::event::EventEnvelope;
use protobuf::parse_from_bytes;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::from_utf8;
use zmq::{Context, SUB, XPUB};

fn main() {
    let ctx = Context::new();

    // TODO: need to better parameterize these ports;
    // they can't be hard-coded if we're going to have multiple event
    // servers running on a given host.
    //
    // Maybe we gossip this information around?

    // Publishers need to connect their PUB socket to this.
    let sub_sock = ctx.socket(SUB).unwrap();
    assert!(sub_sock.bind("tcp://*:34570").is_ok());
    assert!(sub_sock.set_subscribe(b"").is_ok()); // Subscribe to everything

    // Subscribers need to connect their SUB port to this.
    let xpub_sock = ctx.socket(XPUB).unwrap();
    assert!(xpub_sock.bind("tcp://*:34571").is_ok());

    // We'll cache the most recent messages from each service and each
    // ring member. When new subscribers connect, we can send them
    // this "snapshot" of current activity.
    let mut service_cache = HashMap::new();
    let mut member_cache = HashMap::new();

    let mut poll_items = [
        sub_sock.as_poll_item(zmq::POLLIN),
        xpub_sock.as_poll_item(zmq::POLLIN)
    ];

    loop {
        // A timeout of -1 says to wait indefinitely until a message comes
        if zmq::poll(&mut poll_items, -1).is_err() {
            break; // This will stop the event service
        }

        if poll_items[0].is_readable() {
            // An event was published!

            let bytes = sub_sock.recv_bytes(0).unwrap();
            let event = parse_from_bytes::<EventEnvelope>(&bytes).unwrap();
            let member_id = event.get_member_id();
            let timestamp = event.get_timestamp();
            let service = event.get_service().to_string();
            if service.is_empty() {
                warn!("missing service: {:?}", event);
                continue;
            }

            println!("EVENTSRV: Timestamp {}", timestamp);
            println!("EVENTSRV: Member ID {}", member_id);
            println!("EVENTSRV: Service {}", service);

            // Store the bytes of the message in the cache. For the
            // service cache, we also record the member ID, and vice
            // versa for the member cache; these data will be used for
            // deduplication of messages being sent to new subscribers.
            service_cache.insert(service.clone(), (member_id, bytes.clone()));
            member_cache.insert(member_id, (service, bytes.clone()));

            println!("EVENTSRV: Service Cache {:?}", service_cache.keys());
            println!("EVENTSRV: Member Cache {:?}\n", member_cache.keys());

            xpub_sock.send(&bytes, 0).unwrap();
        }

        if poll_items[1].is_readable() {
            // A subscriber connected; let's ensure they've got a
            // snapshot of what's currently happening.

            // Event is one byte 0=unsub or 1=sub, followed by topic
            let event = xpub_sock.recv_bytes(0).unwrap();
            if event[0] == 1 {
                // The subscriber has subscribed. Send all cached
                // messages to it.
                //
                // First we'll send all the latest messages from the
                // services, keeping track of which ring members those
                // were from. Then, we'll send the latest messages
                // from the members, but only if we didn't just send a
                // service message from them. This prevents us from
                // sending one message twice.
                //
                // NOTE: we're not doing anything with topics; the
                // subscriber is going to get everything.

                let mut members_encountered = HashSet::new();

                for (service, &(member_id, ref message)) in &service_cache {
                    members_encountered.insert(member_id);
                    println!("\tSending message for {}/{}", service, member_id);
                    xpub_sock.send(&message, 0).unwrap();
                }
                println!("\t---");
                for (member_id, &(ref service, ref message)) in &member_cache {
                    if !(members_encountered.contains(member_id)) {
                        println!("\tSending message for {}/{}", service, member_id);
                        xpub_sock.send(&message, 0).unwrap();
                    }
                }
            }
        }
    }
}
