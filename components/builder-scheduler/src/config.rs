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

//! Configuration for a Habitat Scheduler service

use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;

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
    /// List of Job Servers to subscribe for status
    pub job_servers: JobSrvCfg,
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
    /// Path to packages on-disk (for migration)
    pub migration_path: String,
}

impl Config {
    pub fn jobsrv_addrs(&self) -> Vec<String> {
        let mut addrs = vec![];
        for job_server in &self.job_servers {
            let addr = format!("tcp://{}:{}", job_server.host, job_server.port);
            addrs.push(addr);
        }
        addrs
    }
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
    /// List of Job Servers to subscribe for status
    pub jobsrv: JobSrvCfg,
    /// Path to packages on-disk (for migration)
    pub migration_path: String,
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub datastore: DataStoreCfg,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            jobsrv: vec![JobSrvAddr::default()],
            migration_path: String::from("/hab/svc/hab-builder-scheduler/pkgs"),
            shards: (0..SHARD_COUNT).collect(),
            worker_threads: Self::default_worker_count(),
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
            job_servers: self.jobsrv,
            migration_path: self.migration_path,
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
            database: String::from("builder_scheduler"),
            connection_retry_ms: 300,
            connection_timeout_sec: 3600,
            connection_test: false,
            pool_size: db::config::default_pool_size(),
        }
    }
}

pub type JobSrvCfg = Vec<JobSrvAddr>;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct JobSrvAddr {
    pub host: IpAddr,
    pub port: u16,
}

impl Default for JobSrvAddr {
    fn default() -> Self {
        JobSrvAddr {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5568,
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

        [[jobsrv]]
        host = "1.1.1.1"
        port = 9000

        [[jobsrv]]
        host = "2.2.2.2"
        port = 9000

        [[routers]]
        host = "1.1.1.1"
        port = 9000

        [datastore]
        host = "1.1.1.1"
        port = 9000
        user = "test"
        database = "test_scheduler"
        connection_retry_ms = 500
        connection_timeout_sec = 4800
        connection_test = true
        pool_size = 1
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.job_servers[0].host), "1.1.1.1");
        assert_eq!(config.job_servers[0].port, 9000);
        assert_eq!(&format!("{}", config.job_servers[1].host), "2.2.2.2");
        assert_eq!(config.job_servers[1].port, 9000);
        assert_eq!(config.datastore_connection_url,
                   "postgresql://test@1.1.1.1:9000/test_scheduler");
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
