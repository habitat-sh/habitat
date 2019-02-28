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

use crate::error::{Error,
                   Result};
use habitat_core::env;
use std::{fmt,
          net::{IpAddr,
                Ipv4Addr,
                SocketAddr,
                SocketAddrV4},
          result,
          str::FromStr};

use super::env_config::EnvConfig;
use crate::error::{Error,
                   Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ListenCtlAddr(SocketAddr);

impl ListenCtlAddr {
    pub const DEFAULT_PORT: u16 = 9632;

    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        ListenCtlAddr(SocketAddr::V4(SocketAddrV4::new(ip, port)))
    }

    pub fn ip(&self) -> IpAddr { self.0.ip() }

    pub fn port(&self) -> u16 { self.0.port() }
}

impl Default for ListenCtlAddr {
    fn default() -> ListenCtlAddr {
        ListenCtlAddr::new(Ipv4Addr::LOCALHOST, ListenCtlAddr::DEFAULT_PORT)
    }
}

impl env::Config for ListenCtlAddr {
    const ENVVAR: &'static str = "HAB_LISTEN_CTL";
}

impl FromStr for ListenCtlAddr {
    type Err = Error;

    fn from_str(val: &str) -> Result<Self> { Ok(val.parse::<SocketAddr>()?.into()) }
}

impl fmt::Display for ListenCtlAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl From<SocketAddr> for ListenCtlAddr {
    fn from(socket_addr: SocketAddr) -> Self { ListenCtlAddr(socket_addr) }
}

impl AsRef<SocketAddr> for ListenCtlAddr {
    fn as_ref(&self) -> &SocketAddr { &self.0 }
}
