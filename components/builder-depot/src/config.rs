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

use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use hab_core::config::{ConfigFile, ParseInto};
use hab_core::os::system::{Architecture, Platform};
use hab_net::config::{GitHubOAuth, RouteAddrs};
use hab_core::package::PackageTarget;
use redis;
use toml;

use error::{Error, Result};

/// URL to GitHub API endpoint
const GITHUB_URL: &'static str = "https://api.github.com";
// Default Client ID for providing a default value in development environments only. This is
// associated to Jamie Winsor's GitHub account and is configured to re-direct and point to a local
// builder-api.
const DEV_GITHUB_CLIENT_ID: &'static str = "0c2f738a7d0bd300de10";
// Default Client Secret for development purposes only. See the `DEV_GITHUB_CLIENT_ID` for
// additional comments.
const DEV_GITHUB_CLIENT_SECRET: &'static str = "438223113eeb6e7edf2d2f91a232b72de72b9bdf";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub path: String,
    pub listen_addr: SocketAddr,
    pub datastore_addr: SocketAddr,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<SocketAddr>,
    /// URL to GitHub API
    pub github_url: String,
    /// Client identifier used for GitHub API requests
    pub github_client_id: String,
    /// Client secret used for GitHub API requests
    pub github_client_secret: String,
    /// allows you to upload packages and public keys without auth
    pub insecure: bool,
    /// Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Where to record log events for funnel metrics
    pub log_dir: String,
    /// Supported targets - comma separated
    pub supported_targets: Vec<PackageTarget>,
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("pkg.svc_data_path", &mut cfg.path));
        try!(toml.parse_into("cfg.bind_addr", &mut cfg.listen_addr));
        try!(toml.parse_into("cfg.datastore_addr", &mut cfg.datastore_addr));
        try!(toml.parse_into("cfg.router_addrs", &mut cfg.routers));
        try!(toml.parse_into("cfg.events_enabled", &mut cfg.events_enabled));
        try!(toml.parse_into("pkg.svc_var_path", &mut cfg.log_dir));
        try!(toml.parse_into("cfg.supported_targets", &mut cfg.supported_targets));
        Ok(cfg)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: "/hab/svc/hab-depot/data".to_string(),
            listen_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9632)),
            datastore_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6379)),
            routers: vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5562))],
            github_url: GITHUB_URL.to_string(),
            github_client_id: DEV_GITHUB_CLIENT_ID.to_string(),
            github_client_secret: DEV_GITHUB_CLIENT_SECRET.to_string(),
            insecure: false,
            events_enabled: false, // TODO: change to default to true later
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
            supported_targets: vec![PackageTarget::new(Platform::Linux, Architecture::X86_64),
                                    PackageTarget::new(Platform::Windows, Architecture::X86_64)],
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

impl RouteAddrs for Config {
    fn route_addrs(&self) -> &Vec<SocketAddr> {
        &self.routers
    }
}

impl GitHubOAuth for Config {
    fn github_url(&self) -> &str {
        &self.github_url
    }

    fn github_client_id(&self) -> &str {
        &self.github_client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github_client_secret
    }
}
