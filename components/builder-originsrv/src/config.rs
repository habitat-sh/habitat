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

//! Configuration for a Habitat OriginSrv service

use hab_net::app::config::*;
use db::config::DataStoreCfg;

use error::SrvError;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub app: AppCfg,
    pub datastore: DataStoreCfg,
}

impl Default for Config {
    fn default() -> Self {
        let mut datastore = DataStoreCfg::default();
        datastore.database = String::from("builder_originsrv");
        Config {
            app: AppCfg::default(),
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
        database = "test_originsrv"
        connection_retry_ms = 500
        connection_timeout_sec = 4800
        connection_test = true
        pool_size = 1
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.datastore.port, 9000);
        assert_eq!(config.datastore.user, "test");
        assert_eq!(config.datastore.database, "test_originsrv");
        assert_eq!(config.datastore.connection_retry_ms, 500);
        assert_eq!(config.datastore.connection_timeout_sec, 4800);
        assert_eq!(config.datastore.connection_test, true);
        assert_eq!(config.datastore.pool_size, 1);
    }
}
