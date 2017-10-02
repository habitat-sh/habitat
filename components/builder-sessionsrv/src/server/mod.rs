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

mod handlers;

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::sync::RwLock;
use std::time::{Duration, Instant};

use github_api_client::GitHubClient;
use hab_net::app::prelude::*;
use protocol::sessionsrv as proto;

use config::{Config, PermissionsCfg};
use data_store::DataStore;
use error::{SrvError, SrvResult};

lazy_static! {
    static ref DISPATCH_TABLE: DispatchTable<SessionSrv> = {
        let mut map = DispatchTable::new();
        map.register(proto::AccountGet::descriptor_static(None), handlers::account_get);
        map.register(proto::AccountGetId::descriptor_static(None), handlers::account_get_id);
        map.register(proto::AccountCreate::descriptor_static(None), handlers::account_create);
        map.register(proto::AccountFindOrCreate::descriptor_static(None),
            handlers::account_find_or_create);
        map.register(proto::SessionCreate::descriptor_static(None), handlers::session_create);
        map.register(proto::SessionGet::descriptor_static(None), handlers::session_get);
        map.register(proto::AccountInvitationListRequest::descriptor_static(None),
            handlers::account_invitation_list);
        map.register(proto::AccountOriginInvitationCreate::descriptor_static(None),
            handlers::account_origin_invitation_create);
        map.register(proto::AccountOriginInvitationAcceptRequest::descriptor_static(None),
            handlers::account_origin_invitation_accept
        );
        map.register(proto::AccountOriginInvitationIgnoreRequest::descriptor_static(None),
            handlers::account_origin_invitation_ignore
        );
        map.register(proto::AccountOriginInvitationRescindRequest::descriptor_static(None),
            handlers::account_origin_invitation_rescind
        );
        map.register(proto::AccountOriginListRequest::descriptor_static(None),
            handlers::account_origin_list_request);
        map.register(proto::AccountOriginCreate::descriptor_static(None),
            handlers::account_origin_create);
        map
    };

    static ref SESSION_DURATION: Duration = {
        Duration::from_secs(1 * 24 * 60 * 60)
    };
}

#[derive(Clone)]
pub struct Session {
    pub created_at: Instant,
    inner: proto::Session,
}

impl Session {
    pub fn expired(&self) -> bool {
        self.created_at.elapsed() >= *SESSION_DURATION
    }
}

impl Borrow<str> for Session {
    fn borrow(&self) -> &str {
        self.inner.get_token()
    }
}

impl Default for Session {
    fn default() -> Self {
        Session {
            created_at: Instant::now(),
            inner: proto::Session::default(),
        }
    }
}

impl Deref for Session {
    type Target = proto::Session;

    fn deref(&self) -> &proto::Session {
        &self.inner
    }
}

impl DerefMut for Session {
    fn deref_mut(&mut self) -> &mut proto::Session {
        &mut self.inner
    }
}

impl Eq for Session {}

impl Hash for Session {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.inner.get_token().hash(state);
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Session) -> bool {
        self.inner.get_token() == other.inner.get_token()
    }
}

#[derive(Clone)]
pub struct ServerState {
    datastore: DataStore,
    github: Arc<Box<GitHubClient>>,
    permissions: Arc<PermissionsCfg>,
    sessions: Arc<Box<RwLock<HashSet<Session>>>>,
}

impl ServerState {
    fn new(cfg: &Config) -> SrvResult<Self> {
        Ok(ServerState {
            datastore: DataStore::new(cfg)?,
            github: Arc::new(Box::new(GitHubClient::new(cfg.github.clone()))),
            permissions: Arc::new(cfg.permissions.clone()),
            sessions: Arc::new(Box::new(RwLock::new(HashSet::default()))),
        })
    }
}

impl AppState for ServerState {
    type Config = Config;
    type Error = SrvError;
    type InitState = Self;

    fn build(_config: &Self::Config, init_state: Self::InitState) -> SrvResult<Self> {
        Ok(init_state)
    }
}

struct SessionSrv;
impl Dispatcher for SessionSrv {
    const APP_NAME: &'static str = "builder-sessionsrv";
    const PROTOCOL: Protocol = Protocol::SessionSrv;

    type Error = SrvError;
    type State = ServerState;

    fn app_init(
        config: &<Self::State as AppState>::Config,
        _: Arc<String>,
    ) -> SrvResult<<Self::State as AppState>::InitState> {
        let state = ServerState::new(&config)?;
        state.datastore.setup()?;
        Ok(state)
    }

    fn dispatch_table() -> &'static DispatchTable<Self> {
        &DISPATCH_TABLE
    }
}

pub fn run(config: Config) -> AppResult<(), SrvError> {
    app_start::<SessionSrv>(config)
}
