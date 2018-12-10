// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

use std::fmt;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::option;
use std::result;
use std::str::FromStr;

use error::{Error, Result};
use EnvConfig;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ListenCtlAddr(SocketAddr);

impl ListenCtlAddr {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        ListenCtlAddr(SocketAddr::V4(SocketAddrV4::new(ip, port)))
    }

    pub fn ip(&self) -> IpAddr {
        self.0.ip()
    }

    pub fn port(&self) -> u16 {
        self.0.port()
    }
}

impl Default for ListenCtlAddr {
    fn default() -> ListenCtlAddr {
        ListenCtlAddr::new(Ipv4Addr::LOCALHOST, 9632)
    }
}

impl EnvConfig for ListenCtlAddr {
    const ENVVAR: &'static str = "HAB_LISTEN_CTL";
}

impl FromStr for ListenCtlAddr {
    type Err = Error;

    fn from_str(val: &str) -> Result<Self> {
        Ok(ListenCtlAddr(SocketAddr::from_str(val)?))
    }
}

impl ToSocketAddrs for ListenCtlAddr {
    type Iter = option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.0.to_socket_addrs()
    }
}

impl fmt::Display for ListenCtlAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}
