// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::sync::Arc;

use dbcache::{self, Bucket, ConnectionPool, ExpiringSet, IndexSet, InstaSet};
use protocol::sessionsrv;
use r2d2_redis::RedisConnectionManager;
use redis;

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
    github: GitHub2AccountIdx,
    user_to_account: GitHubUser2AccountIdx,
}

impl AccountTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let directory = GitHub2AccountIdx::new(pool1);
        let user_to_account = GitHubUser2AccountIdx::new(pool2);
        AccountTable {
            pool: pool,
            github: directory,
            user_to_account: user_to_account,
        }
    }

    pub fn find_or_create(&self, req: &sessionsrv::SessionCreate) -> Result<sessionsrv::Account> {
        let id = match req.get_provider() {
            sessionsrv::OAuthProvider::GitHub => self.github.find(&req.get_extern_id()).ok(),
        };
        if let Some(ref id) = id {
            let account = try!(self.find(id));
            Ok(account)
        } else {
            let mut account = sessionsrv::Account::new();
            account.set_email(req.get_email().to_string());
            account.set_name(req.get_name().to_string());
            // JW TODO: make these two database calls transactional
            try!(self.write(&mut account));
            try!(self.github.write(&req.get_extern_id(), account.get_id()));
            // TODO: route a message to the appropriate sessionsrv, and
            // that sessionsrv will write to the db
            try!(self.user_to_account.write(&req.get_name().to_string(), account.get_id()));
            Ok(account)
        }
    }

    pub fn find_by_username(&self, username: &str) -> dbcache::Result<sessionsrv::Account> {
        let account_id = try!(self.user_to_account.find(&username.to_string()));
        self.find(&account_id)
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

impl ExpiringSet for SessionTable {
    type Record = sessionsrv::SessionToken;

    fn expiry() -> usize {
        86400
    }
}

struct GitHub2AccountIdx {
    pool: Arc<ConnectionPool>,
}

impl GitHub2AccountIdx {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        GitHub2AccountIdx { pool: pool }
    }
}

impl Bucket for GitHub2AccountIdx {
    fn prefix() -> &'static str {
        "github2account"
    }

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }
}

impl IndexSet for GitHub2AccountIdx {
    type Key = u64;
    type Value = u64;
}


/// maps github usernames -> Account.id's
struct GitHubUser2AccountIdx {
    pool: Arc<ConnectionPool>,
}

impl GitHubUser2AccountIdx {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        GitHubUser2AccountIdx { pool: pool }
    }
}

impl Bucket for GitHubUser2AccountIdx {
    fn prefix() -> &'static str {
        "githubuser2account"
    }

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }
}

impl IndexSet for GitHubUser2AccountIdx {
    type Key = String;
    type Value = u64;
}


