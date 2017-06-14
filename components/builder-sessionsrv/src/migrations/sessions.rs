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
                 r#"CREATE TABLE IF NOT EXISTS account_sessions (
                        account_id bigint REFERENCES accounts(id),
                        token text,
                        provider text,
                        extern_id bigint,
                        is_admin bool DEFAULT false,
                        is_early_access bool DEFAULT false,
                        is_build_worker bool DEFAULT false,
                        created_at timestamptz DEFAULT now(),
                        expires_at timestamptz DEFAULT now() + interval '1 day',
                        UNIQUE (account_id)
                        )"#)?;
    migrator.migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_account_session_v1 (
                    a_account_id bigint,
                    account_token text,
                    account_provider text,
                    account_extern_id bigint,
                    account_is_admin bool,
                    account_is_early_access bool,
                    account_is_build_worker bool
                 ) RETURNS SETOF account_sessions AS $$
                     BEGIN
                        RETURN QUERY INSERT INTO account_sessions (account_id, token, provider, extern_id, is_admin, is_early_access, is_build_worker)
                                        VALUES (a_account_id, account_token, account_provider, account_extern_id, account_is_admin, account_is_early_access, account_is_build_worker)
                                        ON CONFLICT (account_id) DO UPDATE
                                        SET token = account_token, expires_at = now() + interval '1 day', provider = account_provider, extern_id = account_extern_id, is_admin = account_is_admin, is_early_access = account_is_early_access, is_build_worker = account_is_build_worker
                                        RETURNING *;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION get_account_session_v1 (
                    account_name text,
                    account_token text
                 ) RETURNS TABLE(id bigint, email text, name text, token text, is_admin bool, is_early_access bool, is_build_worker bool) AS $$
                     DECLARE
                        this_account accounts%rowtype;
                     BEGIN
                        SELECT * FROM accounts WHERE accounts.name = account_name LIMIT 1 INTO this_account;
                        IF FOUND THEN
                            DELETE FROM account_sessions WHERE account_id = this_account.id AND account_sessions.token = account_token AND expires_at < now();
                            IF NOT FOUND THEN
                                RETURN QUERY
                                    SELECT accounts.id, accounts.email,
                                           accounts.name, account_sessions.token,
                                           account_sessions.is_admin,
                                           account_sessions.is_early_access,
                                           account_sessions.is_build_worker
                                      FROM accounts
                                        INNER JOIN account_sessions ON account_sessions.account_id = accounts.id
                                      WHERE accounts.id = this_account.id
                                        AND account_sessions.token = account_token;
                            END IF;
                        END IF;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;

    Ok(())
}
