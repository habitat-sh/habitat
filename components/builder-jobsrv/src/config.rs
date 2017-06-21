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
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

use db::config::DataStoreCfg;
use hab_core::config::ConfigFile;
use hab_net::config::{DispatcherCfg, RouterAddr, RouterCfg, Shards};
use protocol::sharding::{ShardId, SHARD_COUNT};
use server::log_archiver::ArchiveBackend;

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

    /// Directory to which log output of running build processes will
    /// be written. Defaults to the system temp directory. Must exist
    /// and be writable by the server process.
    pub log_dir: PathBuf,

    /// Configuration for the job log archiver
    pub archive: ArchiveCfg,
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
            log_dir: env::temp_dir(),
            archive: ArchiveCfg::default(),
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
    /// Worker Command socket's listening address
    pub worker_command_listen: IpAddr,
    /// Worker Command socket's port
    pub worker_command_port: u16,
    /// Worker Heartbeat socket's listening address
    pub worker_heartbeat_listen: IpAddr,
    /// Worker Heartbeat socket's port
    pub worker_heartbeat_port: u16,
    /// Worker Log Ingestion socket's listening address
    pub log_ingestion_listen: IpAddr,
    /// Worker Log Ingestion socket's port
    pub log_ingestion_port: u16,
}

impl NetCfg {
    pub fn worker_command_addr(&self) -> String {
        format!(
            "tcp://{}:{}",
            self.worker_command_listen,
            self.worker_command_port
        )
    }

    pub fn worker_heartbeat_addr(&self) -> String {
        format!(
            "tcp://{}:{}",
            self.worker_heartbeat_listen,
            self.worker_heartbeat_port
        )
    }

    pub fn log_ingestion_addr(&self) -> String {
        format!(
            "tcp://{}:{}",
            self.log_ingestion_listen,
            self.log_ingestion_port
        )
    }
}

impl Default for NetCfg {
    fn default() -> Self {
        NetCfg {
            worker_command_listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            worker_command_port: 5566,
            worker_heartbeat_listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            worker_heartbeat_port: 5567,
            log_ingestion_listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            log_ingestion_port: 5568,
        }
    }
}

////////////////////////////////////////////////////////////////////////
// Archive Configuration

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ArchiveCfg {
    pub backend: Option<ArchiveBackend>,

    // These are for S3 archiving
    pub key: Option<String>,
    pub secret: Option<String>,
    pub endpoint: Option<String>,
    pub bucket: Option<String>,
    pub region: String,

    // These are for local log archiving
    pub local_dir: Option<PathBuf>,
}

impl Default for ArchiveCfg {
    fn default() -> Self {
        ArchiveCfg {
            backend: None,

            key: None,
            secret: None,
            endpoint: None,
            bucket: None,
            region: String::from("us-east-1"),

            local_dir: None,
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
        log_ingestion_listen = "2.2.2.2"
        log_ingestion_port = 9999

        [archive]
        backend = "s3"
        key = "THIS_IS_THE_KEY"
        secret = "THIS_IS_THE_SECRET"
        bucket = "bukkit"
        endpoint = "http://minio.mycompany.com:9000"

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
        assert_eq!(
            &format!("{}", config.net.worker_command_listen),
            "1:1:1:1:1:1:1:1"
        );
        assert_eq!(
            &format!("{}", config.net.worker_heartbeat_listen),
            "1.1.1.1"
        );
        assert_eq!(&format!("{}", config.net.log_ingestion_listen), "2.2.2.2");

        assert_eq!(config.net.worker_command_port, 9000);
        assert_eq!(config.net.worker_heartbeat_port, 9000);
        assert_eq!(config.net.log_ingestion_port, 9999);
        assert_eq!(config.shards, vec![0]);
        assert_eq!(config.worker_threads, 1);
        assert_eq!(config.datastore.port, 9000);
        assert_eq!(config.datastore.user, "test");
        assert_eq!(config.datastore.database, "test_jobsrv");
        assert_eq!(config.datastore.connection_retry_ms, 500);
        assert_eq!(config.datastore.connection_timeout_sec, 4800);
        assert_eq!(config.datastore.connection_test, true);
        assert_eq!(config.datastore.pool_size, 1);

        assert_eq!(config.archive.backend, Some(ArchiveBackend::S3));
        assert_eq!(config.archive.key, Some("THIS_IS_THE_KEY".to_string()));
        assert_eq!(
            config.archive.secret,
            Some("THIS_IS_THE_SECRET".to_string())
        );
        assert_eq!(config.archive.bucket, Some("bukkit".to_string()));
        assert_eq!(
            config.archive.endpoint,
            Some("http://minio.mycompany.com:9000".to_string())
        );
        assert_eq!(config.archive.region, "us-east-1");

        assert_eq!(config.archive.local_dir, None);

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
