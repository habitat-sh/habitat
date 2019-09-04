//! Periodically check membership rumors to automatically "time out"
//! `Suspect` rumors to `Confirmed`, and `Confirmed` rumors to
//! `Departed`. Also purge any rumors that have expired.

use crate::server::{timing::Timing,
                    Server};
use habitat_common::liveliness_checker;
use std::{thread,
          time::Duration};

pub fn spawn_thread(name: String, mut server: Server, timing: Timing) -> std::io::Result<()> {
    habitat_core::env_config_duration!(ExpireThreadSleepMillis, HAB_EXPIRE_THREAD_SLEEP_MS => from_millis, Duration::from_millis(500));
    let sleep_ms: Duration = ExpireThreadSleepMillis::configured_value().into();

    thread::Builder::new().name(name)
                          .spawn(move || -> ! { run_loop(&mut server, &timing, sleep_ms) })
                          .map(|_| ())
}

fn run_loop(server: &mut Server, timing: &Timing, sleep_ms: Duration) -> ! {
    loop {
        liveliness_checker::mark_thread_alive().and_divergent();

        server.member_list
              .members_expired_to_confirmed_mlw(timing.suspicion_timeout_duration());

        server.member_list
              .members_expired_to_departed_mlw(timing.departure_timeout_duration());

        thread::sleep(sleep_ms);
    }
}
