// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

//! Configuration for a Habitat Builder-Admin service

use std::net;

use hab_net::config::{GitHubOAuth, RouteAddrs};
use hab_core::config::{ConfigFile, ParseInto};
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

#[derive(Debug)]
pub struct Config {
    /// Public listening net address for HTTP requests
    pub http_addr: net::SocketAddrV4,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<net::SocketAddrV4>,
    /// URL to GitHub API
    pub github_url: String,
    /// Client identifier used for GitHub API requests
    pub github_client_id: String,
    /// Client secret used for GitHub API requests
    pub github_client_secret: String,
    /// Path to UI files to host over HTTP. If not set the UI will be disabled.
    pub ui_root: Option<String>,
}

impl Config {
    /// Set the port of the http listener
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.http_addr = net::SocketAddrV4::new(*self.http_addr.ip(), port);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 8080),
            routers: vec![net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5562)],
            github_url: GITHUB_URL.to_string(),
            github_client_id: DEV_GITHUB_CLIENT_ID.to_string(),
            github_client_secret: DEV_GITHUB_CLIENT_SECRET.to_string(),
            ui_root: None,
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        let mut pkg_path = String::new();
        if try!(toml.parse_into("pkg.svc_static_path", &mut pkg_path)) {
            cfg.ui_root = Some(pkg_path);
        }
        try!(toml.parse_into("cfg.http_addr", &mut cfg.http_addr));
        try!(toml.parse_into("cfg.router_addrs", &mut cfg.routers));
        try!(toml.parse_into("cfg.github.url", &mut cfg.github_url));
        if !try!(toml.parse_into("cfg.github.client_id", &mut cfg.github_client_id)) {
            return Err(Error::RequiredConfigField("github.client_id"));
        }
        if !try!(toml.parse_into("cfg.github.client_secret", &mut cfg.github_client_secret)) {
            return Err(Error::RequiredConfigField("github.client_secret"));
        }
        Ok(cfg)
    }
}

impl RouteAddrs for Config {
    fn route_addrs(&self) -> &Vec<net::SocketAddrV4> {
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
