use time::{Duration as TimeDuration,
           SteadyTime};

/// How long to wait for an Ack after we ping
const PING_TIMING_DEFAULT_MS: i64 = 1000;
/// How long to wait for an Ack after we PingReq - should be at least 2x the PING_TIMING_DEFAULT_MS
const PINGREQ_TIMING_DEFAULT_MS: i64 = 2100;
/// How many protocol periods before a suspect member is marked as confirmed.
const SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS: i64 = 3;
/// How long is the gossip period
const GOSSIP_PERIOD_DEFAULT_MS: i64 = 1000;
/// How long before we set a confirmed member to a departed member, removing them from quorums
///   just for your own sanity - this is 3 days.
const DEPARTURE_TIMEOUT_DEFAULT_MS: i64 = 259_200_000;

/// The timing of the outbound threads.
#[derive(Debug, Clone)]
pub struct Timing {
    pub ping_ms: i64,
    pub pingreq_ms: i64,
    pub gossip_period_ms: i64,
    pub suspicion_timeout_protocol_periods: i64,
    pub departure_timeout_ms: i64,
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
    /// When should this gossip period expire
    pub fn gossip_timeout(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.gossip_period_ms)
    }

    /// How long is a protocol period, in millis.
    pub fn protocol_period_ms(&self) -> i64 { self.ping_ms + self.pingreq_ms }

    /// When should this ping record time out?
    pub fn ping_timeout(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.ping_ms)
    }

    /// When should this pingreq timeout?
    pub fn pingreq_timeout(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.pingreq_ms)
    }

    /// How long before the next scheduled protocol period
    pub fn next_protocol_period(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.ping_ms + self.pingreq_ms)
    }

    /// How long before this suspect entry times out
    pub fn suspicion_timeout_duration(&self) -> TimeDuration {
        habitat_core::env_config_int!(ConfirmedTimeoutMillis, i64, HAB_CONFIRMED_TIMEOUT_MS, -1);
        let mut confirmed_timeout_ms: i64 = ConfirmedTimeoutMillis::configured_value().into();
        if confirmed_timeout_ms == -1 {
            confirmed_timeout_ms =
                self.protocol_period_ms() * self.suspicion_timeout_protocol_periods;
        }
        TimeDuration::milliseconds(confirmed_timeout_ms)
    }

    pub fn departure_timeout_duration(&self) -> TimeDuration {
        habitat_core::env_config_int!(DepartureTimeoutMillis,
                                      i64,
                                      HAB_DEPARTURE_TIMEOUT_MS,
                                      DEPARTURE_TIMEOUT_DEFAULT_MS);
        let departure_timeout_ms: i64 = DepartureTimeoutMillis::configured_value().into();
        TimeDuration::milliseconds(departure_timeout_ms)
    }
}
