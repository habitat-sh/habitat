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
    // The graph packages table
    migrator.migrate(
        "jobsrv",
        r#"CREATE TABLE IF NOT EXISTS graph_packages (
                                 id bigserial PRIMARY KEY,
                                 ident text UNIQUE,
                                 deps text[],
                                 target TEXT DEFAULT NULL,
                                 created_at timestamptz DEFAULT now()
                          )"#,
    )?;

    // The groups table
    migrator.migrate(
        "jobsrv",
        r#"CREATE SEQUENCE IF NOT EXISTS groups_id_seq;"#,
    )?;

    migrator.migrate(
        "jobsrv",
        r#"CREATE TABLE IF NOT EXISTS groups (
                                id bigint PRIMARY KEY DEFAULT next_id_v1('groups_id_seq'),
                                group_state text,
                                project_name text DEFAULT NULL,
                                created_at timestamptz DEFAULT now(),
                                updated_at timestamptz
                         )"#,
    )?;

    migrator.migrate(
        "jobsrv",
        r#"DROP INDEX IF EXISTS pending_groups_index_v1;"#,
    )?;

    migrator.migrate("jobsrv",
                     r#"CREATE INDEX pending_groups_index_v1 on groups(created_at) WHERE group_state = 'Pending'"#)?;

    // The group projects table
    migrator.migrate(
        "jobsrv",
        r#"CREATE TABLE IF NOT EXISTS group_projects (
                                  id bigserial PRIMARY KEY,
                                  owner_id bigint,
                                  project_name text,
                                  project_ident text,
                                  project_state text,
                                  job_id bigint DEFAULT 0,
                                  created_at timestamptz DEFAULT now(),
                                  updated_at timestamptz
                           )"#,
    )?;

    // Insert or update a new package into the packages table
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION upsert_graph_package_v1 (
                                in_ident text,
                                in_deps text[],
                                in_target text
                            ) RETURNS SETOF graph_packages AS $$
                                    BEGIN
                                        RETURN QUERY INSERT INTO graph_packages (ident, deps, target)
                                        VALUES (in_ident, in_deps, in_target)
                                        ON CONFLICT(ident)
                                        DO UPDATE SET deps=in_deps, target=in_target RETURNING *;
                                        RETURN;
                                    END
                                $$ LANGUAGE plpgsql VOLATILE
                            "#,
    )?;

    // Retrieve all packages from the packages table
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_graph_packages_v1 () RETURNS SETOF graph_packages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM graph_packages;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
    )?;

    // Count all packages from the packages table that belong to specific origin
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION count_graph_packages_v1 (origin text) RETURNS bigint AS $$
                        BEGIN
                          RETURN COUNT(*) FROM graph_packages WHERE ident ~ ('^' || origin || '/');
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
    )?;

    // Retrieve a single packages from the packages table
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_graph_package_v1 (pident text) RETURNS SETOF graph_packages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM graph_packages
                          WHERE ident = pident;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
    )?;

    // Insert a new group into the groups table, and add it's projects to the projects table
    migrator.migrate("jobsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_group_v1 (
                            root_project text,
                            project_names text[],
                            project_idents text[]
                            ) RETURNS SETOF groups
                              LANGUAGE SQL
                              VOLATILE AS $$
                              WITH my_group AS (
                                      INSERT INTO groups (project_name, group_state)
                                      VALUES (root_project, 'Pending') RETURNING *
                                  ), my_project AS (
                                      INSERT INTO group_projects (owner_id, project_name, project_ident, project_state)
                                      SELECT g.id, project_info.name, project_info.ident, 'NotStarted'
                                      FROM my_group AS g, unnest(project_names, project_idents) AS project_info(name, ident)
                                  )
                              SELECT * FROM my_group;
                            $$"#)?;

    // Retrieve a group from the groups table
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_group_v1 (gid bigint) RETURNS SETOF groups AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM groups WHERE id = gid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
    )?;

    // Retrieve the projects for a group
    migrator.migrate("jobsrv",
                 r#"CREATE OR REPLACE FUNCTION get_group_projects_for_group_v1 (gid bigint) RETURNS SETOF group_projects AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM group_projects WHERE owner_id = gid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#)?;

    // Count all projects from the projects table that belong to specific origin
    migrator
        .migrate("jobsrv",
                 r#"CREATE OR REPLACE FUNCTION count_group_projects_v1 (origin text) RETURNS bigint AS $$
                        BEGIN
                          RETURN COUNT(*) FROM group_projects WHERE project_ident ~ ('^' || origin || '/');
                        END
                        $$ LANGUAGE plpgsql STABLE"#)?;

    // Retrieve Pending groups, while atomically setting their state to Dispatched
    migrator.migrate("jobsrv",
                     r#"CREATE OR REPLACE FUNCTION pending_groups_v1 (integer) RETURNS SETOF groups AS
                            $$
                            DECLARE
                                r groups % rowtype;
                            BEGIN
                                FOR r IN
                                    SELECT * FROM groups
                                    WHERE group_state = 'Pending'
                                    ORDER BY created_at ASC
                                    FOR UPDATE SKIP LOCKED
                                    LIMIT $1
                                LOOP
                                    UPDATE groups SET group_state='Dispatching', updated_at=now() WHERE id=r.id RETURNING * INTO r;
                                    RETURN NEXT r;
                                END LOOP;
                              RETURN;
                            END
                            $$ LANGUAGE plpgsql VOLATILE"#)?;

    // Update the state of a group
    migrator.migrate("jobsrv",
                     r#"CREATE OR REPLACE FUNCTION set_group_state_v1 (gid bigint, gstate text) RETURNS void AS $$
                        BEGIN
                            UPDATE groups SET group_state=gstate, updated_at=now() WHERE id=gid;
                        END
                     $$ LANGUAGE plpgsql VOLATILE"#)?;

    // Update the state of a project
    migrator.migrate("jobsrv",
                      r#"CREATE OR REPLACE FUNCTION set_group_project_state_v1 (pid bigint, jid bigint, state text) RETURNS void AS $$
                         BEGIN
                             UPDATE group_projects SET project_state=state, job_id=jid, updated_at=now() WHERE id=pid;
                         END
                      $$ LANGUAGE plpgsql VOLATILE"#)?;

    // Retrieve a group project
    migrator.migrate("jobsrv",
              r#"CREATE OR REPLACE FUNCTION find_group_project_v1 (gid bigint, name text) RETURNS SETOF group_projects AS $$
                     BEGIN
                       RETURN QUERY SELECT * FROM group_projects WHERE owner_id = gid AND project_name = name;
                       RETURN;
                     END
                     $$ LANGUAGE plpgsql STABLE"#)?;

    // Update the state of a project given its name
    migrator.migrate("jobsrv",
                         r#"CREATE OR REPLACE FUNCTION set_group_project_name_state_v1 (gid bigint, pname text, state text) RETURNS void AS $$
                            BEGIN
                                UPDATE group_projects SET project_state=state, updated_at=now() WHERE owner_id=gid AND project_name=pname;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#)?;

    // Update the state and ident of a project
    migrator.migrate("jobsrv",
                       r#"CREATE OR REPLACE FUNCTION set_group_project_state_ident_v1 (pid bigint, jid bigint, state text, ident text) RETURNS void AS $$
                              UPDATE group_projects SET project_state=state, job_id=jid, project_ident=ident, updated_at=now() WHERE id=pid;
                       $$ LANGUAGE SQL VOLATILE"#)?;

    // Check for an active (Pending or Dispatched) group with a given root project name
    migrator.migrate("jobsrv",
                    r#"CREATE OR REPLACE FUNCTION check_active_group_v1(pname text) RETURNS SETOF groups AS $$
                           SELECT * FROM groups
                           WHERE project_name = pname
                           AND group_state IN ('Pending', 'Dispatching')
                    $$ LANGUAGE SQL VOLATILE"#)?;

    // get a count on the number of unique packages in an origin
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION count_unique_graph_packages_v1 (
                              op_origin text
                            ) RETURNS bigint
                            LANGUAGE SQL
                            STABLE AS $$
                            WITH idents AS (
                              SELECT regexp_split_to_array(ident, '/') as parts
                              FROM graph_packages
                            )
                            SELECT COUNT(DISTINCT i.parts[2]) AS total
                            FROM idents i
                            WHERE i.parts[1] = op_origin
                            $$"#,
    )?;

    // Abort a group (experimental)
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION abort_group_v1(in_gid bigint) RETURNS void AS $$
            UPDATE group_projects SET project_state='Failure'
                WHERE owner_id = in_gid
                AND (project_state = 'InProgress' OR project_state = 'NotStarted');
            UPDATE groups SET group_state='Complete' where id = in_gid;
        $$ LANGUAGE SQL VOLATILE
        "#,
    )?;

    // Insert a new group into the groups table, and add it's projects to the projects table
    migrator.migrate("jobsrv",
         r#"CREATE OR REPLACE FUNCTION insert_group_v2 (
                    root_project text,
                    project_names text[],
                    project_idents text[]
                    ) RETURNS SETOF groups
                      LANGUAGE SQL
                      VOLATILE AS $$
                      WITH my_group AS (
                              INSERT INTO groups (project_name, group_state)
                              VALUES (root_project, 'Queued') RETURNING *
                          ), my_project AS (
                              INSERT INTO group_projects (owner_id, project_name, project_ident, project_state)
                              SELECT g.id, project_info.name, project_info.ident, 'NotStarted'
                              FROM my_group AS g, unnest(project_names, project_idents) AS project_info(name, ident)
                          )
                      SELECT * FROM my_group;
                    $$"#)?;

    // Retrieve a queued group from the groups table
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_queued_group_v1 (pname text) RETURNS SETOF groups AS $$
                  SELECT * FROM groups
                  WHERE project_name = pname
                  AND group_state = 'Queued'
            $$ LANGUAGE SQL VOLATILE"#,
    )?;

    migrator.migrate(
        "jobsrv",
        r#"DROP INDEX IF EXISTS queued_groups_index_v1;"#,
    )?;

    migrator.migrate("jobsrv",
                     r#"CREATE INDEX queued_groups_index_v1 on groups(created_at) WHERE group_state = 'Queued'"#)?;

    // Retrieve all queued groups from the groups table
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_queued_groups_v1 () RETURNS SETOF groups AS $$
                  SELECT * FROM groups
                  WHERE group_state = 'Queued'
            $$ LANGUAGE SQL VOLATILE"#,
    )?;

    // Cancel a job group
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION cancel_group_v1(in_gid bigint) RETURNS void AS $$
            UPDATE group_projects SET project_state='Canceled'
                WHERE owner_id = in_gid
                AND (project_state = 'NotStarted');
            UPDATE groups SET group_state='Canceled' where id = in_gid;
        $$ LANGUAGE SQL VOLATILE"#,
    )?;

    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_job_groups_for_origin_v1 (
            op_origin text
        ) RETURNS TABLE(id bigint, group_state text, created_at timestamptz, project_name text, project_ident text, project_state text, job_id bigint) AS $$
            SELECT g.id, g.group_state, g.created_at, gp.project_name, gp.project_ident, gp.project_state, gp.job_id
            FROM groups g INNER JOIN group_projects gp ON g.id = gp.owner_id
            WHERE g.project_name LIKE (op_origin || '/%')
            ORDER BY g.group_state, g.project_name, gp.project_ident
        $$ LANGUAGE SQL STABLE"#,
    )?;

    Ok(())
}
