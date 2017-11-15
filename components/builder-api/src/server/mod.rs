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

use depot;
use github_api_client::GitHubClient;
use hab_core::event::EventLogger;
use http_gateway;
use http_gateway::app::prelude::*;
use iron;
use mount::Mount;
use persistent::{self, Read};
use segment_api_client::SegmentClient;
use staticfile::Static;

use super::github;
use self::handlers::*;
use config::Config;

struct ApiSrv;
impl HttpGateway for ApiSrv {
    const APP_NAME: &'static str = "builder-api";

    type Config = Config;

    fn add_middleware(config: Arc<Self::Config>, chain: &mut iron::Chain) {
        chain.link(persistent::Read::<Self::Config>::both(config.clone()));
        chain.link(persistent::Read::<GitHubCli>::both(
            GitHubClient::new(config.github.clone()),
        ));
        chain.link(persistent::Read::<SegmentCli>::both(
            SegmentClient::new(config.segment.clone()),
        ));
        chain.link(Read::<EventLog>::both(
            EventLogger::new(&config.log_dir, config.events_enabled),
        ));
        chain.link_after(Cors);
    }

    fn mount(config: Arc<Self::Config>, chain: iron::Chain) -> Mount {
        let mut depot_config = config.depot.clone();
        depot_config.segment = config.segment.clone();
        let depot = depot::DepotUtil::new(depot_config);
        let depot_chain = depot::server::router(depot).unwrap();
        let mut mount = Mount::new();
        if let Some(ref path) = config.ui.root {
            debug!("Mounting UI at filepath {}", path);
            mount.mount("/", Static::new(path));
        }
        mount.mount("/v1", chain);
        mount.mount("/v1/depot", depot_chain);
        mount
    }

    fn router(config: Arc<Self::Config>) -> Router {
        let basic = Authenticated::new(config.github.clone());

        router!(
            status: get "/status" => status,
            authenticate: get "/authenticate/:code" => github_authenticate,
            notify: post "/notify" => notify,
            update_profile: patch "/profile" => XHandler::new(update_profile).before(basic.clone()),
            get_profile: get "/profile" => XHandler::new(get_profile).before(basic.clone()),

            job: get "/jobs/:id" => XHandler::new(job_show).before(basic.clone()),
            job_log: get "/jobs/:id/log" => XHandler::new(job_log).before(basic.clone()),
            job_group_promote: post "/jobs/group/:id/promote/:channel" => {
                XHandler::new(job_group_promote).before(basic.clone())
            },
            job_group_demote: post "/jobs/group/:id/demote/:channel" => {
                XHandler::new(job_group_demote).before(basic.clone())
            },
            job_group_cancel: post "/jobs/group/:id/cancel" => {
                XHandler::new(job_group_cancel).before(basic.clone())
            },
            rdeps: get "/rdeps/:origin/:name" => rdeps_show,

            user_invitations: get "/user/invitations" => {
                XHandler::new(list_account_invitations).before(basic.clone())
            },
            user_origins: get "/user/origins" => {
                XHandler::new(list_user_origins).before(basic.clone())
            },

            projects: post "/projects" => XHandler::new(project_create).before(basic.clone()),
            project: get "/projects/:origin/:name" => {
                XHandler::new(project_show).before(basic.clone())
            },
            project_list: get "/projects/:origin" => {
                XHandler::new(project_list).before(basic.clone())
            },
            project_jobs: get "/projects/:origin/:name/jobs" => {
                XHandler::new(project_jobs).before(basic.clone())
            },
            edit_project: put "/projects/:origin/:name" => {
                XHandler::new(project_update).before(basic.clone())
            },
            delete_project: delete "/projects/:origin/:name" => {
                XHandler::new(project_delete).before(basic.clone())
            },
            project_privacy_toggle: patch "/projects/:origin/:name/:visibility" => {
                XHandler::new(project_privacy_toggle).before(basic.clone())
            },
            project_integration_get: get
                "/projects/:origin/:name/integrations/:integration/default" => {
                XHandler::new(get_project_integration).before(basic.clone())
            },
            project_integration_put: put
                "/projects/:origin/:name/integrations/:integration/default" => {
                XHandler::new(create_project_integration).before(basic.clone())
            },
            project_integration_delete: delete
                "/projects/:origin/:name/integrations/:integration/default" => {
                XHandler::new(delete_project_integration).before(basic.clone())
            },
            ext_search_code: get "/ext/installations/:install_id/search/code" => {
                XHandler::new(github::search_code).before(basic.clone())
            },
            ext_repo_content: get "/ext/installations/:install_id/repos/:repo/contents/:path" => {
                XHandler::new(github::repo_file_content).before(basic.clone())
            },
            ext_credentials_registry: post "/ext/integrations/:registry_type/credentials/validate" => {
                XHandler::new(validate_registry_credentials).before(basic.clone())
            },
        )
    }
}

pub fn run(config: Config) -> AppResult<()> {
    http_gateway::start::<ApiSrv>(config)
}
