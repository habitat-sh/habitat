// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::io::Read;
use std::str::FromStr;

use bldr_core::build_config::{BLDR_CFG, BuildCfg};
use constant_time_eq::constant_time_eq;
use github_api_client::GitHubClient;
use hab_core::package::Plan;
use hex::ToHex;
use http_gateway::http::controller::*;
use iron::status;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use persistent;
use protocol::originsrv::{OriginProject, OriginProjectGet};
use protocol::scheduler::{Group, GroupCreate};
use router::Router;
use serde_json;

use error::Error;
use headers::*;
use types::*;

pub enum GitHubEvent {
    Push,
    Ping,
}

impl FromStr for GitHubEvent {
    type Err = Error;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        match event {
            "ping" => Ok(GitHubEvent::Ping),
            "push" => Ok(GitHubEvent::Push),
            _ => Err(Error::UnknownGitHubEvent(event.to_string())),
        }
    }
}

pub fn handle_event(req: &mut Request) -> IronResult<Response> {
    let event = match req.headers.get::<XGitHubEvent>() {
        Some(&XGitHubEvent(ref event)) => {
            match GitHubEvent::from_str(event) {
                Ok(event) => event,
                Err(err) => return Ok(Response::with((status::BadRequest, err.to_string()))),
            }
        }
        _ => return Ok(Response::with(status::BadRequest)),
    };

    // Authenticate the hook
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    let gh_signature = match req.headers.get::<XHubSignature>() {
        Some(&XHubSignature(ref sig)) => sig.clone(),
        None => {
            warn!("Received a GitHub hook with no signature");
            return Ok(Response::with(status::BadRequest));
        }
    };

    let mut payload = String::new();
    if let Err(err) = req.body.read_to_string(&mut payload) {
        warn!("Unable to read GitHub Hook request body, {}", err);
        return Ok(Response::with(status::BadRequest));
    }
    trace!("handle-notify, {}", payload);

    let key = PKey::hmac(github.webhook_secret.as_bytes()).unwrap();
    let mut signer = Signer::new(MessageDigest::sha1(), &key).unwrap();
    signer.update(payload.as_bytes()).unwrap();
    let hmac = signer.finish().unwrap();
    let computed_signature = format!("sha1={}", &hmac.to_hex());

    if !constant_time_eq(gh_signature.as_bytes(), computed_signature.as_bytes()) {
        warn!(
            "Web hook signatures don't match. GH = {}, Our = {}",
            gh_signature,
            computed_signature
        );
        return Ok(Response::with(status::BadRequest));
    }

    match event {
        GitHubEvent::Ping => Ok(Response::with(status::Ok)),
        GitHubEvent::Push => handle_push(req, &payload),
    }
}

pub fn repo_file_content(req: &mut Request) -> IronResult<Response> {
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    let session = req.extensions.get::<Authenticated>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let path = match params.find("path") {
        Some(path) => path,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let install_id = match params.find("install_id") {
        Some(install_id) => {
            match install_id.parse::<u32>() {
                Ok(install_id) => install_id,
                Err(_) => return Ok(Response::with(status::BadRequest)),
            }
        }
        None => return Ok(Response::with(status::BadRequest)),
    };
    let token = {
        match github.app_installation_token(install_id) {
            Ok(token) => token,
            Err(err) => {
                return Ok(Response::with((status::BadGateway, err.to_string())));
            }
        }
    };
    let repo = {
        let repo = match params.find("repo") {
            Some(repo) => repo,
            None => return Ok(Response::with(status::BadRequest)),
        };
        let repos = match github.repositories(session.get_oauth_token(), install_id) {
            Ok(repos) => repos,
            Err(err) => return Ok(Response::with((status::BadGateway, err.to_string()))),
        };
        match repos.into_iter().find(|r| r.name == repo) {
            Some(repo) => repo,
            None => return Ok(Response::with(status::NotFound)),
        }
    };
    match github.contents(&token, repo.id, path) {
        Ok(search) => Ok(render_json(status::Ok, &search)),
        Err(err) => Ok(Response::with((status::BadGateway, err.to_string()))),
    }
}

pub fn search_code(req: &mut Request) -> IronResult<Response> {
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let query = match req.url.query() {
        Some(query) => query,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let install_id = match params.find("install_id") {
        Some(install_id) => {
            match install_id.parse::<u32>() {
                Ok(install_id) => install_id,
                Err(_) => return Ok(Response::with(status::BadRequest)),
            }
        }
        None => return Ok(Response::with(status::BadRequest)),
    };
    let token = {
        match github.app_installation_token(install_id) {
            Ok(token) => token,
            Err(err) => {
                return Ok(Response::with((status::BadGateway, err.to_string())));
            }
        }
    };
    match github.search_code(&token, query) {
        Ok(search) => Ok(render_json(status::Ok, &search)),
        Err(err) => Ok(Response::with((status::BadGateway, err.to_string()))),
    }
}

fn handle_push(req: &mut Request, body: &str) -> IronResult<Response> {
    let hook = match serde_json::from_str::<GitHubWebhookPush>(&body) {
        Ok(hook) => hook,
        Err(err) => {
            return Ok(Response::with(
                (status::UnprocessableEntity, err.to_string()),
            ));
        }
    };

    if hook.commits.is_empty() {
        return Ok(Response::with(status::Ok));
    }

    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    let token = match github.app_installation_token(hook.installation.id) {
        Ok(token) => token,
        Err(err) => {
            return Ok(Response::with((status::BadGateway, err.to_string())));
        }
    };

    let config = read_bldr_config(&*github, &token, &hook);
    debug!("Config, {:?}", config);
    let plans = read_plans(&github, &token, &hook, &config);
    debug!("Triggered Plans, {:?}", plans);
    build_plans(req, &hook.repository.clone_url, plans)
}

fn build_plans(req: &mut Request, repo_url: &str, plans: Vec<Plan>) -> IronResult<Response> {
    let mut request = GroupCreate::new();

    for plan in plans.iter() {
        let mut project_get = OriginProjectGet::new();
        project_get.set_name(format!("{}/{}", &plan.origin, &plan.name));

        match route_message::<OriginProjectGet, OriginProject>(req, &project_get) {
            Ok(project) => {
                if repo_url != project.get_vcs_data() {
                    warn!(
                        "Repo URL ({}) doesn't match project vcs data ({}). Aborting.",
                        repo_url,
                        project.get_vcs_data()
                    );
                    continue;
                }
            }
            Err(err) => {
                warn!("Failed to fetch project, {}", err);
                continue;
            }
        }

        debug!("Scheduling, {:?}", plan);
        request.set_origin(plan.origin.clone());
        request.set_package(plan.name.clone());
        // JW TODO: We need to be able to determine which platform this build is for based on
        // the directory structure the plan is found in or metadata inside the plan. We will need
        // to have this done before we support building additional targets with Builder.
        request.set_target("x86_64-linux".to_string());
        match route_message::<GroupCreate, Group>(req, &request) {
            Ok(group) => debug!("Group created, {:?}", group),
            Err(err) => debug!("Failed to create group, {:?}", err),
        }
    }
    Ok(render_json(status::Ok, &plans))
}

fn read_bldr_config(github: &GitHubClient, token: &str, hook: &GitHubWebhookPush) -> BuildCfg {
    match github.contents(token, hook.repository.id, BLDR_CFG) {
        Ok(Some(contents)) => {
            match contents.decode() {
                Ok(ref bytes) => {
                    match BuildCfg::from_slice(bytes) {
                        Ok(cfg) => cfg,
                        Err(err) => {
                            debug!("unable to parse bldr.toml, {}", err);
                            BuildCfg::default()
                        }
                    }
                }
                Err(err) => {
                    debug!("unable to read bldr.toml, {}", err);
                    BuildCfg::default()
                }
            }
        }
        Ok(None) => BuildCfg::default(),
        Err(err) => {
            warn!("unable to retrieve bldr.toml, {}", err);
            BuildCfg::default()
        }
    }
}

fn read_plans(
    github: &GitHubClient,
    token: &str,
    hook: &GitHubWebhookPush,
    config: &BuildCfg,
) -> Vec<Plan> {
    let mut plans = Vec::with_capacity(config.projects().len());
    for project in config.triggered_by(hook.branch(), hook.changed().as_slice()) {
        if let Some(plan) = read_plan(github, token, hook, &project.plan_file().to_string_lossy()) {
            plans.push(plan)
        }
    }
    plans
}

fn read_plan(
    github: &GitHubClient,
    token: &str,
    hook: &GitHubWebhookPush,
    path: &str,
) -> Option<Plan> {
    match github.contents(token, hook.repository.id, path) {
        Ok(Some(contents)) => {
            match contents.decode() {
                Ok(bytes) => {
                    match Plan::from_bytes(bytes.as_slice()) {
                        Ok(plan) => Some(plan),
                        Err(err) => {
                            debug!("unable to read plan, {}, {}", path, err);
                            None
                        }
                    }
                }
                Err(err) => {
                    debug!("unable to read plan, {}, {}", path, err);
                    None
                }
            }
        }
        Ok(None) => None,
        Err(err) => {
            warn!("unable to retrieve plan, {}, {}", path, err);
            None
        }
    }
}
