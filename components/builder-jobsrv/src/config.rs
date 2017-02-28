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

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;

use db;
use hab_core::config::{ConfigFile, ParseInto};
use hab_net::config::{DispatcherCfg, RouteAddrs, Shards};
use protocol::sharding::{ShardId, SHARD_COUNT};
use toml;

use error::{Error, Result};

#[derive(Debug)]
pub struct Config {
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<SocketAddr>,
    /// Listening net address for command traffic to and from Workers.
    pub worker_command_addr: SocketAddr,
    /// Listening net address for heartbeat traffic from Workers.
    pub worker_heartbeat_addr: SocketAddr,
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
    /// Router's heartbeat port to connect to.
    pub heartbeat_port: u16,
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            routers: vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5562))],
            worker_command_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 5566)),
            worker_heartbeat_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0),
                                                                    5567)),
            datastore_connection_url: String::from("postgresql:://hab@127.0.0.1/builder_db_test"),
            datastore_connection_retry_ms: 300,
            datastore_connection_timeout: Duration::from_secs(3600),
            datastore_connection_test: false,
            pool_size: db::config::default_pool_size(),
            shards: (0..SHARD_COUNT).collect(),
            heartbeat_port: 5563,
            worker_threads: Self::default_worker_count(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.routers", &mut cfg.routers));
        try!(toml.parse_into("cfg.worker_command_addr", &mut cfg.worker_command_addr));
        try!(toml.parse_into("cfg.worker_heartbeat_addr", &mut cfg.worker_heartbeat_addr));
        let mut connection_user = String::new();
        try!(toml.parse_into("cfg.datastore_connection_user", &mut connection_user));
        let mut connection_address = String::new();
        try!(toml.parse_into("cfg.datastore_connection_address", &mut connection_address));
        let mut connection_db = String::new();
        try!(toml.parse_into("cfg.datastore_connection_db", &mut connection_db));

        cfg.datastore_connection_url = format!("postgresql://{}@{}/{}",
                                               connection_user,
                                               connection_address,
                                               connection_db);
        try!(toml.parse_into("cfg.datastore_connection_retry_ms",
                             &mut cfg.datastore_connection_retry_ms));
        let mut timeout_seconds = 3600;
        try!(toml.parse_into("cfg.datastore_connection_timeout", &mut timeout_seconds));
        cfg.datastore_connection_timeout = Duration::from_secs(timeout_seconds);
        try!(toml.parse_into("cfg.pool_size", &mut cfg.pool_size));
        try!(toml.parse_into("cfg.heartbeat_port", &mut cfg.heartbeat_port));
        try!(toml.parse_into("cfg.shards", &mut cfg.shards));
        try!(toml.parse_into("cfg.worker_threads", &mut cfg.worker_threads));
        Ok(cfg)
    }
}

impl DispatcherCfg for Config {
    fn worker_count(&self) -> usize {
        self.worker_threads
    }
}

impl RouteAddrs for Config {
    fn route_addrs(&self) -> &Vec<SocketAddr> {
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
