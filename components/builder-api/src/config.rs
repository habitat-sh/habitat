// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::net;

use core::config::{ConfigFile, ParseInto};
use depot;
use toml;

use error::{Error, Result};

pub struct Config {
    pub http_addr: net::SocketAddrV4,
    pub depot: depot::Config,
    sessionsrv_addr: net::SocketAddrV4,
    vaultsrv_addr: net::SocketAddrV4,
}

impl Config {
    pub fn sessionsrv_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.sessionsrv_addr.ip(),
                self.sessionsrv_addr.port())
    }

    pub fn vaultsrv_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.vaultsrv_addr.ip(),
                self.vaultsrv_addr.port())
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.http_addr = net::SocketAddrV4::new(*self.http_addr.ip(), port);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 9636),
            sessionsrv_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5560),
            vaultsrv_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5561),
            depot: depot::Config::default(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("cfg.http_addr", &mut cfg.http_addr));
        try!(toml.parse_into("cfg.sessionsrv_addr", &mut cfg.sessionsrv_addr));
        try!(toml.parse_into("cfg.vaultsrv_addr", &mut cfg.vaultsrv_addr));
        try!(toml.parse_into("cfg.depot.path", &mut cfg.depot.path));
        try!(toml.parse_into("cfg.depot.datastore_addr", &mut cfg.depot.datastore_addr));
        Ok(cfg)
    }
}
