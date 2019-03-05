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

//! The pull thread.
//!
//! This module handles pulling all the pushed rumors from every member off a ZMQ socket.

use std::{thread,
          time::Duration};

use habitat_core::util::ToI64;
use prometheus::{IntCounterVec,
                 IntGaugeVec};
use zmq;

use crate::{rumor::{RumorEnvelope,
                    RumorKind},
            server::Server,
            trace::TraceKind,
            ZMQ_CONTEXT};

lazy_static! {
    static ref GOSSIP_MESSAGES_RECEIVED: IntCounterVec =
        register_int_counter_vec!("hab_butterfly_gossip_messages_received_total",
                                  "Total number of gossip messages received",
                                  &["type", "mode", "blocked"]).unwrap();
    static ref GOSSIP_BYTES_RECEIVED: IntGaugeVec =
        register_int_gauge_vec!("hab_butterfly_gossip_received_bytes",
                                "Gossip message size received in bytes",
                                &["type", "mode", "blocked"]).unwrap();
}

/// Takes a reference to the server itself
pub struct Pull {
    pub server: Server,
}

impl Pull {
    /// Create a new Pull
    pub fn new(server: Server) -> Pull { Pull { server } }

    /// Run this thread. Creates a socket, binds to the `gossip_addr`, then processes messages as
    /// they are received. Uses a ZMQ pull socket, so inbound messages are fair-queued.
    pub fn run(&mut self) {
        let socket = (**ZMQ_CONTEXT).as_mut()
                                    .socket(zmq::PULL)
                                    .expect("Failure to create the ZMQ pull socket");
        socket.set_linger(0)
              .expect("Failure to set the ZMQ Pull socket to not linger");
        socket.set_tcp_keepalive(0)
              .expect("Failure to set the ZMQ Pull socket to not use keepalive");
        socket.bind(&format!("tcp://{}", self.server.gossip_addr()))
              .expect("Failure to bind the ZMQ Pull socket to the port");
        'recv: loop {
            if self.server.paused() {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            let msg = match socket.recv_msg(0) {
                Ok(msg) => msg,
                Err(e) => {
                    error!("Error receiving message: {:?}", e);
                    continue 'recv;
                }
            };

            let payload = match self.server.unwrap_wire(&msg) {
                Ok(payload) => payload,
                Err(e) => {
                    // NOTE: In the future, we might want to block people who send us
                    // garbage all the time.
                    error!("Error parsing protocol message: {:?}", e);
                    let label_values = &["unwrap_wire", "failure", "unknown"];
                    GOSSIP_BYTES_RECEIVED.with_label_values(label_values)
                                         .set(msg.len().to_i64());
                    GOSSIP_MESSAGES_RECEIVED.with_label_values(label_values)
                                            .inc();
                    continue;
                }
            };

            let proto = match RumorEnvelope::decode(&payload) {
                Ok(proto) => proto,
                Err(e) => {
                    error!("Error parsing protocol message: {:?}", e);
                    let label_values = &["undecodable", "failure", "unknown"];
                    GOSSIP_BYTES_RECEIVED.with_label_values(label_values)
                                         .set(payload.len().to_i64());
                    GOSSIP_MESSAGES_RECEIVED.with_label_values(label_values)
                                            .inc();
                    continue 'recv;
                }
            };

            let blocked = self.server.is_member_blocked(&proto.from_id);
            let blocked_label = if blocked { "true" } else { "false" };
            let label_values = &[&proto.type_.to_string(), "success", blocked_label];

            GOSSIP_MESSAGES_RECEIVED.with_label_values(label_values)
                                    .inc();
            GOSSIP_BYTES_RECEIVED.with_label_values(label_values)
                                 .set(payload.len().to_i64());

            if blocked {
                warn!("Not processing message from {} - it is blocked",
                      proto.from_id);
                continue 'recv;
            }

            trace_it!(GOSSIP: &self.server, TraceKind::RecvRumor, &proto.from_id, &proto);
            match proto.kind {
                RumorKind::Membership(membership) => {
                    self.server
                        .insert_member_from_rumor(membership.member, membership.health);
                }
                RumorKind::Service(service) => self.server.insert_service(service),
                RumorKind::ServiceConfig(service_config) => {
                    self.server.insert_service_config(service_config);
                }
                RumorKind::ServiceFile(service_file) => {
                    self.server.insert_service_file(service_file);
                }
                RumorKind::Election(election) => {
                    self.server.insert_election(election);
                }
                RumorKind::ElectionUpdate(election) => {
                    self.server.insert_update_election(election);
                }
                RumorKind::Departure(departure) => {
                    self.server.insert_departure(departure);
                }
            }
        }
    }
}
