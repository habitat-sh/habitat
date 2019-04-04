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

use std::{thread,
          time::Duration};

use crate::{rumor::{RumorKey,
                    RumorType},
            server::{timing::Timing,
                     Server}};

const LOOP_DELAY_MS: u64 = 500;

pub struct Expire {
    pub server: Server,
    pub timing: Timing,
}

impl Expire {
    pub fn new(server: Server, timing: Timing) -> Expire { Expire { server, timing } }

    pub fn run(&self) {
        loop {
            warn!("{} top of loop",
                  thread::current().name().unwrap_or_default());
            let newly_confirmed_members =
                self.server
                    .member_list
                    .members_expired_to_confirmed(self.timing.suspicion_timeout_duration());
            warn!("{} members_expired_to_confirmed({} ms) => {:?}",
                  thread::current().name().unwrap_or_default(),
                  self.timing.suspicion_timeout_duration().num_milliseconds(),
                  newly_confirmed_members);

            for id in newly_confirmed_members {
                self.server
                    .rumor_heat
                    .start_hot_rumor(RumorKey::new(RumorType::Member, &id, ""));
            }

            warn!("{} calling members_expired_to_departed",
                  thread::current().name().unwrap_or_default());

            let newly_departed_members =
                self.server
                    .member_list
                    .members_expired_to_departed(self.timing.departure_timeout_duration());

            warn!("{} members_expired_to_departed({} ms) => {:?}",
                  thread::current().name().unwrap_or_default(),
                  self.timing.departure_timeout_duration().num_milliseconds(),
                  newly_departed_members);

            for id in newly_departed_members {
                self.server.rumor_heat.purge(&id);
                self.server
                    .rumor_heat
                    .start_hot_rumor(RumorKey::new(RumorType::Member, &id, ""));
            }

            warn!("{} sleeping for {} ms",
                  thread::current().name().unwrap_or_default(),
                  LOOP_DELAY_MS);
            thread::sleep(Duration::from_millis(LOOP_DELAY_MS));
        }
    }
}
