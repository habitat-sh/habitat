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
    /// Path to packages on-disk (for migration)
    pub migration_path: String,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    /// List of Job Servers to subscribe for status
    pub jobsrv: JobSrvCfg,
    pub datastore: DataStoreCfg,
}

impl Config {
    pub fn jobsrv_addrs(&self) -> Vec<String> {
        let mut addrs = vec![];
        for job_server in &self.jobsrv {
            let addr = format!("tcp://{}:{}", job_server.host, job_server.port);
            addrs.push(addr);
        }
        addrs
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut datastore = DataStoreCfg::default();
        datastore.database = String::from("builder_scheduler");
        Config {
            shards: (0..SHARD_COUNT).collect(),
            worker_threads: Self::default_worker_count(),
            migration_path: String::from("/hab/svc/builder-scheduler/pkgs"),
            routers: vec![RouterAddr::default()],
            jobsrv: vec![JobSrvAddr::default()],
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
        host = "1:1:1:1:1:1:1:1"
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

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.jobsrv[0].host), "1:1:1:1:1:1:1:1");
        assert_eq!(config.jobsrv[0].port, 9000);
        assert_eq!(&format!("{}", config.jobsrv[1].host), "2.2.2.2");
        assert_eq!(config.jobsrv[1].port, 9000);
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
    }
}
