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

use hab_net::config::{DEFAULT_ROUTER_LISTEN_PORT, DEFAULT_ROUTER_HEARTBEAT_PORT};
use hab_core::config::ConfigFile;
use toml;

use error::{Error, Result};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Listening ip address for client connections
    pub listen: IpAddr,
    /// Port for receiving routable messages from services and gateways
    pub client_port: u16,
    /// Port for receiving service heartbeats
    pub heartbeat_port: u16,
}

impl Config {
    pub fn fe_addr(&self) -> String {
        format!("tcp://{}:{}", self.listen, self.client_port)
    }

    pub fn hb_addr(&self) -> String {
        format!("tcp://{}:{}", self.listen, self.heartbeat_port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            client_port: DEFAULT_ROUTER_LISTEN_PORT,
            heartbeat_port: DEFAULT_ROUTER_HEARTBEAT_PORT,
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
        client_port = 9000
        heartbeat_port = 9001
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.listen), "::1");
        assert_eq!(config.client_port, 9000);
        assert_eq!(config.heartbeat_port, 9001);
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
