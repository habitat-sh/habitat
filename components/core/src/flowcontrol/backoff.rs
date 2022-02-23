use std::time::Duration;

use rand::{distributions::Uniform,
           prelude::Distribution,
           thread_rng};
use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct Backoff {
    base_backoff: Duration,
    max_backoff:  Duration,
    multiplier:   f64,
    last_attempt: Option<RetryAttempt>,
}

#[derive(Debug, Clone)]
struct RetryAttempt {
    attempted_at:   Instant,
    sleep_duration: Duration,
}

impl Default for Backoff {
    fn default() -> Self {
        Self { base_backoff: Duration::from_secs(10),
               max_backoff:  Duration::from_secs(300),
               multiplier:   3f64,
               last_attempt: None, }
    }
}

impl Backoff {
    pub fn new(base_backoff: Duration, max_backoff: Duration, multiplier: f64) -> Backoff {
        Backoff { base_backoff,
                  max_backoff,
                  multiplier,
                  last_attempt: None }
    }

    // Record the completion of attempting an operation
    pub fn record_attempt(&mut self) -> Option<Duration> {
        match &self.last_attempt {
            Some(last_attempt)
                if Instant::now().duration_since(last_attempt.attempted_at)
                   >= last_attempt.sleep_duration =>
            {
                let distribution = Uniform::new(self.base_backoff,
                                                last_attempt.sleep_duration
                                                            .mul_f64(self.multiplier));
                let mut rng = thread_rng();
                let new_sleep_duration = self.max_backoff.min(distribution.sample(&mut rng));
                self.last_attempt = Some(RetryAttempt { attempted_at:   Instant::now(),
                                                        sleep_duration: new_sleep_duration, });
                Some(new_sleep_duration)
            }
            // Otherwise we simply have to wait until the next attempt
            Some(_) => None,
            None => {
                self.last_attempt = Some(RetryAttempt { attempted_at:   Instant::now(),
                                                        sleep_duration: self.base_backoff, });
                Some(self.base_backoff)
            }
        }
    }

    // Check if we should try the next attempt of the operation
    pub fn duration_until_next_attempt(&self) -> Option<Duration> {
        match &self.last_attempt {
            // If the sleep duration has elasped we can make a new attempt
            Some(last_attempt)
                if Instant::now().duration_since(last_attempt.attempted_at)
                   >= last_attempt.sleep_duration =>
            {
                None
            }
            // Otherwise we simply have to wait until the next attempt
            Some(last_attempt) => {
                let next_attempt = last_attempt.attempted_at + last_attempt.sleep_duration;
                next_attempt.checked_duration_since(Instant::now())
            }
            // If we don't have a last attempt, this is our first attempt
            None => None,
        }
    }
}
