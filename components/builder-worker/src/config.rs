// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::net;

pub struct Config {
    pub jobsrv_addr: net::SocketAddrV4,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { jobsrv_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 5560) }
    }
}
