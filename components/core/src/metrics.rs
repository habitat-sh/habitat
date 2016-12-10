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

use std::fmt;
use statsd::Client;
use env;

// Statsd Application name
pub const APP_NAME: &'static str = "bldr";

// Statsd Listener Address
pub const STATS_ENV: &'static str = "HAB_STATS_ADDR";

// Supported metrics
#[derive(Debug, Clone)]
pub enum Counter {
    SearchPackages,
}

fn statsd_client() -> Option<Client> {
    match env::var(STATS_ENV) {
        Ok(addr) => Some(Client::new(&addr, APP_NAME).unwrap()),
        Err(_) => None,
    }
}

impl Counter {
    pub fn increment(&self) {
        match statsd_client() {
            Some(mut client) => client.incr(&self.to_string()),
            None => (),
        }
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Counter::SearchPackages => "search-packages",
        };

        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod test {
    use super::{MetricsManager, Counter};

    #[test]
    fn display_counter() {
        let expected = r#"search-packages"#;
        let disp = format!("{}", Counter::SearchPackages);
        assert!(disp == expected);
    }

    #[test]
    fn increment_counter() {
        Counter::SearchPackages.increment();
    }
}
