// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.  // You may obtain a copy of the License at
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
    migrator.migrate("originsrv",
                     r#"CREATE SEQUENCE IF NOT EXISTS origin_channel_id_seq;"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE TABLE origin_channels (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('origin_channel_id_seq'),
                    origin_id bigint REFERENCES origins(id),
                    owner_id bigint,
                    name text,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION insert_origin_channel_v1 (
                    occ_origin_id bigint,
                    occ_owner_id bigint,
                    occ_name text
                 ) RETURNS SETOF origin_channels AS $$
                     BEGIN
                         RETURN QUERY INSERT INTO origin_channels (origin_id, owner_id, name)
                                VALUES (occ_origin_id, occ_owner_id, occ_name)
                                RETURNING *;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_channels_for_origin_v1 (
                   occ_origin_id bigint
                 ) RETURNS SETOF origin_channels AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_channels WHERE origin_id = occ_origin_id
                          ORDER BY name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    Ok(())
}
