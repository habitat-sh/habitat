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
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use hab_net::config::{GitHubCfg, GitHubOAuth, RouterAddr, RouterCfg};
use hab_core::config::ConfigFile;
use depot;
use toml;

use error::{Error, Result};

#[derive(Debug)]
pub struct Config {
    /// Public listening net address for HTTP requests
    pub http_addr: SocketAddr,
    /// Depot's configuration
    pub depot: depot::Config,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub github: GitHubCfg,
    /// Path to UI files to host over HTTP. If not set the UI will be disabled.
    pub ui_root: Option<String>,
    /// Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Where to record log events for funnel metrics
    pub log_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Cfg::default().into()
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        // let mut pkg_path = String::new();
        // if try!(toml.parse_into("pkg.svc_static_path", &mut pkg_path)) {
        //     cfg.ui_root = Some(pkg_path);
        // }
        // try!(toml.parse_into("cfg.http_addr", &mut cfg.http_addr));
        // try!(toml.parse_into("cfg.router_addrs", &mut cfg.routers));
        // try!(toml.parse_into("pkg.svc_data_path", &mut cfg.depot.path));
        // try!(toml.parse_into("cfg.depot.datastore_addr", &mut cfg.depot.datastore_addr));
        // try!(toml.parse_into("cfg.github.url", &mut cfg.github_url));
        // try!(toml.parse_into("cfg.github.url", &mut cfg.depot.github_url));
        // try!(toml.parse_into("cfg.github.client_id", &mut cfg.github_client_id));
        // if cfg.github_client_id.is_empty() {
        //     return Err(Error::from(hab_net::Error::RequiredConfigField("github.client_id")));
        // }
        // try!(toml.parse_into("cfg.github.client_id", &mut cfg.depot.github_client_id));
        // try!(toml.parse_into("cfg.github.client_secret", &mut cfg.github_client_secret));
        // if cfg.github_client_secret.is_empty() {
        //     return Err(Error::from(hab_net::Error::RequiredConfigField("github.client_secret")));
        // }
        // try!(toml.parse_into("cfg.github.client_secret",
        //                      &mut cfg.depot.github_client_secret));
        // try!(toml.parse_into("cfg.events_enabled", &mut cfg.events_enabled));
        // try!(toml.parse_into("cfg.events_enabled", &mut cfg.depot.events_enabled));
        // try!(toml.parse_into("cfg.builds_enabled", &mut cfg.depot.builds_enabled));
        // try!(toml.parse_into("pkg.svc_var_path", &mut cfg.log_dir));
        // try!(toml.parse_into("pkg.svc_var_path", &mut cfg.depot.log_dir));
        // try!(toml.parse_into("cfg.supported_targets", &mut cfg.depot.supported_targets));
        Ok(cfg)
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

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Cfg {
    pub http: HttpCfg,
    pub routers: Vec<RouterAddr>,
    pub github: GitHubCfg,
    pub ui: UiCfg,
    pub depot: depot::config::Cfg,
    pub events_enabled: bool,
    pub log_dir: String,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            http: HttpCfg::default(),
            routers: vec![RouterAddr::default()],
            github: GitHubCfg::default(),
            ui: UiCfg::default(),
            depot: depot::config::Cfg::default(),
            events_enabled: false,
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
        }
    }
}

impl Into<Config> for Cfg {
    fn into(self) -> Config {
        Config {
            events_enabled: self.events_enabled,
            log_dir: self.log_dir,
            github: self.github,
            routers: self.routers,
            http_addr: SocketAddr::new(self.http.listen, self.http.port),
            ui_root: self.ui.root,
            // JW TODO: Need to populate pieces of the depot config with some of the
            // api config
            depot: self.depot.into(),
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
            port: 9636,
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
        port = 9636

        [ui]
        root = "/some/path"

        [depot]
        path = "/hab/svc/hab-depot/data"
        insecure = true
        builds_enabled = true
        events_enabled = true
        log_dir = "/hab/svc/hab-depot/var/log"
        supported_targets = [
            "Whatever",
            "This"
        ]

        [[routers]]
        host = "172.18.0.2"
        port = 9632

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.http_addr), "0.0.0.0:9636");
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
