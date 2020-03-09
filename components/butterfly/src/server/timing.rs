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

/// The timing of the outbound threads.
#[derive(Debug, Clone)]
pub struct Timing {
    ping_ms: u64,
    pingreq_ms: u64,
    gossip_interval_ms: u64,
    suspicion_timeout_protocol_periods: u64,
    departure_timeout_ms: u64,
}

impl Default for Timing {
    fn default() -> Timing {
        Timing { ping_ms: PING_TIMING_DEFAULT_MS,
                 pingreq_ms: PINGREQ_TIMING_DEFAULT_MS,
                 gossip_interval_ms: GOSSIP_INTERVAL_DEFAULT_MS,
                 suspicion_timeout_protocol_periods: SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS,
                 departure_timeout_ms: DEPARTURE_TIMEOUT_DEFAULT_MS, }
    }
}

impl Timing {
    /// How long a gossip period should last.
    fn gossip_interval(&self) -> Duration { Duration::from_millis(self.gossip_interval_ms) }

    /// How long is a protocol period, in millis.
    pub fn protocol_period_ms(&self) -> u64 { self.ping_ms + self.pingreq_ms }

    /// How long a ping has to timeout.
    pub fn ping(&self) -> Duration { Duration::from_millis(self.ping_ms) }

    /// How long a pingreq has to timeout.
    pub fn pingreq(&self) -> Duration { Duration::from_millis(self.pingreq_ms) }

    /// How long to space out individual SWIM probe rounds
    fn swim_probe_interval(&self) -> Duration {
        Duration::from_millis(self.ping_ms + self.pingreq_ms)
    }

    /// If the amount of time since `starting_point` is less than a
    /// gossip interval, sleep for the remainder of that gossip interval.
    pub fn sleep_for_remaining_gossip_interval(&self, starting_point: Instant) {
        maybe_sleep(starting_point, self.gossip_interval())
    }

    /// How long before this suspect entry times out
    pub fn suspicion_timeout_duration(&self) -> Duration {
        Duration::from_millis(self.protocol_period_ms() * self.suspicion_timeout_protocol_periods)
    }

    /// If the amount of time since `starting_point` is less than a
    /// SWIM protocol probe interval, sleep for the remainder of that
    /// interval.
    pub fn sleep_for_remaining_swim_protocol_interval(&self, starting_point: Instant) {
        maybe_sleep(starting_point, self.swim_probe_interval())
    }

    pub fn departure_timeout_duration(&self) -> Duration {
        Duration::from_millis(self.departure_timeout_ms)
    }
}

/// If the amount of time elapsed from `start` is less than `timeout`,
/// sleep for the difference.
fn maybe_sleep(start: Instant, timeout: Duration) {
    if let Some(amount) = timeout.checked_sub(start.elapsed()) {
        thread::sleep(amount)
    }
}
