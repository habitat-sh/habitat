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

//! Configuration for a Habitat RouteSrv service

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use hab_core::config::{ConfigFile, ParseInto};
use toml;

use error::{Error, Result};

#[derive(Debug)]
pub struct Config {
    /// Listening net address for client connections
    pub listen_addr: SocketAddr,
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
        self.listen_addr.set_port(port);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 5562)),
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
