use std::{thread,
          time::{Duration,
                 Instant}};

/// How long to wait for an Ack after we ping
const PING_TIMING_DEFAULT_MS: u64 = 1000;
/// How long to wait for an Ack after we PingReq - should be at least 2x the PING_TIMING_DEFAULT_MS
const PINGREQ_TIMING_DEFAULT_MS: u64 = 2100;
/// How many protocol periods before a suspect member is marked as confirmed.
const SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS: u64 = 3;
/// How long to wait between each time we send rumors out.
const GOSSIP_INTERVAL_DEFAULT_MS: u64 = 1000;
/// How long before we set a confirmed member to a departed member, removing them from quorums
///   just for your own sanity - this is 3 days.
const DEPARTURE_TIMEOUT_DEFAULT_MS: u64 = 259_200_000;

/// Collects important timing durations and timekeeping activities for
/// the underlying gossip protocols.
#[derive(Debug, Clone)]
pub struct Timing {
    ping:      Duration,
    pingreq:   Duration,
    confirm:   Duration,
    departure: Duration,

    gossip_interval:     Duration,
    swim_probe_interval: Duration,
}

impl Default for Timing {
    fn default() -> Timing {
        let swim_interval_ms = PING_TIMING_DEFAULT_MS + PINGREQ_TIMING_DEFAULT_MS;
        let confirm_ms = swim_interval_ms * SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS;

        Timing { ping:                Duration::from_millis(PING_TIMING_DEFAULT_MS),
                 pingreq:             Duration::from_millis(PINGREQ_TIMING_DEFAULT_MS),
                 confirm:             Duration::from_millis(confirm_ms),
                 departure:           Duration::from_millis(DEPARTURE_TIMEOUT_DEFAULT_MS),
                 gossip_interval:     Duration::from_millis(GOSSIP_INTERVAL_DEFAULT_MS),
                 swim_probe_interval: Duration::from_millis(swim_interval_ms), }
    }
}

impl Timing {
    /// How long a ping has to timeout.
    pub fn ping(&self) -> Duration { self.ping }

    /// How long a pingreq has to timeout.
    pub fn pingreq(&self) -> Duration { self.pingreq }

    /// How long after not hearing from a suspect member before we
    /// consider it confirmed.
    pub fn confirm(&self) -> Duration { self.confirm }

    /// How long after not hearing from a confirmed member before we
    /// consider it departed.
    pub fn departure(&self) -> Duration { self.departure }

    /// If the amount of time since `starting_point` is less than a
    /// gossip interval, sleep for the remainder of that gossip interval.
    pub fn sleep_for_remaining_gossip_interval(&self, starting_point: Instant) {
        maybe_sleep(starting_point, self.gossip_interval)
    }

    /// If the amount of time since `starting_point` is less than a
    /// SWIM protocol probe interval, sleep for the remainder of that
    /// interval.
    pub fn sleep_for_remaining_swim_protocol_interval(&self, starting_point: Instant) {
        maybe_sleep(starting_point, self.swim_probe_interval)
    }
}

/// If the amount of time elapsed from `start` is less than `timeout`,
/// sleep for the difference.
fn maybe_sleep(start: Instant, timeout: Duration) {
    if let Some(amount) = timeout.checked_sub(start.elapsed()) {
        thread::sleep(amount)
    }
}
