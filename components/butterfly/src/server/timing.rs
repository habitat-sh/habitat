use std::time::Duration;

/// How long to wait for an Ack after we ping
const PING_TIMING_DEFAULT_MS: u64 = 1000;
/// How long to wait for an Ack after we PingReq - should be at least 2x the PING_TIMING_DEFAULT_MS
const PINGREQ_TIMING_DEFAULT_MS: u64 = 2100;
/// How many protocol periods before a suspect member is marked as confirmed.
const SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS: u64 = 3;
/// How long is the gossip period
const GOSSIP_PERIOD_DEFAULT_MS: u64 = 1000;
/// How long before we set a confirmed member to a departed member, removing them from quorums
///   just for your own sanity - this is 3 days.
const DEPARTURE_TIMEOUT_DEFAULT_MS: u64 = 259_200_000;

/// The timing of the outbound threads.
#[derive(Debug, Clone)]
pub struct Timing {
    ping_ms: u64,
    pingreq_ms: u64,
    gossip_period_ms: u64,
    suspicion_timeout_protocol_periods: u64,
    departure_timeout_ms: u64,
}

impl Default for Timing {
    fn default() -> Timing {
        Timing { ping_ms: PING_TIMING_DEFAULT_MS,
                 pingreq_ms: PINGREQ_TIMING_DEFAULT_MS,
                 gossip_period_ms: GOSSIP_PERIOD_DEFAULT_MS,
                 suspicion_timeout_protocol_periods: SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS,
                 departure_timeout_ms: DEPARTURE_TIMEOUT_DEFAULT_MS, }
    }
}

impl Timing {
    /// How long a gossip period should last.
    pub fn gossip_period(&self) -> Duration { Duration::from_millis(self.gossip_period_ms) }

    /// How long is a protocol period, in millis.
    pub fn protocol_period_ms(&self) -> u64 { self.ping_ms + self.pingreq_ms }

    /// How long a ping has to timeout.
    pub fn ping(&self) -> Duration { Duration::from_millis(self.ping_ms) }

    /// How long a pingreq has to timeout.
    pub fn pingreq(&self) -> Duration { Duration::from_millis(self.pingreq_ms) }

    /// How long before the next scheduled protocol period
    pub fn protocol_period(&self) -> Duration {
        Duration::from_millis(self.ping_ms + self.pingreq_ms)
    }

    /// How long before this suspect entry times out
    pub fn suspicion_timeout_duration(&self) -> Duration {
        Duration::from_millis(self.protocol_period_ms() * self.suspicion_timeout_protocol_periods)
    }

    pub fn departure_timeout_duration(&self) -> Duration {
        Duration::from_millis(self.departure_timeout_ms)
    }
}
