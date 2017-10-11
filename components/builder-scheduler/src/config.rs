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

//! Configuration for a Habitat Scheduler service

use std::path::PathBuf;

use db::config::DataStoreCfg;
use hab_core::config::ConfigFile;
use hab_net::app::config::*;

use error::SrvError;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub app: AppCfg,
    /// Path to packages on-disk (for migration)
    pub migration_path: String,
    /// Path to scheduler event logs
    pub log_path: PathBuf,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    /// Configuration for the DB to connect to
    pub datastore: DataStoreCfg,
}

impl Default for Config {
    fn default() -> Self {
        let mut datastore = DataStoreCfg::default();
        datastore.database = String::from("builder_scheduler");
        Config {
            app: AppCfg::default(),
            migration_path: String::from("/hab/svc/builder-scheduler/pkgs"),
            log_path: PathBuf::from("/tmp"),
            routers: vec![RouterAddr::default()],
            datastore: datastore,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        [datastore]
        host = "1.1.1.1"
        port = 9000
        user = "test"
        database = "test_scheduler"
        connection_retry_ms = 500
        connection_timeout_sec = 4800
        connection_test = true
        pool_size = 1
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.datastore.port, 9000);
        assert_eq!(config.datastore.user, "test");
        assert_eq!(config.datastore.database, "test_scheduler");
        assert_eq!(config.datastore.connection_retry_ms, 500);
        assert_eq!(config.datastore.connection_timeout_sec, 4800);
        assert_eq!(config.datastore.connection_test, true);
        assert_eq!(config.datastore.pool_size, 1);
    }
}
