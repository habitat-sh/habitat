// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use r2d2;
use r2d2_redis;
use redis::{self, Commands, PipelineCommands};
use time;

use model::Model;
use error::{Error, Result};

/// Time from which we begin issuing identifiers. This number can be used to determine how old
/// an entity is by subtracting it's timestamp from this value.
pub const EPOCH_MS: u64 = 1460499133628;
/// Maximum shard count. This number cannot exceed the value of `MAX_SHARD_ID`. If this number
/// is altered then the entities in the data store must all be issued new identifiers.
pub const SHARD_COUNT: u64 = 128;
/// Maximum value allowed for a Shard ID (2^13).
pub const MAX_SHARD_ID: u16 = 8192;

const ID_MASK: u64 = 0x3FF;
const SHARD_MASK: u64 = 0x1FFF;

pub type ConnectionPool = r2d2::Pool<r2d2_redis::RedisConnectionManager>;

#[derive(Clone, Copy, Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct InstaId(pub u64);

impl InstaId {
    pub fn generate(auto_id: u64) -> Self {
        let time = Self::since_epoch();
        let id = auto_id % 1024;
        let shard_id = id % SHARD_COUNT;
        let mut iid: u64 = time << 23;
        iid |= id << 13;
        iid |= shard_id;
        InstaId(iid)
    }

    pub fn since_epoch() -> u64 {
        let timespec = time::get_time();
        let sec: u64 = timespec.sec as u64 * 1000;
        let nsec: u64 = timespec.nsec as u64 / 1000 / 1000;
        (sec + nsec) - EPOCH_MS
    }

    pub fn inner_id(&self) -> u64 {
        (self.0 >> 13) & ID_MASK
    }

    pub fn shard(&self) -> u64 {
        self.0 & SHARD_MASK
    }

    pub fn timestamp(&self) -> u64 {
        self.0 >> 23
    }
}

impl Default for InstaId {
    fn default() -> Self {
        InstaId(0)
    }
}

impl FromStr for InstaId {
    type Err = Error;

    fn from_str(id: &str) -> Result<InstaId> {
        // JW TODO: handle parse int error;
        let id = u64::from_str(id).unwrap();
        Ok(InstaId(id))
    }
}

impl From<u64> for InstaId {
    fn from(id: u64) -> InstaId {
        InstaId(id)
    }
}

impl fmt::Display for InstaId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialOrd for InstaId {
    fn partial_cmp(&self, other: &InstaId) -> Option<Ordering> {
        match self.timestamp().cmp(&other.timestamp()) {
            Ordering::Equal => {
                match self.inner_id().cmp(&other.inner_id()) {
                    Ordering::Equal => None,
                    ordering => Some(ordering),
                }
            }
            ordering => Some(ordering),
        }
    }
}

impl redis::FromRedisValue for InstaId {
    fn from_redis_value(value: &redis::Value) -> redis::RedisResult<InstaId> {
        let id = try!(redis::from_redis_value::<u64>(value));
        Ok(InstaId(id))
    }
}

impl redis::ToRedisArgs for InstaId {
    fn to_redis_args(&self) -> Vec<Vec<u8>> {
        self.0.to_redis_args()
    }
}

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

    fn find(&self, id: &InstaId) -> Result<Self::Record> {
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

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn extract_shard_id_from_insta_id() {
        let auto_id = 1984198419841984;
        let insta_id = InstaId::generate(auto_id);
        assert_eq!(insta_id.shard(), 64);
    }

    #[test]
    fn extract_timestamp_from_insta_id() {
        let auto_id = 1984198419841984;
        let gen_time = InstaId::since_epoch();
        let insta_id = InstaId::generate(auto_id);
        assert!(insta_id.timestamp() >= gen_time);
    }

    #[test]
    fn ordering_of_insta_id() {
        // validate when generated within the same millisecond
        let id1 = InstaId::generate(1);
        let id2 = InstaId::generate(2);
        let id3 = InstaId::generate(3);
        assert!(id1 < id2);
        assert!(id1 < id3);
        assert!(id2 > id1);
        assert!(id2 < id3);
        assert!(id3 > id1);
        assert!(id3 > id2);
        // validated when generated with same id at different time
        thread::sleep(Duration::from_millis(1));
        let id1 = InstaId::generate(1);
        thread::sleep(Duration::from_millis(1));
        let id2 = InstaId::generate(1);
        thread::sleep(Duration::from_millis(1));
        let id3 = InstaId::generate(1);
        assert!(id1 < id2);
        assert!(id1 < id3);
        assert!(id2 > id1);
        assert!(id2 < id3);
        assert!(id3 > id1);
        assert!(id3 > id2);
    }

    #[test]
    fn extract_inner_id_from_insta_id() {
        let inner_id = 298674917348924;
        let insta_id = InstaId::generate(inner_id);
        assert_eq!(insta_id.inner_id(), inner_id % 1024);
    }
}
