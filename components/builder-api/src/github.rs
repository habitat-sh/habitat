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

use std::str::FromStr;

use bodyparser;
use bldr_core::build_config::{BLDR_CFG, BuildCfg};
use github_api_client::GitHubClient;
use hab_core::package::Plan;
use http_gateway::http::controller::*;
use iron::status;
use persistent;
use protocol::scheduler::{Group, GroupCreate};
use router::Router;

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
    // JW TODO: Authenticate hook
    match event {
        GitHubEvent::Ping => Ok(Response::with(status::Ok)),
        GitHubEvent::Push => handle_push(req),
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
        let repos = match github.repositories(session.get_token(), install_id) {
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

fn handle_push(req: &mut Request) -> IronResult<Response> {
    let hook = match req.get::<bodyparser::Struct<GitHubWebhookPush>>() {
        Ok(Some(hook)) => hook,
        Ok(None) => return Ok(Response::with(status::UnprocessableEntity)),
        Err(err) => {
            return Ok(Response::with(
                (status::UnprocessableEntity, err.to_string()),
            ));
        }
    };
    if hook.commits.is_empty() {
        return Ok(Response::with((status::Ok)));
    }
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    let token = match github.app_installation_token(hook.installation.id) {
        Ok(token) => token,
        Err(err) => {
            return Ok(Response::with((status::BadGateway, err.to_string())));
        }
    };
    let mut query = format!("q={}+in:path+repo:{}", "plan.sh", hook.repository.full_name);
    let plans = match github.search_code(&token, &query) {
        Ok(search) => search.items,
        Err(err) => return Ok(Response::with((status::BadGateway, err.to_string()))),
    };
    if plans.is_empty() {
        return Ok(Response::with(status::Ok));
    }
    query = format!("q={}+in:path+repo:{}", BLDR_CFG, hook.repository.full_name);
    let config = match github.search_code(&token, &query) {
        Ok(search) => {
            match search
                .items
                .into_iter()
                .filter(|i| i.path == BLDR_CFG)
                .collect::<Vec<SearchItem>>()
                .pop() {
                Some(item) => {
                    match read_bldr_config(&*github, &token, &hook, &item.path) {
                        HandleResult::Ok(cfg) => Some(cfg),
                        HandleResult::Err(response) => return Ok(response),
                    }
                }
                None => None,
            }
        }
        Err(err) => return Ok(Response::with((status::BadGateway, err.to_string()))),
    };
    debug!("Config, {:?}", config);
    let mut plans = match read_plans(&github, &token, &hook, plans) {
        HandleResult::Ok(plans) => plans,
        HandleResult::Err(err) => return Ok(err),
    };
    debug!("Plans, {:?}", plans);
    if let Some(cfg) = config {
        plans.retain(|plan| match cfg.get(&plan.name) {
            Some(project) => hook.changed().iter().any(|f| project.triggered_by(f)),
            None => false,
        })
    }
    build_plans(req, plans)
}

fn build_plans(req: &mut Request, plans: Vec<Plan>) -> IronResult<Response> {
    // JW TODO: Validate that this repository is where these plans belong. You could theoretically
    // create a plan in a different repo and force a build of another piece of software without
    // this check.
    let mut request = GroupCreate::new();
    for plan in plans.into_iter() {
        debug!("Scheduling, {:?}", plan);
        request.set_origin(plan.origin);
        request.set_package(plan.name);
        // JW TODO: We need to be able to determine which platform this build is for based on
        // the directory structure the plan is found in or metadata inside the plan. We will need
        // to have this done before we support building additional targets with Builder.
        request.set_target("x86_64-linux".to_string());
        match route_message::<GroupCreate, Group>(req, &request) {
            Ok(group) => debug!("Group created, {:?}", group),
            Err(err) => debug!("Failed to create group, {:?}", err),
        }
    }
    Ok(Response::with(status::Ok))
}

fn read_bldr_config(
    github: &GitHubClient,
    token: &str,
    hook: &GitHubWebhookPush,
    path: &str,
) -> HandleResult<BuildCfg> {
    match github.contents(token, hook.repository.id, path) {
        Ok(contents) => {
            match contents.decode() {
                Ok(ref bytes) => {
                    match BuildCfg::from_slice(bytes) {
                        Ok(cfg) => HandleResult::Ok(cfg),
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
            Ok(contents) => {
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
            Err(err) => {
                return HandleResult::Err(Response::with(
                    (status::BadGateway, format!("{}, {}", plan.path, err)),
                ))
            }
        }
    }
    HandleResult::Ok(parsed)
}
