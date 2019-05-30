//! Periodically check membership rumors to automatically "time out"
//! `Suspect` rumors to `Confirmed`, and `Confirmed` rumors to
//! `Departed`. Also purge any rumors that have expired.

use crate::server::{timing::Timing,
                    Server};
use chrono::offset::Utc;
use habitat_common::liveliness_checker;
use std::{thread,
          time::Duration};

pub fn spawn_thread(name: String, mut server: Server, timing: Timing) -> std::io::Result<()> {
    habitat_core::env_config_duration!(ExpireThreadSleepMillis, HAB_EXPIRE_THREAD_SLEEP_MS => from_millis, Duration::from_millis(500));
    let sleep_ms: Duration = ExpireThreadSleepMillis::configured_value().into();

    habitat_core::env_config_duration!(ExpireThreadPurgeSecs, HAB_EXPIRE_THREAD_PURGE_SECS => from_secs, Duration::from_secs(60));
    let purge_secs: Duration = ExpireThreadPurgeSecs::configured_value().into();

    thread::Builder::new().name(name)
                          .spawn(move || -> ! {
                              run_loop(&mut server, &timing, sleep_ms, purge_secs)
                          })
                          .map(|_| ())
}

fn run_loop(server: &mut Server, timing: &Timing, sleep_ms: Duration, purge_secs: Duration) -> ! {
    let mut purge_counter = Duration::from_secs(0);

    loop {
        liveliness_checker::mark_thread_alive().and_divergent();

        server.member_list
              .members_expired_to_confirmed_mlw(timing.suspicion_timeout_duration());

        server.member_list
              .members_expired_to_departed_mlw(timing.departure_timeout_duration());

        purge_counter += sleep_ms;

        // Rather than trying to do this potentially expensive operation every loop iteration,
        // let's only do it every once in awhile.
        if purge_counter >= purge_secs {
            trace!("Purge counter {:?} has exceeded purge seconds {:?}. Expired rumors will now \
                    be purged.",
                   purge_counter,
                   purge_secs);
            let now = Utc::now();
            server.purge_expired(now);
            purge_counter = Duration::from_secs(0);
        }

        thread::sleep(sleep_ms);
    }
}
