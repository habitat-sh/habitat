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
    use std::time::Duration;

    use std::sync::{Once, ONCE_INIT};

    use pool::Pool;

    static INIT: Once = ONCE_INIT;
    pub fn create_database() {
        INIT.call_once(|| {
            let pool = Pool::new("postgresql://hab@127.0.0.1/template1",
                                 1,
                                 300,
                                 Duration::from_secs(3600),
                                 vec![0])
                    .expect("Failed to create pool");
            let conn = pool.get_raw().expect("Failed to get connection");
            let _ = conn.execute("DROP DATABASE IF EXISTS builder_db_test_template", &[]);
            let _ = conn.execute("CREATE DATABASE builder_db_test_template", &[]);
        })
    }
}

#[macro_export]
macro_rules! datastore_test {
    ($datastore:ident) => {
        {
            use std::time::Duration;
            use std::sync::atomic::Ordering;
            use $crate::pool::Pool;

            use $crate::test::{postgres, SHARD_COUNT, INIT_TEMPLATE, TEST_COUNT};

            postgres::start();

            INIT_TEMPLATE.call_once(|| {
                let pool = Pool::new("postgresql://hab@127.0.0.1/template1",
                                     1,
                                     300,
                                     Duration::from_secs(3600),
                                     vec![0])
                    .expect("Failed to create pool");
                let conn = pool.get_raw().expect("Failed to get connection");
                let _ = conn.execute("DROP DATABASE IF EXISTS builder_db_test_template", &[]).expect("Failed to drop existing template database");
                let _ = conn.execute("CREATE DATABASE builder_db_test_template", &[]).expect("Failed to create template database");
                let template_pool = Pool::new("postgresql://hab@127.0.0.1/builder_db_test_template", 1, 300, Duration::from_secs(3600), (0..SHARD_COUNT).collect()).expect("Failed to create pool");
                let ds = $datastore::from_pool(template_pool).expect("Failed to create data store from pool");
                ds.setup().expect("Failed to migrate data");
            });
            let test_number = TEST_COUNT.fetch_add(1, Ordering::SeqCst);
            let db_name = format!("builder_db_test_{}", test_number);
            let create_pool = Pool::new("postgresql://hab@127.0.0.1/template1",
                                        1,
                                        300,
                                        Duration::from_secs(3600),
                                        vec![0])
                .expect("Failed to create pool");
            let conn = create_pool.get_raw().expect("Failed to get connection");
            let drop_db = format!("DROP DATABASE IF EXISTS {}", &db_name);
            let create_db = format!("CREATE DATABASE {} TEMPLATE builder_db_test_template", &db_name);

            let _ = conn.execute(&drop_db, &[]).expect("Failed to drop test database");
            let _ = conn.execute(&create_db, &[]).expect("Failed to create test database from template");

            let pool = Pool::new(&format!("postgresql://hab@127.0.0.1/{}", db_name), 5, 300, Duration::from_secs(3600), (0..SHARD_COUNT).collect()).expect("Failed to create pool");
            let ds = $datastore::from_pool(pool).expect("Failed to create data store from pool");
            ds
        }
    }
}

/// The `with_pool` macro injects a `Pool` instance thats dynamically configured to use the test
/// database, and set to create a new schema for the test.
#[macro_export]
macro_rules! with_pool {
    ($pool:ident, $code:block) => {
        use std::time::Duration;
        use $crate::pool::Pool;

        use $crate::test::{init, postgres, SHARD_COUNT};

        postgres::start();
        init::create_database();
        let $pool = Pool::new("postgresql://hab@127.0.0.1/builder_db_test", 5, 300, Duration::from_secs(3600), (0..SHARD_COUNT).collect()).expect("Failed to create pool");
        $code
    }
}

/// The `with_pool` macro injects a `Pool` instance thats dynamically configured to use the test
/// database, and set to create a new schema for the test.
#[macro_export]
macro_rules! pool {
    () => {
         {
            use std::time::Duration;
            use $crate::pool::Pool;

            use $crate::test::{init, postgres, SHARD_COUNT};

            postgres::start();
            init::create_database();
            Pool::new("postgresql://hab@127.0.0.1/builder_db_test", 5, 300, Duration::from_secs(3600), (0..SHARD_COUNT).collect(), true).expect("Failed to create pool")
        }
    }
}

/// The `with_migration` macro injects both a new `Pool` and a `Migration` into your test.
#[macro_export]
macro_rules! with_migration {
    ($pool:ident, $migration:ident, $code:block) => {
        use std::time::Duration;
        use $crate::migration::Migrator;
        use $crate::pool::Pool;

        use $crate::test::{init, postgres, SHARD_COUNT};

        postgres::start();
        init::create_database();
        let $pool = Pool::new("postgresql://hab@127.0.0.1/builder_db_test", 5, 300, Duration::from_secs(3600), (0..SHARD_COUNT).collect(), true).expect("Failed to create pool");
        let conn = $pool.get_raw().expect("Failed to get connection for migration");
        let xact = conn.transaction().expect("Failed to get transaction for migration");
        let mut $migration = Migrator::new(xact, (0..SHARD_COUNT).collect());
        $migration.testing = true;
        $migration.test_number = $pool.test_number;
        $migration.setup().expect("Migration setup failed");
        $code
    }
}
