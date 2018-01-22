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

//! Configuration for a Habitat Builder-API service

use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::option::IntoIter;

use depot;
use http_gateway::config::prelude::*;
use segment_api_client::SegmentCfg;
use typemap;

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub github: GitHubCfg,
    pub segment: SegmentCfg,
    pub ui: UiCfg,
    /// Depot's configuration
    pub depot: depot::config::Config,
    /// Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Whether to enable builds for non-core origins
    pub non_core_builds_enabled: bool,
    /// Where to record log events for funnel metrics
    pub log_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpCfg::default(),
            routers: vec![RouterAddr::default()],
            github: GitHubCfg::default(),
            segment: SegmentCfg::default(),
            ui: UiCfg::default(),
            depot: depot::config::Config::default(),
            events_enabled: false,
            non_core_builds_enabled: true,
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

impl GatewayCfg for Config {
    fn handler_count(&self) -> usize {
        self.http.handler_count
    }

    fn listen_addr(&self) -> &IpAddr {
        &self.http.listen
    }

    fn listen_port(&self) -> u16 {
        self.http.port
    }

    fn route_addrs(&self) -> &[RouterAddr] {
        self.routers.as_slice()
    }
}

impl typemap::Key for Config {
    type Value = Self;
}

/// Public listening net address for HTTP requests
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpCfg {
    pub listen: IpAddr,
    pub port: u16,
    pub handler_count: usize,
}

impl Default for HttpCfg {
    fn default() -> Self {
        HttpCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 9636,
            handler_count: Config::default_handler_count(),
        }
    }
}

impl ToSocketAddrs for HttpCfg {
    type Iter = IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<IntoIter<SocketAddr>> {
        match self.listen {
            IpAddr::V4(ref a) => (*a, self.port).to_socket_addrs(),
            IpAddr::V6(ref a) => (*a, self.port).to_socket_addrs(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct UiCfg {
    /// Path to UI files to host over HTTP. If not set the UI will be disabled.
    pub root: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        events_enabled = true
        non_core_builds_enabled = true

        [http]
        listen = "0:0:0:0:0:0:0:1"
        port = 9636
        handler_count = 128

        [ui]
        root = "/some/path"

        [depot]
        path = "/hab/svc/hab-depot/data"
        events_enabled = true
        log_dir = "/hab/svc/hab-depot/var/log"

        [[targets]]
        platform = "linux"
        architecture = "x86_64"

        [[targets]]
        platform = "windows"
        architecture = "x86_64"

        [[routers]]
        host = "172.18.0.2"
        port = 9632

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.events_enabled, true);
        assert_eq!(config.non_core_builds_enabled, true);
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.http.port, 9636);
        assert_eq!(config.http.handler_count, 128);
        assert_eq!(&format!("{}", config.routers[0]), "172.18.0.2:9632");
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(
            config.github.client_secret,
            "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        );
        assert_eq!(config.ui.root, Some("/some/path".to_string()));
        assert_eq!(config.segment.url, "https://api.segment.io");
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        [http]
        port = 9000
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.events_enabled, false);
        assert_eq!(config.non_core_builds_enabled, true);
        assert_eq!(config.http.port, 9000);
    }
}
