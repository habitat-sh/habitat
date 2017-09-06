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
        r#"CREATE SEQUENCE IF NOT EXISTS origin_id_seq;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origins (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'),
                    name text UNIQUE,
                    owner_id bigint,
                    session_sync bool DEFAULT false,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origin_members (
                    origin_id bigint REFERENCES origins(id),
                    origin_name text,
                    account_id bigint,
                    account_name text,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz,
                    PRIMARY KEY (origin_id, account_id)
                )"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION insert_origin_member_v1 (
                     om_origin_id bigint,
                     om_origin_name text,
                     om_account_id bigint,
                     om_account_name text
                 ) RETURNS void AS $$
                     BEGIN
                         INSERT INTO origin_members (origin_id, origin_name, account_id, account_name)
                                VALUES (om_origin_id, om_origin_name, om_account_id, om_account_name);
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION insert_origin_v1 (
                     origin_name text,
                     origin_owner_id bigint,
                     origin_owner_name text
                 ) RETURNS SETOF origins AS $$
                     DECLARE
                       inserted_origin origins;
                     BEGIN
                         INSERT INTO origins (name, owner_id)
                                VALUES (origin_name, origin_owner_id) RETURNING * into inserted_origin;
                         PERFORM insert_origin_member_v1(inserted_origin.id, origin_name, origin_owner_id, origin_owner_name);
                         PERFORM insert_origin_channel_v1(inserted_origin.id, origin_owner_id, 'unstable');
                         PERFORM insert_origin_channel_v1(inserted_origin.id, origin_owner_id, 'stable');
                         RETURN NEXT inserted_origin;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION list_origin_members_v1 (
                   om_origin_id bigint
                 ) RETURNS TABLE(account_name text) AS $$
                    BEGIN
                        RETURN QUERY SELECT origin_members.account_name FROM origin_members WHERE origin_id = om_origin_id
                          ORDER BY account_name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION check_account_in_origin_members_v1 (
                   om_origin_name text,
                   om_account_id bigint
                 ) RETURNS TABLE(is_member bool) AS $$
                    BEGIN
                        RETURN QUERY SELECT true FROM origin_members WHERE origin_name = om_origin_name AND account_id = om_account_id;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION list_origin_by_account_id_v1 (
                   o_account_id bigint
                 ) RETURNS TABLE(origin_name text) AS $$
                    BEGIN
                        RETURN QUERY SELECT origin_members.origin_name FROM origin_members WHERE account_id = o_account_id
                          ORDER BY origin_name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION sync_origins_v1() RETURNS TABLE(account_id bigint, account_name text, origin_id bigint, origin_name text) AS $$
                    BEGIN
                        RETURN QUERY SELECT origins.owner_id, origin_members.account_name, origins.id, origins.name FROM origins, origin_members WHERE origins.session_sync = false AND origins.owner_id = origin_members.account_id;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION set_session_sync_v1(in_origin_id bigint) RETURNS VOID AS $$
                    BEGIN
                        UPDATE origins SET session_sync = true WHERE id = in_origin_id;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                     r#"ALTER TABLE IF EXISTS origins ADD COLUMN IF NOT EXISTS default_package_visibility text NOT NULL DEFAULT 'public';"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION insert_origin_v2 (
                     origin_name text,
                     origin_owner_id bigint,
                     origin_owner_name text,
                     origin_default_package_visibility text
                 ) RETURNS SETOF origins AS $$
                     DECLARE
                       inserted_origin origins;
                     BEGIN
                         INSERT INTO origins (name, owner_id, default_package_visibility)
                                VALUES (origin_name, origin_owner_id, origin_default_package_visibility) RETURNING * into inserted_origin;
                         PERFORM insert_origin_member_v1(inserted_origin.id, origin_name, origin_owner_id, origin_owner_name);
                         PERFORM insert_origin_channel_v1(inserted_origin.id, origin_owner_id, 'unstable');
                         PERFORM insert_origin_channel_v1(inserted_origin.id, origin_owner_id, 'stable');
                         RETURN NEXT inserted_origin;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION update_origin_v1 (
                        origin_id bigint,
                        op_default_package_visibility text
                 ) RETURNS void AS $$
                        UPDATE origins SET
                            default_package_visibility = op_default_package_visibility,
                            updated_at = now()
                            WHERE id = origin_id;
                 $$ LANGUAGE SQL VOLATILE"#,
    )?;
    Ok(())
}
