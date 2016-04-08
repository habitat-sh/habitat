// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::net;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Config {
    pub path: String,
    pub listen_addr: super::ListenAddr,
    pub port: super::ListenPort,
}

impl Config {
    /// Create a default `Config`
    pub fn new() -> Config {
        Config::default()
    }

    pub fn depot_addr(&self) -> net::SocketAddrV4 {
        net::SocketAddrV4::new(self.listen_addr.0.clone(), self.port.0.clone())
    }
}
