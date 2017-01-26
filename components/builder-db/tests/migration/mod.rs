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

use db::migration::Migrator;

#[test]
fn setup() {
    with_pool!(pool, {
        let migrator = Migrator::new(&pool);
        migrator.setup().expect("Migration setup failed");
        migrator.setup().expect("Migration setup must be idempotent");
    });
}

#[test]
fn migrate() {
    with_migration!(pool, migration, {
        migration.migrate("metal",
                     1,
                     r#"CREATE TABLE bands (
                        name text PRIMARY KEY,
                        style text
                     )"#)
            .expect("Migration should be run successfully");

        // Running the same migration twice should not fail, due to the internal migration checking
        // logic.
        migration.migrate("metal",
                     1,
                     r#"CREATE TABLE bands (
                        name text PRIMARY KEY,
                        style text
                     )"#)
            .expect("Migration should pass if its sequence number has been used, even if it \
                     should fail by sql");
    });

    // ml.migrate("packages", 1, r#"CREATE TABLE packages (
    //     ident text PRIMARY KEY,
    //     origin text,
    //     name text,
    //     version text,
    //     release text,
    //     checksum text,
    //     manifest text,
    //     deps text[],
    //     tdeps text[],
    //     exposes integer[],
    //     config text,
    //     created_at timestamptz DEFAULT now(),
    //     updated_at timestamptz,
    // )"#);
}
