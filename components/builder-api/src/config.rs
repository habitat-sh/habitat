// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::net;

pub struct Config {
    pub http_addr: net::SocketAddrV4,
    sessionsrv_addr: net::SocketAddrV4,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn sessionsrv_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.sessionsrv_addr.ip(),
                self.sessionsrv_addr.port())
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
        }
    }
}
