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
        "jobsrv",
        r#"CREATE SEQUENCE IF NOT EXISTS job_id_seq;"#,
    )?;

    // The core jobs table
    migrator.migrate(
        "jobsrv",
        r#"CREATE TABLE IF NOT EXISTS jobs (
                                id bigint PRIMARY KEY DEFAULT next_id_v1('job_id_seq'),
                                owner_id bigint,
                                job_state text,
                                project_id bigint,
                                project_name text,
                                project_owner_id bigint,
                                project_plan_path text,
                                vcs text,
                                vcs_arguments text[],
                                net_error_code int,
                                net_error_msg text,
                                scheduler_sync bool DEFAULT false,
                                created_at timestamptz DEFAULT now(),
                                updated_at timestamptz
                         )"#,
    )?;

    // Insert a new job into the jobs table
    migrator.migrate("jobsrv",
                         r#"CREATE OR REPLACE FUNCTION insert_job_v1 (
                            owner_id bigint,
                            project_id bigint,
                            project_name text,
                            project_owner_id bigint,
                            project_plan_path text,
                            vcs text,
                            vcs_arguments text[]
                            ) RETURNS SETOF jobs AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO jobs (owner_id, job_state, project_id, project_name, project_owner_id, project_plan_path, vcs, vcs_arguments)
                                        VALUES (owner_id, 'Pending', project_id, project_name, project_owner_id, project_plan_path, vcs, vcs_arguments)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#)?;
    // Hey, Adam - why did you do `select *` here? Isn't that bad?
    //
    // So glad you asked. In this case, it's better - essentially we have an API call that
    // returns a job object, which is flattened into the table structure above. We then
    // translate those job rows into Job structs. Since the table design is purely additive,
    // this allows us to add data to the table without having to re-roll functions that
    // generate Job structs, and keeps things DRY.
    //
    // Just make sure you always address the columns by name, not by position.
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_job_v1 (jid bigint) RETURNS SETOF jobs AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM jobs WHERE id = jid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
    )?;

    // The pending_jobs function acts as an internal queue. It looks for jobs that are
    // 'Pending', and sorts them according to the time they were entered - first in, first out.
    //
    // You can pass this function a number of jobs to return - it will return up-to that many.
    //
    // The way it works internally - we select the rows for update, skipping rows that are
    // already locked. That means multiple jobsrvs, or multiple worker managers, can run in
    // parallel against the table - they will simply skip other rows that are currently on the
    // way out the door.
    //
    // Any row selected gets its state updated to "Dispatched" before being sent back, ensuring
    // that no other worker could receive the job. If we fail to dispatch the job, its the
    // callers job to change the jobs status back to "Pending", which puts it back in the
    // queue.
    //
    // Note that the sort order ensures that jobs that fail to dispatch and are then returned
    // will be the first job selected, making FIFO a reality.
    migrator.migrate("jobsrv",
                     r#"CREATE OR REPLACE FUNCTION pending_jobs_v1 (integer) RETURNS SETOF jobs AS
                            $$
                            DECLARE
                                r jobs % rowtype;
                            BEGIN
                                FOR r IN
                                    SELECT * FROM jobs
                                    WHERE job_state = 'Pending'
                                    ORDER BY created_at ASC
                                    FOR UPDATE SKIP LOCKED
                                    LIMIT $1
                                LOOP
                                    UPDATE jobs SET job_state='Dispatched', scheduler_sync=false, updated_at=now() WHERE id=r.id RETURNING * INTO r;
                                    RETURN NEXT r;
                                END LOOP;
                              RETURN;
                            END
                            $$ LANGUAGE plpgsql VOLATILE"#)?;

    // Reset the state of Dispatched jobs back to Pending (for failure recovery)
    migrator.migrate("jobsrv",
                     r#"CREATE OR REPLACE FUNCTION reset_jobs_v1 () RETURNS void AS $$
                            BEGIN
                                UPDATE jobs SET job_state='Pending', scheduler_sync=false, updated_at=now() WHERE job_state='Dispatched';
                            END
                            $$ LANGUAGE plpgsql VOLATILE"#)?;

    // Update the state of a job. Takes a job id and a state, and updates that row.
    migrator.migrate("jobsrv",
                     r#"CREATE OR REPLACE FUNCTION set_job_state_v1 (jid bigint, jstate text) RETURNS void AS $$
                        BEGIN
                            UPDATE jobs SET job_state=jstate, scheduler_sync=false, updated_at=now() WHERE id=jid;
                        END
                     $$ LANGUAGE plpgsql VOLATILE"#)?;

    // Helpers to sync job state notifications
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION sync_jobs_v1() RETURNS SETOF jobs AS $$
                     BEGIN
                         RETURN QUERY SELECT * FROM jobs WHERE scheduler_sync = false;
                         RETURN;
                     END
                     $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION set_jobs_sync_v1(in_job_id bigint) RETURNS VOID AS $$
                     BEGIN
                         UPDATE jobs SET scheduler_sync = true WHERE id = in_job_id;
                     END
                     $$ LANGUAGE plpgsql VOLATILE"#,
    )?;

    migrator.migrate(
        "jobsrv",
        r#"DROP INDEX IF EXISTS pending_jobs_index_v1;"#,
    )?;
    migrator.migrate(
        "jobsrv",
        r#"CREATE INDEX pending_jobs_index_v1 on jobs(created_at) WHERE job_state = 'Pending'"#,
    )?;

    // We're deliberately returning only the 50 most
    // recently-created jobs here. A future version of this
    // function may take additional parameters for sorting,
    // filtering, and pagination.
    //
    // Also deliberately using `SELECT *` here, for the same
    // reasons listed above for `get_job_v1`.
    //
    // Note that `project_name` here is an origin-qualified
    // project name, e.g. "core/nginx".
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_jobs_for_project_v1(p_project_name TEXT)
                     RETURNS SETOF jobs
                     LANGUAGE SQL STABLE AS $$
                       SELECT *
                       FROM jobs
                       WHERE project_name = p_project_name
                       ORDER BY created_at DESC
                       LIMIT 50;
                     $$"#,
    )?;

    migrator.migrate(
        "jobsrv",
        r#"ALTER TABLE jobs ADD COLUMN IF NOT EXISTS build_started_at TIMESTAMPTZ DEFAULT NULL"#,
    )?;
    migrator.migrate("jobsrv", r#"ALTER TABLE jobs ADD COLUMN IF NOT EXISTS build_finished_at TIMESTAMPTZ DEFAULT NULL"#)?;
    migrator.migrate(
        "jobsrv",
        r#"ALTER TABLE jobs ADD COLUMN IF NOT EXISTS package_ident TEXT DEFAULT NULL"#,
    )?;

    // Removing the `set_job_state_v1` function in favor of a more
    // general `update_job_v1` function.
    migrator.migrate(
        "jobsrv",
        r#"DROP FUNCTION IF EXISTS set_job_state_v1(bigint, text)"#,
    )?;
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION update_job_v1(
                       p_job_id bigint,
                       p_state text,
                       p_build_started_at timestamptz,
                       p_build_finished_at timestamptz,
                       p_package_ident text)
                     RETURNS VOID
                     LANGUAGE SQL VOLATILE AS $$
                       UPDATE jobs
                       SET job_state = p_state,
                           scheduler_sync = false,
                           updated_at = now(),
                           build_started_at = p_build_started_at,
                           build_finished_at = p_build_finished_at,
                           package_ident = p_package_ident
                       WHERE id = p_job_id;
                     $$"#,
    )?;

    migrator.migrate(
        "jobsrv",
        r#"ALTER TABLE jobs ADD COLUMN IF NOT EXISTS archived BOOLEAN DEFAULT FALSE NOT NULL"#,
    )?;
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION mark_as_archived_v1(p_job_id BIGINT)
                     RETURNS VOID
                     LANGUAGE SQL VOLATILE as $$
                       UPDATE jobs
                       SET archived = TRUE
                       WHERE id = p_job_id;
                     $$"#,
    )?;

    migrator.migrate(
        "jobsrv-2",
        r#"CREATE OR REPLACE FUNCTION update_job_v2(
                       p_job_id bigint,
                       p_state text,
                       p_build_started_at timestamptz,
                       p_build_finished_at timestamptz,
                       p_package_ident text,
                       p_err_code int,
                       p_err_msg text)
                     RETURNS VOID
                     LANGUAGE SQL VOLATILE AS $$
                       UPDATE jobs
                       SET job_state = p_state,
                           scheduler_sync = false,
                           updated_at = now(),
                           build_started_at = p_build_started_at,
                           build_finished_at = p_build_finished_at,
                           package_ident = p_package_ident,
                           net_error_code = p_err_code,
                           net_error_msg = p_err_msg
                       WHERE id = p_job_id;
                     $$"#,
    )?;
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_jobs_for_project_v2(p_project_name TEXT, p_limit bigint, p_offset bigint)
                     RETURNS TABLE (total_count bigint, id bigint, owner_id bigint, job_state text, created_at timestamptz,
                                    build_started_at timestamptz, build_finished_at timestamptz, package_ident text,
                                    project_id bigint, project_name text, project_owner_id bigint, project_plan_path text,
                                    vcs text, vcs_arguments text[], net_error_msg text, net_error_code integer, archived boolean)
                     LANGUAGE SQL STABLE AS $$
                       SELECT COUNT(*) OVER () AS total_count, id, owner_id, job_state, created_at, build_started_at,
                       build_finished_at, package_ident, project_id, project_name, project_owner_id, project_plan_path, vcs,
                       vcs_arguments, net_error_msg, net_error_code, archived
                       FROM jobs
                       WHERE project_name = p_project_name
                       ORDER BY created_at DESC
                       LIMIT p_limit
                       OFFSET p_offset;
                     $$"#,
    )?;
    migrator.migrate(
        "jobsrv",
        r#"ALTER TABLE jobs ADD COLUMN IF NOT EXISTS channel TEXT DEFAULT NULL"#,
    )?;

    migrator.migrate("jobsrv",
                         r#"CREATE OR REPLACE FUNCTION insert_job_v2 (
                            p_owner_id bigint,
                            p_project_id bigint,
                            p_project_name text,
                            p_project_owner_id bigint,
                            p_project_plan_path text,
                            p_vcs text,
                            p_vcs_arguments text[],
                            p_channel text
                            ) RETURNS SETOF jobs AS $$
                                INSERT INTO jobs (owner_id, job_state, project_id, project_name, project_owner_id, project_plan_path, vcs, vcs_arguments, channel)
                                VALUES (p_owner_id, 'Pending', p_project_id, p_project_name, p_project_owner_id, p_project_plan_path, p_vcs, p_vcs_arguments, p_channel)
                                RETURNING *;
                            $$ LANGUAGE SQL VOLATILE"#)?;

    // The busy workers table
    migrator.migrate(
        "jobsrv",
        r#"CREATE TABLE IF NOT EXISTS busy_workers (
                            ident text,
                            job_id bigint,
                            quarantined bool,
                            created_at timestamptz DEFAULT now(),
                            updated_at timestamptz,
                            UNIQUE(ident, job_id)
                     )"#,
    )?;

    // Insert or update a new worker into the busy workers table
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION upsert_busy_worker_v1 (
                            in_ident text,
                            in_job_id bigint,
                            in_quarantined bool
                        ) RETURNS SETOF busy_workers AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO busy_workers (ident, job_id, quarantined)
                                    VALUES (in_ident, in_job_id, in_quarantined)
                                    ON CONFLICT(ident, job_id)
                                    DO UPDATE SET quarantined=in_quarantined RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                        "#,
    )?;

    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION get_busy_workers_v1()
                         RETURNS SETOF busy_workers AS $$
                           SELECT * FROM busy_workers
                         $$ LANGUAGE SQL STABLE
                        "#,
    )?;

    // Delete a worker from the busy workers table
    migrator.migrate(
        "jobsrv",
        r#"CREATE OR REPLACE FUNCTION delete_busy_worker_v1 (
                            in_ident text,
                            in_job_id bigint
                        ) RETURNS void AS $$
                            DELETE FROM busy_workers
                            WHERE ident = in_ident AND job_id = in_job_id
                        $$ LANGUAGE SQL VOLATILE
                    "#,
    )?;

    Ok(())
}
