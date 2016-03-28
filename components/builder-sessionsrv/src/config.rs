// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::net;

use redis;

pub struct Config {
    pub port: u16,
    pub datastore_addr: net::SocketAddrV4,
    pub worker_count: usize,
    listen_addr: net::SocketAddrV4,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn fe_addr(&self) -> String {
        format!("tcp://{}:{}",
                self.listen_addr.ip(),
                self.listen_addr.port())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 9636,
            listen_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 5560),
            datastore_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 6379),
            worker_count: 8,
        }
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
