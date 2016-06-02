// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::sync::Arc;

use dbcache::{ConnectionPool, Bucket, InstaSet};
use protocol::InstaId;
use protocol::jobsrv::Job;
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, PipelineCommands};

use error::Result;

pub struct DataStore {
    pub pool: Arc<ConnectionPool>,
    pub jobs: Arc<JobTable>,
    pub job_queue: JobQueue,
}

impl DataStore {
    pub fn open<C: redis::IntoConnectionInfo>(config: C) -> Result<Self> {
        // JW TODO: tune pool from config?
        let pool_cfg = Default::default();
        let manager = RedisConnectionManager::new(config).unwrap();
        let pool = Arc::new(ConnectionPool::new(pool_cfg, manager).unwrap());
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let jobs = Arc::new(JobTable::new(pool1));
        let jobs1 = jobs.clone();
        let job_queue = JobQueue::new(pool2, jobs1);
        Ok(DataStore {
            pool: pool,
            jobs: jobs,
            job_queue: job_queue,
        })
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
