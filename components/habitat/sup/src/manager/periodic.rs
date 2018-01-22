// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

// TODO (CM): Eventually this may move out to a common crate.

use std::thread;
use std::time::Duration;

use time::{SteadyTime, Duration as TimeDuration};

/// Encapsulate logic for carrying out periodic tasks (or at least
/// managing the timing of such).
pub trait Periodic {
    /// When is the next time we should start a new task, given that
    /// we're going to start one right now?
    fn next_period_start(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.update_period())
    }

    /// Given the time we should start the next task, sleep as long as
    /// we need to until that time.
    fn sleep_until(&self, next_period_start: SteadyTime) {
        let time_to_wait = (next_period_start - SteadyTime::now()).num_milliseconds();
        if time_to_wait > 0 {
            thread::sleep(Duration::from_millis(time_to_wait as u64));
        }
    }

    /// Returns the number of milliseconds between tasks.
    fn update_period(&self) -> i64;
}
