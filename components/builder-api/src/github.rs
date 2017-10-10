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

enum HandleResult<T> {
    Ok(T),
    Err(Response),
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

    let query = format!("q={}+in:path+repo:{}", "plan.sh", hook.repository.full_name);
    let plans = match github.search_code(&token, &query) {
        Ok(plans) => plans,
        Err(err) => return Ok(Response::with((status::BadGateway, err.to_string()))),
    };

    if plans.is_empty() {
        return Ok(Response::with(status::Ok));
    }

    let config = match read_bldr_config(&*github, &token, &hook) {
        HandleResult::Ok(config) => config,
        HandleResult::Err(err) => return Ok(err),
    };
    debug!("Config, {:?}", config);

    let mut plans = match read_plans(&github, &token, &hook, plans) {
        HandleResult::Ok(plans) => plans,
        HandleResult::Err(err) => return Ok(err),
    };
    debug!("Plans, {:?}", plans);

    if let Some(cfg) = config {
        plans.retain(|plan| match cfg.get(&plan.name) {
            Some(project) => project.triggered_by(hook.branch(), hook.changed().as_slice()),
            None => false,
        })
    }

    debug!("Filtered Plans, {:?}", plans);
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

fn read_bldr_config(
    github: &GitHubClient,
    token: &str,
    hook: &GitHubWebhookPush,
) -> HandleResult<Option<BuildCfg>> {
    match github.contents(token, hook.repository.id, BLDR_CFG) {
        Ok(Some(contents)) => {
            match contents.decode() {
                Ok(ref bytes) => {
                    match BuildCfg::from_slice(bytes) {
                        Ok(cfg) => HandleResult::Ok(Some(cfg)),
                        Err(err) => HandleResult::Err(Response::with(
                            (status::UnprocessableEntity, err.to_string()),
                        )),
                    }
                }
                Err(err) => {
                    HandleResult::Err(Response::with(
                        (status::UnprocessableEntity, err.to_string()),
                    ))
                }
            }
        }
        Ok(None) => HandleResult::Ok(None),
        Err(err) => HandleResult::Err(Response::with((status::BadGateway, err.to_string()))),
    }
}

fn read_plans(
    github: &GitHubClient,
    token: &str,
    hook: &GitHubWebhookPush,
    plans: Vec<SearchItem>,
) -> HandleResult<Vec<Plan>> {
    let mut parsed = Vec::with_capacity(plans.len());
    for plan in plans {
        match github.contents(token, hook.repository.id, &plan.path) {
            Ok(Some(contents)) => {
                match contents.decode() {
                    Ok(bytes) => {
                        match Plan::from_bytes(bytes.as_slice()) {
                            Ok(plan) => parsed.push(plan),
                            Err(err) => debug!("{}, {}", plan.path, err),
                        }
                    }
                    Err(err) => {
                        return HandleResult::Err(Response::with((
                            status::UnprocessableEntity,
                            format!("{}, {}", plan.path, err),
                        )))
                    }
                }
            }
            Ok(None) => (),
            Err(err) => {
                return HandleResult::Err(Response::with(
                    (status::BadGateway, format!("{}, {}", plan.path, err)),
                ))
            }
        }
    }
    HandleResult::Ok(parsed)
}
