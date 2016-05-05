// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Configuration for a Habitat RouteSrv service

use std::net;

use hab_core::config::{ConfigFile, ParseInto};
use toml;

use error::{Error, Result};

#[derive(Debug)]
pub struct Config {
    /// Listening net address for client connections
    pub listen_addr: net::SocketAddrV4,
    /// Port for receiving service heartbeats
    pub heartbeat_port: u16,
}

impl Config {
    pub fn fe_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.listen_addr.ip(),
                self.listen_addr.port())
    }

    pub fn hb_addr(&self) -> String {
        format!("tcp://{}:{}", self.listen_addr.ip(), self.heartbeat_port)
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.listen_addr = net::SocketAddrV4::new(*self.listen_addr.ip(), port);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 5562),
            heartbeat_port: 5563,
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.listen_addr", &mut cfg.listen_addr));
        try!(toml.parse_into("cfg.heartbeat_port", &mut cfg.heartbeat_port));
        Ok(cfg)
    }
}
