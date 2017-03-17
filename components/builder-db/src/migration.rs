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

use error::{Error, Result};
use pool::Pool;

#[derive(Debug)]
pub struct Migrator<'a> {
    pool: &'a Pool,
    sequence_number: i64,
}

impl<'a> Migrator<'a> {
    pub fn new(pool: &'a Pool) -> Migrator {
        Migrator {
            pool: pool,
            sequence_number: 1,
        }
    }

    pub fn setup(&self) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute(r#"CREATE TABLE IF NOT EXISTS builder_db_migrations (
            prefix text NOT NULL,
            sequence_number bigint NOT NULL,
            created_at timestamptz DEFAULT now(),
            updated_at timestamptz,
            PRIMARY KEY(prefix, sequence_number)
        )"#,
                     &[])
            .map_err(Error::MigrationTable)?;
        conn.execute(r#"CREATE OR REPLACE FUNCTION migration_has_run_v1(p text, sn bigint) RETURNS bool AS $$
        DECLARE
            result BOOLEAN;
        BEGIN
            SELECT true FROM builder_db_migrations WHERE prefix = p AND sequence_number = sn INTO result;
            RETURN result;
        END
        $$ LANGUAGE plpgsql
        "#,
                     &[])
            .map_err(Error::FunctionCreate)?;
        Ok(())
    }

    pub fn check_migration_has_run(&self,
                                   prefix: &str,
                                   sequence_number: i64)
                                   -> Result<Option<bool>> {
        let conn = self.pool.get()?;
        let check_result = conn.query("SELECT migration_has_run_v1($1, $2)",
                                      &[&prefix, &sequence_number])
            .map_err(Error::MigrationCheck)?;

        Ok(check_result.get(0).get(0))
    }

    pub fn migrate(&mut self, prefix: &str, sql: &str) -> Result<()> {
        let result = self.check_migration_has_run(prefix, self.sequence_number)?;
        if !result.is_some() {
            let conn = self.pool.get()?;
            let tr = conn.transaction().map_err(Error::TransactionCreate)?;
            tr.execute(sql, &[]).map_err(Error::Migration)?;
            tr.execute("INSERT INTO builder_db_migrations (prefix, sequence_number) VALUES \
                          ($1, $2)",
                         &[&prefix, &self.sequence_number])
                .map_err(Error::MigrationTracking)?;
            tr.commit().map_err(Error::TransactionCommit)?;
        }
        self.sequence_number += 1;
        Ok(())
    }
}
