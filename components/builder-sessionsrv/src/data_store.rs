// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::str::FromStr;
use std::sync::Arc;

use dbcache::{self, ConnectionPool, DataRecord, ValueTable, InstaId, IndexTable, RecordTable,
              Table};
use dbcache::model::{Fields, Model};
use protocol::sessionsrv;
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, PipelineCommands};
use rustc_serialize::Encodable;

use super::oauth::github;
use error::{Error, Result};

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

impl Table for AccountTable {
    type IdType = InstaId;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "account"
    }
}

impl RecordTable for AccountTable {
    type Record = Account;

    fn seq_id() -> &'static str {
        "accounts_seq"
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Account {
    pub id: InstaId,
    pub email: String,
    pub name: String,
}

impl Account {
    pub fn new(name: String, email: String) -> Self {
        Account {
            id: InstaId::default(),
            email: email,
            name: name,
        }
    }
}

impl Model for Account {
    type Error = Error;
    type Table = AccountTable;

    fn fields(&self) -> Fields {
        vec![("email", self.email.clone()), ("name", self.name.clone())]
    }

    fn id(&self) -> &InstaId {
        &self.id
    }

    fn set_id(&mut self, id: InstaId) {
        self.id = id;
    }
}

impl From<github::User> for Account {
    fn from(user: github::User) -> Account {
        Account::new(user.login, user.email)
    }
}

impl From<DataRecord> for Account {
    fn from(record: DataRecord) -> Account {
        Account {
            id: InstaId::from_str(&record["id"]).unwrap(),
            email: record["email"].to_string(),
            name: record["name"].to_string(),
        }
    }
}

impl Into<sessionsrv::Session> for Account {
    fn into(self) -> sessionsrv::Session {
        let mut session = sessionsrv::Session::new();
        session.set_id(self.id.0);
        session.set_email(self.email);
        session.set_name(self.name);
        session
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

impl Table for SessionTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "session"
    }
}

impl ValueTable for SessionTable {
    type Value = InstaId;

    fn write(&self, id: &<Self as Table>::IdType, value: Self::Value) -> dbcache::Result<()> {
        try!(self.pool().get().unwrap().set_ex(Self::key(id), value, 86400));
        Ok(())
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Session {
    pub token: String,
    pub owner_id: InstaId,
}

impl Session {
    pub fn new(token: String, owner_id: InstaId) -> Self {
        Session {
            token: token,
            owner_id: owner_id,
        }
    }

    pub fn get(index: &SessionTable, token: &str) -> Result<Self> {
        let token = token.to_string();
        match index.find(&token) {
            Ok(id) => Ok(Self::new(token, id)),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn create(&self, index: &SessionTable) -> Result<()> {
        try!(index.write(&self.token, self.owner_id));
        Ok(())
    }
}
