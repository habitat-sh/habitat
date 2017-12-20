// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

//! Contains core functionality for the Application's main server.

mod handlers;

use github_api_client::GitHubClient;
use hab_net::privilege::FeatureFlags;
use http_gateway;
use http_gateway::app::prelude::*;
use iron;
use persistent;
use staticfile::Static;

use self::handlers::*;
use config::Config;

struct AdminSrv;
impl HttpGateway for AdminSrv {
    const APP_NAME: &'static str = "builder-admin";

    type Config = Config;

    fn add_middleware(config: Arc<Self::Config>, chain: &mut iron::Chain) {
        chain.link(persistent::Read::<GitHubCli>::both(
            GitHubClient::new(config.github.clone()),
        ));
    }

    fn mount(config: Arc<Self::Config>, chain: iron::Chain) -> Mount {
        let mut mount = Mount::new();
        if let Some(ref path) = config.ui.root {
            debug!("Mounting UI at filepath {}", path);
            mount.mount("/", Static::new(path));
        }
        mount.mount("/v1", chain);
        mount
    }

    fn router(config: Arc<Self::Config>) -> Router {
        let admin = Authenticated::new(config.github.clone()).require(FeatureFlags::ADMIN);
        router!(
            status: get "/status" => status,
            search: post "/search" => XHandler::new(search).before(admin.clone()),
            account: get "/accounts/:id" => XHandler::new(account_show).before(admin.clone()),
        )
    }
}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
pub fn run(config: Config) -> AppResult<()> {
    http_gateway::start::<AdminSrv>(config)
}
