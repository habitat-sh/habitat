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

//! Configuration for a Habitat Builder-Admin service

use std::io;
use std::net::{Ipv4Addr, IpAddr, SocketAddr, ToSocketAddrs};
use std::option::IntoIter;

use hab_net::config::{GitHubCfg, GitHubOAuth, RouterAddr, RouterCfg};
use hab_core::config::ConfigFile;

use error::Error;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    pub routers: Vec<RouterAddr>,
    pub github: GitHubCfg,
    pub ui: UiCfg,
}

impl ConfigFile for Config {
    type Error = Error;
}

impl GitHubOAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_web_url(&self) -> &str {
        &self.github.web_url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
    }
}

impl RouterCfg for Config {
    fn route_addrs(&self) -> &Vec<RouterAddr> {
        &self.routers
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpCfg {
    pub listen: IpAddr,
    pub port: u16,
}

impl Default for HttpCfg {
    fn default() -> Self {
        HttpCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 8080,
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
    pub root: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        [http]
        listen = "0:0:0:0:0:0:0:1"
        port = 8080

        [ui]
        root = "/some/path"

        [[routers]]
        host = "172.18.0.2"
        port = 9632

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.http.port, 8080);
        assert_eq!(&format!("{}", config.routers[0]), "172.18.0.2:9632");
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(
            config.github.client_secret,
            "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        );
        assert_eq!(config.ui.root, Some("/some/path".to_string()));
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        [http]
        port = 9000
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.http.port, 9000);
    }
}
