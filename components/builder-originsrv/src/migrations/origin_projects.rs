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
        r#"CREATE SEQUENCE IF NOT EXISTS origin_project_id_seq;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origin_projects (
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
                        )"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION insert_origin_project_v1 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_project_v1 (
                    project_name text
                 ) RETURNS SETOF origin_projects AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_projects WHERE name = project_name;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION delete_origin_project_v1 (
                    project_name text
                 ) RETURNS void AS $$
                    BEGIN
                        DELETE FROM origin_projects WHERE name = project_name;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION update_origin_project_v1 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"ALTER TABLE IF EXISTS origin_projects ADD COLUMN IF NOT EXISTS vcs_auth_token text;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"ALTER TABLE IF EXISTS origin_projects ADD COLUMN IF NOT EXISTS vcs_username text;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"ALTER TABLE IF EXISTS origin_projects ADD COLUMN IF NOT EXISTS vcs_installation_id bigint;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION insert_origin_project_v2 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION update_origin_project_v2 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION insert_origin_project_v3 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION update_origin_project_v3 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE SEQUENCE IF NOT EXISTS origin_project_integration_id_seq;"#,
    )?;
    migrator.migrate(
                     "originsrv",
                     r#"CREATE TABLE IF NOT EXISTS origin_project_integrations (
                                     id bigint PRIMARY KEY DEFAULT next_id_v1('origin_project_integration_id_seq'),
                                     origin text,
                                     name text,
                                     integration text,
                                     integration_name text,
                                     body text,
                                     created_at timestamptz DEFAULT now(),
                                     updated_at timestamptz,
                                     UNIQUE (origin, name, integration, integration_name)
                                     )"#,
                 )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION upsert_origin_project_integration_v1 (
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
                              $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_project_integrations_v1 (
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
                                 $$ LANGUAGE SQL STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_project_integrations_for_project_v1 (
                        in_origin text,
                        in_name text
                 ) RETURNS SETOF origin_project_integrations AS $$
                        SELECT * FROM origin_project_integrations
                        WHERE origin = in_origin AND name = in_name
                        ORDER BY integration, integration_name
                    $$ LANGUAGE SQL STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_project_list_v1 (
                        in_origin text
                 ) RETURNS SETOF origin_projects AS $$
                        SELECT * FROM origin_projects
                        WHERE origin_name = in_origin
                    $$ LANGUAGE SQL STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"ALTER TABLE IF EXISTS origin_projects ADD COLUMN IF NOT EXISTS visibility text NOT NULL DEFAULT 'public';"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION insert_origin_project_v3 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION update_origin_project_v3 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION insert_origin_project_v4 (
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
                 $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"ALTER TABLE IF EXISTS origin_project_integrations
            ADD COLUMN IF NOT EXISTS project_id bigint REFERENCES origin_projects(id) ON DELETE CASCADE,
            ADD COLUMN IF NOT EXISTS integration_id bigint REFERENCES origin_integrations(id) ON DELETE CASCADE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"DO $$
        BEGIN
            IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='origin_project_integrations' AND column_name='name') THEN
                UPDATE origin_project_integrations as u1 SET project_id = u2.project_id, integration_id = u2.integration_id FROM
                    (SELECT opi.id as opiid, op.id as project_id, oi.id as integration_id
                    FROM origin_project_integrations opi
                    JOIN origin_projects op ON opi.name = op.package_name
                    JOIN origin_integrations as oi on opi.integration = oi.name
                    WHERE opi.project_id IS NULL
                    AND opi.integration_id IS NULL) as u2
                    WHERE u2.opiid = u1.id;
            END IF;
        END $$"#,
    )?;
    migrator.migrate(
        "originsrv",
        "UPDATE origin_project_integrations SET updated_at = NOW() WHERE updated_at IS NULL;",
    )?;
    migrator.migrate(
        "originsrv",
        r#"ALTER TABLE origin_project_integrations
            DROP COLUMN IF EXISTS name,
            DROP COLUMN IF EXISTS integration,
            DROP COLUMN IF EXISTS integration_name,
            ALTER COLUMN IF EXISTS updated_at SET DEFAULT NOW(),
            ALTER COLUMN body SET NOT NULL,
            ALTER COLUMN created_at SET NOT NULL,
            ALTER COLUMN updated_at SET NOT NULL,
            ALTER COLUMN origin SET NOT NULL,
            ALTER COLUMN project_id SET NOT NULL,
            ALTER COLUMN integration_id SET NOT NULL,
            ADD UNIQUE (project_id, integration_id);"#,
    )?;
    migrator.migrate(
    "originsrv",
    r#"CREATE OR REPLACE FUNCTION upsert_origin_project_integration_v2 (
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
                            $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_project_integrations_v2 (
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
                                 $$ LANGUAGE SQL STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_project_integrations_for_project_v2 (
                        in_origin text,
                        in_name text
                 ) RETURNS SETOF origin_project_integrations AS $$
                        SELECT opi.* FROM origin_project_integrations opi
                        JOIN origin_projects op ON op.id = opi.project_id
                        WHERE origin = in_origin
                        AND package_name = in_name
                    $$ LANGUAGE SQL STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION upsert_origin_project_integration_v3 (
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
                                $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    Ok(())
}
