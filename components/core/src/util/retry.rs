use std::time::Duration;

use rand::{distributions::Uniform,
           prelude::Distribution,
           thread_rng};
use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct Retry {
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

impl Default for Retry {
    fn default() -> Self {
        Self { base_backoff: Duration::from_secs(1),
               max_backoff:  Duration::from_secs(20),
               multiplier:   3f64,
               last_attempt: None, }
    }
}

impl Retry {
    pub fn new(base_backoff: Duration, max_backoff: Duration, multiplier: f64) -> Retry {
        Retry { base_backoff,
                max_backoff,
                multiplier,
                last_attempt: None }
    }

    // Record the completion of attempting an operation
    pub fn record_attempt(&mut self) {
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
            }
            // Otherwise we simply have to wait until the next attempt
            Some(_) => {}
            None => {
                self.last_attempt = Some(RetryAttempt { attempted_at:   Instant::now(),
                                                        sleep_duration: self.base_backoff, });
            }
        }
    }

    // Check if we should try the next attempt of the operation
    pub fn should_try_next_attempt(&self) -> bool {
        match &self.last_attempt {
            // If the sleep duration has elasped we can make a new attempt
            Some(last_attempt)
                if Instant::now().duration_since(last_attempt.attempted_at)
                   >= last_attempt.sleep_duration =>
            {
                true
            }
            // Otherwise we simply have to wait until the next attempt
            Some(_) => false,
            // If we don't have a last attempt, this is our first attempt
            None => true,
        }
    }
}
