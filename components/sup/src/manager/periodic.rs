// TODO (CM): Eventually this may move out to a common crate.

use std::{thread,
          time::{Duration,
                 Instant}};

/// Encapsulate logic for carrying out periodic tasks (or at least
/// managing the timing of such).
pub trait Periodic {
    /// When is the next time we should start a new task, given that
    /// we're going to start one right now?
    fn next_period_start(&self) -> Instant { Instant::now() + self.update_period() }

    /// Given the time we should start the next task, sleep as long as
    /// we need to until that time.
    fn sleep_until(&self, next_period_start: Instant) {
        if let Some(time_to_wait) = Self::time_to_wait(Instant::now(), next_period_start) {
            thread::sleep(time_to_wait);
        }
    }

    /// Given the time we should start the next task, determine if we need to sleep,
    /// and if so for how long.
    fn time_to_wait(now: Instant, next_period_start: Instant) -> Option<Duration> {
        if next_period_start > now {
            Some(next_period_start - now)
        } else {
            None
        }
    }

    /// Returns the amount of time to wait between tasks.
    fn update_period(&self) -> Duration;
}

#[cfg(test)]
mod test {
    use super::*;

    struct P {}
    impl Periodic for P {
        fn update_period(&self) -> Duration { unimplemented!() }
    }

    #[test]
    fn time_to_wait_is_some_when_next_period_start_is_in_the_future() {
        let now = Instant::now();
        let time_until_next_period = Duration::from_secs(1);
        let next_period_start = now + time_until_next_period;
        assert_eq!(P::time_to_wait(now, next_period_start),
                   Some(time_until_next_period));
    }

    #[test]
    fn time_to_wait_is_none_when_next_period_start_is_in_the_past() {
        let now = Instant::now();
        let time_until_next_period = Duration::from_secs(1);
        let next_period_start = now - time_until_next_period;
        assert_eq!(P::time_to_wait(now, next_period_start), None);
    }

    #[test]
    fn time_to_wait_is_none_when_next_period_start_is_now() {
        let now = Instant::now();
        assert_eq!(P::time_to_wait(now, now), None);
    }
}
