// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use time::{SteadyTime, Duration as TimeDuration};

/// How long to wait for an Ack after we ping
const PING_TIMING_DEFAULT_MS: i64 = 1000;
/// How long to wait for an Ack after we PingReq - should be at least 2x the PING_TIMING_DEFAULT_MS
const PINGREQ_TIMING_DEFAULT_MS: i64 = 2100;
/// How many protocol periods before a suspect member is marked as confirmed.
const SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS: i64 = 3;
/// How long is the gossip period
const GOSSIP_PERIOD_DEFAULT_MS: i64 = 1000;

/// The timing of the outbound threads.
#[derive(Debug, Clone)]
pub struct Timing {
    pub ping_ms: i64,
    pub pingreq_ms: i64,
    pub gossip_period_ms: i64,
    pub suspicion_timeout_protocol_periods: i64,
}

impl Default for Timing {
    fn default() -> Timing {
        Timing {
            ping_ms: PING_TIMING_DEFAULT_MS,
            pingreq_ms: PINGREQ_TIMING_DEFAULT_MS,
            gossip_period_ms: GOSSIP_PERIOD_DEFAULT_MS,
            suspicion_timeout_protocol_periods: SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS,
        }
    }
}

impl Timing {
    /// Set up a new Timing
    pub fn new(ping_ms: i64,
               pingreq_ms: i64,
               gossip_period_ms: i64,
               suspicion_timeout_protocol_periods: i64)
               -> Timing {
        Timing {
            ping_ms: ping_ms,
            pingreq_ms: pingreq_ms,
            gossip_period_ms: gossip_period_ms,
            suspicion_timeout_protocol_periods: suspicion_timeout_protocol_periods,
        }
    }

    /// When should this gossip period expire
    pub fn gossip_timeout(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.gossip_period_ms)
    }

    /// How long is a protocl period, in millis.
    pub fn protocol_period_ms(&self) -> i64 {
        self.ping_ms + self.pingreq_ms
    }

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
        TimeDuration::milliseconds(self.protocol_period_ms() *
                                   self.suspicion_timeout_protocol_periods)
    }
}
