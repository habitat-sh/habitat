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

//! The push thread.
//!
//! This is the thread for distributing rumors to members. It distributes to `FANOUT` members, no
//! more often than `Timing::GOSSIP_PERIOD_DEFAULT_MS`.

use std::{thread, time::Duration};

use habitat_core::util::ToI64;
use prometheus::{IntCounterVec, IntGaugeVec};
use time::SteadyTime;
use zmq;

use crate::{
    member::{Member, Membership},
    rumor::{RumorEnvelope, RumorKey, RumorKind, RumorType},
    server::{timing::Timing, Server},
    trace::TraceKind,
    ZMQ_CONTEXT,
};

const FANOUT: usize = 5;

lazy_static! {
    static ref GOSSIP_MESSAGES_SENT: IntCounterVec = register_int_counter_vec!(
        "hab_butterfly_gossip_messages_sent_total",
        "Total number of gossip messages sent",
        &["type", "mode"]
    )
    .unwrap();
    static ref GOSSIP_BYTES_SENT: IntGaugeVec = register_int_gauge_vec!(
        "hab_butterfly_gossip_sent_bytes",
        "Gossip message size sent in bytes",
        &["type", "mode"]
    )
    .unwrap();
}

/// The Push server
#[derive(Debug)]
pub struct Push {
    pub server: Server,
    pub timing: Timing,
}

impl Push {
    /// Creates a new Push instance from a Server and Timing
    pub fn new(server: Server, timing: Timing) -> Push {
        Push { server, timing }
    }

    /// Executes the Push thread. Gets a list of members to talk to that are not Confirmed; then
    /// proceeds to process the list in `FANOUT` sized chunks. If we finish sending the messages to
    /// all FANOUT targets faster than `Timing::GOSSIP_PERIOD_DEFAULT_MS`, we will block until we
    /// exceed that time.
    pub fn run(&mut self) {
        loop {
            if self.server.paused() {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            self.server.update_gossip_round();

            let mut check_list = self.server.member_list.check_list(self.server.member_id());
            let long_wait = self.timing.gossip_timeout();

            'fanout: loop {
                let mut thread_list = Vec::with_capacity(FANOUT);
                if check_list.is_empty() {
                    break 'fanout;
                }
                let drain_length = if check_list.len() >= FANOUT {
                    FANOUT
                } else {
                    check_list.len()
                };
                let next_gossip = self.timing.gossip_timeout();
                for member in check_list.drain(0..drain_length) {
                    if self.server.is_member_blocked(&member.id) {
                        debug!("Not sending rumors to {} - it is blocked", member.id);

                        continue;
                    }
                    // Unlike the SWIM mechanism, we don't actually want to send gossip traffic to
                    // persistent members that are confirmed dead. When the failure detector thread
                    // finds them alive again, we'll go ahead and get back to the business at hand.
                    if self.server.member_list.pingable(&member)
                        && !self.server.member_list.persistent_and_confirmed(&member)
                    {
                        let rumors = self.server.rumor_heat.currently_hot_rumors(&member.id);
                        if !rumors.is_empty() {
                            let sc = self.server.clone();

                            let guard = match thread::Builder::new()
                                .name(String::from("push-worker"))
                                .spawn(move || {
                                    PushWorker::new(sc).send_rumors(member, rumors);
                                }) {
                                Ok(guard) => guard,
                                Err(e) => {
                                    error!("Could not spawn thread: {}", e);
                                    continue;
                                }
                            };
                            thread_list.push(guard);
                        }
                    }
                }
                let num_threads = thread_list.len();
                for guard in thread_list.drain(0..num_threads) {
                    let _ = guard
                        .join()
                        .map_err(|e| error!("Push worker died: {:?}", e));
                }
                if SteadyTime::now() < next_gossip {
                    let wait_time = (next_gossip - SteadyTime::now()).num_milliseconds();
                    if wait_time > 0 {
                        thread::sleep(Duration::from_millis(wait_time as u64));
                    }
                }
            }
            if SteadyTime::now() < long_wait {
                let wait_time = (long_wait - SteadyTime::now()).num_milliseconds();
                if wait_time > 0 {
                    thread::sleep(Duration::from_millis(wait_time as u64));
                }
            }
        }
    }
}

/// A worker thread for pushing messages to a target
struct PushWorker {
    pub server: Server,
}

impl PushWorker {
    /// Create a new PushWorker.
    pub fn new(server: Server) -> PushWorker {
        PushWorker { server }
    }

    /// Send the list of rumors to a given member. This method creates an outbound socket and then
    /// closes the connection as soon as we are done sending rumors. ZeroMQ may choose to keep the
    /// connection and socket open for 1 second longer - so it is possible, but unlikely, that this
    /// method can lose messages.
    fn send_rumors(&self, member: Member, rumors: Vec<RumorKey>) {
        let socket = (**ZMQ_CONTEXT)
            .as_mut()
            .socket(zmq::PUSH)
            .expect("Failure to create the ZMQ push socket");
        socket
            .set_linger(1000)
            .expect("Failure to set the ZMQ push socket to not linger");
        socket
            .set_tcp_keepalive(0)
            .expect("Failure to set the ZMQ push socket to not use keepalive");
        socket
            .set_immediate(true)
            .expect("Failure to set the ZMQ push socket to immediate");
        socket
            .set_sndhwm(1000)
            .expect("Failure to set the ZMQ push socket hwm");
        socket
            .set_sndtimeo(500)
            .expect("Failure to set the ZMQ send timeout");
        let to_addr = format!("{}:{}", member.address, member.gossip_port);
        match socket.connect(&format!("tcp://{}", to_addr)) {
            Ok(()) => debug!("Connected push socket to {:?}", member),
            Err(e) => {
                error!("Cannot connect push socket to {:?}: {:?}", member, e);
                let label_values = &["socket_connect", "failure"];
                GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                GOSSIP_BYTES_SENT.with_label_values(label_values).set(0);
                return;
            }
        }
        'rumorlist: for rumor_key in rumors.iter() {
            let rumor_as_bytes = match rumor_key.kind {
                RumorType::Member => {
                    let send_rumor = match self.create_member_rumor(&rumor_key) {
                        Some(rumor) => rumor,
                        None => continue 'rumorlist,
                    };
                    trace_it!(GOSSIP: &self.server,
                              TraceKind::SendRumor,
                              &member.id,
                              &send_rumor);
                    match send_rumor.encode() {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            error!(
                                "Could not write our own rumor to bytes; abandoning sending \
                                 rumor: {:?}",
                                e
                            );
                            let label_values = &["member_rumor_encode", "failure"];
                            GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                            GOSSIP_BYTES_SENT.with_label_values(label_values).set(0);
                            continue 'rumorlist;
                        }
                    }
                }
                RumorType::Service => {
                    // trace_it!(GOSSIP: &self.server,
                    //           TraceKind::SendRumor,
                    //           &member.id,
                    //           &send_rumor);
                    match self
                        .server
                        .service_store
                        .encode(&rumor_key.key, &rumor_key.id)
                    {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            error!(
                                "Could not write our own rumor to bytes; abandoning sending \
                                 rumor: {:?}",
                                e
                            );
                            let label_values = &["service_rumor_encode", "failure"];
                            GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                            GOSSIP_BYTES_SENT.with_label_values(label_values).set(0);
                            continue 'rumorlist;
                        }
                    }
                }
                RumorType::ServiceConfig => {
                    // trace_it!(GOSSIP: &self.server,
                    //           TraceKind::SendRumor,
                    //           &member.id,
                    //           &send_rumor);
                    match self
                        .server
                        .service_config_store
                        .encode(&rumor_key.key, &rumor_key.id)
                    {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            error!(
                                "Could not write our own rumor to bytes; abandoning sending \
                                 rumor: {:?}",
                                e
                            );
                            let label_values = &["service_config_rumor_encode", "failure"];
                            GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                            GOSSIP_BYTES_SENT.with_label_values(label_values).set(0);
                            continue 'rumorlist;
                        }
                    }
                }
                RumorType::ServiceFile => {
                    // trace_it!(GOSSIP: &self.server,
                    //           TraceKind::SendRumor,
                    //           &member.id,
                    //           &send_rumor);
                    match self
                        .server
                        .service_file_store
                        .encode(&rumor_key.key, &rumor_key.id)
                    {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            error!(
                                "Could not write our own rumor to bytes; abandoning sending \
                                 rumor: {:?}",
                                e
                            );
                            let label_values = &["service_file_rumor_encode", "failure"];
                            GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                            GOSSIP_BYTES_SENT.with_label_values(label_values).set(0);
                            continue 'rumorlist;
                        }
                    }
                }
                RumorType::Departure => match self
                    .server
                    .departure_store
                    .encode(&rumor_key.key, &rumor_key.id)
                {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        error!(
                            "Could not write our own rumor to bytes; abandoning sending rumor: \
                             {:?}",
                            e
                        );
                        let label_values = &["departure_rumor_encode", "failure"];
                        GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                        GOSSIP_BYTES_SENT.with_label_values(label_values).set(0);
                        continue 'rumorlist;
                    }
                },
                RumorType::Election => {
                    // trace_it!(GOSSIP: &self.server,
                    //           TraceKind::SendRumor,
                    //           &member.id,
                    //           &send_rumor);
                    match self
                        .server
                        .election_store
                        .encode(&rumor_key.key, &rumor_key.id)
                    {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            error!(
                                "Could not write our own rumor to bytes; abandoning sending \
                                 rumor: {:?}",
                                e
                            );
                            let label_values = &["election_rumor_encode", "failure"];
                            GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                            GOSSIP_BYTES_SENT.with_label_values(label_values).set(0);
                            continue 'rumorlist;
                        }
                    }
                }
                RumorType::ElectionUpdate => match self
                    .server
                    .update_store
                    .encode(&rumor_key.key, &rumor_key.id)
                {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        error!(
                            "Could not write our own rumor to bytes; abandoning sending rumor: \
                             {:?}",
                            e
                        );
                        let label_values = &["election_update_rumor_encode", "failure"];
                        GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                        GOSSIP_BYTES_SENT.with_label_values(label_values).set(0);
                        continue 'rumorlist;
                    }
                },
                RumorType::Fake | RumorType::Fake2 => {
                    debug!("You have fake rumors; how odd!");
                    continue 'rumorlist;
                }
            };
            let rumor_len = rumor_as_bytes.len().to_i64();
            let payload = match self.server.generate_wire(rumor_as_bytes) {
                Ok(payload) => payload,
                Err(e) => {
                    error!("Generating protobuf failed: {}", e);
                    let label_values = &["generate_wire", "failure"];
                    GOSSIP_MESSAGES_SENT.with_label_values(label_values).inc();
                    GOSSIP_BYTES_SENT
                        .with_label_values(label_values)
                        .set(rumor_len);
                    continue 'rumorlist;
                }
            };
            match socket.send(&payload, 0) {
                Ok(()) => {
                    GOSSIP_MESSAGES_SENT
                        .with_label_values(&[&rumor_key.kind.to_string(), "success"])
                        .inc();
                    GOSSIP_BYTES_SENT
                        .with_label_values(&[&rumor_key.kind.to_string(), "success"])
                        .set(payload.len().to_i64());
                    debug!("Sent rumor {:?} to {:?}", rumor_key, member);
                }
                Err(e) => warn!(
                    "Could not send rumor to {:?} @ {:?}; ZMQ said: {:?}",
                    member.id, to_addr, e
                ),
            }
        }
        self.server.rumor_heat.cool_rumors(&member.id, &rumors);
    }

    /// Given a rumorkey, creates a protobuf rumor for sharing.
    fn create_member_rumor(&self, rumor_key: &RumorKey) -> Option<RumorEnvelope> {
        let member = self.server.member_list.get_cloned(&rumor_key.key())?;
        let payload = Membership {
            member,
            health: self
                .server
                .member_list
                .health_of_by_id(&rumor_key.key())
                .unwrap(),
        };
        let rumor = RumorEnvelope {
            type_: RumorType::Member,
            from_id: self.server.member_id().to_string(),
            kind: RumorKind::Membership(payload),
        };
        Some(rumor)
    }
}
