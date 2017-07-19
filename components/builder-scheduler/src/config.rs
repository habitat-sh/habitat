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

use std::path::PathBuf;
use db::config::DataStoreCfg;
use hab_core::url;
use hab_core::config::ConfigFile;
use hab_net::config::{DispatcherCfg, RouterAddr, RouterCfg, Shards};
use protocol::sharding::{ShardId, SHARD_COUNT};

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Token for authenticating with the public builder-api
    pub auth_token: String,
    /// Default URL to determine which Builder Depot to use
    pub depot_url: String,
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
    /// Path to packages on-disk (for migration)
    pub migration_path: String,
    /// Path to scheduler event logs
    pub log_path: PathBuf,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    /// Configuration for the DB to connect to
    pub datastore: DataStoreCfg,
    /// Channel to auto-promote successful groups to
    pub promote_channel: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut datastore = DataStoreCfg::default();
        datastore.database = String::from("builder_scheduler");
        Config {
            auth_token: "".to_string(),
            depot_url: url::default_depot_url(),
            shards: (0..SHARD_COUNT).collect(),
            worker_threads: Self::default_worker_count(),
            migration_path: String::from("/hab/svc/builder-scheduler/pkgs"),
            log_path: PathBuf::from("/tmp"),
            routers: vec![RouterAddr::default()],
            datastore: datastore,
            promote_channel: String::from("unstable"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        auth_token = "mytoken"
        depot_url = "mydepot"
        promote_channel = "foo"
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
        database = "test_scheduler"
        connection_retry_ms = 500
        connection_timeout_sec = 4800
        connection_test = true
        pool_size = 1
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&config.auth_token, "mytoken");
        assert_eq!(&config.depot_url, "mydepot");
        assert_eq!(&config.promote_channel, "foo");
        assert_eq!(config.datastore.port, 9000);
        assert_eq!(config.datastore.user, "test");
        assert_eq!(config.datastore.database, "test_scheduler");
        assert_eq!(config.datastore.connection_retry_ms, 500);
        assert_eq!(config.datastore.connection_timeout_sec, 4800);
        assert_eq!(config.datastore.connection_test, true);
        assert_eq!(config.datastore.pool_size, 1);
        assert_eq!(config.shards, vec![0]);
        assert_eq!(config.worker_threads, 1);
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        worker_threads = 0
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.worker_threads, 0);
        assert_eq!(&config.auth_token, "");
        assert_eq!(&config.promote_channel, "unstable");
    }
}
