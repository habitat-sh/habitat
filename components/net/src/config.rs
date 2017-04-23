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

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use num_cpus;

pub const DEFAULT_ROUTER_LISTEN_PORT: u16 = 5562;
pub const DEFAULT_ROUTER_HEARTBEAT_PORT: u16 = 5563;

/// URL to GitHub API endpoint
pub const DEFAULT_GITHUB_URL: &'static str = "https://api.github.com";
/// Default Client ID for providing a default value in development environments only. This is
/// associated to the habitat-sh GitHub account and is configured to re-direct and point to a local
/// builder-api.
///
/// See https://github.com/settings/connections/applications/0c2f738a7d0bd300de10
pub const DEV_GITHUB_CLIENT_ID: &'static str = "0c2f738a7d0bd300de10";
/// Default Client Secret for development purposes only. See the `DEV_GITHUB_CLIENT_ID` for
/// additional comments.
pub const DEV_GITHUB_CLIENT_SECRET: &'static str = "438223113eeb6e7edf2d2f91a232b72de72b9bdf";

pub trait DispatcherCfg {
    fn default_worker_count() -> usize {
        // JW TODO: increase default count after r2d2 connection pools are moved to be owned
        // by main thread of servers instead of dispatcher threads.
        // num_cpus::get() * 8
        num_cpus::get()
    }

    fn worker_count(&self) -> usize;
}

pub trait GitHubOAuth {
    fn github_url(&self) -> &str;
    fn github_client_id(&self) -> &str;
    fn github_client_secret(&self) -> &str;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitHubCfg {
    /// URL to GitHub API
    pub url: String,
    /// Client identifier used for GitHub API requests
    pub client_id: String,
    /// Client secret used for GitHub API requests
    pub client_secret: String,
}

impl Default for GitHubCfg {
    fn default() -> Self {
        GitHubCfg {
            url: DEFAULT_GITHUB_URL.to_string(),
            client_id: DEV_GITHUB_CLIENT_ID.to_string(),
            client_secret: DEV_GITHUB_CLIENT_SECRET.to_string(),
        }
    }
}

/// Configuration structure for connecting to a Router
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct RouterAddr {
    /// Listening address of command and heartbeat socket
    pub host: IpAddr,
    /// Listening port of command socket
    pub port: u16,
    /// Listening port of heartbeat socket
    pub heartbeat: u16,
}

impl Default for RouterAddr {
    fn default() -> Self {
        RouterAddr {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: DEFAULT_ROUTER_LISTEN_PORT,
            heartbeat: DEFAULT_ROUTER_HEARTBEAT_PORT,
        }
    }
}

impl fmt::Display for RouterAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// Apply to server configurations which connect to a cluster of Routers
pub trait RouterCfg {
    /// Return a list of router addresses
    fn route_addrs(&self) -> &Vec<RouterAddr>;
}

/// Apply to a server configuration which belongs to a sharded service
pub trait Shards {
    fn shards(&self) -> &Vec<u32>;
}

/// Convert types into stringy socket addresses for ZeroMQ
pub trait ToAddrString {
    fn to_addr_string(&self) -> String;
}

impl ToAddrString for SocketAddr {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

impl ToAddrString for SocketAddrV4 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

impl ToAddrString for SocketAddrV6 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

impl ToAddrString for RouterAddr {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.host, self.port)
    }
}
