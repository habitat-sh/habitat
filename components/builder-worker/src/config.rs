// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Configuration for a Habitat JobSrv Worker

use std::collections::BTreeMap;
use std::net;

use hab_core::config::{ConfigFile, ParseInto};
use toml;

use error::{Error, Result};

pub struct Config {
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
        Config { job_servers: vec![jobsrv] }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        // JW TODO: parse job_servers into the config
        // try!(toml.parse_into("cfg.job_servers", &mut cfg.job_servers));
        Ok(cfg)
    }
}
