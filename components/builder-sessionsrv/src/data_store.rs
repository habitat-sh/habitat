// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::sync::Arc;

use dbcache::{self, Bucket, ConnectionPool, BasicSet, InstaSet};
use protocol::Persistable;
use protocol::sessionsrv;
use protobuf::Message;
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, PipelineCommands};

use error::Result;

pub struct DataStore {
    pub pool: Arc<ConnectionPool>,
    pub accounts: AccountTable,
    pub sessions: SessionTable,
}

impl DataStore {
    pub fn open<C: redis::IntoConnectionInfo>(config: C) -> Result<Self> {
        // JW TODO: tune pool from config?
        let pool_cfg = Default::default();
        let manager = RedisConnectionManager::new(config).unwrap();
        let pool = Arc::new(ConnectionPool::new(pool_cfg, manager).unwrap());
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let accounts = AccountTable::new(pool1);
        let sessions = SessionTable::new(pool2);
        Ok(DataStore {
            pool: pool,
            accounts: accounts,
            sessions: sessions,
        })
    }
}

pub struct AccountTable {
    pool: Arc<ConnectionPool>,
}

impl AccountTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        AccountTable { pool: pool }
    }
}

impl Bucket for AccountTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "account"
    }
}

impl InstaSet for AccountTable {
    type Record = sessionsrv::Account;

    fn seq_id() -> &'static str {
        "accounts_seq"
    }
}

pub struct SessionTable {
    pool: Arc<ConnectionPool>,
}

impl SessionTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        SessionTable { pool: pool }
    }
}

impl Bucket for SessionTable {
    fn prefix() -> &'static str {
        "session"
    }

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }
}

impl BasicSet for SessionTable {
    type Record = sessionsrv::SessionToken;

    fn write(&self, record: &Self::Record) -> dbcache::Result<()> {
        let conn = try!(self.pool().get());
        try!(conn.set_ex(Self::key(&record.primary_key()),
                         record.write_to_bytes().unwrap(),
                         86400));
        Ok(())
    }
}
