// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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
//! This module keeps track of suspected members, and sets their stauts to confirmed if they remain
//! suspect long enough.

use std::thread;
use std::time::Duration;

use time::SteadyTime;

use member::Health;
use rumor::RumorKey;
use server::Server;
use server::outbound::Timing;

pub struct Expire<'a> {
    pub server: &'a Server,
    pub timing: Timing,
}

impl<'a> Expire<'a> {
    /// Takes a reference to a server, and a `Timing`, returns you an Expire struct.
    pub fn new(server: &'a Server, timing: Timing) -> Expire {
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
                if *suspect + self.timing.suspicion_timeout_duration() > now {
                    expired_list.push(String::from(id));
                    self.server.member_list.insert_health_by_id(id, Health::Confirmed);
                    self.server.member_list.with_member(id, |has_member| {
                        let member = has_member.expect("Member does not exist when expiring it");
                        debug!("Marking {:?} as Confirmed", member);
                        trace_swim!(&self.server,
                                    "probe-marked-confirmed",
                                    member.get_address(),
                                    None);
                    });
                }
            });
            for mid in expired_list.iter() {
                self.server.member_list.expire(mid);
                self.server.rumor_list.insert(RumorKey::new("member", mid.clone()));
            }
            thread::sleep(Duration::from_millis(500));
        }
    }
}
