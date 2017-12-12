CREATE SEQUENCE IF NOT EXISTS origin_id_seq;

CREATE TABLE IF NOT EXISTS origins (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'),
  name text UNIQUE,
  owner_id bigint,
  session_sync bool DEFAULT false,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

CREATE TABLE IF NOT EXISTS origin_members (
  origin_id bigint REFERENCES origins(id),
  origin_name text,
  account_id bigint,
  account_name text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  PRIMARY KEY (origin_id, account_id)
);

CREATE OR REPLACE FUNCTION insert_origin_member_v1 (
  om_origin_id bigint,
  om_origin_name text,
  om_account_id bigint,
  om_account_name text
) RETURNS void AS $$
  BEGIN
    INSERT INTO origin_members (origin_id, origin_name, account_id, account_name)
          VALUES (om_origin_id, om_origin_name, om_account_id, om_account_name);
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION insert_origin_v1 (
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
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION list_origin_members_v1 (
  om_origin_id bigint
) RETURNS TABLE(account_name text) AS $$
  BEGIN
      RETURN QUERY SELECT origin_members.account_name FROM origin_members WHERE origin_id = om_origin_id
        ORDER BY account_name ASC;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION check_account_in_origin_members_v1 (
  om_origin_name text,
  om_account_id bigint
) RETURNS TABLE(is_member bool) AS $$
  BEGIN
    RETURN QUERY SELECT true FROM origin_members WHERE origin_name = om_origin_name AND account_id = om_account_id;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION list_origin_by_account_id_v1 (
  o_account_id bigint
) RETURNS TABLE(origin_name text) AS $$
  BEGIN
    RETURN QUERY SELECT origin_members.origin_name FROM origin_members WHERE account_id = o_account_id
      ORDER BY origin_name ASC;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION sync_origins_v1() RETURNS TABLE(account_id bigint, account_name text, origin_id bigint, origin_name text) AS $$
  BEGIN
    RETURN QUERY SELECT origins.owner_id, origin_members.account_name, origins.id, origins.name FROM origins, origin_members WHERE origins.session_sync = false AND origins.owner_id = origin_members.account_id;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION set_session_sync_v1(in_origin_id bigint) RETURNS VOID AS $$
  BEGIN
      UPDATE origins SET session_sync = true WHERE id = in_origin_id;
  END
$$ LANGUAGE plpgsql VOLATILE;

ALTER TABLE IF EXISTS origins ADD COLUMN IF NOT EXISTS default_package_visibility text NOT NULL DEFAULT 'public';

CREATE OR REPLACE FUNCTION insert_origin_v2 (
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
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION update_origin_v1 (
  origin_id bigint,
  op_default_package_visibility text
) RETURNS void AS $$
  UPDATE origins SET
    default_package_visibility = op_default_package_visibility,
    updated_at = now()
    WHERE id = origin_id;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION delete_origin_member_v1 (
  om_origin_id bigint,
  om_account_name text
) RETURNS void AS $$
      DELETE FROM origin_members WHERE origin_id=om_origin_id AND account_name=om_account_name;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION my_origins_v1 (
  om_account_id bigint
) RETURNS SETOF origins AS $$
  DECLARE
    schema RECORD;
  BEGIN
    FOR schema IN EXECUTE
      format(
        'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
        'shard_'
      )
    LOOP
      RETURN QUERY EXECUTE
      format('SELECT o.* FROM %I.origins o INNER JOIN %I.origin_members om ON o.id = om.origin_id WHERE om.account_id = %L ORDER BY o.name', schema.schema_name, schema.schema_name, om_account_id);
    END LOOP;
    RETURN;
  END;
$$ LANGUAGE plpgsql STABLE;

CREATE SEQUENCE IF NOT EXISTS origin_public_key_id_seq;

CREATE TABLE IF NOT EXISTS origin_public_keys (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_public_key_id_seq'),
  origin_id bigint REFERENCES origins(id),
  owner_id bigint,
  name text,
  revision text,
  full_name text UNIQUE,
  body bytea,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

CREATE OR REPLACE FUNCTION insert_origin_public_key_v1 (
  opk_origin_id bigint,
  opk_owner_id bigint,
  opk_name text,
  opk_revision text,
  opk_full_name text,
  opk_body bytea
) RETURNS SETOF origin_public_keys AS $$
    BEGIN
      RETURN QUERY INSERT INTO origin_public_keys (origin_id, owner_id, name, revision, full_name, body)
          VALUES (opk_origin_id, opk_owner_id, opk_name, opk_revision, opk_full_name, opk_body)
          RETURNING *;
      RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_public_key_v1 (
  opk_name text,
  opk_revision text
) RETURNS SETOF origin_public_keys AS $$
  BEGIN
    RETURN QUERY SELECT * FROM origin_public_keys WHERE name = opk_name and revision = opk_revision
      ORDER BY revision DESC
      LIMIT 1;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_public_key_latest_v1 (
  opk_name text
) RETURNS SETOF origin_public_keys AS $$
  BEGIN
    RETURN QUERY SELECT * FROM origin_public_keys WHERE name = opk_name
      ORDER BY revision DESC
      LIMIT 1;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_public_keys_for_origin_v1 (
  opk_origin_id bigint
) RETURNS SETOF origin_public_keys AS $$
  BEGIN
      RETURN QUERY SELECT * FROM origin_public_keys WHERE origin_id = opk_origin_id
        ORDER BY revision DESC;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

ALTER TABLE origin_public_keys DROP CONSTRAINT IF EXISTS origin_public_keys_full_name_key;

DELETE FROM origin_public_keys
  WHERE id IN (
    SELECT id FROM (
      SELECT id, ROW_NUMBER() OVER (
        partition BY full_name ORDER BY id
      ) AS rnum FROM origin_public_keys
    ) t WHERE t.rnum > 1);

ALTER TABLE origin_public_keys ADD CONSTRAINT origin_public_keys_full_name_key  UNIQUE (full_name);

CREATE SEQUENCE IF NOT EXISTS origin_secret_key_id_seq;

CREATE TABLE IF NOT EXISTS origin_secret_keys (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_secret_key_id_seq'),
  origin_id bigint REFERENCES origins(id),
  owner_id bigint,
  name text,
  revision text,
  full_name text UNIQUE,
  body bytea,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

CREATE OR REPLACE VIEW origins_with_secret_key_full_name_v1 AS
  SELECT origins.id, origins.name, origins.owner_id,
          origin_secret_keys.full_name AS private_key_name
    FROM origins
    LEFT OUTER JOIN origin_secret_keys ON (origins.id = origin_secret_keys.origin_id)
    ORDER BY origins.id, origin_secret_keys.full_name DESC;

CREATE OR REPLACE FUNCTION insert_origin_secret_key_v1 (
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
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_secret_key_v1 (
  osk_name text
) RETURNS SETOF origin_secret_keys AS $$
  BEGIN
    RETURN QUERY SELECT * FROM origin_secret_keys WHERE name = osk_name
      ORDER BY full_name DESC
      LIMIT 1;
    RETURN;
  END
  $$ LANGUAGE plpgsql STABLE;

ALTER TABLE origin_secret_keys DROP CONSTRAINT IF EXISTS origin_secret_keys_full_name_key;

DELETE FROM origin_secret_keys
  WHERE id IN (
      SELECT id FROM (
          SELECT id, ROW_NUMBER() OVER (
              partition BY full_name ORDER BY id
          ) AS rnum FROM origin_secret_keys
      ) t WHERE t.rnum > 1);

ALTER TABLE origin_secret_keys ADD CONSTRAINT origin_secret_keys_full_name_key UNIQUE (full_name);

CREATE OR REPLACE VIEW origins_with_secret_key_full_name_v2 AS
  SELECT origins.id, origins.name, origins.owner_id,
          origin_secret_keys.full_name AS private_key_name,
          origins.default_package_visibility
    FROM origins
    LEFT OUTER JOIN origin_secret_keys ON (origins.id = origin_secret_keys.origin_id)
    ORDER BY origins.id, origin_secret_keys.full_name DESC;

CREATE SEQUENCE IF NOT EXISTS origin_invitations_id_seq;

CREATE TABLE IF NOT EXISTS origin_invitations (
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
);

CREATE OR REPLACE FUNCTION insert_origin_invitation_v1 (
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
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_invitations_for_origin_v1 (
  oi_origin_id bigint
) RETURNS SETOF origin_invitations AS $$
  BEGIN
      RETURN QUERY SELECT * FROM origin_invitations WHERE origin_id = oi_origin_id
        ORDER BY account_name ASC;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_invitations_for_account_v1 (
  oi_account_id bigint
) RETURNS SETOF origin_invitations AS $$
  BEGIN
      RETURN QUERY SELECT * FROM origin_invitations WHERE account_id = oi_account_id AND ignored = false
        ORDER BY origin_name ASC;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION accept_origin_invitation_v1 (
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
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION validate_origin_invitation_v1 (
  oi_invite_id bigint, oi_account_id bigint
) RETURNS TABLE(is_valid bool) AS $$
  BEGIN
    RETURN QUERY SELECT true FROM origin_invitations WHERE id = oi_invite_id AND account_id = oi_account_id;
    RETURN;
  END
  $$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_invitations_not_synced_with_account_v1 () RETURNS SETOF origin_invitations AS $$
BEGIN
  RETURN QUERY SELECT * FROM origin_invitations WHERE account_sync = false
    ORDER BY created_at ASC;
  RETURN;
END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION set_account_sync_v1 (oi_id bigint) RETURNS void AS $$
  BEGIN
    UPDATE origin_invitations SET account_sync = true, updated_at = now() WHERE id = oi_id;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION ignore_origin_invitation_v1 (
  oi_invitation_id bigint, oi_account_id bigint
) RETURNS void AS $$
    UPDATE origin_invitations
    SET ignored = true, updated_at = now()
    WHERE id = oi_invitation_id AND account_id = oi_account_id;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION rescind_origin_invitation_v1 (
  oi_invitation_id bigint, oi_owner_id bigint
) RETURNS void AS $$
    DELETE FROM origin_invitations
    WHERE id = oi_invitation_id
    AND owner_id = oi_owner_id
    AND ignored = false;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_invitation_v1 (
  oi_invitation_id bigint
) RETURNS SETOF origin_invitations AS $$
    SELECT * FROM origin_invitations
    WHERE id = oi_invitation_id;
$$ LANGUAGE SQL VOLATILE;

CREATE SEQUENCE IF NOT EXISTS origin_integration_id_seq;

CREATE TABLE IF NOT EXISTS origin_integrations (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_integration_id_seq'),
  origin text,
  integration text,
  name text,
  body text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  UNIQUE (origin, name)
);

CREATE OR REPLACE FUNCTION insert_origin_integration_v1 (
  in_origin text,
  in_integration text,
  in_name text,
  in_body text
) RETURNS SETOF origin_integrations AS $$
  INSERT INTO origin_integrations(origin, integration, name, body)
  VALUES (in_origin, in_integration, in_name, in_body)
  RETURNING *
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_integrations_v1 (
  in_origin text,
  in_integration text
) RETURNS SETOF origin_integrations AS $$
  SELECT * FROM origin_integrations
  WHERE origin = in_origin AND integration = in_integration
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION delete_origin_integration_v1 (
  in_origin text,
  in_integration text,
  in_name text
) RETURNS void AS $$
  DELETE FROM origin_integrations
  WHERE origin = in_origin AND integration = in_integration AND name = in_name
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_integrations_for_origin_v1 (
  in_origin text
) RETURNS SETOF origin_integrations AS $$
    SELECT * FROM origin_integrations
    WHERE origin = in_origin
    ORDER BY integration, name
$$ LANGUAGE SQL STABLE;

ALTER TABLE IF EXISTS origin_integrations DROP CONSTRAINT IF EXISTS origin_integrations_origin_name_key;

ALTER TABLE IF EXISTS origin_integrations ADD CONSTRAINT origin_integrations_origin_integration_name_key UNIQUE(origin, integration, name);

CREATE SEQUENCE IF NOT EXISTS origin_project_id_seq;

CREATE TABLE IF NOT EXISTS origin_projects (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_project_id_seq'),
  origin_id bigint REFERENCES origins(id),
  origin_name text,
  package_name text,
  name text,
  plan_path text,
  owner_id bigint,
  vcs_type text,
  vcs_data text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  UNIQUE (origin_name, package_name, name)
);

CREATE OR REPLACE FUNCTION insert_origin_project_v1 (
  project_origin_name text,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint
) RETURNS SETOF origin_projects AS $$
    BEGIN
      RETURN QUERY INSERT INTO origin_projects (origin_id,
                                  origin_name,
                                  package_name,
                                  name,
                                  plan_path,
                                  owner_id,
                                  vcs_type,
                                  vcs_data)
            VALUES (
                (SELECT id FROM origins where name = project_origin_name),
                project_origin_name,
                project_package_name,
                project_origin_name || '/' || project_package_name,
                project_plan_path,
                project_owner_id,
                project_vcs_type,
                project_vcs_data)
            RETURNING *;
      RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_project_v1 (
  project_name text
) RETURNS SETOF origin_projects AS $$
  BEGIN
    RETURN QUERY SELECT * FROM origin_projects WHERE name = project_name;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION delete_origin_project_v1 (
  project_name text
) RETURNS void AS $$
  BEGIN
      DELETE FROM origin_projects WHERE name = project_name;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION update_origin_project_v1 (
  project_id bigint,
  project_origin_id bigint,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint
) RETURNS void AS $$
    BEGIN
      UPDATE origin_projects SET
          package_name = project_package_name,
          name = (SELECT name FROM origins WHERE id = project_origin_id) || '/' || project_package_name,
          plan_path = project_plan_path,
          vcs_type = project_vcs_type,
          vcs_data = project_vcs_data,
          owner_id = project_owner_id,
          updated_at = now()
          WHERE id = project_id;
    END
$$ LANGUAGE plpgsql VOLATILE;

ALTER TABLE IF EXISTS origin_projects ADD COLUMN IF NOT EXISTS vcs_auth_token text;

ALTER TABLE IF EXISTS origin_projects ADD COLUMN IF NOT EXISTS vcs_username text;

ALTER TABLE IF EXISTS origin_projects ADD COLUMN IF NOT EXISTS vcs_installation_id bigint;

CREATE OR REPLACE FUNCTION insert_origin_project_v2 (
  project_origin_name text,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint,
  project_vcs_auth_token text,
  project_vcs_username text
) RETURNS SETOF origin_projects AS $$
    BEGIN
      RETURN QUERY INSERT INTO origin_projects (origin_id,
                                  origin_name,
                                  package_name,
                                  name,
                                  plan_path,
                                  owner_id,
                                  vcs_type,
                                  vcs_data,
                                  vcs_auth_token,
                                  vcs_username)
            VALUES (
                (SELECT id FROM origins where name = project_origin_name),
                project_origin_name,
                project_package_name,
                project_origin_name || '/' || project_package_name,
                project_plan_path,
                project_owner_id,
                project_vcs_type,
                project_vcs_data,
                project_vcs_auth_token,
                project_vcs_username)
            RETURNING *;
        RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION update_origin_project_v2 (
  project_id bigint,
  project_origin_id bigint,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint,
  project_vcs_auth_token text,
  project_vcs_username text
) RETURNS void AS $$
    BEGIN
      UPDATE origin_projects SET
          package_name = project_package_name,
          name = (SELECT name FROM origins WHERE id = project_origin_id) || '/' || project_package_name,
          plan_path = project_plan_path,
          vcs_type = project_vcs_type,
          vcs_data = project_vcs_data,
          owner_id = project_owner_id,
          updated_at = now(),
          vcs_auth_token = project_vcs_auth_token,
          vcs_username = project_vcs_username
          WHERE id = project_id;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION insert_origin_project_v3 (
  project_origin_name text,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint,
  project_vcs_installation_id bigint
) RETURNS SETOF origin_projects AS $$
    BEGIN
      RETURN QUERY INSERT INTO origin_projects (origin_id,
                                  origin_name,
                                  package_name,
                                  name,
                                  plan_path,
                                  owner_id,
                                  vcs_type,
                                  vcs_data,
                                  vcs_installation_id)
            VALUES (
                (SELECT id FROM origins where name = project_origin_name),
                project_origin_name,
                project_package_name,
                project_origin_name || '/' || project_package_name,
                project_plan_path,
                project_owner_id,
                project_vcs_type,
                project_vcs_data,
                project_vcs_installation_id)
            RETURNING *;
        RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION update_origin_project_v3 (
  project_id bigint,
  project_origin_id bigint,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint,
  project_vcs_installation_id bigint,
  project_visibility text
) RETURNS void AS $$
    BEGIN
      UPDATE origin_projects SET
          package_name = project_package_name,
          name = (SELECT name FROM origins WHERE id = project_origin_id) || '/' || project_package_name,
          plan_path = project_plan_path,
          vcs_type = project_vcs_type,
          vcs_data = project_vcs_data,
          owner_id = project_owner_id,
          updated_at = now(),
          vcs_installation_id = project_vcs_installation_id,
          visibility = project_visibility
          WHERE id = project_id;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE SEQUENCE IF NOT EXISTS origin_project_integration_id_seq;

CREATE TABLE IF NOT EXISTS origin_project_integrations (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_project_integration_id_seq'),
  origin text,
  name text,
  integration text,
  integration_name text,
  body text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  UNIQUE (origin, name, integration, integration_name)
);

CREATE OR REPLACE FUNCTION upsert_origin_project_integration_v1 (
  in_origin text,
  in_name text,
  in_integration text,
  in_integration_name text,
  in_body text
) RETURNS SETOF origin_project_integrations AS $$
    BEGIN
      RETURN QUERY INSERT INTO origin_project_integrations(
          origin,
          name,
          integration,
          integration_name,
          body)
        VALUES (
            in_origin,
            in_name,
            in_integration,
            in_integration_name,
            in_body)
        ON CONFLICT(origin, name, integration, integration_name)
          DO UPDATE SET body=in_body RETURNING *;
      RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_project_integrations_v1 (
  in_origin text,
  in_name text,
  in_integration text,
  in_integration_name text
) RETURNS SETOF origin_project_integrations AS $$
    SELECT * FROM origin_project_integrations
    WHERE origin = in_origin AND
          name = in_name AND
          integration = in_integration AND
          in_integration_name = in_integration_name
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION get_origin_project_integrations_for_project_v1 (
  in_origin text,
  in_name text
) RETURNS SETOF origin_project_integrations AS $$
    SELECT * FROM origin_project_integrations
    WHERE origin = in_origin AND name = in_name
    ORDER BY integration, integration_name
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION get_origin_project_list_v1 (
  in_origin text
) RETURNS SETOF origin_projects AS $$
    SELECT * FROM origin_projects
    WHERE origin_name = in_origin
$$ LANGUAGE SQL STABLE;

ALTER TABLE IF EXISTS origin_projects ADD COLUMN IF NOT EXISTS visibility text NOT NULL DEFAULT 'public';

CREATE OR REPLACE FUNCTION insert_origin_project_v3 (
  project_origin_name text,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint,
  project_vcs_auth_token text,
  project_vcs_username text
) RETURNS SETOF origin_projects AS $$
    BEGIN
      RETURN QUERY INSERT INTO origin_projects (origin_id,
                                  origin_name,
                                  package_name,
                                  name,
                                  plan_path,
                                  owner_id,
                                  vcs_type,
                                  vcs_data,
                                  vcs_auth_token,
                                  vcs_username,
                                  visibility)
            VALUES (
                (SELECT id FROM origins WHERE name = project_origin_name),
                project_origin_name,
                project_package_name,
                project_origin_name || '/' || project_package_name,
                project_plan_path,
                project_owner_id,
                project_vcs_type,
                project_vcs_data,
                project_vcs_auth_token,
                project_vcs_username,
                (SELECT default_package_visibility FROM origins WHERE name = project_origin_name))
            RETURNING *;
        RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION update_origin_project_v3 (
  project_id bigint,
  project_origin_id bigint,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint,
  project_vcs_auth_token text,
  project_vcs_username text,
  project_visibility text
) RETURNS void AS $$
    BEGIN
      UPDATE origin_projects SET
        package_name = project_package_name,
        name = (SELECT name FROM origins WHERE id = project_origin_id) || '/' || project_package_name,
        plan_path = project_plan_path,
        vcs_type = project_vcs_type,
        vcs_data = project_vcs_data,
        owner_id = project_owner_id,
        updated_at = now(),
        vcs_auth_token = project_vcs_auth_token,
        vcs_username = project_vcs_username,
        visibility = project_visibility
        WHERE id = project_id;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION insert_origin_project_v4 (
  project_origin_name text,
  project_package_name text,
  project_plan_path text,
  project_vcs_type text,
  project_vcs_data text,
  project_owner_id bigint,
  project_vcs_installation_id bigint,
  project_visibility text
) RETURNS SETOF origin_projects AS $$
    BEGIN
      RETURN QUERY INSERT INTO origin_projects (origin_id,
                                  origin_name,
                                  package_name,
                                  name,
                                  plan_path,
                                  owner_id,
                                  vcs_type,
                                  vcs_data,
                                  vcs_installation_id,
                                  visibility)
            VALUES (
                (SELECT id FROM origins where name = project_origin_name),
                project_origin_name,
                project_package_name,
                project_origin_name || '/' || project_package_name,
                project_plan_path,
                project_owner_id,
                project_vcs_type,
                project_vcs_data,
                project_vcs_installation_id,
                project_visibility)
            RETURNING *;
        RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

-- BEGIN cascade delete migration
-- Issue: https://github.com/habitat-sh/habitat/issues/4090

ALTER TABLE IF EXISTS origin_project_integrations
  ADD COLUMN IF NOT EXISTS project_id bigint REFERENCES origin_projects(id) ON DELETE CASCADE,
  ADD COLUMN IF NOT EXISTS integration_id bigint REFERENCES origin_integrations(id) ON DELETE CASCADE;

DO $$
BEGIN
  IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='origin_project_integrations' AND column_name='name') THEN
    UPDATE origin_project_integrations as u1 SET project_id = u2.project_id, integration_id = u2.integration_id FROM
      (SELECT opi.id as opiid, op.id as project_id, oi.id as integration_id
      FROM origin_project_integrations AS opi
      JOIN origin_projects AS op ON opi.name = op.package_name AND opi.origin = op.origin_name
      JOIN origin_integrations AS oi ON opi.integration = oi.name AND opi.origin = oi.origin
      WHERE opi.project_id IS NULL
      AND opi.integration_id IS NULL) as u2
      WHERE u2.opiid = u1.id;
  END IF;
END $$;

UPDATE origin_project_integrations SET updated_at = NOW() WHERE updated_at IS NULL;
-- Cleanup any orphaned origin_project_integrations

DELETE FROM origin_project_integrations WHERE project_id IS NULL or integration_id IS NULL;

ALTER TABLE origin_project_integrations
  DROP COLUMN IF EXISTS name,
  DROP COLUMN IF EXISTS integration,
  DROP COLUMN IF EXISTS integration_name,
  ALTER COLUMN updated_at SET DEFAULT NOW(),
  ALTER COLUMN body SET NOT NULL,
  ALTER COLUMN created_at SET NOT NULL,
  ALTER COLUMN updated_at SET NOT NULL,
  ALTER COLUMN origin SET NOT NULL,
  ALTER COLUMN project_id SET NOT NULL,
  ALTER COLUMN integration_id SET NOT NULL,
  ADD UNIQUE (project_id, integration_id);

CREATE OR REPLACE FUNCTION upsert_origin_project_integration_v2 (
  in_origin text,
  in_name text,
  in_integration text,
  in_body text
) RETURNS SETOF origin_project_integrations AS $$
    BEGIN
        RETURN QUERY INSERT INTO origin_project_integrations(
            origin,
            body,
            updated_at,
            project_id,
            integration_id)
            VALUES (
                in_origin,
                in_body,
                NOW(),
                (SELECT id FROM origin_projects WHERE package_name = in_name AND origin_name = in_origin),
                (SELECT id FROM origin_integrations WHERE origin = in_origin AND name = in_integration)
            )
            ON CONFLICT(project_id, integration_id)
            DO UPDATE SET body=in_body RETURNING *;
        RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_project_integrations_v2 (
  in_origin text,
  in_name text,
  in_integration text
) RETURNS SETOF origin_project_integrations AS $$
  SELECT opi.* FROM origin_project_integrations opi
  JOIN origin_integrations oi ON oi.id = opi.integration_id
  JOIN origin_projects op ON op.id = opi.project_id
  WHERE opi.origin = in_origin
  AND op.package_name = in_name
  AND oi.name = in_integration
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION get_origin_project_integrations_for_project_v2 (
  in_origin text,
  in_name text
) RETURNS SETOF origin_project_integrations AS $$
    SELECT opi.* FROM origin_project_integrations opi
    JOIN origin_projects op ON op.id = opi.project_id
    WHERE origin = in_origin
    AND package_name = in_name
$$ LANGUAGE SQL STABLE;
-- END cascade delete migration

CREATE OR REPLACE FUNCTION upsert_origin_project_integration_v3 (
  in_origin text,
  in_name text,
  in_integration text,
  in_body text
) RETURNS SETOF origin_project_integrations AS $$
  BEGIN
    -- We currently support running only one publish step per build job. This
    -- temporary fix ensures we store (and can retrieve) only one project integration.
    DELETE FROM origin_project_integrations
    WHERE origin = in_origin
    AND project_id = (SELECT id FROM origin_projects WHERE package_name = in_name AND origin_name = in_origin);

    RETURN QUERY INSERT INTO origin_project_integrations(
        origin,
        body,
        updated_at,
        project_id,
        integration_id)
        VALUES (
          in_origin,
          in_body,
          NOW(),
          (SELECT id FROM origin_projects WHERE package_name = in_name AND origin_name = in_origin),
          (SELECT id FROM origin_integrations WHERE origin = in_origin AND name = in_integration)
        )
        ON CONFLICT(project_id, integration_id)
        DO UPDATE SET body=in_body RETURNING *;
    RETURN;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION delete_origin_project_integration_v1 (
  p_origin text,
  p_package text,
  p_integration text
) RETURNS void AS $$
    BEGIN
        DELETE FROM origin_project_integrations
        WHERE origin = p_origin
        AND project_id = (SELECT id FROM origin_projects WHERE origin_name = p_origin AND package_name = p_package)
        AND integration_id = (SELECT id FROM origin_integrations WHERE origin = p_origin AND name = p_integration);
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE SEQUENCE IF NOT EXISTS origin_package_id_seq;

CREATE TABLE IF NOT EXISTS origin_packages (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_package_id_seq'),
  origin_id bigint REFERENCES origins(id),
  owner_id bigint,
  name text,
  ident text UNIQUE,
  checksum text,
  manifest text,
  config text,
  target text,
  deps text,
  tdeps text,
  exposes text,
  scheduler_sync bool DEFAULT false,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

CREATE OR REPLACE FUNCTION insert_origin_package_v1 (
  op_origin_id bigint,
  op_owner_id bigint,
  op_name text,
  op_ident text,
  op_checksum text,
  op_manifest text,
  op_config text,
  op_target text,
  op_deps text,
  op_tdeps text,
  op_exposes text
) RETURNS SETOF origin_packages AS $$
    DECLARE
      inserted_package origin_packages;
      channel_id bigint;
    BEGIN
        INSERT INTO origin_packages (origin_id, owner_id, name, ident, checksum, manifest, config, target, deps, tdeps, exposes)
              VALUES (op_origin_id, op_owner_id, op_name, op_ident, op_checksum, op_manifest, op_config, op_target, op_deps, op_tdeps, op_exposes)
              RETURNING * into inserted_package;

        SELECT id FROM origin_channels WHERE origin_id = op_origin_id AND name = 'unstable' INTO channel_id;
        PERFORM promote_origin_package_v1(channel_id, inserted_package.id);

        RETURN NEXT inserted_package;
        RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION search_origin_packages_for_origin_distinct_v1 (
  op_origin text,
  op_query text,
  op_limit bigint,
  op_offset bigint
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
      RETURN QUERY SELECT COUNT(p.partial_ident[1] || '/' || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || '/' || p.partial_ident[2] AS ident
      FROM (SELECT regexp_split_to_array(op.ident, '/') as partial_ident FROM origins o INNER JOIN origin_packages op ON o.id = op.origin_id WHERE o.name = op_origin AND op.name LIKE ('%' || op_query || '%')) AS p
      GROUP BY (p.partial_ident[1] || '/' || p.partial_ident[2])
      LIMIT op_limit OFFSET op_offset;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION sync_packages_v1() RETURNS TABLE(account_id bigint, package_id bigint, package_ident text, package_deps text) AS $$
  BEGIN
      RETURN QUERY SELECT origin_packages.owner_id, origin_packages.id, origin_packages.ident, origin_packages.deps FROM origin_packages WHERE origin_packages.scheduler_sync = false;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION set_packages_sync_v1(in_package_id bigint) RETURNS VOID AS $$
  BEGIN
      UPDATE origin_packages SET scheduler_sync = true WHERE id = in_package_id;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION sync_packages_v2() RETURNS TABLE(account_id bigint, package_id bigint, package_ident text, package_deps text, package_target text) AS $$
  SELECT owner_id, id, ident, deps, target FROM origin_packages WHERE scheduler_sync = false;
$$ LANGUAGE SQL STABLE;

ALTER TABLE IF EXISTS origin_packages ADD COLUMN IF NOT EXISTS visibility text NOT NULL DEFAULT 'public';

CREATE OR REPLACE FUNCTION insert_origin_package_v2 (
  op_origin_id bigint,
  op_owner_id bigint,
  op_name text,
  op_ident text,
  op_checksum text,
  op_manifest text,
  op_config text,
  op_target text,
  op_deps text,
  op_tdeps text,
  op_exposes text
) RETURNS SETOF origin_packages AS $$
    DECLARE
      inserted_package origin_packages;
      channel_id bigint;
    BEGIN
      INSERT INTO origin_packages (origin_id, owner_id, name, ident, checksum, manifest, config, target, deps, tdeps, exposes, visibility)
            VALUES (op_origin_id, op_owner_id, op_name, op_ident, op_checksum, op_manifest, op_config, op_target, op_deps, op_tdeps, op_exposes,
            (SELECT default_package_visibility FROM origins WHERE id = op_origin_id))
            RETURNING * into inserted_package;

      SELECT id FROM origin_channels WHERE origin_id = op_origin_id AND name = 'unstable' INTO channel_id;
      PERFORM promote_origin_package_v1(channel_id, inserted_package.id);

      RETURN NEXT inserted_package;
      RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION update_origin_package_v1 (
  op_id bigint,
  op_owner_id bigint,
  op_name text,
  op_ident text,
  op_checksum text,
  op_manifest text,
  op_config text,
  op_target text,
  op_deps text,
  op_tdeps text,
  op_exposes text,
  op_visibility text
) RETURNS void AS $$
  UPDATE origin_packages SET
    owner_id = op_owner_id,
    name = op_name,
    ident = op_ident,
    checksum = op_checksum,
    manifest = op_manifest,
    config = op_config,
    target = op_target,
    deps = op_deps,
    tdeps = op_tdeps,
    exposes = op_exposes,
    visibility = op_visibility,
    scheduler_sync = false,
    updated_at = now()
    WHERE id = op_id;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_package_v3 (
  op_ident text,
  op_account_id bigint,
  op_show_hidden bool
) RETURNS SETOF origin_packages AS $$
  BEGIN
    RETURN QUERY SELECT *
    FROM origin_packages
    WHERE ident = op_ident
    AND (visibility='public' OR
          (visibility='hidden' AND op_show_hidden = true) OR
          (visibility IN ('private', 'hidden') AND origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)));
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION insert_origin_package_v3 (
  op_origin_id bigint,
  op_owner_id bigint,
  op_name text,
  op_ident text,
  op_checksum text,
  op_manifest text,
  op_config text,
  op_target text,
  op_deps text,
  op_tdeps text,
  op_exposes text,
  op_visibility text
) RETURNS SETOF origin_packages AS $$
    DECLARE
      inserted_package origin_packages;
      channel_id bigint;
    BEGIN
        INSERT INTO origin_packages (origin_id, owner_id, name, ident, checksum, manifest, config, target, deps, tdeps, exposes, visibility)
              VALUES (op_origin_id, op_owner_id, op_name, op_ident, op_checksum, op_manifest, op_config, op_target, op_deps, op_tdeps, op_exposes, op_visibility)
              RETURNING * into inserted_package;

        SELECT id FROM origin_channels WHERE origin_id = op_origin_id AND name = 'unstable' INTO channel_id;
        PERFORM promote_origin_package_v1(channel_id, inserted_package.id);

        RETURN NEXT inserted_package;
        RETURN;
    END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_all_origin_packages_for_ident_v1 (
  op_ident text
) RETURNS SETOF origin_packages AS $$
  BEGIN
    RETURN QUERY SELECT * FROM origin_packages WHERE ident LIKE (op_ident || '%') ORDER BY ident;
    RETURN;
  END
  $$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_all_origin_packages_for_origin_v1 (
  op_id bigint
) RETURNS SETOF origin_packages AS $$
  BEGIN
    RETURN QUERY SELECT * FROM origin_packages WHERE id = op_id;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION update_package_visibility_in_bulk_v1 (
  op_visibility text,
  op_ids bigint[]
) RETURNS void AS $$
    UPDATE origin_packages
    SET visibility = op_visibility
    WHERE id IN (SELECT(unnest(op_ids)));
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_package_latest_v4 (
  op_ident text,
  op_target text,
  op_account_id bigint
) RETURNS SETOF origin_packages AS $$
  BEGIN
      RETURN QUERY SELECT *
      FROM origin_packages
      WHERE ident LIKE (op_ident  || '%')
      AND target = op_target
      AND (visibility='public' OR (visibility IN ('private', 'hidden') AND origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)));
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_package_platforms_for_package_v3 (
  op_ident text,
  op_account_id bigint
) RETURNS TABLE (target text)
LANGUAGE SQL VOLATILE AS $$
SELECT DISTINCT target
FROM origin_packages
WHERE ident LIKE (op_ident || '%')
AND (visibility='public' OR (visibility IN ('private', 'hidden') AND origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
$$;

CREATE OR REPLACE FUNCTION get_origin_package_versions_for_origin_v6 (
  op_origin text,
  op_pkg text,
  op_account_id bigint
) RETURNS TABLE(version text, release_count bigint, latest text, platforms text)
LANGUAGE SQL STABLE AS $$
  WITH packages AS (
    SELECT *
    FROM origin_packages op INNER JOIN origins o ON o.id = op.origin_id
    WHERE o.name = op_origin
    AND op.name = op_pkg
    AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
  ), idents AS (
    SELECT regexp_split_to_array(ident, '/') as parts, target
    FROM packages
  )
  SELECT i.parts[3] AS version,
  COUNT(i.parts[4]) AS release_count,
  MAX(i.parts[4]) as latest,
  ARRAY_TO_STRING(ARRAY_AGG(DISTINCT i.target), ',')
  FROM idents i
  GROUP BY version
  ORDER BY version DESC
$$;

CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_distinct_v3 (
  op_ident text,
  op_limit bigint,
  op_offset bigint,
  op_account_id bigint
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
      RETURN QUERY SELECT COUNT(p.partial_ident[1] || '/' || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || '/' || p.partial_ident[2] AS ident
      FROM (SELECT regexp_split_to_array(op.ident, '/') as partial_ident
            FROM origin_packages op
            WHERE op.ident LIKE ('%' || op_ident || '%')
            AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
            ) AS p
      GROUP BY (p.partial_ident[1] || '/' || p.partial_ident[2])
      LIMIT op_limit
      OFFSET op_offset;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_v4 (
  op_ident text,
  op_limit bigint,
  op_offset bigint,
  op_account_id bigint
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
      RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
        FROM origin_packages op
        WHERE op.ident LIKE (op_ident  || '%')
        AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
        ORDER BY op.ident DESC
        LIMIT op_limit
        OFFSET op_offset;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_packages_unique_for_origin_v3 (
  op_origin text,
  op_limit bigint,
  op_offset bigint,
  op_account_id bigint
) RETURNS TABLE(total_count bigint, name text) AS $$
  BEGIN
    RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.name
      FROM origins o INNER JOIN origin_packages op ON o.id = op.origin_id
      WHERE o.name = op_origin
      AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
      GROUP BY op.name
      ORDER BY op.name ASC
      LIMIT op_limit
      OFFSET op_offset;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION search_all_origin_packages_dynamic_v5 (
  op_query text,
  op_account_id bigint
) RETURNS TABLE(ident text) AS $$
  DECLARE
    schema RECORD;
  BEGIN
    FOR schema IN EXECUTE
      format(
        'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
        'shard_'
      )
    LOOP
      RETURN QUERY EXECUTE
      format('SELECT p.partial_ident[1] || %L || p.partial_ident[2] AS ident FROM (SELECT regexp_split_to_array(op.ident, %L) as partial_ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L) AND (op.visibility=%L OR (op.visibility IN (%L, %L) AND op.origin_id IN (SELECT origin_id FROM %I.origin_members WHERE account_id = %L)))) AS p GROUP BY (p.partial_ident[1] || %L || p.partial_ident[2])', '/', '/', schema.schema_name, '%', op_query, '%', 'public', 'private', 'hidden', schema.schema_name, op_account_id, '/');
    END LOOP;
    RETURN;
  END;
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION search_all_origin_packages_v4 (
  op_query text,
  op_account_id bigint
) RETURNS TABLE(ident text) AS $$
  DECLARE
    schema RECORD;
  BEGIN
    FOR schema IN EXECUTE
      format(
        'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
        'shard_'
      )
    LOOP
      RETURN QUERY EXECUTE
      format('SELECT op.ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L) AND (op.visibility=%L OR (op.visibility IN (%L, %L) AND op.origin_id IN (SELECT origin_id FROM %I.origin_members WHERE account_id = %L))) ORDER BY op.ident ASC', schema.schema_name, '%', op_query, '%', 'public', 'private', 'hidden', schema.schema_name, op_account_id);
    END LOOP;
    RETURN;
  END;
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION search_origin_packages_for_origin_v3 (
  op_origin text,
  op_query text,
  op_limit bigint,
  op_offset bigint,
  op_account_id bigint
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
    RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
      FROM origins o INNER JOIN origin_packages op ON o.id = op.origin_id
      WHERE o.name = op_origin
      AND op.name LIKE ('%' || op_query || '%')
      AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
      ORDER BY op.ident ASC
      LIMIT op_limit
      OFFSET op_offset;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_package_v1(text);

DROP FUNCTION IF EXISTS get_origin_package_v2(text, bigint);

CREATE OR REPLACE FUNCTION get_origin_package_v4 (
  op_ident text,
  op_visibilities text
) RETURNS SETOF origin_packages AS $$
  BEGIN
    RETURN QUERY SELECT *
    FROM origin_packages
    WHERE ident = op_ident
    AND visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','));
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_package_latest_v1(text, text);

DROP FUNCTION IF EXISTS get_origin_package_latest_v2(text, text);

DROP FUNCTION IF EXISTS get_origin_package_latest_v3(text, text, bigint);

CREATE OR REPLACE FUNCTION get_origin_package_latest_v5 (
  op_ident text,
  op_target text,
  op_visibilities text
) RETURNS SETOF origin_packages AS $$
  BEGIN
      RETURN QUERY SELECT *
      FROM origin_packages
      WHERE ident LIKE (op_ident  || '%')
      AND target = op_target
      AND visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','));
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_package_versions_for_origin_v1(text, text);

DROP FUNCTION IF EXISTS get_origin_package_versions_for_origin_v2(text, text);

DROP FUNCTION IF EXISTS get_origin_package_versions_for_origin_v3(text, text);

DROP FUNCTION IF EXISTS get_origin_package_versions_for_origin_v4(text, text);

DROP FUNCTION IF EXISTS get_origin_package_versions_for_origin_v5(text, text, bigint);

CREATE OR REPLACE FUNCTION get_origin_package_versions_for_origin_v7 (
    op_origin text,
    op_pkg text,
    op_visibilities text
) RETURNS TABLE(version text, release_count bigint, latest text, platforms text)
LANGUAGE SQL STABLE AS $$
  WITH packages AS (
    SELECT *
    FROM origin_packages op INNER JOIN origins o ON o.id = op.origin_id
    WHERE o.name = op_origin
    AND op.name = op_pkg
    AND op.visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','))
  ), idents AS (
    SELECT regexp_split_to_array(ident, '/') as parts, target
    FROM packages
  )
  SELECT i.parts[3] AS version,
  COUNT(i.parts[4]) AS release_count,
  MAX(i.parts[4]) as latest,
  ARRAY_TO_STRING(ARRAY_AGG(DISTINCT i.target), ',')
  FROM idents i
  GROUP BY version
  ORDER BY version DESC
$$;

DROP FUNCTION IF EXISTS get_origin_package_platforms_for_package_v1(text);

DROP FUNCTION IF EXISTS get_origin_package_platforms_for_package_v2(text, bigint);

CREATE OR REPLACE FUNCTION get_origin_package_platforms_for_package_v4 (
  op_ident text,
  op_visibilities text
) RETURNS TABLE (target text)
LANGUAGE SQL STABLE AS $$
  SELECT DISTINCT target
  FROM origin_packages
  WHERE ident LIKE (op_ident || '%')
  AND visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','))
$$;

DROP FUNCTION IF EXISTS get_origin_packages_for_origin_distinct_v1(text, bigint, bigint);

DROP FUNCTION IF EXISTS get_origin_packages_for_origin_distinct_v2(text, bigint, bigint, bigint);

CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_distinct_v4 (
  op_ident text,
  op_limit bigint,
  op_offset bigint,
  op_visibilities text
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
    RETURN QUERY SELECT COUNT(p.partial_ident[1] || '/' || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || '/' || p.partial_ident[2] AS ident
    FROM (SELECT regexp_split_to_array(op.ident, '/') as partial_ident
          FROM origin_packages op
          WHERE op.ident LIKE ('%' || op_ident || '%')
          AND op.visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','))
          ) AS p
    GROUP BY (p.partial_ident[1] || '/' || p.partial_ident[2])
    LIMIT op_limit
    OFFSET op_offset;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_packages_for_origin_v1(text, bigint, bigint);

DROP FUNCTION IF EXISTS get_origin_packages_for_origin_v2(text, bigint, bigint);

DROP FUNCTION IF EXISTS get_origin_packages_for_origin_v3(text, bigint, bigint, bigint);

CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_v5 (
  op_ident text,
  op_limit bigint,
  op_offset bigint,
  op_visibilities text
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
      RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
        FROM origin_packages op
        WHERE op.ident LIKE (op_ident  || '%')
        AND op.visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','))
        ORDER BY op.ident DESC
        LIMIT op_limit
        OFFSET op_offset;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_packages_unique_for_origin_v1(text, bigint, bigint);

DROP FUNCTION IF EXISTS get_origin_packages_unique_for_origin_v2(text, bigint, bigint, bigint);

CREATE OR REPLACE FUNCTION get_origin_packages_unique_for_origin_v4 (
  op_origin text,
  op_limit bigint,
  op_offset bigint,
  op_visibilities text
) RETURNS TABLE(total_count bigint, name text) AS $$
  BEGIN
      RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.name
        FROM origins o INNER JOIN origin_packages op ON o.id = op.origin_id
        WHERE o.name = op_origin
        AND op.visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','))
        GROUP BY op.name
        ORDER BY op.name ASC
        LIMIT op_limit
        OFFSET op_offset;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS search_all_origin_packages_dynamic_v1(text, bigint, bigint);

DROP FUNCTION IF EXISTS search_all_origin_packages_dynamic_v2(text, bigint, bigint);

DROP FUNCTION IF EXISTS search_all_origin_packages_dynamic_v3(text, bigint, bigint, bigint);

DROP FUNCTION IF EXISTS search_all_origin_packages_dynamic_v4(text, bigint);

CREATE OR REPLACE FUNCTION search_all_origin_packages_dynamic_v6 (
  op_query text,
  op_my_origins text
) RETURNS TABLE(ident text) AS $$
  DECLARE
    schema RECORD;
  BEGIN
    FOR schema IN EXECUTE
      format(
        'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
        'shard_'
      )
    LOOP
      RETURN QUERY EXECUTE
      format('SELECT p.partial_ident[1] || %L || p.partial_ident[2] AS ident FROM (SELECT regexp_split_to_array(op.ident, %L) as partial_ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L) AND (op.visibility = %L OR (op.visibility IN (%L, %L) AND op.origin_id IN (SELECT id FROM %I.origins WHERE name = ANY(STRING_TO_ARRAY(%L, %L)))))) AS p GROUP BY (p.partial_ident[1] || %L || p.partial_ident[2])', '/', '/', schema.schema_name, '%', op_query, '%', 'public', 'hidden', 'private', schema.schema_name, op_my_origins, ',', '/');
    END LOOP;
    RETURN;
  END;
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS search_all_origin_packages_v1(text, bigint, bigint);

DROP FUNCTION IF EXISTS search_all_origin_packages_v2(text, bigint, bigint, bigint);

DROP FUNCTION IF EXISTS search_all_origin_packages_v3(text, bigint);

CREATE OR REPLACE FUNCTION search_all_origin_packages_v5 (
  op_query text,
  op_my_origins text
) RETURNS TABLE(ident text) AS $$
  DECLARE
    schema RECORD;
  BEGIN
    FOR schema IN EXECUTE
      format(
        'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
        'shard_'
      )
    LOOP
      RETURN QUERY EXECUTE
      format('SELECT op.ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L) AND (op.visibility = %L OR (op.visibility IN (%L, %L) AND op.origin_id IN (SELECT id FROM %I.origins WHERE name = ANY(STRING_TO_ARRAY(%L, %L))))) ORDER BY op.ident ASC', schema.schema_name, '%', op_query, '%', 'public', 'hidden', 'private', schema.schema_name, op_my_origins, ',');
    END LOOP;
    RETURN;
  END;
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS search_origin_packages_for_origin_v1(text, text, bigint, bigint);

DROP FUNCTION IF EXISTS search_origin_packages_for_origin_v2(text, text, bigint, bigint, bigint);

CREATE OR REPLACE FUNCTION search_origin_packages_for_origin_v4 (
  op_origin text,
  op_query text,
  op_limit bigint,
  op_offset bigint,
  op_my_origins text
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
      RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
        FROM origins o INNER JOIN origin_packages op ON o.id = op.origin_id
        WHERE o.name = op_origin
        AND op.name LIKE ('%' || op_query || '%')
        AND (op.visibility='public' OR (op.visibility IN ('hidden', 'private') AND o.name = ANY(STRING_TO_ARRAY(op_my_origins, ','))))
        ORDER BY op.ident ASC
        LIMIT op_limit
        OFFSET op_offset;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

UPDATE origin_packages SET scheduler_sync = false;

CREATE SEQUENCE IF NOT EXISTS origin_channel_id_seq;

CREATE TABLE IF NOT EXISTS origin_channels (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_channel_id_seq'),
  origin_id bigint REFERENCES origins(id),
  owner_id bigint,
  name text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  UNIQUE(origin_id, name)
);

CREATE TABLE IF NOT EXISTS origin_channel_packages (
  channel_id bigint REFERENCES origin_channels(id) ON DELETE CASCADE,
  package_id bigint REFERENCES origin_packages(id),
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  PRIMARY KEY (channel_id, package_id)
);

CREATE OR REPLACE FUNCTION insert_origin_channel_v1 (
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
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_channel_v1 (
  ocg_origin text,
  ocg_name text
) RETURNS SETOF origin_channels AS $$
  BEGIN
      RETURN QUERY SELECT origin_channels.*
        FROM origins INNER JOIN origin_channels ON origins.id = origin_channels.origin_id
        WHERE origins.name=ocg_origin AND origin_channels.name = ocg_name;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_channels_for_origin_v1 (
  occ_origin_id bigint
) RETURNS SETOF origin_channels AS $$
  BEGIN
      RETURN QUERY SELECT * FROM origin_channels WHERE origin_id = occ_origin_id
        ORDER BY name ASC;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION promote_origin_package_v1 (
  opp_channel_id bigint,
  opp_package_id bigint
) RETURNS void AS $$
      INSERT INTO origin_channel_packages (channel_id, package_id) VALUES (opp_channel_id, opp_package_id)
      ON CONFLICT ON CONSTRAINT origin_channel_packages_pkey DO NOTHING;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION promote_origin_package_group_v1 (
  opp_channel_id bigint,
  opp_package_ids bigint[]
) RETURNS void
  LANGUAGE SQL
  VOLATILE AS $$
    INSERT INTO origin_channel_packages (channel_id, package_id)
    SELECT opp_channel_id, package_ids.id
    FROM unnest(opp_package_ids) AS package_ids(id)
    ON CONFLICT ON CONSTRAINT origin_channel_packages_pkey DO NOTHING;
$$;

CREATE OR REPLACE FUNCTION demote_origin_package_v1 (
  opp_channel_id bigint,
  opp_package_id bigint
) RETURNS void AS $$
      DELETE FROM origin_channel_packages WHERE channel_id=opp_channel_id AND package_id=opp_package_id;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION delete_origin_channel_v1 (
  channel_id bigint
) RETURNS void AS $$
  BEGIN
      DELETE FROM origin_channels WHERE id = channel_id;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_origin_channel_packages_for_channel_v2 (
  op_origin text,
  op_channel text,
  op_ident text,
  op_account_id bigint,
  op_limit bigint,
  op_offset bigint
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
    RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
      FROM origin_packages op
      INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
      INNER JOIN origin_channels oc on ocp.channel_id = oc.id
      INNER JOIN origins o on oc.origin_id = o.id
      WHERE o.name = op_origin
      AND oc.name = op_channel
      AND op.ident LIKE (op_ident  || '%')
      AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
      ORDER BY ident ASC
      LIMIT op_limit OFFSET op_offset;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_channel_package_latest_v4 (
  op_origin text,
  op_channel text,
  op_ident text,
  op_target text,
  op_account_id bigint
) RETURNS SETOF origin_packages AS $$
  BEGIN
      RETURN QUERY SELECT op.*
        FROM origin_packages op
        INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
        INNER JOIN origin_channels oc on ocp.channel_id = oc.id
        INNER JOIN origins o on oc.origin_id = o.id
        WHERE o.name = op_origin
        AND oc.name = op_channel
        AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
        AND op.ident LIKE (op_ident  || '%')
        AND op.target = op_target;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_channel_package_v3 (
  op_origin text,
  op_channel text,
  op_ident text,
  op_account_id bigint
) RETURNS SETOF origin_packages AS $$
  BEGIN
      RETURN QUERY SELECT op.*
        FROM origin_packages op
        INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
        INNER JOIN origin_channels oc on ocp.channel_id = oc.id
        INNER JOIN origins o on oc.origin_id = o.id
        WHERE op.ident = op_ident
        AND o.name = op_origin
        AND oc.name = op_channel
        AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)));
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_package_channels_for_package_v3 (
  op_ident text,
  op_account_id bigint
) RETURNS SETOF origin_channels AS $$
  BEGIN
      RETURN QUERY SELECT oc.*
          FROM origin_channels oc INNER JOIN origin_channel_packages ocp ON oc.id = ocp.channel_id
          INNER JOIN origin_packages op ON op.id = ocp.package_id
          WHERE op.ident = op_ident
          AND (op.visibility='public' OR (op.visibility IN ('private', 'hidden') AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
          ORDER BY oc.name;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_channel_package_v1(text, text, text);

DROP FUNCTION IF EXISTS get_origin_channel_package_v2(text, text, text, bigint);

CREATE OR REPLACE FUNCTION get_origin_channel_package_v4 (
  op_origin text,
  op_channel text,
  op_ident text,
  op_visibilities text
) RETURNS SETOF origin_packages AS $$
  BEGIN
    RETURN QUERY SELECT op.*
      FROM origin_packages op
      INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
      INNER JOIN origin_channels oc on ocp.channel_id = oc.id
      INNER JOIN origins o on oc.origin_id = o.id
      WHERE op.ident = op_ident
      AND o.name = op_origin
      AND oc.name = op_channel
      AND op.visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','));
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_channel_package_latest_v1(text, text, text, text);

DROP FUNCTION IF EXISTS get_origin_channel_package_latest_v2(text, text, text, text);

DROP FUNCTION IF EXISTS get_origin_channel_package_latest_v3(text, text, text, text, bigint);

CREATE OR REPLACE FUNCTION get_origin_channel_package_latest_v5 (
  op_origin text,
  op_channel text,
  op_ident text,
  op_target text,
  op_visibilities text
) RETURNS SETOF origin_packages AS $$
  BEGIN
    RETURN QUERY SELECT op.*
      FROM origin_packages op
      INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
      INNER JOIN origin_channels oc on ocp.channel_id = oc.id
      INNER JOIN origins o on oc.origin_id = o.id
      WHERE o.name = op_origin
      AND oc.name = op_channel
      AND op.target = op_target
      AND op.visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','))
      AND op.ident LIKE (op_ident  || '%');
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_package_channels_for_package_v1(text);

DROP FUNCTION IF EXISTS get_origin_package_channels_for_package_v2(text, bigint);

CREATE OR REPLACE FUNCTION get_origin_package_channels_for_package_v4 (
  op_ident text,
  op_visibilities text
) RETURNS SETOF origin_channels AS $$
  BEGIN
      RETURN QUERY SELECT oc.*
          FROM origin_channels oc INNER JOIN origin_channel_packages ocp ON oc.id = ocp.channel_id
          INNER JOIN origin_packages op ON op.id = ocp.package_id
          WHERE op.ident = op_ident
          AND op.visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','))
          ORDER BY oc.name;
      RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

DROP FUNCTION IF EXISTS get_origin_channel_packages_for_channel_v1(text, text, text, bigint, bigint);

CREATE OR REPLACE FUNCTION get_origin_channel_packages_for_channel_v3 (
  op_origin text,
  op_channel text,
  op_ident text,
  op_visibilities text,
  op_limit bigint,
  op_offset bigint
) RETURNS TABLE(total_count bigint, ident text) AS $$
  BEGIN
    RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
      FROM origin_packages op
      INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
      INNER JOIN origin_channels oc on ocp.channel_id = oc.id
      INNER JOIN origins o on oc.origin_id = o.id
      WHERE o.name = op_origin
      AND oc.name = op_channel
      AND op.visibility = ANY(STRING_TO_ARRAY(op_visibilities, ','))
      AND op.ident LIKE (op_ident  || '%')
      ORDER BY ident ASC
      LIMIT op_limit OFFSET op_offset;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_origin_channels_for_origin_v2 (
  occ_origin_id bigint,
  occ_include_sandbox_channels bool
) RETURNS SETOF origin_channels AS $$
    SELECT *
    FROM origin_channels
    WHERE origin_id = occ_origin_id
    AND (occ_include_sandbox_channels = true OR (occ_include_sandbox_channels = false AND name NOT LIKE 'bldr-%'))
    ORDER BY name ASC;
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION demote_origin_package_group_v1 (
  opp_channel_id bigint,
  opp_package_ids bigint[]
) RETURNS void
  LANGUAGE SQL
  VOLATILE AS $$
    DELETE FROM origin_channel_packages WHERE channel_id=opp_channel_id AND package_id = ANY(opp_package_ids);
$$;

CREATE OR REPLACE FUNCTION get_origin_project_list_v2 (
  in_origin text
) RETURNS SETOF origin_projects AS $$
    SELECT * FROM origin_projects
    WHERE origin_name = in_origin
    ORDER BY package_name;
$$ LANGUAGE SQL STABLE;