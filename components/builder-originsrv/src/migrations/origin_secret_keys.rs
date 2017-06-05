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
    migrator
        .migrate("originsrv",
                 r#"CREATE SEQUENCE IF NOT EXISTS origin_secret_key_id_seq;"#)?;
    migrator
        .migrate("originsrv",
                 r#"CREATE TABLE origin_secret_keys (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('origin_secret_key_id_seq'),
                    origin_id bigint REFERENCES origins(id),
                    owner_id bigint,
                    name text,
                    revision text,
                    full_name text UNIQUE,
                    body bytea,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#)?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE VIEW origins_with_secret_key_full_name_v1 AS
                        SELECT origins.id, origins.name, origins.owner_id,
                               origin_secret_keys.full_name AS private_key_name
                          FROM origins
                          LEFT OUTER JOIN origin_secret_keys ON (origins.id = origin_secret_keys.origin_id)
                          ORDER BY origins.id, origin_secret_keys.full_name DESC"#)?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_origin_secret_key_v1 (
                    osk_origin_id bigint,
                    osk_owner_id bigint,
                    osk_name text,
                    osk_revision text,
                    osk_full_name text,
                    osk_body bytea
                 ) RETURNS SETOF origin_secret_keys AS $$
                     BEGIN
                         RETURN QUERY INSERT INTO origin_secret_keys (origin_id, owner_id, name, revision, full_name, body)
                                VALUES (osk_origin_id, osk_owner_id, osk_name, osk_revision, osk_full_name, osk_body)
                                RETURNING *;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator
        .migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION get_origin_secret_key_v1 (
                    osk_name text
                 ) RETURNS SETOF origin_secret_keys AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_secret_keys WHERE name = osk_name
                          ORDER BY full_name DESC
                          LIMIT 1;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator
        .migrate("originsrv",
                 r#"ALTER TABLE origin_secret_keys
                        DROP CONSTRAINT IF EXISTS
                          origin_secret_keys_full_name_key"#)?;
    migrator
        .migrate("originsrv-2",
                 r#"DELETE FROM origin_secret_keys
                        WHERE id IN (
                            SELECT id FROM (
                                SELECT id, ROW_NUMBER() OVER (
                                    partition BY full_name ORDER BY id
                                ) AS rnum FROM origin_secret_keys
                            ) t WHERE t.rnum > 1)"#)?;
    migrator
        .migrate("originsrv-3",
                 r#"ALTER TABLE origin_secret_keys
                        ADD CONSTRAINT origin_secret_keys_full_name_key
                        UNIQUE (full_name)"#)?;
    Ok(())
}
