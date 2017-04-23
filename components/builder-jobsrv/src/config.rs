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

//! Configuration for a Habitat JobSrv service

use std::net::{IpAddr, Ipv4Addr};

use db::config::DataStoreCfg;
use hab_core::config::ConfigFile;
use hab_net::config::{DispatcherCfg, RouterAddr, RouterCfg, Shards};
use protocol::sharding::{ShardId, SHARD_COUNT};

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
    pub net: NetCfg,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub datastore: DataStoreCfg,
}

impl Default for Config {
    fn default() -> Self {
        let mut datastore = DataStoreCfg::default();
        datastore.database = String::from("builder_jobsrv");
        Config {
            shards: (0..SHARD_COUNT).collect(),
            worker_threads: Self::default_worker_count(),
            net: NetCfg::default(),
            routers: vec![RouterAddr::default()],
            datastore: datastore,
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

impl DispatcherCfg for Config {
    fn worker_count(&self) -> usize {
        self.worker_threads
    }
}

impl RouterCfg for Config {
    fn route_addrs(&self) -> &Vec<RouterAddr> {
        &self.routers
    }
}

impl Shards for Config {
    fn shards(&self) -> &Vec<u32> {
        &self.shards
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct NetCfg {
    /// Publisher socket's listening address
    pub publisher_listen: IpAddr,
    /// Publisher socket's port
    pub publisher_port: u16,
    /// Worker Command socket's listening address
    pub worker_command_listen: IpAddr,
    /// Worker Command socket's port
    pub worker_command_port: u16,
    /// Worker Heartbeat socket's listening address
    pub worker_heartbeat_listen: IpAddr,
    /// Worker Heartbeat socket's port
    pub worker_heartbeat_port: u16,
}

impl NetCfg {
    pub fn publisher_addr(&self) -> String {
        format!("tcp://{}:{}", self.publisher_listen, self.publisher_port)
    }

    pub fn worker_command_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.worker_command_listen,
                self.worker_command_port)
    }

    pub fn worker_heartbeat_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.worker_heartbeat_listen,
                self.worker_heartbeat_port)
    }
}

impl Default for NetCfg {
    fn default() -> Self {
        NetCfg {
            worker_command_listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            worker_command_port: 5566,
            worker_heartbeat_listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            worker_heartbeat_port: 5567,
            publisher_listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            publisher_port: 5568,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        shards = [
            0
        ]
        worker_threads = 1

        [net]
        worker_command_listen = "1:1:1:1:1:1:1:1"
        worker_command_port = 9000
        worker_heartbeat_listen = "1.1.1.1"
        worker_heartbeat_port = 9000
        publisher_listen = "1.1.1.1"
        publisher_port = 9000

        [[routers]]
        host = "1.1.1.1"
        port = 9000

        [datastore]
        host = "1.1.1.1"
        port = 9000
        user = "test"
        database = "test_jobsrv"
        connection_retry_ms = 500
        connection_timeout_sec = 4800
        connection_test = true
        pool_size = 1
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.routers[0]), "1.1.1.1:9000");
        assert_eq!(&format!("{}", config.net.worker_command_listen),
                   "1:1:1:1:1:1:1:1");
        assert_eq!(&format!("{}", config.net.worker_heartbeat_listen),
                   "1.1.1.1");
        assert_eq!(&format!("{}", config.net.publisher_listen), "1.1.1.1");
        assert_eq!(config.net.worker_command_port, 9000);
        assert_eq!(config.net.worker_heartbeat_port, 9000);
        assert_eq!(config.net.publisher_port, 9000);
        assert_eq!(config.shards, vec![0]);
        assert_eq!(config.worker_threads, 1);
        assert_eq!(config.datastore.port, 9000);
        assert_eq!(config.datastore.user, "test");
        assert_eq!(config.datastore.database, "test_jobsrv");
        assert_eq!(config.datastore.connection_retry_ms, 500);
        assert_eq!(config.datastore.connection_timeout_sec, 4800);
        assert_eq!(config.datastore.connection_test, true);
        assert_eq!(config.datastore.pool_size, 1);
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        worker_threads = 0
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.worker_threads, 0);
    }
}
