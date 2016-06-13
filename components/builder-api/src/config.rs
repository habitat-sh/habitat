// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! Configuration for a Habitat Builder-API service

use std::net;

use hab_net::config::{GitHubOAuth, RouteAddrs};
use hab_core::config::{ConfigFile, ParseInto};
use depot;
use toml;

use error::{Error, Result};

/// URL to GitHub API endpoint
const GITHUB_URL: &'static str = "https://api.github.com";
// Default Client ID for providing a default value in development environments only. This is
// associated to Jamie Winsor's GitHub account and is configured to re-direct and point to a local
// builder-api.
const DEV_GITHUB_CLIENT_ID: &'static str = "e98d2a94787be9af9c00";
// Default Client Secret for development purposes only. See the `DEV_GITHUB_CLIENT_ID` for
// additional comments.
const DEV_GITHUB_CLIENT_SECRET: &'static str = "e5ff94188e3cf01d42f3e2bcbbe4faabe11c71ba";

#[derive(Debug)]
pub struct Config {
    /// Public listening net address for HTTP requests
    pub http_addr: net::SocketAddrV4,
    /// Depot's configuration
    pub depot: depot::Config,
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
            http_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 9636),
            routers: vec![net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5562)],
            depot: depot::Config::default(),
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
        try!(toml.parse_into("pkg.svc_data_path", &mut cfg.depot.path));
        try!(toml.parse_into("cfg.depot.datastore_addr", &mut cfg.depot.datastore_addr));
        try!(toml.parse_into("cfg.github.url", &mut cfg.github_url));
        try!(toml.parse_into("cfg.github.url", &mut cfg.depot.github_url));
        if !try!(toml.parse_into("cfg.github.client_id", &mut cfg.github_client_id)) {
            return Err(Error::RequiredConfigField("github.client_id"));
        }
        try!(toml.parse_into("cfg.github.client_id", &mut cfg.depot.github_client_id));
        if !try!(toml.parse_into("cfg.github.client_secret", &mut cfg.github_client_secret)) {
            return Err(Error::RequiredConfigField("github.client_secret"));
        }
        try!(toml.parse_into("cfg.github.client_secret",
                             &mut cfg.depot.github_client_secret));
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
