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

use std::ops::Deref;
use std::sync::Arc;

use dbcache::{self, data_store, ConnectionPool, BasicSet, Bucket, IndexSet, InstaSet};
use protobuf::Message;
use protocol::{vault, InstaId, Persistable};
use redis::{self, Commands, PipelineCommands};

use config::Config;
use protocol::vault as proto;

pub struct DataStore {
    pub pool: Arc<ConnectionPool>,
    pub origins: OriginTable,
    pub projects: ProjectTable,
}

impl data_store::Pool for DataStore {
    type Config = Config;

    fn init(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let origins = OriginTable::new(pool1);
        let projects = ProjectTable::new(pool2);

        DataStore {
            pool: pool,
            origins: origins,
            projects: projects,
        }
    }
}

pub struct OriginTable {
    pool: Arc<ConnectionPool>,

    pub origin_secret_keys: OriginSecretKeysTable,
    pub invites: OriginInvitesTable,
    pub name_idx: OriginNameIdx,
}

impl OriginTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let pool3 = pool.clone();

        let origin_secret_keys = OriginSecretKeysTable::new(pool1);
        let invites = OriginInvitesTable::new(pool2);
        let name_idx = OriginNameIdx::new(pool3);

        OriginTable {
            pool: pool,
            origin_secret_keys: origin_secret_keys,
            invites: invites,
            name_idx: name_idx,
        }
    }

    /// modify_invite is located in the OriginTable so it can interact with
    /// origin members AND origin invites
    pub fn modify_invite(&self,
                         invite: &proto::OriginInvitation,
                         ignore: bool)
                         -> dbcache::Result<()> {
        debug!("Accepting invitation ({})", ignore);

        // account_origins stores account_id -> origin *name*
        // origin_members stores origin_id -> account *name*
        //  This is cheating a bit, but the names are stored
        //  on the SessionSrv in the Account obj so this
        //  will do for now.
        let account_origins_key = format!("account_origins:{}", &invite.get_account_id());
        let origin_members_key = format!("origin_members:{}", &invite.get_origin_id());
        debug!("account_origins_key = {}", &account_origins_key);
        debug!("origin_members_key = {}", &origin_members_key);

        let conn = try!(self.pool().get());

        if !ignore {
            // accept the invite: add the account to the origin and delete the
            // invite
            try!(redis::transaction(conn.deref(),
                                    &[account_origins_key.clone(), origin_members_key.clone()],
                                    |txn| {
                txn.sadd(account_origins_key.clone(), invite.get_origin_name())
                    .sadd(origin_members_key.clone(), invite.get_account_name())
                    .del(OriginInvitesTable::key(invite.get_id()))
                    .query(conn.deref())
            }));
        } else {
            // "ignore" the invite, meaning: just delete it
            try!(conn.del(invite.get_id()));
        }

        Ok(())
    }

    pub fn account_origins_key(&self, account_id: &u64) -> String {
        format!("account_origins:{}", account_id)
    }

    pub fn origin_members_key(&self, origin_id: &u64) -> String {
        format!("origin_members:{}", origin_id)
    }

    /// this is used to add the owner of the account to the full list of members
    /// right after an origin is created
    pub fn add_origin_member(&self,
                             account_id: u64,
                             account_name: &str,
                             origin_name: &str)
                             -> dbcache::Result<()> {

        let conn = try!(self.pool().get());

        let origin_id = try!(self.name_idx.find(&origin_name.to_string()));
        let account_origins_key = self.account_origins_key(&account_id);
        let origin_members_key = self.origin_members_key(&origin_id);
        try!(redis::transaction(conn.deref(),
                                &[account_origins_key.clone(), origin_members_key.clone()],
                                |txn| {
                                    txn.sadd(account_origins_key.clone(), origin_name)
                                        .sadd(origin_members_key.clone(), account_name)
                                        .query(conn.deref())
                                }));
        Ok(())
    }

    pub fn list_origin_members(&self, origin_id: u64) -> dbcache::Result<Vec<String>> {
        let origin_members_key = self.origin_members_key(&origin_id);
        let conn = try!(self.pool().get());
        let members = try!(conn.smembers::<String, Vec<String>>(origin_members_key));

        Ok(members)
    }

    pub fn list_account_origins(&self, account_id: u64) -> dbcache::Result<Vec<String>> {
        let account_origins_key = self.account_origins_key(&account_id);
        let conn = try!(self.pool().get());
        let origins = try!(conn.smembers::<String, Vec<String>>(account_origins_key));
        Ok(origins)
    }

    pub fn is_origin_member(&self, account_id: u64, origin_name: &str) -> dbcache::Result<bool> {
        let account_origins_key = self.account_origins_key(&account_id);
        let conn = try!(self.pool().get());
        let result = try!(conn.sismember::<String, String, bool>(account_origins_key,
            origin_name.to_string()));
        Ok(result)
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
    type Key = String;
    type Value = u64;
}

pub struct OriginSecretKeysTable {
    pool: Arc<ConnectionPool>,
}

impl OriginSecretKeysTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        OriginSecretKeysTable { pool: pool }
    }
}

impl Bucket for OriginSecretKeysTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin_secret_key"
    }
}

impl InstaSet for OriginSecretKeysTable {
    type Record = vault::OriginSecretKey;

    fn seq_id() -> &'static str {
        "origin_secret_key_seq"
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
                .query(conn.deref())

        }));
        Ok(())
    }
}

pub struct OriginInvitesTable {
    pool: Arc<ConnectionPool>,
}

impl OriginInvitesTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        OriginInvitesTable { pool: pool }
    }

    /// return a Vec of invite_id's for a given account
    pub fn get_by_account_id(&self,
                             account_id: u64)
                             -> dbcache::Result<Vec<proto::OriginInvitation>> {
        let conn = self.pool().get().unwrap();
        let account_to_invites_key = format!("account_to_invites:{}", &account_id);
        match conn.smembers::<String, Vec<u64>>(account_to_invites_key) {
            Ok(invite_ids) => {
                let account_invites = invite_ids.iter().fold(Vec::new(),
                                                             |mut acc, ref invite_id| {
                    match self.find(invite_id) {
                        Ok(invite) => acc.push(invite),
                        Err(e) => {
                            debug!("Can't find origin invite for invite_id {}:{}",
                                   &invite_id,
                                   e);
                        }
                    };
                    acc
                });
                Ok(account_invites)
            }
            Err(e) => Err(dbcache::Error::from(e)),
        }
    }

    /// return a Vec of invite_id's for a given origin
    pub fn get_by_origin_id(&self,
                            origin_id: u64)
                            -> dbcache::Result<Vec<proto::OriginInvitation>> {
        let conn = self.pool().get().unwrap();
        let origin_to_invites_key = format!("origin_to_invites:{}", &origin_id);
        match conn.smembers::<String, Vec<u64>>(origin_to_invites_key) {
            Ok(invite_ids) => {
                let origin_invites = invite_ids.iter().fold(Vec::new(), |mut acc, ref invite_id| {
                    match self.find(invite_id) {
                        Ok(invite) => acc.push(invite),
                        Err(e) => {
                            debug!("Can't find origin invite for invite_id {}:{}",
                                   &invite_id,
                                   e);
                        }
                    };
                    acc
                });
                Ok(origin_invites)
            }
            Err(e) => Err(dbcache::Error::from(e)),
        }
    }
}

impl Bucket for OriginInvitesTable {
    fn prefix() -> &'static str {
        "origin_invite"
    }

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }
}

impl InstaSet for OriginInvitesTable {
    type Record = vault::OriginInvitation;

    fn seq_id() -> &'static str {
        "origin_invites_key_seq"
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
            let account_to_invites_key = format!("account_to_invites:{}", record.get_account_id());
            let origin_to_invites_key = format!("origin_to_invites:{}", record.get_origin_id());
            debug!("origin invite = {:?}", &record);
            txn.set(Self::seq_id(), record.primary_key())
                .ignore()
                .set(Self::key(&record.primary_key()),
                     record.write_to_bytes().unwrap())
                .ignore()
                .sadd(account_to_invites_key, record.primary_key())
                .ignore()
                .sadd(origin_to_invites_key, record.primary_key())
                .ignore()
                .query(conn.deref())
        }));

        Ok(())
    }
}

pub struct ProjectTable {
    pool: Arc<ConnectionPool>,
}

impl ProjectTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        ProjectTable { pool: pool }
    }
}

impl Bucket for ProjectTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "project"
    }
}

impl BasicSet for ProjectTable {
    type Record = vault::Project;
}
