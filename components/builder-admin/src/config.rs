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

use std::net::{Ipv4Addr, IpAddr, SocketAddr};
use std::str::FromStr;

use hab_net::config::{GitHubCfg, GitHubOAuth, RouterAddr, RouterCfg};
use hab_core::config::ConfigFile;
use toml;

use error::{Error, Result};

#[derive(Debug)]
pub struct Config {
    /// Public listening net address for HTTP requests
    pub http_addr: SocketAddr,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub github: GitHubCfg,
    /// Path to UI files to host over HTTP. If not set the UI will be disabled.
    pub ui_root: Option<String>,
}

impl Config {
    /// Set the port of the http listener
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.http_addr.set_port(port);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Cfg::default().into()
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let cfg = toml.try_into::<Cfg>().expect("JW TODO: Don't unwrap");
        Ok(cfg.into())
    }
}

impl GitHubOAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
    }
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(toml: &str) -> Result<Self> {
        let config: Cfg = toml::from_str(toml).unwrap();
        Ok(config.into())
    }
}

impl RouterCfg for Config {
    fn route_addrs(&self) -> &Vec<RouterAddr> {
        &self.routers
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Cfg {
    pub http: HttpCfg,
    pub routers: Vec<RouterAddr>,
    pub github: GitHubCfg,
    pub ui: UiCfg,
}

impl Into<Config> for Cfg {
    fn into(self) -> Config {
        Config {
            github: self.github,
            routers: self.routers,
            http_addr: SocketAddr::new(self.http.listen, self.http.port),
            ui_root: self.ui.root,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct HttpCfg {
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

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct UiCfg {
    pub root: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        [http]
        listen = "0.0.0.0"
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

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.http_addr), "0.0.0.0:8080");
        assert_eq!(&format!("{}", config.routers[0]), "172.18.0.2:9632");
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(config.github.client_secret,
                   "438223113eeb6e7edf2d2f91a232b72de72b9bdf");
        assert_eq!(config.ui_root, Some("/some/path".to_string()));
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        [http]
        port = 9000
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.http_addr), "0.0.0.0:9000");
    }
}
