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

use std::fmt;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

pub use core::config::ConfigFile;
use num_cpus;
use protocol::routesrv::DEFAULT_ROUTER_PORT;
use protocol::sharding::{SHARD_COUNT, ShardId};
use toml;

use socket::ToAddrString;

/// Configuration structure for connecting to a Router
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct RouterAddr {
    /// Listening address of command and heartbeat socket
    pub host: IpAddr,
    /// Listening port of command socket
    pub port: u16,
}

impl Default for RouterAddr {
    fn default() -> Self {
        RouterAddr {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: DEFAULT_ROUTER_PORT,
        }
    }
}

impl ToAddrString for RouterAddr {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.host, self.port)
    }
}

impl fmt::Display for RouterAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// Configuration structure for connecting to a Router
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct AppCfg {
    /// Return a list of router addresses.
    #[serde(default = "AppCfg::default_routers")]
    pub routers: Vec<RouterAddr>,
    /// Return a list of shards which this service is hosting.
    ///
    /// A value of `None` indicates that this is not a sharded service.
    #[serde(default = "AppCfg::default_shards")]
    pub shards: Option<Vec<ShardId>>,
    /// Count of Dispatch workers to start and supervise.
    #[serde(default = "AppCfg::default_worker_count")]
    pub worker_count: usize,
}

impl AppCfg {
    pub fn default_routers() -> Vec<RouterAddr> {
        vec![RouterAddr::default()]
    }

    pub fn default_shards() -> Option<Vec<ShardId>> {
        Some((0..SHARD_COUNT).collect())
    }

    /// Default size of Dispatch worker pool.
    pub fn default_worker_count() -> usize {
        num_cpus::get() * 8
    }
}

impl Default for AppCfg {
    fn default() -> Self {
        AppCfg {
            routers: Self::default_routers(),
            shards: Self::default_shards(),
            worker_count: Self::default_worker_count(),
        }
    }
}

impl FromStr for AppCfg {
    type Err = toml::de::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        toml::de::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn config_from_file() {
        let content = r#"
        shards = [0]
        worker_count = 1

        [[routers]]
        host = "1:1:1:1:1:1:1:1"
        port = 9000
        "#;

        let config = AppCfg::from_str(&content).unwrap();
        assert_eq!(config.shards, Some(vec![0]));
        assert_eq!(config.worker_count, 1);
        assert_eq!(&format!("{}", config.routers[0]), "1:1:1:1:1:1:1:1:9000");
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        worker_count = 0
        "#;

        let config = AppCfg::from_str(&content).unwrap();
        assert_eq!(config.worker_count, 0);
    }
}
