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

use hab_net::app::prelude::*;
use protocol::originsrv::*;

use config::Config;
use data_store::DataStore;
use error::{SrvError, SrvResult};

lazy_static! {
    static ref DISPATCH_TABLE: DispatchTable<OriginSrv> = {
        let mut map = DispatchTable::new();
        map.register(CheckOriginAccessRequest::descriptor_static(None),
            handlers::origin_check_access);
        map.register(OriginCreate::descriptor_static(None), handlers::origin_create);
        map.register(OriginUpdate::descriptor_static(None), handlers::origin_update);
        map.register(OriginGet::descriptor_static(None), handlers::origin_get);
        map.register(OriginIntegrationGetNames::descriptor_static(None),
            handlers::origin_integration_get_names);
        map.register(OriginIntegrationCreate::descriptor_static(None),
            handlers::origin_integration_create);
        map.register(OriginIntegrationDelete::descriptor_static(None),
            handlers::origin_integration_delete);
        map.register(OriginIntegrationRequest::descriptor_static(None),
            handlers::origin_integration_request);
        map.register(OriginInvitationAcceptRequest::descriptor_static(None),
            handlers::origin_invitation_accept);
        map.register(OriginInvitationCreate::descriptor_static(None),
            handlers::origin_invitation_create);
        map.register(OriginInvitationIgnoreRequest::descriptor_static(None),
            handlers::origin_invitation_ignore);
        map.register(OriginInvitationListRequest::descriptor_static(None),
            handlers::origin_invitation_list);
        map.register(OriginInvitationRescindRequest::descriptor_static(None),
            handlers::origin_invitation_rescind);
        map.register(OriginMemberListRequest::descriptor_static(None),
            handlers::origin_member_list);
        map.register(OriginSecretKeyCreate::descriptor_static(None),
            handlers::origin_secret_key_create);
        map.register(OriginSecretKeyGet::descriptor_static(None),
            handlers::origin_secret_key_get);
        map.register(OriginPublicKeyCreate::descriptor_static(None),
            handlers::origin_public_key_create);
        map.register(OriginPublicKeyGet::descriptor_static(None),
            handlers::origin_public_key_get);
        map.register(OriginPublicKeyLatestGet::descriptor_static(None),
            handlers::origin_public_key_latest_get);
        map.register(OriginPublicKeyListRequest::descriptor_static(None),
            handlers::origin_public_key_list);
        map.register(OriginProjectCreate::descriptor_static(None), handlers::project_create);
        map.register(OriginProjectDelete::descriptor_static(None), handlers::project_delete);
        map.register(OriginProjectGet::descriptor_static(None), handlers::project_get);
        map.register(OriginProjectUpdate::descriptor_static(None), handlers::project_update);
        map.register(OriginProjectIntegrationCreate::descriptor_static(None),
            handlers::project_integration_create);
        map.register(OriginProjectIntegrationGet::descriptor_static(None),
            handlers::project_integration_get);
        map.register(OriginProjectIntegrationRequest::descriptor_static(None),
            handlers::origin_project_integration_request);
        map.register(OriginPackageCreate::descriptor_static(None), handlers::origin_package_create);
        map.register(OriginPackageGet::descriptor_static(None), handlers::origin_package_get);
        map.register(OriginPackageLatestGet::descriptor_static(None),
            handlers::origin_package_latest_get);
        map.register(OriginPackageListRequest::descriptor_static(None),
            handlers::origin_package_list);
        map.register(OriginPackagePlatformListRequest::descriptor_static(None),
            handlers::origin_package_platform_list);
        map.register(OriginPackageChannelListRequest::descriptor_static(None),
            handlers::origin_package_channel_list);
        map.register(OriginPackageVersionListRequest::descriptor_static(None),
            handlers::origin_package_version_list);
        map.register(OriginPackageDemote::descriptor_static(None), handlers::origin_package_demote);
        map.register(OriginPackageGroupPromote::descriptor_static(None),
            handlers::origin_package_group_promote);
        map.register(OriginPackagePromote::descriptor_static(None),
            handlers::origin_package_promote);
        map.register(OriginPackageUniqueListRequest::descriptor_static(None),
            handlers::origin_package_unique_list);
        map.register(OriginPackageSearchRequest::descriptor_static(None),
            handlers::origin_package_search);
        map.register(OriginChannelCreate::descriptor_static(None), handlers::origin_channel_create);
        map.register(OriginChannelDelete::descriptor_static(None), handlers::origin_channel_delete);
        map.register(OriginChannelGet::descriptor_static(None), handlers::origin_channel_get);
        map.register(OriginChannelListRequest::descriptor_static(None),
            handlers::origin_channel_list);
        map.register(OriginChannelPackageGet::descriptor_static(None),
            handlers::origin_channel_package_get);
        map.register(OriginChannelPackageLatestGet::descriptor_static(None),
            handlers::origin_channel_package_latest_get);
        map.register(OriginChannelPackageListRequest::descriptor_static(None),
            handlers::origin_channel_package_list);
        map
    };
}

#[derive(Clone)]
pub struct ServerState {
    datastore: DataStore,
}

impl ServerState {
    fn new(cfg: &Config, router_pipe: Arc<String>) -> SrvResult<Self> {
        Ok(ServerState { datastore: DataStore::new(cfg, router_pipe)? })
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

struct OriginSrv;
impl Dispatcher for OriginSrv {
    const APP_NAME: &'static str = "builder-originsrv";
    const PROTOCOL: Protocol = Protocol::OriginSrv;

    type Error = SrvError;
    type State = ServerState;

    fn app_init(
        config: &<Self::State as AppState>::Config,
        router_pipe: Arc<String>,
    ) -> SrvResult<<Self::State as AppState>::InitState> {
        let state = ServerState::new(config, router_pipe)?;
        state.datastore.setup()?;
        state.datastore.start_async();
        Ok(state)
    }

    fn dispatch_table() -> &'static DispatchTable<Self> {
        &DISPATCH_TABLE
    }
}

pub fn run(config: Config) -> AppResult<(), SrvError> {
    app_start::<OriginSrv>(config)
}
