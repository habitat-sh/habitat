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

//! Configuration for a Habitat VaultSrv service

use std::net;

use dbcache::config::DataStoreCfg;
use hab_core::config::{ConfigFile, ParseInto};
use hab_net::config::{DispatcherCfg, RouteAddrs, Shards};
use protocol::sharding::{ShardId, SHARD_COUNT};
use redis;
use toml;

use error::{Error, Result};

pub struct Config {
    /// List of net addresses for routing servers to connect to.
    pub routers: Vec<net::SocketAddrV4>,
    /// Net dddress to the persistent datastore.
    pub datastore_addr: net::SocketAddrV4,
    /// Connection retry timeout in milliseconds for datastore.
    pub datastore_retry_ms: u64,
    /// Number of database connections to start in pool.
    pub pool_size: u32,
    /// Router's hearbeat port to connect to.
    pub heartbeat_port: u16,
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            routers: vec![net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5562)],
            datastore_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 6379),
            datastore_retry_ms: Self::default_connection_retry_ms(),
            pool_size: Self::default_pool_size(),
            heartbeat_port: 5563,
            shards: (0..SHARD_COUNT).collect(),
            worker_threads: Self::default_worker_count(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.routers", &mut cfg.routers));
        try!(toml.parse_into("cfg.datastore_addr", &mut cfg.datastore_addr));
        try!(toml.parse_into("cfg.datastore_retry_ms", &mut cfg.datastore_retry_ms));
        try!(toml.parse_into("cfg.pool_size", &mut cfg.pool_size));
        try!(toml.parse_into("cfg.heartbeat_port", &mut cfg.heartbeat_port));
        try!(toml.parse_into("cfg.shards", &mut cfg.shards));
        try!(toml.parse_into("cfg.worker_threads", &mut cfg.worker_threads));
        Ok(cfg)
    }
}

impl DataStoreCfg for Config {
    fn datastore_addr(&self) -> &net::SocketAddrV4 {
        &self.datastore_addr
    }

    fn connection_retry_ms(&self) -> u64 {
        self.datastore_retry_ms
    }

    fn pool_size(&self) -> u32 {
        self.pool_size
    }
}

impl DispatcherCfg for Config {
    fn worker_count(&self) -> usize {
        self.worker_threads
    }
}

impl RouteAddrs for Config {
    fn route_addrs(&self) -> &Vec<net::SocketAddrV4> {
        &self.routers
    }

    fn heartbeat_port(&self) -> u16 {
        self.heartbeat_port
    }
}

impl Shards for Config {
    fn shards(&self) -> &Vec<u32> {
        &self.shards
    }
}

impl<'a> redis::IntoConnectionInfo for &'a Config {
    fn into_connection_info(self) -> redis::RedisResult<redis::ConnectionInfo> {
        format!("redis://{}:{}",
                self.datastore_addr.ip(),
                self.datastore_addr.port())
            .into_connection_info()
    }
}
