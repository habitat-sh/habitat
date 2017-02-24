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

use std::error::Error;
use std::net::{Ipv4Addr, IpAddr};

use num_cpus;
use postgres::params::{ConnectParams, Host, IntoConnectParams};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DataStoreCfg {
    pub host: IpAddr,
    pub port: u16,
    pub user: String,
    pub password: Option<String>,
    pub database: String,
    /// Timing to retry the connection to the data store if it cannot be established
    pub connection_retry_ms: u64,
    /// How often to cycle a connection from the pool
    pub connection_timeout_sec: u64,
    /// If the datastore connection is under test
    pub connection_test: bool,
    /// Number of database connections to start in pool.
    pub pool_size: u32,
}

impl Default for DataStoreCfg {
    fn default() -> Self {
        DataStoreCfg {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5432,
            user: String::from("hab"),
            password: None,
            database: String::from(""),
            connection_retry_ms: 300,
            connection_timeout_sec: 3600,
            connection_test: false,
            pool_size: (num_cpus::get() * 2) as u32,
        }
    }
}

impl<'a> IntoConnectParams for &'a DataStoreCfg {
    fn into_connect_params(self) -> Result<ConnectParams, Box<Error + Sync + Send>> {
        let mut builder = ConnectParams::builder();
        builder.port(self.port);
        builder.user(&self.user, self.password.as_ref().map(|p| &**p));
        builder.database(&self.database);
        Ok(builder.build(Host::Tcp(self.host.to_string())))
    }
}
