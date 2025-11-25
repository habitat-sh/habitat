use std::time::Duration;

use rand::{Rng,
           rng};
use tokio::time::Instant;

/// A stateful object that can be used to maintain the current state of a backoff operation
#[derive(Debug, Clone)]
pub struct Backoff {
    base_backoff: Duration,
    max_backoff:  Duration,
    multiplier:   f64,
    last_attempt: Option<RetryAttempt>,
}

#[derive(Debug, Clone)]
struct RetryAttempt {
    attempt_started_at: Instant,
    attempt_ended_at:   Option<Instant>,
    sleep_duration:     Duration,
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
    /// Creates a new backoff state.
    /// On the first attempt we wait for the `base_backoff` duration.
    /// On every successive attempt we wait for a random duration between `base_backoff` and
    /// `multiplier` * `previous_backoff_duration` capped at `max_backoff`. This ensures that
    /// once we hit the max backoff duration we still have a good distribution of random waits.
    pub fn new(base_backoff: Duration, max_backoff: Duration, multiplier: f64) -> Backoff {
        Backoff { base_backoff: base_backoff.min(max_backoff),
                  max_backoff,
                  multiplier,
                  last_attempt: None }
    }

    /// Record the start of an attempted operation.
    /// **Calling this function without checking if an attempt is in
    /// progress will essentially end the previous attempt early**
    pub fn record_attempt_start(&mut self) -> Option<Duration> {
        match &self.last_attempt {
            Some(RetryAttempt { sleep_duration, .. }) => {
                let mut rng = rng();
                // We use the decorrelated jitter algorithm mentioned here:
                // https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/
                let new_sleep_duration =
                    self.max_backoff
                        .min(rng.random_range(self.base_backoff
                                              ..=sleep_duration.mul_f64(self.multiplier)));
                self.last_attempt = Some(RetryAttempt { attempt_started_at: Instant::now(),
                                                        attempt_ended_at:   None,
                                                        sleep_duration:     new_sleep_duration, });
                Some(new_sleep_duration)
            }
            None => {
                self.last_attempt = Some(RetryAttempt { attempt_started_at: Instant::now(),
                                                        attempt_ended_at:   None,
                                                        sleep_duration:     self.base_backoff, });
                Some(self.base_backoff)
            }
        }
    }

    /// Record the end of an attempted operation.
    /// **Calling this function without checking if an attempt is in
    /// progress will do nothing**
    pub fn record_attempt_end(&mut self) {
        if let Some(attempt) = &mut self.last_attempt {
            attempt.attempt_ended_at = Some(Instant::now());
        }
    }

    /// Resets the backoff state erasing any attempt information
    pub fn reset(&mut self) { self.last_attempt = None }

    pub fn duration_elapsed_since_last_attempt_started(&self) -> Option<Duration> {
        self.last_attempt
            .as_ref()
            .map(|attempt| attempt.attempt_started_at.elapsed())
    }

    /// Returns the duration elapased is the last attempt. There are several possible scenarios:
    /// - returns None if there was no previous attempt
    /// - returns Some(Duration::from_secs(0)) if an attempt is in progress
    /// - returns Some(Duration) if some time has elapsed since the last attempt
    pub fn duration_elapsed_since_last_attempt_ended(&self) -> Option<Duration> {
        self.last_attempt
            .as_ref()
            .and_then(|attempt| attempt.attempt_ended_at)
            .map(|attempt_ended_at| attempt_ended_at.elapsed())
    }

    /// Get the duration until the next attempt. There are several possible scenarios:
    /// - returns None if no attempts have been made yet, or we are not in the middle of an attempt
    /// - returns Some(Duration) if there is time remaining until the attempt can be ended
    pub fn duration_until_next_attempt_start(&self) -> Option<Duration> {
        match &self.last_attempt {
            // If the sleep duration has elasped we can make a new attempt
            Some(RetryAttempt { attempt_ended_at: Some(_),
                                .. }) => None,
            // There is an attempt in progress
            Some(RetryAttempt { attempt_started_at: instant,
                                sleep_duration,
                                .. }) => sleep_duration.checked_sub(instant.elapsed()),
            // If we don't have a last attempt, this is our first attempt
            None => None,
        }
    }
}
