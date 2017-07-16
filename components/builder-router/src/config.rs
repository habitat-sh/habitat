// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

//! Configuration for a Habitat RouteSrv service

use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use hab_core::config::ConfigFile;
use protocol::routesrv::DEFAULT_ROUTER_PORT;
use toml;

use error::{Error, Result};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Listening ip address for client connections
    pub listen: IpAddr,
    /// Port for receiving routable messages from services and gateways
    pub port: u16,
}

impl Config {
    pub fn addr(&self) -> String {
        format!("tcp://{}:{}", self.listen, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: DEFAULT_ROUTER_PORT,
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(toml: &str) -> Result<Self> {
        let config: Config = toml::from_str(toml).unwrap();
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn config_from_file() {
        let content = r#"
        listen = "0:0:0:0:0:0:0:1"
        port = 9000
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.listen), "::1");
        assert_eq!(config.port, 9000);
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        listen = "172.18.0.1"
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.listen), "172.18.0.1");
    }
}
