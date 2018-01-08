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

use error::Result;
use diesel::query_dsl::RunQueryDsl;
use diesel::sql_query;
use diesel::pg::PgConnection;

pub fn shard_setup(conn: &PgConnection, shard_id: u32) -> Result<()> {
    debug!("Migrating shard_{:?}", shard_id);
    sql_query(format!("CREATE SCHEMA IF NOT EXISTS shard_{}", shard_id))
        .execute(conn)
        .unwrap();
    sql_query(format!("SET SEARCH_PATH TO shard_{}", shard_id))
        .execute(conn)
        .unwrap();
    // TED: We have to do this here because we rely on the shard id which isn't available in raw sql
    sql_query(format!(
        r#"CREATE OR REPLACE FUNCTION next_id_v1(sequence_id regclass, OUT result bigint) AS $$
                DECLARE
                    our_epoch bigint := 1409266191000;
                    seq_id bigint;
                    now_millis bigint;
                    shard_id int := {};
                BEGIN
                    SELECT nextval(sequence_id) % 1024 INTO seq_id;
                    SELECT FLOOR(EXTRACT(EPOCH FROM clock_timestamp()) * 1000) INTO now_millis;
                    result := (now_millis - our_epoch) << 23;
                    result := result | (seq_id << 13);
                    result := result | (shard_id);
                END;
                $$ LANGUAGE PLPGSQL;"#,
        shard_id
    )).execute(conn)
        .unwrap();
    Ok(())
}
