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

use crate::core::config::ConfigFile;
use crate::protocol::{DEFAULT_CONSUMER_PORT, DEFAULT_PRODUCER_PORT};

use crate::error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub consumer_port: u16,
    pub producer_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            consumer_port: DEFAULT_CONSUMER_PORT,
            producer_port: DEFAULT_PRODUCER_PORT,
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        producer_port = 9000
        consumer_port = 9001
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.producer_port, 9000);
        assert_eq!(config.consumer_port, 9001);
    }
}
