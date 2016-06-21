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

use std::sync::Arc;

use dbcache::{self, data_store, Bucket, ConnectionPool, ExpiringSet, IndexSet, InstaSet};
use protocol::sessionsrv;

use config::Config;
use error::Result;

pub struct DataStore {
    pub pool: Arc<ConnectionPool>,
    pub accounts: AccountTable,
    pub sessions: SessionTable,
}

impl data_store::Pool for DataStore {
    type Config = Config;

    fn init(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let accounts = AccountTable::new(pool1);
        let sessions = SessionTable::new(pool2);
        DataStore {
            pool: pool,
            accounts: accounts,
            sessions: sessions,
        }
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
