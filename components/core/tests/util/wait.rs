// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate time;
extern crate habitat_core as hcore;

// call a closure in a loop until it returns Ok(T),
// or the 30 second timeout
pub fn wait_until_ok<F,T>(some_fn: F) -> Option<T>
    where F: Fn() -> Result<T, hcore::error::Error>
{
    let wait_duration = time::Duration::seconds(30);
    let current_time = time::now_utc().to_timespec();
    let stop_time = current_time + wait_duration;
    while time::now_utc().to_timespec() < stop_time {
        if let Ok(s) = some_fn() {
            return Some(s);
        }
    }
    None
}


