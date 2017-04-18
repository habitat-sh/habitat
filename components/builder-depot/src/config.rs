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
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use hab_core::config::ConfigFile;
use hab_core::os::system::{Architecture, Platform};
use hab_core::package::PackageTarget;
use hab_net::config::{GitHubCfg, GitHubOAuth, RouterAddr, RouterCfg};
use redis;
use toml;

use error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub path: String,
    pub listen_addr: SocketAddr,
    pub datastore_addr: SocketAddr,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub github: GitHubCfg,
    /// allows you to upload packages and public keys without auth
    pub insecure: bool,
    /// Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Whether to schedule builds on package upload
    pub builds_enabled: bool,
    /// Where to record log events for funnel metrics
    pub log_dir: String,
    /// Supported targets - comma separated
    pub supported_targets: Vec<PackageTarget>,
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        Ok(cfg)
    }
}

impl Default for Config {
    fn default() -> Self {
        Cfg::default().into()
    }
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(toml: &str) -> Result<Self> {
        let config: Cfg = toml::from_str(toml).unwrap();
        Ok(config.into())
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

impl RouterCfg for Config {
    fn route_addrs(&self) -> &Vec<RouterAddr> {
        &self.routers
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Cfg {
    pub http: HttpCfg,
    pub routers: Vec<RouterAddr>,
    pub datastore: DataStoreCfg,
    pub github: GitHubCfg,
    /// Enable unauthenticated uploads for all entities
    pub insecure: bool,
    /// Filepath to location on disk to store entities
    pub path: String,
    /// Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Whether to schedule builds on package upload
    pub builds_enabled: bool,
    /// Where to record log events for funnel metrics
    pub log_dir: String,
    /// List of supported package targets
    pub supported_targets: Vec<PackageTarget>,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            http: HttpCfg::default(),
            datastore: DataStoreCfg::default(),
            routers: vec![RouterAddr::default()],
            github: GitHubCfg::default(),
            path: "/hab/svc/hab-depot/data".to_string(),
            insecure: false,
            events_enabled: false, // TODO: change to default to true later
            builds_enabled: false,
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
            supported_targets: vec![PackageTarget::new(Platform::Linux, Architecture::X86_64),
                                    PackageTarget::new(Platform::Windows, Architecture::X86_64)],
        }
    }
}

impl Into<Config> for Cfg {
    fn into(self) -> Config {
        Config {
            path: self.path,
            builds_enabled: self.builds_enabled,
            events_enabled: self.events_enabled,
            insecure: self.insecure,
            log_dir: self.log_dir,
            github: self.github,
            routers: self.routers,
            listen_addr: SocketAddr::new(self.http.listen, self.http.port),
            datastore_addr: SocketAddr::new(self.datastore.host, self.datastore.port),
            supported_targets: self.supported_targets,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DataStoreCfg {
    pub host: IpAddr,
    pub port: u16,
}

impl Default for DataStoreCfg {
    fn default() -> Self {
        DataStoreCfg {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 6397,
        }
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
            port: 9632,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        path = "/hab/svc/hab-depot/data"
        insecure = true
        builds_enabled = true
        events_enabled = true
        log_dir = "/hab/svc/hab-depot/var/log"
        supported_targets = [
            "Whatever",
            "This"
        ]

        [http]
        listen = "127.0.0.1"
        port = 9000

        [[routers]]
        host = "172.18.0.2"
        port = 9001

        [datastore]
        host = "172.18.0.2"
        port = 9002

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(config.path, "/hab/svc/hab-depot/data");
        assert_eq!(config.insecure, true);
        assert_eq!(config.builds_enabled, true);
        assert_eq!(config.events_enabled, true);
        assert_eq!(config.log_dir, "/hab/svc/hab-depot/var/log");
        assert_eq!(&format!("{}", config.listen_addr), "127.0.0.1:9000");
        assert_eq!(&format!("{}", config.routers[0]), "172.18.0.2:9001");
        assert_eq!(&format!("{}", config.datastore_addr), "172.18.0.2:9002");
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(config.github.client_secret,
                   "438223113eeb6e7edf2d2f91a232b72de72b9bdf");
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        [http]
        port = 9000
        "#;

        let config = Config::from_str(&content).unwrap();
        assert_eq!(&format!("{}", config.listen_addr), "0.0.0.0:9000");
    }
}
