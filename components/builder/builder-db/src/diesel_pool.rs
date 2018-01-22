// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

use std::ops::{Deref, DerefMut};
use std::thread;
use std::time::Duration;

use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use config::DataStoreCfg;
use error::{Error, Result};

#[derive(Clone)]
pub struct DieselPool {
    inner: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl DieselPool {
    pub fn new(config: &DataStoreCfg) -> Result<DieselPool> {
        loop {
            let manager = ConnectionManager::<PgConnection>::new(config.to_string());
            match r2d2::Pool::builder()
                .max_size(config.pool_size)
                .connection_timeout(Duration::from_secs(config.connection_timeout_sec))
                .build(manager) {
                Ok(pool) => return Ok(DieselPool { inner: pool }),
                Err(e) => {
                    error!(
                        "Error initializing connection pool to Postgres, will retry: {}",
                        e
                    )
                }
            }
            thread::sleep(Duration::from_millis(config.connection_retry_ms));
        }
    }

    pub fn get_raw(&self) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        let conn = self.inner.get().map_err(Error::ConnectionTimeout)?;
        Ok(conn)
    }
}

impl Deref for DieselPool {
    type Target = r2d2::Pool<ConnectionManager<PgConnection>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DieselPool {
    fn deref_mut(&mut self) -> &mut r2d2::Pool<ConnectionManager<PgConnection>> {
        &mut self.inner
    }
}
