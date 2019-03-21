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
            let newly_confirmed_members =
                self.server
                    .member_list
                    .members_expired_to_confirmed(self.timing.suspicion_timeout_duration());

            for id in newly_confirmed_members {
                self.server
                    .rumor_heat
                    .start_hot_rumor(RumorKey::new(RumorType::Member, &id, ""));
            }

            let newly_departed_members =
                self.server
                    .member_list
                    .members_expired_to_departed(self.timing.departure_timeout_duration());

            for id in newly_departed_members {
                self.server.rumor_heat.purge(&id);
                self.server
                    .rumor_heat
                    .start_hot_rumor(RumorKey::new(RumorType::Member, &id, ""));
            }

            thread::sleep(Duration::from_millis(LOOP_DELAY_MS));
        }
    }
}
