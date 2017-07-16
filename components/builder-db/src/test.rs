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

//! Shared code for testing crates that use this database layer.
//!
//! The design uses a database with dynamically created schemas per test, that automatically handle
//! running migrations (if required). Each test *must* have its schema built every time.

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
use std::sync::{Once, ONCE_INIT};

pub use protocol::sharding::SHARD_COUNT;

pub static INIT_TEMPLATE: Once = ONCE_INIT;
pub static TEST_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

pub mod postgres {
    use std::path::PathBuf;
    use std::process::{Child, Command, Stdio};
    use std::sync::{Once, ONCE_INIT};
    use std::thread;

    struct Postgres {
        inner: Child,
    }

    static POSTGRES: Once = ONCE_INIT;

    pub fn start() {
        POSTGRES.call_once(|| {
            thread::spawn(move || {
                let mut postgres = Postgres::new();
                let _ = postgres.inner.wait();
            });
        });
    }

    impl Postgres {
        fn new() -> Postgres {
            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("db");
            let start_path = root_path.join("start.sh");
            let child = Command::new("sudo")
                .arg("-E")
                .arg(start_path)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .env("DB_TEST_DIR", root_path)
                .current_dir("/tmp")
                .spawn()
                .expect("Failed to launch core/postgresql");
            Postgres { inner: child }
        }
    }
}

pub mod init {
    use std::sync::{Once, ONCE_INIT};

    use config::DataStoreCfg;
    use pool::Pool;

    static INIT: Once = ONCE_INIT;
    pub fn create_database() {
        INIT.call_once(|| {
            let mut config = DataStoreCfg::default();
            config.database = "template1".to_string();
            config.pool_size = 1;
            let pool = Pool::new(&config, vec![0]).expect("Failed to create pool");
            let conn = pool.get_raw().expect("Failed to get connection");
            let _ = conn.execute("DROP DATABASE IF EXISTS builder_db_test_template", &[]);
            let _ = conn.execute("CREATE DATABASE builder_db_test_template", &[]);
        })
    }
}

#[macro_export]
#[allow(unused_must_use)]
macro_rules! datastore_test {
    ($datastore:ident) => {
        {
            use std::sync::Arc;
            use std::sync::atomic::Ordering;
            use $crate::config::DataStoreCfg;
            use $crate::pool::Pool;
            use $crate::test::{postgres, SHARD_COUNT, INIT_TEMPLATE, TEST_COUNT};

            postgres::start();

            // JW: We don't run any tests which need to communicate between RouteSrv->DataStore
            // implementor so we don't need to provide an actual pipe for a RouteClient to
            // connect to
            let router_pipe = Arc::new("inproc://database-test.ipc".to_string());

            INIT_TEMPLATE.call_once(|| {
                let mut config = DataStoreCfg::default();
                config.database = "template1".to_string();
                config.pool_size = 1;
                let pool = Pool::new(&config, vec![0]).expect("Failed to create pool");
                let conn = pool.get_raw().expect("Failed to get connection");
                conn.execute("DROP DATABASE IF EXISTS builder_db_test_template", &[])
                    .expect("Failed to drop existing template database");
                conn.execute("CREATE DATABASE builder_db_test_template", &[])
                    .expect("Failed to create template database");
                config.database = "builder_db_test_template".to_string();
                let template_pool = Pool::new(&config, (0..SHARD_COUNT).collect())
                    .expect("Failed to create pool");
                let ds = $datastore::from_pool(template_pool, router_pipe.clone())
                    .expect("Failed to create data store from pool");
                ds.setup().expect("Failed to migrate data");
            });
            let test_number = TEST_COUNT.fetch_add(1, Ordering::SeqCst);
            let db_name = format!("builder_db_test_{}", test_number);
            let mut config = DataStoreCfg::default();
            config.database = "template1".to_string();
            config.pool_size = 1;
            let create_pool = Pool::new(&config, vec![0]).expect("Failed to create pool");
            let conn = create_pool.get_raw().expect("Failed to get connection");
            let drop_db = format!("DROP DATABASE IF EXISTS {}", db_name);
            let create_db =
                format!("CREATE DATABASE {} TEMPLATE builder_db_test_template", db_name);
            conn.execute(&drop_db, &[]).expect("Failed to drop test database");
            conn.execute(&create_db, &[]).expect("Failed to create test database from template");

            config.database = db_name;
            config.pool_size = 5;
            let pool = Pool::new(&config, (0..SHARD_COUNT).collect())
                .expect("Failed to create pool");
            $datastore::from_pool(pool, router_pipe).expect("Failed to create data store from pool")
        }
    }
}

/// The `with_pool` macro injects a `Pool` instance thats dynamically configured to use the test
/// database, and set to create a new schema for the test.
#[macro_export]
macro_rules! with_pool {
    ($pool:ident, $code:block) => {
        use std::sync::atomic::Ordering;
        use $crate::config::DataStoreCfg;
        use $crate::pool::Pool;
        use $crate::test::{init, postgres, SHARD_COUNT, TEST_COUNT};

        postgres::start();
        init::create_database();
        let test_number = TEST_COUNT.fetch_add(1, Ordering::SeqCst);
        let db_name = format!("builder_db_test_{}", test_number);
        let mut config = DataStoreCfg::default();
        config.database = "template1".to_string();
        config.pool_size = 1;
        let create_pool = Pool::new(&config, vec![0]).expect("Failed to create pool");
        let conn = create_pool.get_raw().expect("Failed to get connection");
        let drop_db = format!("DROP DATABASE IF EXISTS {}", db_name);
        let create_db = format!("CREATE DATABASE {} TEMPLATE builder_db_test_template", db_name);
        conn.execute(&drop_db, &[]).expect("Failed to drop test database");
        conn.execute(&create_db, &[]).expect("Failed to create test database from template");

        config.database = db_name;
        config.pool_size = 5;
        let $pool = Pool::new(&config, (0..SHARD_COUNT).collect()).expect("Failed to create pool");
        $code
    }
}
