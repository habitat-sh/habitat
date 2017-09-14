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

use hab_core::config::ConfigFile;
use hab_core::url;

use error::Error;

pub type JobSrvCfg = Vec<JobSrvAddr>;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Token for authenticating with the public builder-api
    pub auth_token: String,
    /// Enable automatic publishing for all builds by default
    pub auto_publish: bool,
    /// Filepath where persistent application data is stored
    pub data_path: PathBuf,
    /// Path to worker event logs
    pub log_path: PathBuf,
    /// Default channel name for Publish post-processor to use to determine which channel to
    /// publish artifacts to
    pub depot_channel: String,
    /// Default URL for Publish post-processor to use to determine which Builder Depot to use
    /// for retrieving signing keys and publishing artifacts
    pub depot_url: String,
    /// List of Job Servers to connect to
    pub jobsrv: JobSrvCfg,
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
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auth_token: "".to_string(),
            auto_publish: true,
            data_path: PathBuf::from("/tmp"),
            log_path: PathBuf::from("/tmp"),
            depot_channel: String::from("unstable"),
            depot_url: url::default_depot_url(),
            jobsrv: vec![JobSrvAddr::default()],
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
        auth_token = "mytoken"
        data_path = "/path/to/data"
        log_path = "/path/to/logs"

        [[jobsrv]]
        host = "1:1:1:1:1:1:1:1"
        port = 9000
        heartbeat = 9001

        [[jobsrv]]
        host = "2.2.2.2"
        port = 9000
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&config.auth_token, "mytoken");
        assert_eq!(&format!("{}", config.data_path.display()), "/path/to/data");
        assert_eq!(&format!("{}", config.log_path.display()), "/path/to/logs");
        assert_eq!(&format!("{}", config.jobsrv[0].host), "1:1:1:1:1:1:1:1");
        assert_eq!(config.jobsrv[0].port, 9000);
        assert_eq!(config.jobsrv[0].heartbeat, 9001);
        assert_eq!(&format!("{}", config.jobsrv[1].host), "2.2.2.2");
        assert_eq!(config.jobsrv[1].port, 9000);
        assert_eq!(config.jobsrv[1].heartbeat, 5567);
    }
}
