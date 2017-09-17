// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use error::SrvResult;

pub fn migrate(migrator: &mut Migrator) -> SrvResult<()> {
    migrator.migrate(
        "originsrv",
        r#"CREATE SEQUENCE IF NOT EXISTS origin_integration_id_seq;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origin_integrations (
                        id bigint PRIMARY KEY DEFAULT next_id_v1('origin_integration_id_seq'),
                        origin text,
                        integration text,
                        name text,
                        body text,
                        created_at timestamptz DEFAULT now(),
                        updated_at timestamptz,
                        UNIQUE (origin, name)
                        )"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION insert_origin_integration_v1 (
                        in_origin text,
                        in_integration text,
                        in_name text,
                        in_body text
                 ) RETURNS SETOF origin_integrations AS $$
                        INSERT INTO origin_integrations(
                                      origin,
                                      integration,
                                      name,
                                      body)
                        VALUES (
                            in_origin,
                            in_integration,
                            in_name,
                            in_body)
                        RETURNING *
                 $$ LANGUAGE SQL VOLATILE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_integrations_v1 (
                        in_origin text,
                        in_integration text
                 ) RETURNS SETOF origin_integrations AS $$
                        SELECT * FROM origin_integrations
                        WHERE origin = in_origin AND integration = in_integration
                    $$ LANGUAGE SQL STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION delete_origin_integration_v1 (
                        in_origin text,
                        in_integration text,
                        in_name text
                 ) RETURNS void AS $$
                        DELETE FROM origin_integrations
                        WHERE origin = in_origin AND integration = in_integration AND name = in_name
                    $$ LANGUAGE SQL VOLATILE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_integrations_for_origin_v1 (
                        in_origin text
                 ) RETURNS SETOF origin_integrations AS $$
                        SELECT * FROM origin_integrations
                        WHERE origin = in_origin
                        ORDER BY integration, name
                    $$ LANGUAGE SQL STABLE"#,
    )?;
    Ok(())
}
