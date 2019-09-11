//! Periodically check membership rumors to automatically "time out"
//! `Suspect` rumors to `Confirmed`, and `Confirmed` rumors to
//! `Departed`.

use crate::{rumor::{RumorKey,
                    RumorType},
            server::{timing::Timing,
                     Server}};
use habitat_common::liveliness_checker;
use std::{thread,
          time::Duration};

const LOOP_DELAY_MS: u64 = 500;

pub fn spawn_thread(name: String, server: Server, timing: Timing) -> std::io::Result<()> {
    thread::Builder::new().name(name)
                          .spawn(move || -> ! { run_loop(&server, &timing) })
                          .map(|_| ())
}

fn run_loop(server: &Server, timing: &Timing) -> ! {
    loop {
        liveliness_checker::mark_thread_alive().and_divergent();

        let newly_confirmed_members =
            server.member_list
                  .members_expired_to_confirmed_mlw(timing.suspicion_timeout_duration());

        for id in newly_confirmed_members {
            server.rumor_heat
                  .start_hot_rumor_rhw(RumorKey::new(RumorType::Member, &id, ""));
        }

        let newly_departed_members =
            server.member_list
                  .members_expired_to_departed_mlw(timing.departure_timeout_duration());

        for id in newly_departed_members {
            server.rumor_heat.purge_rhw(&id);
            server.rumor_heat
                  .start_hot_rumor_rhw(RumorKey::new(RumorType::Member, &id, ""));
        }

        thread::sleep(Duration::from_millis(LOOP_DELAY_MS));
    }
}
