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
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::option::IntoIter;

use hab_core::config::ConfigFile;
use hab_core::os::system::{Architecture, Platform};
use hab_core::package::PackageTarget;
use hab_net::config::{GitHubCfg, GitHubOAuth, RouterAddr, RouterCfg};

use error::Error;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub github: GitHubCfg,
    /// Disable authenticated uploads for all entities
    pub insecure: bool,
    /// Filepath to location on disk to store entities
    pub path: String,
    /// Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Whether to schedule builds on package upload
    pub builds_enabled: bool,
    /// Filepath to where log events for funnel metrics will be recorded
    pub log_dir: String,
    /// A list of package platform and architecture combinations which can be uploaded and hosted
    pub targets: Vec<PackageTarget>,
}

impl ConfigFile for Config {
    type Error = Error;
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpCfg::default(),
            routers: vec![RouterAddr::default()],
            github: GitHubCfg::default(),
            path: "/hab/svc/hab-depot/data".to_string(),
            insecure: false,
            events_enabled: false, // TODO: change to default to true later
            builds_enabled: false,
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
            targets: vec![
                PackageTarget::new(Platform::Linux, Architecture::X86_64),
                PackageTarget::new(Platform::Windows, Architecture::X86_64),
            ],
        }
    }
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

#[derive(Clone, Debug, Deserialize)]
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

impl ToSocketAddrs for HttpCfg {
    type Iter = IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<IntoIter<SocketAddr>> {
        match self.listen {
            IpAddr::V4(ref a) => (*a, self.port).to_socket_addrs(),
            IpAddr::V6(ref a) => (*a, self.port).to_socket_addrs(),
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

        [[targets]]
        platform = "linux"
        architecture = "x86_64"

        [[targets]]
        platform = "windows"
        architecture = "x86_64"

        [http]
        listen = "127.0.0.1"
        port = 9000

        [[routers]]
        host = "172.18.0.2"
        port = 9001
        heartbeat = 9001

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.path, "/hab/svc/hab-depot/data");
        assert_eq!(config.insecure, true);
        assert_eq!(config.builds_enabled, true);
        assert_eq!(config.events_enabled, true);
        assert_eq!(config.log_dir, "/hab/svc/hab-depot/var/log");
        assert_eq!(&format!("{}", config.http.listen), "127.0.0.1");
        assert_eq!(config.http.port, 9000);
        assert_eq!(&format!("{}", config.routers[0]), "172.18.0.2:9001");
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(
            config.github.client_secret,
            "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        );
        assert_eq!(config.targets.len(), 2);
        assert_eq!(config.targets[0].platform, Platform::Linux);
        assert_eq!(config.targets[0].architecture, Architecture::X86_64);
        assert_eq!(config.targets[1].platform, Platform::Windows);
        assert_eq!(config.targets[1].architecture, Architecture::X86_64);
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
