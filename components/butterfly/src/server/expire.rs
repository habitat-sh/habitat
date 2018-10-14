// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Periodically check membership rumors to automatically "time out"
//! `Suspect` rumors to `Confirmed`, and `Confirmed` rumors to
//! `Departed`.

use std::thread;
use std::time::Duration;

use rumor::{RumorKey, RumorType};
use server::timing::Timing;
use server::Server;

const LOOP_DELAY_MS: u64 = 500;

pub struct Expire {
    pub server: Server,
    pub timing: Timing,
}

impl Expire {
    pub fn new(server: Server, timing: Timing) -> Expire {
        Expire {
            server: server,
            timing: timing,
        }
    }

    pub fn run(&self) {
        loop {
            let newly_confirmed_members = self
                .server
                .member_list
                .members_expired_to_confirmed(self.timing.suspicion_timeout_duration());

            for id in newly_confirmed_members {
                self.server
                    .rumor_heat
                    .start_hot_rumor(RumorKey::new(RumorType::Member, id, ""));
            }

            let newly_departed_members = self
                .server
                .member_list
                .members_expired_to_departed(self.timing.departure_timeout_duration());

            for id in newly_departed_members {
                // Purge "heat" information for a member that's gone
                self.server.rumor_heat.purge(&id);
                self.server
                    .rumor_heat
                    .start_hot_rumor(RumorKey::new(RumorType::Member, id, ""));
            }

            thread::sleep(Duration::from_millis(LOOP_DELAY_MS));
        }
    }
}
