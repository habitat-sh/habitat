// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use dbcache::{self, ConnectionPool, DataRecord, IndexTable, RecordTable, Table};
use dbcache::model::{Fields, Model};
use protocol::{self, InstaId};
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, PipelineCommands};
use rustc_serialize::Encodable;

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

impl Table for OriginTable {
    type IdType = InstaId;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin"
    }
}

impl RecordTable for OriginTable {
    type Record = Origin;

    fn seq_id() -> &'static str {
        "origins_seq"
    }

    fn write(&self, record: &mut Self::Record) -> dbcache::Result<()> {
        let conn = self.pool().get().unwrap();
        let keys = [Self::seq_id(), OriginNameIdx::prefix()];
        try!(redis::transaction(conn.deref(), &keys, |txn| {
            let sequence_id: u64 = match conn.get::<&'static str, u64>(Self::seq_id()) {
                Ok(value) => value + 1,
                _ => 0,
            };
            let insta_id = InstaId::generate(sequence_id);
            record.set_id(insta_id);
            txn.set(Self::seq_id(), record.id().0)
                .ignore()
                .hset_multiple(Self::key(record.id()), &record.fields())
                .ignore()
                .hset(OriginNameIdx::prefix(), record.name.clone(), record.id)
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

impl Table for OriginNameIdx {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin:name:index"
    }
}

impl IndexTable for OriginNameIdx {
    type Value = InstaId;
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Origin {
    pub id: InstaId,
    pub name: String,
    pub owner_id: InstaId,
}

impl Origin {
    pub fn new(name: String, owner_id: InstaId) -> Self {
        Origin {
            id: InstaId::default(),
            name: name,
            owner_id: owner_id,
        }
    }
}

impl Model for Origin {
    type Table = OriginTable;

    fn fields(&self) -> Fields {
        vec![("owner_id", self.owner_id.to_string()), ("name", self.name.clone())]
    }

    fn id(&self) -> &InstaId {
        &self.id
    }

    fn set_id(&mut self, id: InstaId) {
        self.id = id;
    }
}

impl Into<protocol::vault::Origin> for Origin {
    fn into(self) -> protocol::vault::Origin {
        let mut msg = protocol::vault::Origin::new();
        msg.set_id(self.id.0);
        msg.set_name(self.name);
        msg.set_owner_id(self.owner_id.0);
        msg
    }
}

impl From<protocol::vault::OriginCreate> for Origin {
    fn from(msg: protocol::vault::OriginCreate) -> Origin {
        Origin::new(msg.get_name().to_string(), msg.get_owner_id().into())
    }
}

impl From<DataRecord> for Origin {
    fn from(record: DataRecord) -> Origin {
        let id = u64::from_str(&record["id"]).unwrap();
        let owner_id = u64::from_str(&record["owner_id"]).unwrap();
        Origin {
            id: InstaId(id),
            name: record["name"].to_string(),
            owner_id: InstaId(owner_id),
        }
    }
}
