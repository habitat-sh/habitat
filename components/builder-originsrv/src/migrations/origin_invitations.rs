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
        r#"CREATE SEQUENCE IF NOT EXISTS origin_invitations_id_seq;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origin_invitations (
                        id bigint PRIMARY KEY DEFAULT next_id_v1('origin_invitations_id_seq'),
                        origin_id bigint REFERENCES origins(id),
                        origin_name text,
                        account_id bigint,
                        account_name text,
                        owner_id bigint,
                        ignored bool DEFAULT false,
                        created_at timestamptz DEFAULT now(),
                        updated_at timestamptz,
                        account_sync bool DEFAULT false,
                        UNIQUE (origin_id, account_id)
                        )"#,
    )?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_origin_invitation_v1 (
                    oi_origin_id bigint,
                    oi_origin_name text,
                    oi_account_id bigint,
                    oi_account_name text,
                    oi_owner_id bigint
                 ) RETURNS SETOF origin_invitations AS $$
                     BEGIN
                        IF NOT EXISTS (SELECT true FROM origin_members WHERE origin_id = oi_origin_id AND account_id = oi_account_id) THEN
                             RETURN QUERY INSERT INTO origin_invitations (origin_id, origin_name, account_id, account_name, owner_id)
                                    VALUES (oi_origin_id, oi_origin_name, oi_account_id, oi_account_name, oi_owner_id)
                                    ON CONFLICT DO NOTHING
                                    RETURNING *;
                             RETURN;
                        END IF;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_invitations_for_origin_v1 (
                   oi_origin_id bigint
                 ) RETURNS SETOF origin_invitations AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_invitations WHERE origin_id = oi_origin_id
                          ORDER BY account_name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_invitations_for_account_v1 (
                   oi_account_id bigint
                 ) RETURNS SETOF origin_invitations AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_invitations WHERE account_id = oi_account_id AND ignored = false
                          ORDER BY origin_name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION accept_origin_invitation_v1 (
                   oi_invite_id bigint, oi_ignore bool
                 ) RETURNS void AS $$
                    DECLARE
                        oi_origin_id bigint;
                        oi_origin_name text;
                        oi_account_id bigint;
                        oi_account_name text;
                    BEGIN
                            IF oi_ignore = true THEN
                                UPDATE origin_invitations SET ignored = true, updated_at = now() WHERE id = oi_invite_id;
                            ELSE
                                SELECT origin_id, origin_name, account_id, account_name INTO oi_origin_id, oi_origin_name, oi_account_id, oi_account_name FROM origin_invitations WHERE id = oi_invite_id;
                                PERFORM insert_origin_member_v1(oi_origin_id, oi_origin_name, oi_account_id, oi_account_name);
                                DELETE FROM origin_invitations WHERE id = oi_invite_id;
                            END IF;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION validate_origin_invitation_v1 (
                   oi_invite_id bigint, oi_account_id bigint
                 ) RETURNS TABLE(is_valid bool) AS $$
                    BEGIN
                        RETURN QUERY SELECT true FROM origin_invitations WHERE id = oi_invite_id AND account_id = oi_account_id;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_invitations_not_synced_with_account_v1 () RETURNS SETOF origin_invitations AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_invitations WHERE account_sync = false
                          ORDER BY created_at ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION set_account_sync_v1 (oi_id bigint) RETURNS void AS $$
                    BEGIN
                        UPDATE origin_invitations SET account_sync = true, updated_at = now() WHERE id = oi_id;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#)?;

    Ok(())
}
