//! Periodically check to see if any rumors need to be purged, and if so, purge them.

use crate::server::Server;
use chrono::offset::Utc;
use habitat_common::liveliness_checker;
use std::{thread,
          time::Duration};

pub fn spawn_thread(name: String, mut server: Server) -> std::io::Result<()> {
    habitat_core::env_config_duration!(RumorExpireThreadSleepMillis, HAB_RUMOR_EXPIRE_THREAD_SLEEP_MS => from_millis, Duration::from_millis(1000));
    let sleep_ms: Duration = RumorExpireThreadSleepMillis::configured_value().into();

    thread::Builder::new().name(name)
                          .spawn(move || -> ! { run_loop(&mut server, sleep_ms) })
                          .map(|_| ())
}

fn run_loop(server: &mut Server, sleep_ms: Duration) -> ! {
    loop {
        liveliness_checker::mark_thread_alive().and_divergent();
        trace!("Expired rumors will now be purged.");
        let now = Utc::now();
        server.purge_expired(now);
        thread::sleep(sleep_ms);
    }
}
