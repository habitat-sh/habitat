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

use error::SrvResult;

pub fn migrate(migrator: &mut Migrator) -> SrvResult<()> {
    migrator.migrate(
        "accountsrv",
        r#"CREATE SEQUENCE IF NOT EXISTS accounts_id_seq;"#,
    )?;
    migrator.migrate(
        "accountsrv",
        r#"CREATE TABLE IF NOT EXISTS accounts (
                        id bigint PRIMARY KEY DEFAULT next_id_v1('accounts_id_seq'),
                        name text UNIQUE,
                        email text UNIQUE,
                        created_at timestamptz DEFAULT now(),
                        updated_at timestamptz
                        )"#,
    )?;
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

    migrator.migrate(
        "accountsrv",
        r#"CREATE OR REPLACE FUNCTION get_account_by_name_v1 (
                    account_name text
                 ) RETURNS SETOF accounts AS $$
                     BEGIN
                        RETURN QUERY SELECT * FROM accounts WHERE name = account_name;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql STABLE"#,
    )?;

    migrator.migrate(
        "accountsrv",
        r#"CREATE OR REPLACE FUNCTION get_account_by_id_v1 (
                    account_id bigint
                 ) RETURNS SETOF accounts AS $$
                     BEGIN
                        RETURN QUERY SELECT * FROM accounts WHERE id = account_id;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate(
        "accountsrv",
        r#"CREATE TABLE IF NOT EXISTS account_origins (
                        account_id bigint,
                        account_name text,
                        origin_id bigint,
                        origin_name text,
                        created_at timestamptz DEFAULT now(),
                        updated_at timestamptz,
                        UNIQUE(account_id, origin_id)
                        )"#,
    )?;
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
    migrator.migrate(
        "accountsrv",
        r#"CREATE OR REPLACE FUNCTION get_account_origins_v1 (
                    in_account_id bigint
                 ) RETURNS SETOF account_origins AS $$
                     BEGIN
                        RETURN QUERY SELECT * FROM account_origins WHERE account_id = in_account_id;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate("accountsrv",
                     r#"CREATE OR REPLACE FUNCTION delete_account_origin_v1 (
                    aod_account_name text,
                    aod_origin_id bigint
                 ) RETURNS void AS $$
                        DELETE FROM account_origins WHERE account_name=aod_account_name AND origin_id=aod_origin_id;
                 $$ LANGUAGE SQL VOLATILE"#)?;

    // This query is a report that's meant to be run manually. There's no code path (as of
    // 2017-10-09) that calls it
    migrator.migrate("accountsrv",
                     r#"CREATE OR REPLACE FUNCTION account_creation_report (
                          op_date timestamptz
                        ) RETURNS TABLE(name text, email text, created_at timestamptz) AS $$
                        DECLARE
                          schema RECORD;
                        BEGIN
                          FOR schema IN EXECUTE
                            format(
                              'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
                              'shard_'
                            )
                            LOOP
                              RETURN QUERY EXECUTE format('SELECT name, email, created_at FROM %I.accounts WHERE created_at >= %L', schema.schema_name, op_date);
                            END LOOP;
                            RETURN;
                          END;
                          $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate(
        "accountsrv",
        r#"ALTER TABLE IF EXISTS accounts DROP CONSTRAINT IF EXISTS accounts_email_key"#,
    )?;
    migrator.migrate(
        "accountsrv",
        r#"CREATE OR REPLACE FUNCTION update_account_v1 (
                          op_id bigint,
                          op_email text
                        ) RETURNS void AS $$
                            UPDATE accounts SET email = op_email WHERE id = op_id;
                          $$ LANGUAGE SQL VOLATILE"#,
    )?;
    Ok(())
}
