// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::net;

use core::config::{ConfigFile, ParseInto};
use toml;

use error::{Error, Result};

pub struct Config {
    pub jobsrv_addr: net::SocketAddrV4,
}

impl Config {
    pub fn jobsrv_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.jobsrv_addr.ip(),
                self.jobsrv_addr.port())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { jobsrv_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5562) }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.jobsrv_addr", &mut cfg.jobsrv_addr));
        Ok(cfg)
    }
}
