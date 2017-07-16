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
// Configuration for a Habitat SessionSrv service

use db::config::DataStoreCfg;
use hab_core::config::ConfigFile;
use hab_net::config::{DispatcherCfg, GitHubCfg, GitHubOAuth, RouterCfg, RouterAddr, Shards};
use protocol::sharding::{ShardId, SHARD_COUNT};

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// List of shard identifiers serviced by the running service.
    pub shards: Vec<ShardId>,
    /// Number of threads to process queued messages.
    pub worker_threads: usize,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    pub datastore: DataStoreCfg,
    pub github: GitHubCfg,
    pub permissions: PermissionsCfg,
}

impl Default for Config {
    fn default() -> Self {
        let mut datastore = DataStoreCfg::default();
        datastore.database = String::from("builder_sessionsrv");
        Config {
            shards: (0..SHARD_COUNT).collect(),
            worker_threads: Self::default_worker_count(),
            routers: vec![RouterAddr::default()],
            datastore: datastore,
            github: GitHubCfg::default(),
            permissions: PermissionsCfg::default(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

impl DispatcherCfg for Config {
    fn worker_count(&self) -> usize {
        self.worker_threads
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

impl Shards for Config {
    fn shards(&self) -> &Vec<u32> {
        &self.shards
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct PermissionsCfg {
    /// A GitHub Team identifier for which members will automatically have administration
    /// privileges assigned to their session
    pub admin_team: u64,
    /// GitHub team's whose members are granted Builder Worker abilities. These abilities
    /// include downloading the latest private key for any origin and uploading a package into
    /// any origin regardless of membership. This is essentially a user who ignores all rules.
    pub build_worker_teams: Vec<u64>,
    /// GitHub team's assigned to the early access group who may have access to features in Builder
    /// before a normal user.
    pub early_access_teams: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        shards = [
            0
        ]
        worker_threads = 1

        [permissions]
        admin_team = 2000
        build_worker_teams = [
            3000,
            3001
        ]
        early_access_teams = [
            4000,
            4001
        ]

        [[routers]]
        host = "1:1:1:1:1:1:1:1"
        port = 9000

        [datastore]
        host = "1.1.1.1"
        port = 9000
        user = "test"
        database = "test_sessionsrv"
        connection_retry_ms = 500
        connection_timeout_sec = 4800
        connection_test = true
        pool_size = 1

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.shards, vec![0]);
        assert_eq!(config.worker_threads, 1);
        assert_eq!(config.permissions.admin_team, 2000);
        assert_eq!(config.permissions.build_worker_teams, vec![3000, 3001]);
        assert_eq!(config.permissions.early_access_teams, vec![4000, 4001]);
        assert_eq!(&format!("{}", config.datastore.host), "1.1.1.1");
        assert_eq!(config.datastore.port, 9000);
        assert_eq!(config.datastore.user, "test");
        assert_eq!(config.datastore.database, "test_sessionsrv");
        assert_eq!(config.datastore.connection_retry_ms, 500);
        assert_eq!(config.datastore.connection_timeout_sec, 4800);
        assert_eq!(config.datastore.connection_test, true);
        assert_eq!(config.datastore.pool_size, 1);
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(
            config.github.client_secret,
            "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        );
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        worker_threads = 0
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.worker_threads, 0);
    }
}
