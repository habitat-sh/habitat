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
// Configuration for a Habitat SessionSrv service

use db::config::DataStoreCfg;
use github_api_client::config::GitHubCfg;
use hab_net::app::config::*;

use error::SrvError;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub app: AppCfg,
    pub datastore: DataStoreCfg,
    pub github: GitHubCfg,
    pub permissions: PermissionsCfg,
}

impl Default for Config {
    fn default() -> Self {
        let mut datastore = DataStoreCfg::default();
        datastore.database = String::from("builder_sessionsrv");
        Config {
            app: AppCfg::default(),
            datastore: datastore,
            github: GitHubCfg::default(),
            permissions: PermissionsCfg::default(),
        }
    }
}

impl AsRef<AppCfg> for Config {
    fn as_ref(&self) -> &AppCfg {
        &self.app
    }
}

impl ConfigFile for Config {
    type Error = SrvError;
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct PermissionsCfg {
    pub app_install_id: u32,
    /// A GitHub Team identifier for which members will automatically have administration
    /// privileges assigned to their session
    pub admin_team: u32,
    /// GitHub team's whose members are granted Builder Worker abilities. These abilities
    /// include downloading the latest private key for any origin and uploading a package into
    /// any origin regardless of membership. This is essentially a user who ignores all rules.
    pub build_worker_teams: Vec<u32>,
    /// GitHub team's assigned to the early access group who may have access to features in Builder
    /// before a normal user.
    pub early_access_teams: Vec<u32>,
}

impl Default for PermissionsCfg {
    fn default() -> Self {
        PermissionsCfg {
            app_install_id: 56940,
            admin_team: 1995301,
            build_worker_teams: vec![1995301, 1996256],
            early_access_teams: vec![1995301],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
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
}
