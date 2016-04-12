// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::str::FromStr;

use dbcache::data_store::{DataStore, DataRecord, InstaId};
use dbcache::model::{Fields, Model};
use protocol;
use redis::{self, Commands, PipelineCommands};
use rustc_serialize::Encodable;

use error::{Error, Result};

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

    pub fn find_by_name(ds: &DataStore, name: &str) -> Result<Self> {
        let conn = ds.conn.as_ref().unwrap();
        let val: u64 = try!(conn.hget("origin:name:index", name));
        // JW TODO: refactor / handle error
        let record = ds.find(&InstaId(val)).unwrap();
        Ok(record)
    }
}

impl Model for Origin {
    type Error = Error;

    fn prefix() -> &'static str {
        "origin"
    }

    fn seq_id() -> &'static str {
        "origins_seq"
    }

    fn fields(&self) -> Fields {
        vec![("owner_id", self.owner_id.to_string()), ("name", self.name.clone())]
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
               .hset("origin:name:index", self.name.clone(), self.id)
               .ignore()
               .query(conn)
        }));
        Ok(())
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
        let (id, map) = record;
        Origin {
            id: id,
            name: map["name"].to_string(),
            owner_id: InstaId::from_str(&map["owner_id"]).unwrap(),
        }
    }
}
