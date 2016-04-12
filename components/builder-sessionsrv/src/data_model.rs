// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use dbcache::data_store::{DataStore, DataRecord, InstaId};
use dbcache::model::{Fields, Model};
use protocol::sessionsrv;
use redis::{self, Commands, PipelineCommands};
use rustc_serialize::Encodable;

use super::oauth::github;
use error::{Error, Result};

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

    fn prefix() -> &'static str {
        "account"
    }

    fn seq_id() -> &'static str {
        "accounts_seq"
    }

    fn fields(&self) -> Fields {
        vec![("email", self.email.clone()), ("name", self.name.clone())]
    }

    fn id(&self) -> &InstaId {
        &self.id
    }

    fn save(&mut self, data_store: &DataStore) -> Result<()> {
        let conn = data_store.conn.as_ref().unwrap();
        try!(redis::transaction(conn, &[Self::seq_id()], |txn| {
            // JW TODO: allow 0 as an id
            let curr_seq: u64 = conn.get(Self::seq_id()).unwrap_or(0);
            let seq: u64 = curr_seq + 1;
            self.id = InstaId::generate(seq);
            txn.set(Self::seq_id(), seq)
               .ignore()
               .hset_multiple(Self::key(&self.id), &self.fields())
               .ignore()
               .query(conn)
        }));
        Ok(())
    }
}

impl From<github::User> for Account {
    fn from(user: github::User) -> Account {
        Account::new(user.login, user.email)
    }
}

impl From<DataRecord> for Account {
    fn from(record: DataRecord) -> Account {
        let (id, map) = record;
        Account {
            id: id,
            email: map["email"].to_string(),
            name: map["name"].to_string(),
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

    pub fn get(data_store: &DataStore, token: &str) -> Result<Self> {
        let key = format!("session:{}", token);
        match data_store.conn.as_ref().unwrap().get(key) {
            Ok(id) => Ok(Self::new(token.to_string(), InstaId(id))),
            Err(e) => {
                match e.kind() {
                    redis::ErrorKind::TypeError => Err(Error::EntityNotFound),
                    _ => Err(Error::from(e)),
                }
            }
        }
    }

    pub fn create(&self, data_store: &DataStore) -> Result<()> {
        try!(data_store.conn
                       .as_ref()
                       .unwrap()
                       .set_ex(format!("session:{}", self.token), self.owner_id, 86400));
        Ok(())
    }
}
