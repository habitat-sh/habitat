// TODO (CM): Eventually this may move out to a common crate.

use std::{thread,
          time::Duration as StdDuration};
use time::{Duration,
           SteadyTime};

/// Encapsulate logic for carrying out periodic tasks (or at least
/// managing the timing of such).
pub trait Periodic {
    /// When is the next time we should start a new task, given that
    /// we're going to start one right now?
    fn next_period_start(&self) -> SteadyTime { SteadyTime::now() + self.update_period() }

    /// Given the time we should start the next task, sleep as long as
    /// we need to until that time.
    fn sleep_until(&self, next_period_start: SteadyTime) {
        let time_to_wait = (next_period_start - SteadyTime::now()).num_milliseconds();
        if time_to_wait > 0 {
            thread::sleep(StdDuration::from_millis(time_to_wait as u64));
        }
    }

    /// Returns the amount of time to wait between tasks.
    fn update_period(&self) -> Duration;
}
