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

//! Traits and Types for persisting protocol messages to a database.
//!
//! Data is grouped within a `Bucket` which prefixes all keys with the bucket name. Three types of
//! data "Sets" are defined in this module to assist with storage of entities. See `BasicSet`,
//! `InstaSet` and `IndexSet` below.

use std::fmt;
use std::net;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

use protobuf::{Message, parse_from_bytes};
use protocol::{InstaId, Persistable};
use r2d2;
use r2d2_redis::{self, RedisConnectionManager};
use redis::{self, Commands, IntoConnectionInfo, PipelineCommands};

use config::DataStoreCfg;
use error::{Error, Result};

pub type ConnectionPool = r2d2::Pool<r2d2_redis::RedisConnectionManager>;

pub trait Pool: Sized {
    type Config: DataStoreCfg;

    fn start(config: &Self::Config) -> Self {
        let retry_ms = config.connection_retry_ms();
        let pool_size = config.pool_size();
        let cfg = redis_connection_info(config.datastore_addr());
        loop {
            {
                let pool_cfg = r2d2::Config::builder().pool_size(pool_size).build();
                let manager = RedisConnectionManager::new(cfg.clone()).unwrap();
                debug!("establishing connection(s) to database...");
                match ConnectionPool::new(pool_cfg, manager) {
                    Ok(pool) => return Self::init(Arc::new(pool)),
                    Err(e) => error!("error initializing datastore connection pool, {}", e),
                }
            }
            thread::sleep(Duration::from_millis(retry_ms));
            info!("retrying database connections...");
        }
    }

    fn init(pool: Arc<ConnectionPool>) -> Self;
}

/// Base trait for storing peristable objects into a data store. A bucket prefixes the key of all
/// entities with a developer-defined prefix.
pub trait Bucket {
    /// Generates a key for the given ID within the Bucket.
    fn key<K: fmt::Display>(id: K) -> String {
        format!("{}:{}", Self::prefix(), id).to_lowercase()
    }

    /// A string that every entity is prefixed with.
    ///
    /// This will result in keys being "namespaced" in a key value datastore. Given the prefix
    /// of `account` a newly created key for an account with the ID of `35563460129464321` will
    /// become `account:35563460129464321`.
    fn prefix() -> &'static str;

    /// Reference to the `Bucket`'s connection pool
    fn pool(&self) -> &ConnectionPool;
}

/// A generic data set for reading and writing entities into the datastore.
///
/// ID creation is left up to the developer. If you need an auto-incremented primary key consider
/// using an `InstaSet`.
pub trait BasicSet: Bucket {
    /// Type of objects stored inside this data set.
    type Record: Persistable;

    /// Delete a record from the data set with the given ID.
    fn delete(&self, id: &<Self::Record as Persistable>::Key) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.del(Self::key(id)));
        Ok(())
    }

    /// Retrieves a record from the data set with the given ID.
    fn find(&self, id: &<Self::Record as Persistable>::Key) -> Result<Self::Record> {
        let conn = try!(self.pool().get());
        let bytes = try!(conn.get::<String, Vec<u8>>(Self::key(id)));
        if bytes.is_empty() {
            return Err(Error::EntityNotFound);
        }
        let value = parse_from_bytes(&bytes).unwrap();
        Ok(value)
    }

    /// Update an existing record in the data set.
    fn update(&self, record: &Self::Record) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.set(Self::key(&record.primary_key()),
                      record.write_to_bytes().unwrap()));
        Ok(())
    }

    /// Write a new record to the data set.
    fn write(&self, record: &Self::Record) -> Result<bool> {
        let conn = try!(self.pool().get());
        match try!(conn.set_nx(Self::key(&record.primary_key()),
                               record.write_to_bytes().unwrap())) {
            1 => Ok(true),
            0 => Ok(false),
            code => unreachable!("received unexpected return code from redis-setnx: {}", code),
        }
    }
}

/// A generic data set for reading and writing entities into the datastore with a time to live.
///
/// This is identical to `BasicSet` with the exception that entities expire.
pub trait ExpiringSet: Bucket {
    /// Type of objects stored inside this data set.
    type Record: Persistable;

    /// Expiration time (in seconds) for any entities written to the set.
    fn expiry() -> usize;

    /// Delete a record from the data set with the given ID.
    fn delete(&self, id: &<Self::Record as Persistable>::Key) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.del(Self::key(id)));
        Ok(())
    }

    /// Retrieves a record from the data set with the given ID.
    fn find(&self, id: &<Self::Record as Persistable>::Key) -> Result<Self::Record> {
        let conn = try!(self.pool().get());
        let bytes = try!(conn.get::<String, Vec<u8>>(Self::key(id)));
        if bytes.is_empty() {
            return Err(Error::EntityNotFound);
        }
        let value = parse_from_bytes(&bytes).unwrap();
        Ok(value)
    }

    /// Write a new record to the data set with a TTL.
    fn write(&self, record: &Self::Record) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.set_ex(Self::key(&record.primary_key()),
                         record.write_to_bytes().unwrap(),
                         Self::expiry()));
        Ok(())
    }
}

/// A specialized data set for reading and writing entities with a unique and sequential
/// identifier.
///
/// InstaId contain their creation time and their auto-assigned sequence id. This allows entities
/// belonging to this data set to be sorted by both their creation time and their sequential
/// identifier. Entities generated by this data set are also sharded, inheriting that behaviour
/// from the InstaId generation process.
pub trait InstaSet: Bucket {
    /// Type of objects stored inside this data set.
    ///
    /// `InstaSet`'s always use a `u64` as the record's primary key. If your type implements the
    /// `Persistable` trait but does not specify `u64` for the Key, you will not be able to store
    /// that type in an `InstaSet`.
    type Record: Persistable<Key = u64>;

    /// A unique keyname for an auto-incrementing sequence used in ID generation
    fn seq_id() -> &'static str;

    /// Delete a record from the data set with the given ID.
    fn delete(&self, id: &<Self::Record as Persistable>::Key) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.del(Self::key(id)));
        Ok(())
    }

    /// Retrieves a record from the data set with the given ID.
    fn find(&self, id: &<Self::Record as Persistable>::Key) -> Result<Self::Record> {
        let conn = try!(self.pool().get());
        let bytes = try!(conn.get::<String, Vec<u8>>(Self::key(id)));
        if bytes.is_empty() {
            return Err(Error::EntityNotFound);
        }
        let value = parse_from_bytes(&bytes).unwrap();
        Ok(value)
    }

    /// Update an existing record in the data set.
    fn update(&self, record: &Self::Record) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.set(Self::key(&record.primary_key()),
                      record.write_to_bytes().unwrap()));
        Ok(())
    }

    /// Write a new record to the data set.
    ///
    /// An ID will be automatically created and assigned as the primary key of given record.
    fn write(&self, record: &mut Self::Record) -> Result<bool> {
        let conn = try!(self.pool().get());
        match try!(redis::transaction(conn.deref(), &[Self::seq_id()], |txn| {
            let sequence_id: u64 = match conn.get::<&'static str, u64>(Self::seq_id()) {
                Ok(value) => value + 1,
                _ => 0,
            };
            let insta_id = InstaId::generate(sequence_id);
            record.set_primary_key(*insta_id);
            txn.set(Self::seq_id(), record.primary_key())
                .ignore()
                .set_nx(Self::key(&record.primary_key()),
                        record.write_to_bytes().unwrap())
                .query(conn.deref())
        })) {
            1 => Ok(true),
            0 => Ok(false),
            code => {
                unreachable!("received unexpected return code from redis-hsetnx: {}",
                             code)
            }
        }
    }
}

/// A data set for writing basic key/value indices.
pub trait IndexSet: Bucket {
    /// Type of the lookup key
    type Key: Clone + redis::FromRedisValue + redis::ToRedisArgs;
    /// Type of the Value stored for each entry in the index.
    type Value: redis::FromRedisValue + redis::ToRedisArgs;

    /// Retrieves the value for the given ID.
    fn find(&self, id: &Self::Key) -> Result<Self::Value> {
        let conn = try!(self.pool().get());
        let value = try!(conn.hget(Self::prefix(), id.clone()));
        Ok(value)
    }

    fn update(&self, id: &Self::Key, value: Self::Value) -> Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.hset(Self::prefix(), id.clone(), value));
        Ok(())
    }

    /// Write a new index entry to the data set.
    fn write(&self, id: &Self::Key, value: Self::Value) -> Result<bool> {
        let conn = try!(self.pool().get());
        match try!(conn.hset_nx(Self::prefix(), id.clone(), value)) {
            1 => Ok(true),
            0 => Ok(false),
            code => {
                unreachable!("received unexpected return code from redis-hsetnx: {}",
                             code)
            }
        }
    }
}

fn redis_connection_info(addr: &net::SocketAddrV4) -> redis::ConnectionInfo {
    format!("redis://{}:{}", addr.ip(), addr.port())
        .into_connection_info()
        .unwrap()
}
