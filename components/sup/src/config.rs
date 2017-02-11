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

//! Configuration for the Supervisor.
//!
//! This module is populated from the CLI options in `main.rs`, and then passed through to the
//! [command](../command) modules. Check out the `config_from_args(..)` function there for more
//! details.
//!
//! See the [Config](struct.Config.html) struct for the specific options available.

use std::fmt;
use std::io;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs, SocketAddr, SocketAddrV4};
use std::ops::{Deref, DerefMut};
use std::option;
use std::result;
use std::str::FromStr;
use std::sync::{Once, ONCE_INIT};

use error::{Error, Result, SupError};
use http_gateway;

static LOGKEY: &'static str = "CFG";

/// The Static Global Configuration.
///
/// This sets up a raw pointer, which we are going to transmute to a Box<Config>
/// with the first call to gcache().
static mut CONFIG: *const Config = 0 as *const Config;

/// Store a configuration, for later use through `gconfig()`.
///
/// MUST BE CALLED BEFORE ANY CALLS TO `gconfig()`.
pub fn gcache(config: Config) {
    static ONCE: Once = ONCE_INIT;
    unsafe {
        ONCE.call_once(|| { CONFIG = mem::transmute(Box::new(config)); });
    }
}

/// Return a reference to our cached configuration.
///
/// This is unsafe, because we are de-referencing the raw pointer stored in
/// CONFIG.
pub fn gconfig() -> &'static Config {
    unsafe { &*CONFIG }
}

#[derive(PartialEq, Eq, Debug)]
pub struct GossipListenAddr(SocketAddr);

impl Default for GossipListenAddr {
    fn default() -> GossipListenAddr {
        GossipListenAddr(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9638)))
    }
}

impl Deref for GossipListenAddr {
    type Target = SocketAddr;

    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}

impl DerefMut for GossipListenAddr {
    fn deref_mut(&mut self) -> &mut SocketAddr {
        &mut self.0
    }
}

impl FromStr for GossipListenAddr {
    type Err = SupError;

    fn from_str(val: &str) -> Result<Self> {
        match SocketAddr::from_str(val) {
            Ok(addr) => Ok(GossipListenAddr(addr)),
            Err(_) => {
                match IpAddr::from_str(val) {
                    Ok(ip) => {
                        let mut addr = GossipListenAddr::default();
                        addr.set_ip(ip);
                        Ok(addr)
                    }
                    Err(_) => Err(sup_error!(Error::IPFailed)),
                }
            }
        }
    }
}

impl ToSocketAddrs for GossipListenAddr {
    type Iter = option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.0.to_socket_addrs()
    }
}

impl fmt::Display for GossipListenAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

/// Holds our configuration options.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Config {
    // Currently only used by `ServiceConfig`
    pub service_config_http_listen: HttpGatewayListenAddr,
    // Currently only used by `Package`
    pub package_config_from: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpGatewayListenAddr {
    addr: http_gateway::ListenAddr,
}

impl HttpGatewayListenAddr {
    pub fn new(listen_addr: http_gateway::ListenAddr) -> HttpGatewayListenAddr {
        HttpGatewayListenAddr { addr: listen_addr }
    }
    pub fn set(&mut self, gateway_addr: http_gateway::ListenAddr) {
        self.addr = gateway_addr;
    }
    pub fn port(&self) -> u64 {
        self.addr.port() as u64
    }
    pub fn ip(&self) -> IpAddr {
        self.addr.ip()
    }
}

impl Default for HttpGatewayListenAddr {
    fn default() -> HttpGatewayListenAddr {
        HttpGatewayListenAddr { addr: http_gateway::ListenAddr::default() }
    }
}
