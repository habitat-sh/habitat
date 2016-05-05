// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::str::FromStr;
use std::sync::Arc;

use dbcache::{ConnectionPool, DataRecord, ValueTable, IndexTable, RecordTable, Table};
use dbcache::model::{Fields, Model};
use protobuf::ProtobufEnum;
use protocol::{self, InstaId};
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, PipelineCommands};
use rustc_serialize::Encodable;

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

impl Table for JobTable {
    type IdType = InstaId;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "job"
    }
}

impl RecordTable for JobTable {
    type Record = Job;

    fn seq_id() -> &'static str {
        "jobs_seq"
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Job {
    pub id: InstaId,
    pub state: protocol::jobsrv::JobState,
}

impl Job {
    pub fn new() -> Self {
        Job {
            id: InstaId::default(),
            state: protocol::jobsrv::JobState::default(),
        }
    }
}

impl Model for Job {
    type Table = JobTable;

    fn fields(&self) -> Fields {
        vec![("state", self.state.value().to_string())]
    }

    fn id(&self) -> &InstaId {
        &self.id
    }

    fn set_id(&mut self, id: InstaId) {
        self.id = id;
    }
}

impl From<DataRecord> for Job {
    fn from(record: DataRecord) -> Job {
        let id = u64::from_str(&record["id"]).unwrap();
        Job {
            id: InstaId(id),
            state: protocol::jobsrv::JobState::from_str(&record["state"]).unwrap(),
        }
    }
}

impl From<protocol::jobsrv::Job> for Job {
    fn from(value: protocol::jobsrv::Job) -> Job {
        let mut job = Job::new();
        job.id = InstaId(value.get_id());
        job.state = value.get_state();
        job
    }
}

impl Into<protocol::jobsrv::Job> for Job {
    fn into(self) -> protocol::jobsrv::Job {
        let mut job = protocol::jobsrv::Job::new();
        job.set_id(self.id.0);
        job.set_state(self.state);
        job
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
        let _count: i32 = conn.rpush("job_queue", *job.id()).unwrap();
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
    pub fn peek(&self) -> Result<Option<protocol::jobsrv::Job>> {
        let conn = self.pool.get().unwrap();
        match conn.lrange::<&'static str, Vec<InstaId>>("job_queue", 0, 0) {
            Ok(id) => {
                if id.is_empty() {
                    return Ok(None);
                }
                match self.jobs.find(id[0]) {
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
