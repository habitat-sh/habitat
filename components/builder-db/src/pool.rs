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

use std::ops::{Deref, DerefMut};
use std::result;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::thread;
use std::time::Duration;

use r2d2;
use r2d2_postgres::{self, PostgresConnectionManager, TlsMode};
use postgres::transaction::Transaction;
use postgres;

use error::{Error, Result};

// We will use this to allocate test schmeas
static GLOBAL_SCHEMA_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Debug, Clone)]
pub struct Pool {
    inner: r2d2::Pool<PostgresConnectionManager>,
}

impl Pool {
    pub fn new(connection_url: &str,
               pool_size: u32,
               connection_retry_ms: u64,
               connection_timeout: Duration,
               testing: bool)
               -> Result<Pool> {
        loop {
            let pool_config_builder = r2d2::Config::builder()
                .pool_size(pool_size)
                .connection_timeout(connection_timeout);
            let pool_config = if testing {
                pool_config_builder.connection_customizer(Box::new(TestConnectionCustomizer {}))
                    .build()
            } else {
                pool_config_builder.build()
            };
            let manager = PostgresConnectionManager::new(connection_url, TlsMode::None)?;
            match r2d2::Pool::new(pool_config, manager) {
                Ok(pool) => return Ok(Pool { inner: pool }),
                Err(e) => {
                    error!("Error initializing connection pool to Postgres, will retry: {}",
                           e)
                }
            }
            thread::sleep(Duration::from_millis(connection_retry_ms));
        }
    }
}

impl Deref for Pool {
    type Target = r2d2::Pool<PostgresConnectionManager>;

    fn deref(&self) -> &r2d2::Pool<PostgresConnectionManager> {
        &self.inner
    }
}

impl DerefMut for Pool {
    fn deref_mut(&mut self) -> &mut r2d2::Pool<PostgresConnectionManager> {
        &mut self.inner
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TestConnectionCustomizer;

impl r2d2::CustomizeConnection<postgres::Connection, r2d2_postgres::Error> for TestConnectionCustomizer {
    fn on_acquire(&self, conn: &mut postgres::Connection) -> result::Result<(), r2d2_postgres::Error> {
        let schema_number = GLOBAL_SCHEMA_COUNT.fetch_add(1, Ordering::SeqCst);
        let sql_drop_schema = format!("DROP SCHEMA IF EXISTS builder_db_test_{} CASCADE", schema_number);
        let sql_create_schema = format!("CREATE SCHEMA builder_db_test_{}", schema_number);
        let sql_search_path = format!("SET search_path TO builder_db_test_{}", schema_number);
        conn.execute(&sql_drop_schema, &[]).map_err(r2d2_postgres::Error::Other)?;
        conn.execute(&sql_create_schema, &[]).map_err(r2d2_postgres::Error::Other)?;
        conn.execute(&sql_search_path, &[]).map_err(r2d2_postgres::Error::Other)?;
        Ok(())
    }
}
