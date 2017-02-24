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

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::str::FromStr;

use db;
use hab_core::config::ConfigFile;
use hab_net::config::{DispatcherCfg, RouterAddr, RouterCfg, Shards};
use protocol::sharding::{ShardId, SHARD_COUNT};
use toml;

use error::{Error, Result};

#[derive(Debug)]
pub struct Config {
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    /// Listening net address for command traffic to and from Workers.
    pub worker_command_addr: SocketAddr,
    /// Listening net address for heartbeat traffic from Workers.
    pub worker_heartbeat_addr: SocketAddr,
    /// Publishing net address for job status updates
    pub status_publisher_addr: SocketAddr,
    /// PostgreSQL connection URL
    pub datastore_connection_url: String,
    /// Timing to retry the connection to the data store if it cannot be established
    pub datastore_connection_retry_ms: u64,
    /// How often to cycle a connection from the pool
    pub datastore_connection_timeout: Duration,
    /// If the datastore connection is under test
    pub datastore_connection_test: bool,
    /// Number of database connections to start in pool.
    pub pool_size: u32,
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Cfg::default().into()
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        Ok(cfg)
    }
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(toml: &str) -> Result<Self> {
        let config: Cfg = toml::from_str(toml).unwrap();
        Ok(config.into())
    }
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
struct Cfg {
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
    pub net: NetCfg,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub datastore: DataStoreCfg,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            shards: (0..SHARD_COUNT).collect(),
            worker_threads: Self::default_worker_count(),
            net: NetCfg::default(),
            routers: vec![RouterAddr::default()],
            datastore: DataStoreCfg::default(),
        }
    }
}

impl DispatcherCfg for Cfg {
    fn worker_count(&self) -> usize {
        self.worker_threads
    }
}

impl Into<Config> for Cfg {
    fn into(self) -> Config {
        Config {
            shards: self.shards,
            worker_threads: self.worker_threads,
            routers: self.routers,
            datastore_connection_url: format!("postgresql://{}@{}:{}/{}",
                                              self.datastore.user,
                                              self.datastore.host,
                                              self.datastore.port,
                                              self.datastore.database),
            datastore_connection_retry_ms: self.datastore.connection_retry_ms,
            datastore_connection_timeout:
                Duration::from_secs(self.datastore.connection_timeout_sec),
            datastore_connection_test: self.datastore.connection_test,
            pool_size: self.datastore.pool_size,
            worker_command_addr: SocketAddr::new(self.net.worker_command_listen,
                                                 self.net.worker_command_port),
            worker_heartbeat_addr: SocketAddr::new(self.net.worker_heartbeat_listen,
                                                   self.net.worker_heartbeat_port),
            status_publisher_addr: SocketAddr::new(self.net.publisher_listen,
                                                   self.net.publisher_port),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DataStoreCfg {
    pub host: IpAddr,
    pub port: u16,
    pub user: String,
    pub database: String,
    /// Timing to retry the connection to the data store if it cannot be established
    pub connection_retry_ms: u64,
    /// How often to cycle a connection from the pool
    pub connection_timeout_sec: u64,
    /// If the datastore connection is under test
    pub connection_test: bool,
    /// Number of database connections to start in pool.
    pub pool_size: u32,
}

impl Default for DataStoreCfg {
    fn default() -> Self {
        DataStoreCfg {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 6397,
            user: String::from("hab"),
            database: String::from("builder_jobsrv"),
            connection_retry_ms: 300,
            connection_timeout_sec: 3600,
            connection_test: false,
            pool_size: db::config::default_pool_size(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct NetCfg {
    /// Worker Command socket's listening address
    pub worker_command_listen: IpAddr,
    /// Worker Command socket's port
    pub worker_command_port: u16,
    /// Worker Heartbeat socket's listening address
    pub worker_heartbeat_listen: IpAddr,
    /// Worker Heartbeat socket's port
    pub worker_heartbeat_port: u16,
    /// Publisher socket's listening address
    pub publisher_listen: IpAddr,
    /// Publisher socket's port
    pub publisher_port: u16,
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
        worker_command_listen = "1.1.1.1"
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

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.routers[0]), "1.1.1.1:9000");
        assert_eq!(&format!("{}", config.worker_command_addr), "1.1.1.1:9000");
        assert_eq!(&format!("{}", config.worker_heartbeat_addr), "1.1.1.1:9000");
        assert_eq!(&format!("{}", config.status_publisher_addr), "1.1.1.1:9000");
        assert_eq!(config.datastore_connection_url,
                   "postgresql://test@1.1.1.1:9000/test_jobsrv");
        assert_eq!(config.datastore_connection_retry_ms, 500);
        assert_eq!(config.datastore_connection_timeout,
                   Duration::from_secs(4800));
        assert_eq!(config.datastore_connection_test, true);
        assert_eq!(config.pool_size, 1);
        assert_eq!(config.shards, vec![0]);
        assert_eq!(config.worker_threads, 1);
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        worker_threads = 0
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(config.worker_threads, 0);
    }
}
