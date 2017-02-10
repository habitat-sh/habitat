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

use std::collections::BTreeMap;

use hab_core::config::{ConfigFile, ParseInto};
use toml;

use error::{Error, Result};

pub struct Config {
    /// Token for authenticating with the public builder-api
    pub auth_token: String,
    /// Filepath where persistent application data is stored
    pub data_path: String,
    /// List of Job Servers to connect to
    pub job_servers: Vec<BTreeMap<String, String>>,
}

impl Config {
    pub fn jobsrv_addrs(&self) -> Vec<(String, String)> {
        let mut addrs = vec![];
        for job_server in &self.job_servers {
            let ip = job_server.get("ip").unwrap();
            let port = job_server.get("port").unwrap();
            let heartbeat = job_server.get("heartbeat").unwrap();
            let hb = format!("tcp://{}:{}", ip, heartbeat);
            let queue = format!("tcp://{}:{}", ip, port);
            addrs.push((hb, queue));
        }
        addrs
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut jobsrv = BTreeMap::new();
        jobsrv.insert("ip".to_string(), "127.0.0.1".to_string());
        jobsrv.insert("port".to_string(), "5566".to_string());
        jobsrv.insert("heartbeat".to_string(), "5567".to_string());
        Config {
            auth_token: "".to_string(),
            data_path: "/tmp".to_string(),
            job_servers: vec![jobsrv],
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.auth_token", &mut cfg.auth_token));
        try!(toml.parse_into("cfg.data_path", &mut cfg.data_path));
        try!(toml.parse_into("cfg.job_servers", &mut cfg.job_servers));
        Ok(cfg)
    }
}
