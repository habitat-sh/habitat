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
        .migrate("accountsrv",
                 r#"CREATE SEQUENCE IF NOT EXISTS account_invitations_id_seq;"#)?;
    migrator
        .migrate("accountsrv",
                 r#"CREATE TABLE account_invitations (
                        id bigint PRIMARY KEY DEFAULT next_id_v1('account_invitations_id_seq'),
                        origin_invitation_id bigint,
                        origin_id bigint,
                        origin_name text,
                        account_id bigint REFERENCES accounts(id),
                        account_name text REFERENCES accounts(name),
                        owner_id bigint,
                        ignored bool DEFAULT false,
                        created_at timestamptz DEFAULT now(),
                        updated_at timestamptz,
                        UNIQUE (origin_id, account_id)
                        )"#)?;
    migrator.migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_account_invitation_v1 (
                    oi_origin_id bigint,
                    oi_origin_name text,
                    oi_origin_invitation_id bigint,
                    oi_account_id bigint,
                    oi_account_name text,
                    oi_owner_id bigint
                 ) RETURNS SETOF account_invitations AS $$
                     BEGIN
                        IF NOT EXISTS (SELECT true FROM account_origins WHERE origin_id = oi_origin_id AND account_id = oi_account_id) THEN
                             RETURN QUERY INSERT INTO account_invitations (origin_id, origin_invitation_id, origin_name, account_id, account_name, owner_id)
                                    VALUES (oi_origin_id, oi_origin_invitation_id, oi_origin_name, oi_account_id, oi_account_name, oi_owner_id)
                                    ON CONFLICT DO NOTHING
                                    RETURNING *;
                             RETURN;
                        END IF;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("accountsrv",
                     r#"CREATE OR REPLACE FUNCTION get_invitations_for_account_v1 (
                   oi_account_id bigint
                 ) RETURNS SETOF account_invitations AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM account_invitations WHERE account_id = oi_account_id AND ignored = false
                          ORDER BY origin_name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION accept_account_invitation_v1 (
                   oi_invite_id bigint, oi_ignore bool
                 ) RETURNS void AS $$
                    DECLARE
                        oi_origin_id bigint;
                        oi_origin_name text;
                        oi_account_id bigint;
                        oi_account_name text;
                    BEGIN
                            IF oi_ignore = true THEN
                                UPDATE account_invitations SET ignored = true, updated_at = now() WHERE origin_invitation_id = oi_invite_id;
                            ELSE
                                SELECT origin_id, origin_name, account_id, account_name INTO oi_origin_id, oi_origin_name, oi_account_id, oi_account_name FROM account_invitations WHERE origin_invitation_id = oi_invite_id;
                                PERFORM insert_account_origin_v1(oi_account_id, oi_account_name, oi_origin_id, oi_origin_name);
                                DELETE FROM account_invitations WHERE origin_invitation_id = oi_invite_id;
                            END IF;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#)?;
    Ok(())
}
