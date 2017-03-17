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
            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("db");
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
                                 false)
                    .expect("Failed to create pool");
            let conn = pool.get().expect("Failed to get connection");
            let _ = conn.execute("DROP DATABASE IF EXISTS builder_db_test", &[]);
            let _ = conn.execute("CREATE DATABASE builder_db_test", &[]);
        })
    }
}

/// The `with_pool` macro injects a `Pool` instance thats dynamically configured to use the test
/// database, and set to create a new schema for the test.
#[macro_export]
macro_rules! with_pool {
    ($pool:ident, $code:block) => {
        use std::time::Duration;
        use $crate::pool::Pool;

        use $crate::test::{init, postgres};

        postgres::start();
        init::create_database();
        let $pool = Pool::new("postgresql://hab@127.0.0.1/builder_db_test", 1, 300, Duration::from_secs(3600), true).expect("Failed to create pool");
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

            use $crate::test::{init, postgres};

            postgres::start();
            init::create_database();
            Pool::new("postgresql://hab@127.0.0.1/builder_db_test", 1, 300, Duration::from_secs(3600), true).expect("Failed to create pool")
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

        use $crate::test::{init, postgres};

        postgres::start();
        init::create_database();
        let $pool = Pool::new("postgresql://hab@127.0.0.1/builder_db_test", 1, 300, Duration::from_secs(3600), true).expect("Failed to create pool");
        let mut $migration = Migrator::new(&$pool);
        $migration.setup().expect("Migration setup failed");
        $code
    }
}
