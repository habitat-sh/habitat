/* Accounts migration */

CREATE SEQUENCE IF NOT EXISTS accounts_id_seq;

CREATE TABLE IF NOT EXISTS accounts (
  id bigint PRIMARY KEY DEFAULT next_id_v1('accounts_id_seq'),
  name text UNIQUE,
  email text UNIQUE,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

CREATE OR REPLACE FUNCTION select_or_insert_account_v1 (
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
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_account_by_name_v1 (
  account_name text
) RETURNS SETOF accounts AS $$
    BEGIN
      RETURN QUERY SELECT * FROM accounts WHERE name = account_name;
      RETURN;
    END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_account_by_id_v1 (
  account_id bigint
) RETURNS SETOF accounts AS $$
    BEGIN
      RETURN QUERY SELECT * FROM accounts WHERE id = account_id;
      RETURN;
    END
$$ LANGUAGE plpgsql STABLE;

CREATE TABLE IF NOT EXISTS account_origins (
  account_id bigint,
  account_name text,
  origin_id bigint,
  origin_name text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  UNIQUE(account_id, origin_id)
);

CREATE OR REPLACE FUNCTION insert_account_origin_v1 (
  o_account_id bigint,
  o_account_name text,
  o_origin_id bigint,
  o_origin_name text
) RETURNS void AS $$
    BEGIN
      INSERT INTO account_origins (account_id, account_name, origin_id, origin_name) VALUES (o_account_id, o_account_name, o_origin_id, o_origin_name);
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_account_origins_v1 (
  in_account_id bigint
) RETURNS SETOF account_origins AS $$
    BEGIN
      RETURN QUERY SELECT * FROM account_origins WHERE account_id = in_account_id;
      RETURN;
    END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION delete_account_origin_v1 (
  aod_account_name text,
  aod_origin_id bigint
) RETURNS void AS $$
    DELETE FROM account_origins WHERE account_name=aod_account_name AND origin_id=aod_origin_id;
$$ LANGUAGE SQL VOLATILE;

/*
  This query is a report that's meant to be run manually. There's no code path (as of 2017-10-09) that calls it
*/
CREATE OR REPLACE FUNCTION account_creation_report (
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
$$ LANGUAGE plpgsql STABLE;

ALTER TABLE IF EXISTS accounts DROP CONSTRAINT IF EXISTS accounts_email_key;

CREATE OR REPLACE FUNCTION update_account_v1 (
  op_id bigint,
  op_email text
) RETURNS void AS $$
    UPDATE accounts SET email = op_email WHERE id = op_id;
$$ LANGUAGE SQL VOLATILE;

/* Invitations Migrations */

CREATE SEQUENCE IF NOT EXISTS account_invitations_id_seq;

CREATE TABLE IF NOT EXISTS account_invitations (
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
);

CREATE OR REPLACE FUNCTION insert_account_invitation_v1 (
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
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_invitations_for_account_v1 (
  oi_account_id bigint
) RETURNS SETOF account_invitations AS $$
  BEGIN
    RETURN QUERY SELECT * FROM account_invitations WHERE account_id = oi_account_id AND ignored = false
      ORDER BY origin_name ASC;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION accept_account_invitation_v1 (
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
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION ignore_account_invitation_v1 (
  oi_invitation_id bigint, oi_account_id bigint
) RETURNS void AS $$
  UPDATE account_invitations
  SET ignored = true, updated_at = now()
  WHERE origin_invitation_id = oi_invitation_id AND account_id = oi_account_id;
$$ LANGUAGE SQL VOLATILE;


CREATE OR REPLACE FUNCTION rescind_account_invitation_v1 (
  oi_invitation_id bigint, oi_account_id bigint
) RETURNS void AS $$
  DELETE FROM account_invitations
  WHERE origin_invitation_id = oi_invitation_id
  AND account_id = oi_account_id
  AND ignored = false;
$$ LANGUAGE SQL VOLATILE;
