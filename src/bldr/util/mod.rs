//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

pub mod http;
pub mod gpg;
pub mod perm;
pub mod sys;
pub mod signals;

use time;

/// Gives us a time to stop for in seconds.
pub fn stop_time(duration: i64) -> time::Timespec {
    let current_time = time::now_utc().to_timespec();
    let wait_duration = time::Duration::seconds(duration as i64);
    let stop_time = current_time + wait_duration;
    stop_time
}
