// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::sync::Arc;

use dbcache::{self, ConnectionPool, Bucket, IndexSet, InstaSet};
use protobuf::Message;
use protocol::{vault, InstaId, Persistable};
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, PipelineCommands};

use error::Result;

pub struct DataStore {
    pub pool: Arc<ConnectionPool>,
    pub origins: OriginTable,
}

impl DataStore {
    pub fn open<C: redis::IntoConnectionInfo>(config: C) -> Result<Self> {
        // JW TODO: tune pool from config?
        let pool_cfg = Default::default();
        let manager = RedisConnectionManager::new(config).unwrap();
        let pool = Arc::new(ConnectionPool::new(pool_cfg, manager).unwrap());
        let pool1 = pool.clone();
        let origins = OriginTable::new(pool1);
        Ok(DataStore {
            pool: pool,
            origins: origins,
        })
    }
}

pub struct OriginTable {
    pool: Arc<ConnectionPool>,
    pub name_idx: OriginNameIdx,
}

impl OriginTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let name_idx = OriginNameIdx::new(pool1);
        OriginTable {
            pool: pool,
            name_idx: name_idx,
        }
    }
}

impl Bucket for OriginTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin"
    }
}

impl InstaSet for OriginTable {
    type Record = vault::Origin;

    fn seq_id() -> &'static str {
        "origins_seq"
    }

    fn write(&self, record: &mut Self::Record) -> dbcache::Result<()> {
        let conn = try!(self.pool().get());
        try!(redis::transaction(conn.deref(), &[Self::seq_id()], |txn| {
            let sequence_id: u64 = match conn.get::<&'static str, u64>(Self::seq_id()) {
                Ok(value) => value + 1,
                _ => 0,
            };
            let insta_id = InstaId::generate(sequence_id);
            record.set_primary_key(*insta_id);
            txn.set(Self::seq_id(), record.primary_key())
                .ignore()
                .set(Self::key(&record.primary_key()),
                     record.write_to_bytes().unwrap())
                .hset(OriginNameIdx::prefix(),
                      record.get_name().to_string(),
                      record.get_id())
                .ignore()
                .query(conn.deref())
        }));
        Ok(())
    }
}

pub struct OriginNameIdx {
    pool: Arc<ConnectionPool>,
}

impl OriginNameIdx {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        OriginNameIdx { pool: pool }
    }
}

impl Bucket for OriginNameIdx {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin:name:index"
    }
}

impl IndexSet for OriginNameIdx {
    type Value = u64;
}
