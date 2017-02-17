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

use db::pool::Pool;
use db::migration::Migrator;
use protocol::{vault, jobsrv};
use postgres;

use config::Config;
use error::{Result, Error};
use rand::{Rng, thread_rng};

/// DataStore inherints being Send + Sync by virtue of having only one member, the pool itself.
#[derive(Debug, Clone)]
pub struct DataStore {
    pool: Pool,
}

impl DataStore {
    /// Create a new DataStore.
    ///
    /// * Can fail if the pool cannot be created
    /// * Blocks creation of the datastore on the existince of the pool; might wait indefinetly.
    pub fn new(config: &Config) -> Result<DataStore> {
        let pool = Pool::new(&config.datastore_connection_url,
                             config.pool_size,
                             config.datastore_connection_retry_ms,
                             config.datastore_connection_timeout,
                             config.datastore_connection_test)?;
        Ok(DataStore { pool: pool })
    }

    /// Create a new DataStore from a pre-existing pool; useful for testing the database.
    pub fn from_pool(pool: Pool) -> Result<DataStore> {
        Ok(DataStore { pool: pool })
    }

    /// Setup the datastore.
    ///
    /// This includes all the schema and data migrations, along with stored procedures for data
    /// access.
    pub fn setup(&self) -> Result<()> {
        let migrator = Migrator::new(&self.pool);
        migrator.setup()?;

        // The core jobs table
        migrator.migrate("jobsrv",
                     1,
                     r#"CREATE TABLE jobs (
                                    id bigint PRIMARY KEY,
                                    owner_id bigint,
                                    job_state text,
                                    project_id text,
                                    project_owner_id bigint,
                                    project_plan_path text,
                                    vcs text,
                                    vcs_arguments text[],
                                    net_error_code int,
                                    net_error_msg text,
                                    created_at timestamptz DEFAULT now(),
                                    updated_at timestamptz
                             )"#)?;

        // Insert a new job into the jobs table
        migrator.migrate("jobsrv",
                             2,
                             r#"CREATE OR REPLACE FUNCTION insert_job_v1 (
                                id bigint,
                                owner_id bigint,
                                project_id text,
                                project_owner_id bigint,
                                project_plan_path text,
                                vcs text,
                                vcs_arguments text[]
                                ) RETURNS void AS $$
                                    BEGIN
                                        INSERT INTO jobs (id, owner_id, job_state, project_id, project_owner_id, project_plan_path, vcs, vcs_arguments)
                                        VALUES
                                            (id, owner_id, 'Pending', project_id, project_owner_id, project_plan_path, vcs, vcs_arguments);
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
        migrator.migrate("jobsrv",
                     3,
                     r#"CREATE OR REPLACE FUNCTION get_job_v1 (jid bigint) RETURNS SETOF jobs AS $$
                            BEGIN
                              RETURN QUERY SELECT * FROM jobs WHERE id = jid;
                              RETURN;
                            END
                            $$ LANGUAGE plpgsql STABLE"#)?;

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
                         4,
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
                                        UPDATE jobs SET job_state='Dispatched', updated_at=now() WHERE id=r.id RETURNING * INTO r;
                                        RETURN NEXT r;
                                    END LOOP;
                                  RETURN;
                                END
                                $$ LANGUAGE plpgsql VOLATILE"#)?;

        // Update the state of a job. Takes a job id and a state, and updates that row.
        migrator.migrate("jobsrv",
                         5,
                         r#"CREATE OR REPLACE FUNCTION set_job_state_v1 (jid bigint, jstate text) RETURNS void AS $$
                            BEGIN
                                UPDATE jobs SET job_state=jstate, updated_at=now() WHERE id=jid;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#)?;
        Ok(())
    }

    /// Create a new job. Sets the state to Pending.
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the job cannot be created
    /// * If the job has an unknown VCS type
    pub fn create_job(&self, job: &mut jobsrv::Job) -> Result<()> {
        let conn = self.pool.get()?;

        if job.get_project().has_git() {
            // BUG - the insert query should be creating and assigning back a job_id,
            // instead of expecting it to be passed in. The random id is a temporary
            // workaround.
            let mut rng = thread_rng();
            let id = rng.gen::<u64>();
            job.set_id(id);

            conn.execute("SELECT insert_job_v1($1, $2, $3, $4, $5, $6, $7)",
                         &[&(job.get_id() as i64),
                           &(job.get_owner_id() as i64),
                           &job.get_project().get_id(),
                           &(job.get_project().get_owner_id() as i64),
                           &job.get_project().get_plan_path(),
                           &"git",
                           &vec![job.get_project().get_git().get_url()]])
                .map_err(Error::JobCreate)?;
        } else {
            return Err(Error::UnknownVCS);
        }
        Ok(())
    }

    /// Translate a database `jobs` row to a `jobsrv::Job`.
    ///
    /// # Errors
    ///
    /// * If the job state is unknown
    /// * If the VCS type is unknown
    fn row_to_job(&self, row: &postgres::rows::Row) -> Result<jobsrv::Job> {
        let mut job = jobsrv::Job::new();
        let id: i64 = row.get("id");
        job.set_id(id as u64);
        let owner_id: i64 = row.get("owner_id");
        job.set_owner_id(owner_id as u64);
        let js: String = row.get("job_state");
        let job_state = match &js[..] {
            "Dispatched" => jobsrv::JobState::Dispatched,
            "Pending" => jobsrv::JobState::Pending,
            "Processing" => jobsrv::JobState::Processing,
            "Complete" => jobsrv::JobState::Complete,
            "Rejected" => jobsrv::JobState::Rejected,
            "Failed" => jobsrv::JobState::Failed,
            _ => return Err(Error::UnknownJobState),
        };
        job.set_state(job_state);

        let mut project = vault::Project::new();
        project.set_id(row.get("project_id"));
        let project_owner_id: i64 = row.get("project_owner_id");
        project.set_owner_id(project_owner_id as u64);
        project.set_plan_path(row.get("project_plan_path"));

        let rvcs: String = row.get("vcs");
        match rvcs.as_ref() {
            "git" => {
                let mut vcs = vault::VCSGit::new();
                let mut vcsa: Vec<String> = row.get("vcs_arguments");
                vcs.set_url(vcsa.remove(0));
                project.set_git(vcs);
            }
            e => {
                error!("Unknown VCS, {}", e);
                return Err(Error::UnknownVCS);
            }
        }
        job.set_project(project);
        Ok(job)
    }

    /// Get a job from the database. If the job does not exist, but the database was active, we'll
    /// get a None result.
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If the job cannot be selected from the database
    pub fn get_job(&self, id: u64) -> Result<Option<jobsrv::Job>> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM get_job_v1($1)", &[&(id as i64)])
            .map_err(Error::JobGet)?;
        for row in rows {
            let job = self.row_to_job(&row)?;
            return Ok(Some(job));
        }
        Ok(None)
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
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM pending_jobs_v1($1)", &[&count])
            .map_err(Error::JobPending)?;
        for row in rows {
            let job = self.row_to_job(&row)?;
            jobs.push(job);
        }
        Ok(jobs)
    }

    /// Set the state of a job. If the job does not exist in the database, its basically a no-op.
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If the jobs state cannot be updated in the database
    pub fn set_job_state(&self, job: &jobsrv::Job) -> Result<()> {
        let conn = self.pool.get()?;
        let job_id = job.get_id() as i64;
        let job_state = match job.get_state() {
            jobsrv::JobState::Dispatched => "Dispatched",
            jobsrv::JobState::Pending => "Pending",
            jobsrv::JobState::Processing => "Processing",
            jobsrv::JobState::Complete => "Complete",
            jobsrv::JobState::Rejected => "Rejected",
            jobsrv::JobState::Failed => "Failed",
        };
        conn.execute("SELECT set_job_state_v1($1, $2)", &[&job_id, &job_state])
            .map_err(Error::JobSetState)?;
        Ok(())
    }
}
