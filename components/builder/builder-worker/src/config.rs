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

//! Configuration for a Habitat JobSrv Worker

use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

use github_api_client::config::GitHubCfg;
use hab_core::config::ConfigFile;
use hab_core::url;

use error::Error;

pub type JobSrvCfg = Vec<JobSrvAddr>;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Enable automatic publishing for all builds by default
    pub auto_publish: bool,
    /// Filepath where persistent application data is stored
    pub data_path: PathBuf,
    /// Filepath to where the builder encryption keys can be found
    pub key_dir: PathBuf,
    /// Path to worker event logs
    pub log_path: PathBuf,
    /// Default channel name for Publish post-processor to use to determine which channel to
    /// publish artifacts to
    pub bldr_channel: String,
    /// Default URL for Publish post-processor to use to determine which Builder to use
    /// for retrieving signing keys and publishing artifacts
    pub bldr_url: String,
    /// List of Job Servers to connect to
    pub jobsrv: JobSrvCfg,
    pub features_enabled: String,
    /// Github application id to use for private repo access
    pub github: GitHubCfg,
    pub airlock_enabled: bool,
    /// Whether or not to recreate network namespace if one already exists
    pub recreate_ns_dir: bool,
    pub network_interface: Option<String>,
    pub network_gateway: Option<IpAddr>,
}

impl Config {
    pub fn jobsrv_addrs(&self) -> Vec<(String, String, String)> {
        let mut addrs = vec![];
        for job_server in &self.jobsrv {
            let hb = format!("tcp://{}:{}", job_server.host, job_server.heartbeat);
            let queue = format!("tcp://{}:{}", job_server.host, job_server.port);
            let log = format!("tcp://{}:{}", job_server.host, job_server.log_port);
            addrs.push((hb, queue, log));
        }
        addrs
    }

    pub fn ns_dir_path(&self) -> PathBuf {
        self.data_path.join("network").join("airlock-ns")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auto_publish: true,
            data_path: PathBuf::from("/tmp"),
            log_path: PathBuf::from("/tmp"),
            key_dir: PathBuf::from("/hab/svc/builder-worker/files"),
            bldr_channel: String::from("unstable"),
            bldr_url: url::default_bldr_url(),
            jobsrv: vec![JobSrvAddr::default()],
            features_enabled: "".to_string(),
            github: GitHubCfg::default(),
            airlock_enabled: true,
            recreate_ns_dir: false,
            network_interface: None,
            network_gateway: None,
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct JobSrvAddr {
    pub host: IpAddr,
    pub port: u16,
    pub heartbeat: u16,
    pub log_port: u16,
}

impl Default for JobSrvAddr {
    fn default() -> Self {
        JobSrvAddr {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5566,
            heartbeat: 5567,
            log_port: 5568,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        data_path = "/path/to/data"
        log_path = "/path/to/logs"
        key_dir = "/path/to/key"
        features_enabled = "FOO,BAR"
        recreate_ns_dir = true
        network_interface = "eth1"
        network_gateway = "192.168.10.1"

        [[jobsrv]]
        host = "1:1:1:1:1:1:1:1"
        port = 9000
        heartbeat = 9001
        log_port = 9021

        [[jobsrv]]
        host = "2.2.2.2"
        port = 9000
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.data_path.display()), "/path/to/data");
        assert_eq!(&format!("{}", config.log_path.display()), "/path/to/logs");
        assert_eq!(&format!("{}", config.key_dir.display()), "/path/to/key");
        assert_eq!(&format!("{}", config.jobsrv[0].host), "1:1:1:1:1:1:1:1");
        assert_eq!(config.jobsrv[0].port, 9000);
        assert_eq!(config.jobsrv[0].heartbeat, 9001);
        assert_eq!(config.jobsrv[0].log_port, 9021);
        assert_eq!(&format!("{}", config.jobsrv[1].host), "2.2.2.2");
        assert_eq!(config.jobsrv[1].port, 9000);
        assert_eq!(config.jobsrv[1].heartbeat, 5567);
        assert_eq!(&config.features_enabled, "FOO,BAR");
        assert_eq!(config.network_interface, Some(String::from("eth1")));
        assert_eq!(config.airlock_enabled, true);
        assert_eq!(config.recreate_ns_dir, true);
        assert_eq!(
            config.network_gateway,
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 10, 1)))
        );
    }
}
