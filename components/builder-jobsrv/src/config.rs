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

//! Configuration for a Habitat JobSrv service

use std::net;

use hab_core::config::{ConfigFile, ParseInto};
use hab_net::config::{RouteAddrs, Shards};
use num_cpus;
use protocol::sharding::{ShardId, SHARD_COUNT};
use redis;
use toml;

use error::{Error, Result};

pub struct Config {
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<net::SocketAddrV4>,
    /// Listening net address for command traffic to and from Workers.
    pub worker_command_addr: net::SocketAddrV4,
    /// Listening net address for heartbeat traffic from Workers.
    pub worker_heartbeat_addr: net::SocketAddrV4,
    /// Net dddress to the persistent datastore.
    pub datastore_addr: net::SocketAddrV4,
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Router's hearbeat port to connect to.
    pub heartbeat_port: u16,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            routers: vec![net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5562)],
            worker_command_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 5566),
            worker_heartbeat_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 5567),
            datastore_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 6379),
            shards: (0..SHARD_COUNT).collect(),
            heartbeat_port: 5563,
            worker_threads: num_cpus::get(),
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
        try!(toml.parse_into("cfg.datastore_addr", &mut cfg.datastore_addr));
        try!(toml.parse_into("cfg.shards", &mut cfg.shards));
        try!(toml.parse_into("cfg.heartbeat_port", &mut cfg.heartbeat_port));
        Ok(cfg)
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
