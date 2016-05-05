// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;

use protocol::InstaId;
use r2d2;
use r2d2_redis;
use redis::{self, Commands, PipelineCommands};

use model::Model;
use error::{Error, Result};

pub type ConnectionPool = r2d2::Pool<r2d2_redis::RedisConnectionManager>;

pub type DataRecord = HashMap<String, String>;

pub trait Table {
    type IdType: fmt::Display;

    fn pool(&self) -> &ConnectionPool;

    fn key(id: &Self::IdType) -> String {
        format!("{}:{}", Self::prefix(), id).to_lowercase()
    }

    fn prefix() -> &'static str;
}

pub trait RecordTable: Table<IdType = InstaId> {
    type Record: Model;

    fn seq_id() -> &'static str;

    fn find<I: Into<InstaId>>(&self, id: I) -> Result<Self::Record> {
        let id: InstaId = id.into();
        let key = Self::key(&id);
        match try!(self.pool().get()).hgetall::<String, HashMap<String, String>>(key) {
            Ok(mut map) => {
                map.insert("id".to_string(), id.to_string());
                Ok(Self::Record::from(map))
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    fn write(&self, record: &mut Self::Record) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(redis::transaction(conn.deref(), &[Self::seq_id()], |txn| {
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
               .query(conn.deref())
        }));
        Ok(())
    }

    fn update(&self, record: &Self::Record) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.hset_multiple(Self::key(record.id()), &record.fields()));
        Ok(())
    }
}

pub trait ValueTable: Table {
    type Value: redis::ToRedisArgs + redis::FromRedisValue;

    fn find(&self, id: &<Self as Table>::IdType) -> Result<Self::Value> {
        let conn = try!(self.pool().get());
        let value = try!(conn.get(Self::key(id)));
        Ok(value)
    }

    fn write(&self, id: &<Self as Table>::IdType, value: Self::Value) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.set(Self::key(id), value));
        Ok(())
    }
}

pub trait IndexTable: Table<IdType = String> {
    type Value: redis::FromRedisValue + redis::ToRedisArgs;

    fn find(&self, id: &str) -> Result<Self::Value> {
        let conn = try!(self.pool().get());
        let value = try!(conn.hget(Self::prefix(), id));
        Ok(value)
    }

    fn write(&self, id: &str, value: Self::Value) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.hset(Self::prefix(), id.clone(), value));
        Ok(())
    }
}
