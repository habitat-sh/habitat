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
// Configuration for a Habitat SessionSrv service

use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use std::str::FromStr;

use db;
use hab_core::config::ConfigFile;
use hab_net::config::{DispatcherCfg, GitHubCfg, GitHubOAuth, RouterCfg, RouterAddr, Shards};
use protocol::sharding::{ShardId, SHARD_COUNT};
use toml;

use error::{Error, Result};

pub struct Config {
    /// A GitHub Team identifier for which members will automatically have administration
    /// privileges assigned to their session
    pub github_admin_team: u64,
    /// GitHub team identifiers for builders
    pub github_builder_teams: Vec<u64>,
    /// GitHub team identifiers for build workers
    pub github_build_worker_teams: Vec<u64>,
    /// List of net addresses for routing servers to connect to.
    pub routers: Vec<RouterAddr>,
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
    pub github: GitHubCfg,
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

impl GitHubOAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
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
    /// A GitHub Team identifier for which members will automatically have administration
    /// privileges assigned to their session
    pub github_admin_team: u64,
    /// GitHub team identifiers for builders
    pub github_builder_teams: Vec<u64>,
    /// GitHub team identifiers for build workers
    pub github_build_worker_teams: Vec<u64>,
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub datastore: DataStoreCfg,
    pub github: GitHubCfg,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            github_admin_team: 0,
            github_builder_teams: Vec::default(),
            github_build_worker_teams: Vec::default(),
            shards: (0..SHARD_COUNT).collect(),
            worker_threads: Self::default_worker_count(),
            routers: vec![RouterAddr::default()],
            datastore: DataStoreCfg::default(),
            github: GitHubCfg::default(),
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
            github_admin_team: self.github_admin_team,
            github_builder_teams: self.github_builder_teams,
            github_build_worker_teams: self.github_build_worker_teams,
            shards: self.shards,
            worker_threads: self.worker_threads,
            github: self.github,
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
            database: String::from("builder_sessionsrv"),
            connection_retry_ms: 300,
            connection_timeout_sec: 3600,
            connection_test: false,
            pool_size: db::config::default_pool_size(),
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

        [[routers]]
        host = "1.1.1.1"
        port = 9000

        [datastore]
        host = "1.1.1.1"
        port = 9000
        user = "test"
        database = "test_sessionsrv"
        connection_retry_ms = 500
        connection_timeout_sec = 4800
        connection_test = true
        pool_size = 1

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(config.datastore_connection_url,
                   "postgresql://test@1.1.1.1:9000/test_sessionsrv");
        assert_eq!(config.datastore_connection_retry_ms, 500);
        assert_eq!(config.datastore_connection_timeout,
                   Duration::from_secs(4800));
        assert_eq!(config.datastore_connection_test, true);
        assert_eq!(config.pool_size, 1);
        assert_eq!(config.shards, vec![0]);
        assert_eq!(config.worker_threads, 1);
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(config.github.client_secret,
                   "438223113eeb6e7edf2d2f91a232b72de72b9bdf");
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
