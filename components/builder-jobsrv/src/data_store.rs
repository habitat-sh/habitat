// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use std::sync::Arc;

use dbcache::{data_store, ConnectionPool, Bucket, InstaSet};
use protocol::InstaId;
use protocol::jobsrv::Job;
use redis::Commands;

use config::Config;
use error::Result;

pub struct DataStore {
    pub pool: Arc<ConnectionPool>,
    pub jobs: Arc<JobTable>,
    pub job_queue: JobQueue,
}

impl data_store::Pool for DataStore {
    type Config = Config;

    fn init(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let jobs = Arc::new(JobTable::new(pool1));
        let jobs1 = jobs.clone();
        let job_queue = JobQueue::new(pool2, jobs1);

        DataStore {
            pool: pool,
            jobs: jobs,
            job_queue: job_queue,
        }
    }
}

pub struct JobTable {
    pool: Arc<ConnectionPool>,
}

impl JobTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        JobTable { pool: pool }
    }
}

impl Bucket for JobTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "job"
    }
}

impl InstaSet for JobTable {
    type Record = Job;

    fn seq_id() -> &'static str {
        "jobs_seq"
    }
}

pub struct JobQueue {
    jobs: Arc<JobTable>,
    pool: Arc<ConnectionPool>,
}

impl JobQueue {
    pub fn new(pool: Arc<ConnectionPool>, jobs: Arc<JobTable>) -> Self {
        JobQueue {
            pool: pool,
            jobs: jobs,
        }
    }

    // JW TODO: clean up this mess
    pub fn enqueue(&self, job: &Job) -> Result<()> {
        let conn = self.pool.get().unwrap();
        let _count: i32 = conn.rpush("job_queue", job.get_id()).unwrap();
        Ok(())
    }

    // JW TODO: clean up this mess
    pub fn dequeue(&self) -> Result<Option<InstaId>> {
        let conn = self.pool.get().unwrap();
        match conn.lpop("job_queue") {
            Ok(id) => Ok(Some(id)),
            Err(_) => Ok(None),
        }
    }

    // JW TODO: clean up this mess
    pub fn peek(&self) -> Result<Option<Job>> {
        let conn = self.pool.get().unwrap();
        match conn.lrange::<&'static str, Vec<u64>>("job_queue", 0, 0) {
            Ok(id) => {
                if id.is_empty() {
                    return Ok(None);
                }
                match self.jobs.find(&id[0]) {
                    Ok(job) => Ok(Some(job.into())),
                    Err(_) => {
                        // JW TODO: we should never hit this
                        try!(self.dequeue());
                        Ok(None)
                    }
                }
            }
            Err(_) => Ok(None),
        }
    }
}
