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

//! The PostgreSQL backend for the Jobsrv.

use std::sync::Arc;

use chrono::{DateTime, UTC};
use db::async::{AsyncServer, EventOutcome};
use db::config::{DataStoreCfg, ShardId};
use db::error::{Error as DbError, Result as DbResult};
use db::migration::Migrator;
use db::pool::Pool;
use hab_net::conn::RouteClient;
use postgres;
use postgres::rows::Rows;
use protobuf;
use protocol::net::{NetOk, NetError, ErrCode};
use protocol::{originsrv, jobsrv, scheduler};
use protocol::originsrv::Pageable;
use protobuf::ProtobufEnum;

use error::{Result, Error};

/// DataStore inherints being Send + Sync by virtue of having only one member, the pool itself.
#[derive(Debug, Clone)]
pub struct DataStore {
    pool: Pool,
    pub async: AsyncServer,
}

impl Drop for DataStore {
    fn drop(&mut self) {
        self.async.stop();
    }
}

impl DataStore {
    /// Create a new DataStore.
    ///
    /// * Can fail if the pool cannot be created
    /// * Blocks creation of the datastore on the existince of the pool; might wait indefinetly.
    pub fn new(
        cfg: &DataStoreCfg,
        shards: Vec<ShardId>,
        router_pipe: Arc<String>,
    ) -> Result<DataStore> {
        let pool = Pool::new(cfg, shards)?;
        let ap = pool.clone();
        Ok(DataStore {
            pool: pool,
            async: AsyncServer::new(ap, router_pipe),
        })
    }

    /// Create a new DataStore from a pre-existing pool; useful for testing the database.
    pub fn from_pool(pool: Pool, router_pipe: Arc<String>) -> Result<DataStore> {
        Ok(DataStore {
            async: AsyncServer::new(pool.clone(), router_pipe),
            pool: pool,
        })
    }

    /// Setup the datastore.
    ///
    /// This includes all the schema and data migrations, along with stored procedures for data
    /// access.
    pub fn setup(&self) -> Result<()> {
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup()?;

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
        migrator.migrate("jobsrv",
                         r#"CREATE INDEX pending_jobs_index_v1 on jobs(created_at) WHERE job_state = 'Pending'"#)?;

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

        migrator.migrate("jobsrv", r#"ALTER TABLE jobs ADD COLUMN IF NOT EXISTS build_started_at TIMESTAMPTZ DEFAULT NULL"#)?;
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
            r#"ALTER TABLE jobs ADD COLUMN IF NOT EXISTS log_url TEXT DEFAULT NULL "#,
        )?;
        migrator.migrate(
            "jobsrv",
            r#"CREATE OR REPLACE FUNCTION set_log_url_v1(p_job_id BIGINT, p_url TEXT)
                         RETURNS VOID
                         LANGUAGE SQL VOLATILE as $$
                           UPDATE jobs
                           SET log_url = p_url
                           WHERE id = p_job_id;
                         $$"#,
        )?;

        migrator.migrate("jobsrv", r#"ALTER TABLE jobs ADD COLUMN IF NOT EXISTS archived BOOLEAN DEFAULT FALSE NOT NULL"#)?;
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

        // We switched away from storing the URL in the database
        // before we released, so we don't need to worry about
        // migrating any old data.
        migrator.migrate(
            "jobsrv",
            r#"ALTER TABLE jobs DROP COLUMN IF EXISTS log_url"#,
        )?;
        migrator.migrate(
            "jobsrv",
            r#"DROP FUNCTION IF EXISTS set_log_url_v1(bigint, text)"#,
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
        migrator.finish()?;

        self.async.register("sync_jobs".to_string(), sync_jobs);

        Ok(())
    }

    pub fn start_async(&self) {
        // This is an arc under the hood
        let async_thread = self.async.clone();
        async_thread.start(4);
    }

    /// Create a new job. Sets the state to Pending.
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the job cannot be created
    /// * If the job has an unknown VCS type
    pub fn create_job(&self, job: &jobsrv::Job) -> Result<jobsrv::Job> {
        let conn = self.pool.get_shard(0)?;

        let channel = if job.has_channel() {
            Some(job.get_channel())
        } else {
            None
        };

        if job.get_project().get_vcs_type() == "git" {
            let project = job.get_project();
            let install_id: Option<String> = {
                if project.has_vcs_installation_id() {
                    Some(project.get_vcs_installation_id().to_string())
                } else {
                    None
                }
            };

            let rows = conn.query(
                "SELECT * FROM insert_job_v2($1, $2, $3, $4, $5, $6, $7, $8)",
                &[
                    &(job.get_owner_id() as i64),
                    &(project.get_id() as i64),
                    &project.get_name(),
                    &(project.get_owner_id() as i64),
                    &project.get_plan_path(),
                    &project.get_vcs_type(),
                    &vec![Some(project.get_vcs_data().to_string()), install_id],
                    &channel,
                ],
            ).map_err(Error::JobCreate)?;
            let job = row_to_job(&rows.get(0))?;
            return Ok(job);
        } else {
            return Err(Error::UnknownVCS);
        }
    }

    /// Get a job from the database. If the job does not exist, but the database was active, we'll
    /// get a None result.
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If the job cannot be selected from the database
    pub fn get_job(&self, get_job: &jobsrv::JobGet) -> Result<Option<jobsrv::Job>> {
        let conn = self.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_job_v1($1)",
            &[&(get_job.get_id() as i64)],
        ).map_err(Error::JobGet)?;
        for row in rows {
            let job = row_to_job(&row)?;
            return Ok(Some(job));
        }
        Ok(None)
    }

    /// Get the 50 most recently-created jobs for a given project
    /// (specified as an origin-qualified name, e.g., "core/nginx").
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If a row returned cannot be translated into a Job
    pub fn get_jobs_for_project(
        &self,
        project: &jobsrv::ProjectJobsGet,
    ) -> Result<jobsrv::ProjectJobsGetResponse> {
        let conn = self.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_jobs_for_project_v2($1, $2, $3)",
            &[
                &(project.get_name()),
                &project.limit(),
                &(project.get_start() as i64),
            ],
        ).map_err(Error::ProjectJobsGet)?;

        let mut jobs = protobuf::RepeatedField::new();
        let mut response = jobsrv::ProjectJobsGetResponse::new();
        response.set_start(project.get_start());
        response.set_stop(self.last_index(project, &rows));

        for row in rows {
            let count: i64 = row.get("total_count");
            response.set_count(count as u64);
            jobs.push(row_to_job(&row)?)
        }
        response.set_jobs(jobs);
        Ok(response)
    }

    /// Get a list of pending jobs, up to a maximum count of jobs.
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If the pending jobs cannot be selected from the database
    /// * If the row returned cannot be translated into a Job
    pub fn pending_jobs(&self, count: i32) -> Result<Vec<jobsrv::Job>> {
        let mut jobs = Vec::new();
        let conn = self.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM pending_jobs_v1($1)", &[&count])
            .map_err(Error::JobPending)?;
        for row in rows {
            let job = row_to_job(&row)?;
            jobs.push(job);
        }
        Ok(jobs)
    }

    /// Reset any Dispatched jobs back to Pending state
    /// This is used for recovery scenario
    ///
    /// # Errors
    /// * If a connection cannot be gotten from the pool
    /// * If the dispatched jobs cannot be selected from the database
    pub fn reset_jobs(&self) -> Result<()> {
        let conn = self.pool.get_shard(0)?;
        conn.query("SELECT reset_jobs_v1()", &[]).map_err(
            Error::JobReset,
        )?;
        Ok(())
    }

    /// Updates a job. Currently, this entails updating the state,
    /// build start and stop times, and recording the identifier of
    /// the package the job produced, if any.
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If the job cannot be updated in the database
    pub fn update_job(&self, job: &jobsrv::Job) -> Result<()> {
        let conn = self.pool.get_shard(0)?;
        let job_id = job.get_id() as i64;
        let job_state = job.get_state().to_string();

        // Note: the following fields may all be NULL. As currently
        // coded, if they are NULL, then the corresponding fields in
        // the database will also be updated to be NULL. This should
        // be OK, though, because they shouldn't be changing anyway.
        let build_started_at = if job.has_build_started_at() {
            Some(job.get_build_started_at().parse::<DateTime<UTC>>().unwrap())
        } else {
            None
        };

        let build_finished_at = if job.has_build_finished_at() {
            Some(
                job.get_build_finished_at()
                    .parse::<DateTime<UTC>>()
                    .unwrap(),
            )
        } else {
            None
        };

        let ident = if job.has_package_ident() {
            Some(job.get_package_ident().to_string())
        } else {
            None
        };

        let (err_code, err_msg) = if job.has_error() {
            (
                Some(job.get_error().get_code() as i32),
                Some(job.get_error().get_msg()),
            )
        } else {
            (None, None)
        };

        conn.execute(
            "SELECT update_job_v2($1, $2, $3, $4, $5, $6, $7)",
            &[
                &job_id,
                &job_state,
                &build_started_at,
                &build_finished_at,
                &ident,
                &err_code,
                &err_msg,
            ],
        ).map_err(Error::JobSetState)?;

        self.async.schedule("sync_jobs")?;

        Ok(())
    }

    /// Marks a given job's logs as having been archived. The location
    /// and mechanism for retrieval are dependent on the configured archiving
    /// mechanism.
    pub fn mark_as_archived(&self, job_id: u64) -> Result<()> {
        let conn = self.pool.get_shard(0)?;
        conn.execute("SELECT mark_as_archived_v1($1)", &[&(job_id as i64)])
            .map_err(Error::JobMarkArchived)?;
        Ok(())
    }

    fn last_index<P: Pageable>(&self, list_request: &P, rows: &Rows) -> u64 {
        if rows.len() == 0 {
            list_request.get_range()[1]
        } else {
            list_request.get_range()[0] + (rows.len() as u64) - 1
        }
    }

    /// Create or update a busy worker
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the busy worker cannot be created
    pub fn upsert_busy_worker(&self, bw: &jobsrv::BusyWorker) -> Result<()> {
        let conn = self.pool.get_shard(0)?;

        conn.execute(
            "SELECT FROM upsert_busy_worker_v1($1, $2, $3)",
            &[
                &bw.get_ident(),
                &(bw.get_job_id() as i64),
                &bw.get_quarantined(),
            ],
        ).map_err(Error::BusyWorkerUpsert)?;
        return Ok(());
    }

    /// Delete a busy worker
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the busy worker cannot be created
    pub fn delete_busy_worker(&self, bw: &jobsrv::BusyWorker) -> Result<()> {
        let conn = self.pool.get_shard(0)?;

        conn.execute(
            "SELECT FROM delete_busy_worker_v1($1, $2)",
            &[&bw.get_ident(), &(bw.get_job_id() as i64)],
        ).map_err(Error::BusyWorkerDelete)?;

        return Ok(());
    }

    /// Get a list of busy workers
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the busy workers cannot be created
    pub fn get_busy_workers(&self) -> Result<Vec<jobsrv::BusyWorker>> {
        let conn = self.pool.get_shard(0)?;

        let rows = conn.query("SELECT * FROM get_busy_workers_v1()", &[])
            .map_err(Error::BusyWorkersGet)?;

        let mut workers = Vec::new();
        for row in rows.iter() {
            let bw = row_to_busy_worker(&row)?;
            workers.push(bw);
        }

        return Ok(workers);
    }
}

/// Translate a database `busy_workers` row to a `jobsrv::BusyWorker`.
///
fn row_to_busy_worker(row: &postgres::rows::Row) -> Result<jobsrv::BusyWorker> {
    let mut bw = jobsrv::BusyWorker::new();
    let ident: String = row.get("ident");
    let job_id: i64 = row.get("job_id");
    let quarantined: bool = row.get("quarantined");

    bw.set_ident(ident);
    bw.set_job_id(job_id as u64);
    bw.set_quarantined(quarantined);

    Ok(bw)
}

/// Translate a database `jobs` row to a `jobsrv::Job`.
///
/// # Errors
///
/// * If the job state is unknown
/// * If the VCS type is unknown
fn row_to_job(row: &postgres::rows::Row) -> Result<jobsrv::Job> {
    let mut job = jobsrv::Job::new();
    let id: i64 = row.get("id");
    job.set_id(id as u64);
    let owner_id: i64 = row.get("owner_id");
    job.set_owner_id(owner_id as u64);

    let js: String = row.get("job_state");
    let job_state: jobsrv::JobState = js.parse().map_err(Error::UnknownJobState)?;
    job.set_state(job_state);

    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    job.set_created_at(created_at.to_rfc3339());

    // Note: these may be null (e.g., a job is scheduled, but hasn't
    // started; a job has started and is currently running)
    if let Some(Ok(start)) = row.get_opt::<&str, DateTime<UTC>>("build_started_at") {
        job.set_build_started_at(start.to_rfc3339());
    }
    if let Some(Ok(stop)) = row.get_opt::<&str, DateTime<UTC>>("build_finished_at") {
        job.set_build_finished_at(stop.to_rfc3339());
    }

    // package_ident will only be present if the build succeeded
    if let Some(Ok(ident_str)) = row.get_opt::<&str, String>("package_ident") {
        let ident: originsrv::OriginPackageIdent = ident_str.parse().unwrap();
        job.set_package_ident(ident);
    }

    let mut project = originsrv::OriginProject::new();
    let project_id: i64 = row.get("project_id");
    project.set_id(project_id as u64);

    // only 'project_name' exists in the jobs table, but it's just
    // "origin/name", so we can set those fields in the Project
    // struct.
    //
    // 'package_ident' may be null, though, so we shouldn't use it to
    // get the origin and name.
    let name: String = row.get("project_name");
    let name_for_split = name.clone();
    let name_split: Vec<&str> = name_for_split.split("/").collect();
    project.set_origin_name(name_split[0].to_string());
    project.set_package_name(name_split[1].to_string());
    project.set_name(name);

    let project_owner_id: i64 = row.get("project_owner_id");
    project.set_owner_id(project_owner_id as u64);
    project.set_plan_path(row.get("project_plan_path"));

    let rvcs: String = row.get("vcs");
    match rvcs.as_ref() {
        "git" => {
            let mut vcsa: Vec<Option<String>> = row.get("vcs_arguments");
            project.set_vcs_type(String::from("git"));
            project.set_vcs_data(vcsa.remove(0).expect("expected vcs data"));
            if vcsa.len() > 0 {
                if let Some(install_id) = vcsa.remove(0) {
                    project.set_vcs_installation_id(install_id.parse::<u32>().map_err(
                        Error::ParseVCSInstallationId,
                    )?);
                }
            }
        }
        e => {
            error!("Unknown VCS, {}", e);
            return Err(Error::UnknownVCS);
        }
    }
    job.set_project(project);

    if let Some(Ok(err_msg)) = row.get_opt::<&str, String>("net_error_msg") {
        let err_code: i32 = row.get("net_error_code");
        let mut err = NetError::new();

        if let Some(net_err_code) = ErrCode::from_i32(err_code) {
            err.set_code(net_err_code);
            err.set_msg(err_msg);
            job.set_error(err);
        }
    }

    job.set_is_archived(row.get("archived"));

    if let Some(Ok(channel)) = row.get_opt::<&str, String>("channel") {
        job.set_channel(channel);
    };

    Ok(job)
}

fn sync_jobs(pool: Pool, mut route_conn: RouteClient) -> DbResult<EventOutcome> {
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(*shard)?;
        let rows = &conn.query("SELECT * FROM sync_jobs_v1()", &[]).map_err(
            DbError::AsyncFunctionCheck,
        )?;
        if rows.len() > 0 {
            let mut request = scheduler::JobStatus::new();
            for row in rows.iter() {
                let job = match row_to_job(&row) {
                    Ok(job) => job,
                    Err(e) => {
                        warn!("Failed to convert row to job {}", e);
                        return Ok(EventOutcome::Retry);
                    }
                };
                let id = job.get_id();
                request.set_job(job);
                match route_conn.route::<scheduler::JobStatus, NetOk>(&request) {
                    Ok(_) => {
                        conn.query("SELECT * FROM set_jobs_sync_v1($1)", &[&(id as i64)])
                            .map_err(DbError::AsyncFunctionUpdate)?;
                        debug!("Updated scheduler service with job status, {:?}", request);
                    }
                    Err(e) => {
                        warn!(
                            "Failed to sync job status with the scheduler service, {:?}: {}",
                            request,
                            e
                        );
                        result = EventOutcome::Retry;
                    }
                }
            }
        }
    }
    Ok(result)
}
