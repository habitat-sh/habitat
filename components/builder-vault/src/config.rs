// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::net;

use core::config::{ConfigFile, ParseInto};
use redis;
use toml;

use error::{Error, Result};

pub struct Config {
    pub listen_addr: net::SocketAddrV4,
    pub datastore_addr: net::SocketAddrV4,
    pub worker_count: usize,
}

impl Config {
    pub fn fe_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.listen_addr.ip(),
                self.listen_addr.port())
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.listen_addr = net::SocketAddrV4::new(*self.listen_addr.ip(), port);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 5561),
            datastore_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 6379),
            worker_count: 8,
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Table) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.listen_addr", &mut cfg.listen_addr));
        try!(toml.parse_into("cfg.datastore_addr", &mut cfg.datastore_addr));
        try!(toml.parse_into("cfg.worker_count", &mut cfg.worker_count));
        Ok(cfg)
    }
}

impl<'a> redis::IntoConnectionInfo for &'a Config {
    fn into_connection_info(self) -> redis::RedisResult<redis::ConnectionInfo> {
        format!("redis://{}:{}",
                self.datastore_addr.ip(),
                self.datastore_addr.port())
            .into_connection_info()
    }
}
