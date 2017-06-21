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

use error::Result;

pub fn migrate(migrator: &mut Migrator) -> Result<()> {
    migrator.migrate(
        "originsrv",
        r#"CREATE SEQUENCE IF NOT EXISTS origin_public_key_id_seq;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origin_public_keys (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('origin_public_key_id_seq'),
                    origin_id bigint REFERENCES origins(id),
                    owner_id bigint,
                    name text,
                    revision text,
                    full_name text UNIQUE,
                    body bytea,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#,
    )?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_origin_public_key_v1 (
                    opk_origin_id bigint,
                    opk_owner_id bigint,
                    opk_name text,
                    opk_revision text,
                    opk_full_name text,
                    opk_body bytea
                 ) RETURNS SETOF origin_public_keys AS $$
                     BEGIN
                         RETURN QUERY INSERT INTO origin_public_keys (origin_id, owner_id, name, revision, full_name, body)
                                VALUES (opk_origin_id, opk_owner_id, opk_name, opk_revision, opk_full_name, opk_body)
                                RETURNING *;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION get_origin_public_key_v1 (
                    opk_name text,
                    opk_revision text
                 ) RETURNS SETOF origin_public_keys AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_public_keys WHERE name = opk_name and revision = opk_revision
                          ORDER BY revision DESC
                          LIMIT 1;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_public_key_latest_v1 (
                    opk_name text
                 ) RETURNS SETOF origin_public_keys AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_public_keys WHERE name = opk_name
                          ORDER BY revision DESC
                          LIMIT 1;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_public_keys_for_origin_v1 (
                   opk_origin_id bigint
                 ) RETURNS SETOF origin_public_keys AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_public_keys WHERE origin_id = opk_origin_id
                          ORDER BY revision DESC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate(
        "originsrv",
        r#"ALTER TABLE origin_public_keys
                        DROP CONSTRAINT IF EXISTS
                          origin_public_keys_full_name_key"#,
    )?;
    migrator.migrate(
        "originsrv-2",
        r#"DELETE FROM origin_public_keys
                        WHERE id IN (
                            SELECT id FROM (
                                SELECT id, ROW_NUMBER() OVER (
                                    partition BY full_name ORDER BY id
                                ) AS rnum FROM origin_public_keys
                            ) t WHERE t.rnum > 1)"#,
    )?;
    migrator.migrate(
        "originsrv-3",
        r#"ALTER TABLE origin_public_keys
                        ADD CONSTRAINT origin_public_keys_full_name_key
                        UNIQUE (full_name)"#,
    )?;
    Ok(())
}
