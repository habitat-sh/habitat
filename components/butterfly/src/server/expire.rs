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

//! Expire suspected members.
//!
//! This module keeps track of suspected members, and sets their status to confirmed if they remain
//! suspect long enough.

use std::thread;
use std::time::Duration;

use time::SteadyTime;

use member::Health;
use rumor::{RumorKey, RumorType};
use server::timing::Timing;
use server::Server;
use trace::TraceKind;

pub struct Expire {
    pub server: Server,
    pub timing: Timing,
}

impl Expire {
    /// Takes a reference to a server, and a `Timing`, returns you an Expire struct.
    pub fn new(server: Server, timing: Timing) -> Expire {
        Expire {
            server: server,
            timing: timing,
        }
    }

    /// Run the expire thread.
    pub fn run(&self) {
        loop {
            let mut expired_list: Vec<String> = Vec::new();
            self.server.member_list.with_suspects(|(id, suspect)| {
                let now = SteadyTime::now();
                if now >= *suspect + self.timing.suspicion_timeout_duration() {
                    expired_list.push(String::from(id));
                    self.server
                        .member_list
                        .insert_health_by_id(id, Health::Confirmed);
                    self.server.member_list.with_member(id, |has_member| {
                        let member = has_member.expect("Member does not exist when expiring it");
                        trace!("Marking {:?} as Confirmed", member);
                        trace_it!(PROBE: &self.server, TraceKind::ProbeConfirmed, &member.id, &member.address);
                    });
                }
            });
            for mid in expired_list.iter() {
                self.server.member_list.expire(mid);
                self.server.member_list.depart(mid);
                self.server.rumor_heat.start_hot_rumor(RumorKey::new(
                    RumorType::Member,
                    mid.clone(),
                    "",
                ));
            }

            let mut departed_list: Vec<String> = Vec::new();
            self.server
                .member_list
                .with_departures(|(id, departure_time)| {
                    let now = SteadyTime::now();
                    if now >= *departure_time + self.timing.departure_timeout_duration() {
                        departed_list.push(String::from(id));
                        self.server
                            .member_list
                            .insert_health_by_id(id, Health::Departed);
                        self.server.member_list.with_member(id, |has_member| {
                            let member =
                                has_member.expect("Member does not exist when departing it");
                            trace!("Marking {:?} as Departed", member);
                            trace_it!(PROBE: &self.server, TraceKind::ProbeDeparted, &member.id, &member.address);
                        });
                    }
                });
            for mid in departed_list.iter() {
                self.server.member_list.depart_remove(mid);
                self.server.rumor_heat.start_hot_rumor(RumorKey::new(
                    RumorType::Member,
                    mid.clone(),
                    "",
                ));
            }

            thread::sleep(Duration::from_millis(500));
        }
    }
}
