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

use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use time::SteadyTime;

use member::{Member, Membership};
use network::{GossipSender, Network};
use rumor::{RumorEnvelope, RumorKey, RumorKind, RumorType};
use server::timing::Timing;
use server::Server;
use trace::TraceKind;

const FANOUT: usize = 5;

/// The Push server
#[derive(Debug)]
pub struct Push<N: Network> {
    pub server: Server<N>,
    pub timing: Timing,
}

impl<N: Network> Push<N> {
    /// Creates a new Push instance from a Server and Timing
    pub fn new(server: Server<N>, timing: Timing) -> Self {
        Self {
            server: server,
            timing: timing,
        }
    }

    /// Executes the Push thread. Gets a list of members to talk to that are not Confirmed; then
    /// proceeds to process the list in `FANOUT` sized chunks. If we finish sending the messages to
    /// all FANOUT targets faster than `Timing::GOSSIP_PERIOD_DEFAULT_MS`, we will block until we
    /// exceed that time.
    pub fn run(&mut self) {
        'send: loop {
            if self.server.pause.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            self.server.update_gossip_round();

            let mut check_list = self.server.member_list.check_list(self.server.member_id());
            let long_wait = self.timing.gossip_timeout();

            'fanout: loop {
                let mut thread_list = Vec::with_capacity(FANOUT);
                if check_list.len() == 0 {
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
                        if rumors.len() > 0 {
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
struct PushWorker<N: Network> {
    pub server: Server<N>,
}

impl<N: Network> PushWorker<N> {
    /// Create a new PushWorker.
    pub fn new(server: Server<N>) -> Self {
        Self { server: server }
    }

    /// Send the list of rumors to a given member. This method creates an outbound socket and then
    /// closes the connection as soon as we are done sending rumors. ZeroMQ may choose to keep the
    /// connection and socket open for 1 second longer - so it is possible, but unlikely, that this
    /// method can loose messages.
    fn send_rumors(&self, member: Member, rumors: Vec<RumorKey>) {
        let sender = match self
            .server
            .read_network()
            .create_gossip_sender(member.gossip_socket_address())
        {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to get the push socket to {:?}: {:?}", member, e);
                return;
            }
        };
        'rumorlist: for ref rumor_key in rumors.iter() {
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
                                "Could not write our own rumor to bytes; abandoning \
                                 sending rumor: {:?}",
                                e
                            );
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
                                "Could not write our own rumor to bytes; abandoning \
                                 sending rumor: {:?}",
                                e
                            );
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
                                "Could not write our own rumor to bytes; abandoning \
                                 sending rumor: {:?}",
                                e
                            );
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
                                "Could not write our own rumor to bytes; abandoning \
                                 sending rumor: {:?}",
                                e
                            );
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
                            "Could not write our own rumor to bytes; abandoning \
                             sending rumor: {:?}",
                            e
                        );
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
                                "Could not write our own rumor to bytes; abandoning \
                                 sending rumor: {:?}",
                                e
                            );
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
                            "Could not write our own rumor to bytes; abandoning sending \
                             rumor: {:?}",
                            e
                        );
                        continue 'rumorlist;
                    }
                },
                RumorType::Fake | RumorType::Fake2 => {
                    debug!("You have fake rumors; how odd!");
                    continue 'rumorlist;
                }
            };
            let payload = match self.server.generate_wire(rumor_as_bytes) {
                Ok(payload) => payload,
                Err(e) => {
                    error!("Generating protobuf failed: {}", e);
                    continue 'rumorlist;
                }
            };
            match sender.send(&payload) {
                Ok(()) => debug!("Sent rumor {:?} to {:?}", rumor_key, member),
                Err(e) => println!(
                    "Could not send rumor to {:?} @ {:?}; {:?}",
                    member.id,
                    member.gossip_socket_address::<N::AddressAndPort>(),
                    e
                ),
            }
        }
        self.server.rumor_heat.cool_rumors(&member.id, &rumors);
    }

    /// Given a rumorkey, creates a protobuf rumor for sharing.
    fn create_member_rumor(&self, rumor_key: &RumorKey) -> Option<RumorEnvelope> {
        let mut member = None;
        self.server.member_list.with_member(&rumor_key.key(), |m| {
            if let Some(m) = m {
                member = Some(m.clone());
            }
        });
        if member.is_none() {
            return None;
        }
        let payload = Membership {
            member: member.unwrap(),
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
