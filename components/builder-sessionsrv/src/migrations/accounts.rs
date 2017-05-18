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
                 r#"CREATE SEQUENCE IF NOT EXISTS accounts_id_seq;"#)?;
    migrator
        .migrate("accountsrv",
                 r#"CREATE TABLE accounts (
                        id bigint PRIMARY KEY DEFAULT next_id_v1('accounts_id_seq'),
                        name text UNIQUE,
                        email text UNIQUE,
                        created_at timestamptz DEFAULT now(),
                        updated_at timestamptz
                        )"#)?;
    migrator.migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION select_or_insert_account_v1 (
                    account_name text,
                    account_email text
                 ) RETURNS SETOF accounts AS $$
                     DECLARE
                        existing_account accounts%rowtype;
                     BEGIN
                        SELECT * INTO existing_account FROM accounts WHERE name = account_name LIMIT 1;
                        IF FOUND THEN
                            RETURN NEXT existing_account;
                        ELSE
                            RETURN QUERY INSERT INTO accounts (name, email) VALUES (account_name, account_email) ON CONFLICT DO NOTHING RETURNING *;
                        END IF;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;

    migrator
        .migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION get_account_by_name_v1 (
                    account_name text
                 ) RETURNS SETOF accounts AS $$
                     BEGIN
                        RETURN QUERY SELECT * FROM accounts WHERE name = account_name;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql STABLE"#)?;

    migrator
        .migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION get_account_by_id_v1 (
                    account_id bigint
                 ) RETURNS SETOF accounts AS $$
                     BEGIN
                        RETURN QUERY SELECT * FROM accounts WHERE id = account_id;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql STABLE"#)?;
    migrator
        .migrate("accountsrv",
                 r#"CREATE TABLE account_origins (
                        account_id bigint,
                        account_name text,
                        origin_id bigint,
                        origin_name text,
                        created_at timestamptz DEFAULT now(),
                        updated_at timestamptz,
                        UNIQUE(account_id, origin_id)
                        )"#)?;
    migrator.migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_account_origin_v1 (
                    o_account_id bigint,
                    o_account_name text,
                    o_origin_id bigint,
                    o_origin_name text
                 ) RETURNS void AS $$
                     BEGIN
                        INSERT INTO account_origins (account_id, account_name, origin_id, origin_name) VALUES (o_account_id, o_account_name, o_origin_id, o_origin_name);
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator
        .migrate("accountsrv",
                 r#"CREATE OR REPLACE FUNCTION get_account_origins_v1 (
                    in_account_id bigint
                 ) RETURNS SETOF account_origins AS $$
                     BEGIN
                        RETURN QUERY SELECT * FROM account_origins WHERE account_id = in_account_id;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql STABLE"#)?;

    Ok(())
}
