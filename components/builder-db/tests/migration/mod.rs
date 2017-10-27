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

use db::migration::Migrator;

#[test]
#[allow(unused_must_use)]
fn setup() {
    with_pool!(pool, {
        let conn = pool.get_raw().expect("cannot get connection");
        let xact = conn.transaction().expect("cannot get transaction");
        let migrator = Migrator::new(xact, vec![0, 1]);
        migrator.setup().expect("Migration setup failed");
        migrator.setup().expect(
            "Migration setup must be idempotent",
        );
        migrator.finish();
    });
}

#[test]
#[allow(unused_must_use)]
fn migrate() {
    with_pool!(pool, {
        let conn = pool.get_raw().expect("cannot get connection");
        {
            let xact = conn.transaction().expect("cannot get transaction");
            let mut migrator = Migrator::new(xact, vec![0, 1]);
            migrator.setup().expect(
                "Migration setup must be idempotent",
            );
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE bands (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator.finish();
        }
        {
            let xact = conn.transaction().expect("cannot get transaction");
            let mut migrator = Migrator::new(xact, vec![0, 1]);
            migrator.setup().expect(
                "Migration setup must be idempotent",
            );
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE bands (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect(
                    "Migration should be run successfully, even thought it would fail if it \
                         was run twice",
                );
            migrator.finish();
        }
    });
}

#[test]
#[allow(unused_must_use)]
fn migrate_out_of_order_edits() {
    with_pool!(pool, {
        let conn = pool.get_raw().expect("cannot get connection");
        {
            let xact = conn.transaction().expect("cannot get transaction");
            let mut migrator = Migrator::new(xact, vec![0, 1]);
            migrator.setup().expect(
                "Migration setup must be idempotent",
            );
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE bands (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE cars (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator.finish();
        }
        {
            let xact = conn.transaction().expect("cannot get transaction");
            let mut migrator = Migrator::new(xact, vec![0, 1]);
            migrator.setup().expect(
                "Migration setup must be idempotent",
            );
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE bands (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE hats (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE cars (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator.finish();
        }
        {
            let xact = conn.transaction().expect("cannot get transaction");
            let mut migrator = Migrator::new(xact, vec![0, 1]);
            migrator.setup().expect(
                "Migration setup must be idempotent",
            );
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE bands (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE hats (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE cars (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator
                .migrate(
                    "metal",
                    r#"CREATE TABLE forks (
                                    name text PRIMARY KEY,
                                    style text
                                 )"#,
                )
                .expect("Migration should be run successfully");
            migrator.finish();
        }
        {
            let xact = conn.transaction().expect("cannot get transaction");
            xact.execute("SELECT * FROM bands", &[]).expect(
                "Table should exist",
            );
            xact.execute("SELECT * FROM hats", &[]).expect(
                "Table should exist",
            );
            xact.execute("SELECT * FROM cars", &[]).expect(
                "Table should exist",
            );
            xact.execute("SELECT * FROM forks", &[]).expect(
                "Table should exist",
            );
        }
    });
}
