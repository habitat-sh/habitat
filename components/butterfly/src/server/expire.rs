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

        let newly_confirmed_members = server.member_list
                                            .members_expired_to_confirmed_mlw(timing.confirm());

        for id in newly_confirmed_members {
            server.rumor_heat
                  .lock_rhw()
                  .start_hot_rumor(RumorKey::new(RumorType::Member, &id, ""));
        }

        let newly_departed_members = server.member_list
                                           .members_expired_to_departed_mlw(timing.departure());

        for id in newly_departed_members {
            server.rumor_heat.lock_rhw().purge(&id);
            server.rumor_heat
                  .lock_rhw()
                  .start_hot_rumor(RumorKey::new(RumorType::Member, &id, ""));
        }

        server.clear_if_partitioned_mlw_rhw();

        thread::sleep(Duration::from_millis(LOOP_DELAY_MS));
    }
}
