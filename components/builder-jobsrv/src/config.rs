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
    pub listen_addr: net::SocketAddrV4,
}

impl Config {
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.listen_addr = net::SocketAddrV4::new(*self.listen_addr.ip(), port);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { listen_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 5562) }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Table) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.listen_addr", &mut cfg.listen_addr));
        Ok(cfg)
    }
}
