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

//! A collection of handlers for the HTTP server's router

use base64;
use bodyparser;
use depot::server::check_origin_access;
use hab_core::package::Plan;
use hab_core::event::*;
use hab_net;
use hab_net::http::controller::*;
use hab_net::routing::Broker;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use persistent;
use protocol::jobsrv::{Job, JobGet, JobSpec};
use protocol::vault::*;
use protocol::net::{self, NetOk, ErrCode};
use router::Router;

define_event_log!();

#[derive(Clone, Serialize, Deserialize)]
struct JobCreateReq {
    project_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ProjectCreateReq {
    origin: String,
    plan_path: String,
    github: GitHubProject,
}

#[derive(Clone, Serialize, Deserialize)]
struct ProjectUpdateReq {
    plan_path: String,
    github: GitHubProject,
}

#[derive(Clone, Serialize, Deserialize)]
struct GitHubProject {
    organization: String,
    repo: String,
}

pub fn github_authenticate(req: &mut Request) -> IronResult<Response> {
    let code = {
        let params = req.extensions.get::<Router>().unwrap();
        params.find("code").unwrap().to_string()
    };

    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    match github.authenticate(&code) {
        Ok(token) => {
            let session = try!(session_create(&github, &token));

            log_event!(req,
                       Event::GithubAuthenticate {
                           user: session.get_name().to_string(),
                           account: session.get_id().to_string(),
                       });

            Ok(render_json(status::Ok, &session))
        }
        Err(hab_net::Error::Net(err)) => Ok(render_net_error(&err)),
        Err(e) => {
            error!("unhandled github authentication, err={:?}", e);
            let err = net::err(ErrCode::BUG, "rg:auth:0");
            Ok(render_net_error(&err))
        }
    }
}

pub fn job_create(req: &mut Request) -> IronResult<Response> {
    let mut project_get = ProjectGet::new();
    {
        match req.get::<bodyparser::Struct<JobCreateReq>>() {
            Ok(Some(body)) => project_get.set_id(body.project_id),
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    // TODO: SA - Eliminate need to clone the session
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let mut conn = Broker::connect().unwrap();
    let project = match conn.route::<ProjectGet, Project>(&project_get) {
        Ok(project) => project,
        Err(err) => return Ok(render_net_error(&err)),
    };

    let mut job_spec: JobSpec = JobSpec::new();
    job_spec.set_owner_id(session.get_id());
    job_spec.set_project(project);

    match conn.route::<JobSpec, Job>(&job_spec) {
        Ok(job) => {
            log_event!(req,
                       Event::JobCreate {
                           package: job.get_project().get_id().to_string(),
                           account: session.get_id().to_string(),
                       });
            Ok(render_json(status::Created, &job))
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn job_show(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let id = match params.find("id").unwrap().parse::<u64>() {
        Ok(id) => id,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    let mut conn = Broker::connect().unwrap();
    let mut request = JobGet::new();
    request.set_id(id);
    match conn.route::<JobGet, Job>(&request) {
        Ok(job) => Ok(render_json(status::Ok, &job)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Endpoint for determining availability of builder-api components.
///
/// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
pub fn status(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

pub fn list_account_invitations(req: &mut Request) -> IronResult<Response> {
    let session = req.extensions.get::<Authenticated>().unwrap();
    let mut conn = Broker::connect().unwrap();
    let mut request = AccountInvitationListRequest::new();
    request.set_account_id(session.get_id());
    match conn.route::<AccountInvitationListRequest, AccountInvitationListResponse>(&request) {
        Ok(invites) => Ok(render_json(status::Ok, &invites)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn list_user_origins(req: &mut Request) -> IronResult<Response> {
    let session = req.extensions.get::<Authenticated>().unwrap();
    let mut conn = Broker::connect().unwrap();
    let mut request = AccountOriginListRequest::new();
    request.set_account_id(session.get_id());
    match conn.route::<AccountOriginListRequest, AccountOriginListResponse>(&request) {
        Ok(invites) => Ok(render_json(status::Ok, &invites)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn accept_invitation(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginInvitationAcceptRequest::new();
    request.set_ignore(false);
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let params = &req.extensions.get::<Router>().unwrap().clone();

    request.set_account_accepting_request(session.get_id());
    match params.find("invitation_id").unwrap().parse::<u64>() {
        Ok(value) => request.set_invite_id(value),
        Err(_) => return Ok(Response::with(status::BadRequest)),
    }

    let mut conn = Broker::connect().unwrap();
    match conn.route::<OriginInvitationAcceptRequest, OriginInvitationAcceptResponse>(&request) {
        Ok(_invites) => {
            log_event!(req,
                       Event::OriginInvitationAccept {
                           id: request.get_invite_id().to_string(),
                           account: session.get_id().to_string(),
                       });
            Ok(Response::with(status::NoContent))
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn ignore_invitation(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginInvitationAcceptRequest::new();
    request.set_ignore(true);
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let params = &req.extensions.get::<Router>().unwrap().clone();

    request.set_account_accepting_request(session.get_id());
    match params.find("invitation_id").unwrap().parse::<u64>() {
        Ok(value) => request.set_invite_id(value),
        Err(_) => return Ok(Response::with(status::BadRequest)),
    }

    let mut conn = Broker::connect().unwrap();
    match conn.route::<OriginInvitationAcceptRequest, OriginInvitationAcceptResponse>(&request) {
        Ok(_invites) => {
            log_event!(req,
                       Event::OriginInvitationIgnore {
                           id: request.get_invite_id().to_string(),
                           account: session.get_id().to_string(),
                       });
            Ok(Response::with(status::NoContent))
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Create a new project as the authenticated user and associated to the given origin
pub fn project_create(req: &mut Request) -> IronResult<Response> {
    let mut request = ProjectCreate::new();
    let mut project = Project::new();
    let mut origin_get = OriginGet::new();
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let (organization, repo) = match req.get::<bodyparser::Struct<ProjectCreateReq>>() {
        Ok(Some(body)) => {
            if body.origin.len() <= 0 {
                return Ok(Response::with((status::UnprocessableEntity,
                                          "Missing value for field: `origin`")));
            }
            if body.plan_path.len() <= 0 {
                return Ok(Response::with((status::UnprocessableEntity,
                                          "Missing value for field: `plan_path`")));
            }
            if body.github.organization.len() <= 0 {
                return Ok(Response::with((status::UnprocessableEntity,
                                          "Missing value for field: `github.organization`")));
            }
            if body.github.repo.len() <= 0 {
                return Ok(Response::with((status::UnprocessableEntity,
                                          "Missing value for field: `github.repo`")));
            }
            let mut vcs = VCSGit::new();
            origin_get.set_name(body.origin);
            project.set_plan_path(body.plan_path);
            match github.repo(&session.get_token(),
                              &body.github.organization,
                              &body.github.repo) {
                Ok(repo) => vcs.set_url(repo.clone_url),
                Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pc:1"))),
            }
            project.set_git(vcs);
            (body.github.organization, body.github.repo)
        }
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    };
    let mut conn = Broker::connect().unwrap();
    let origin = match conn.route::<OriginGet, Origin>(&origin_get) {
        Ok(response) => response,
        Err(err) => return Ok(render_net_error(&err)),
    };
    match github.contents(&session.get_token(),
                          &organization,
                          &repo,
                          &project.get_plan_path()) {
        Ok(contents) => {
            match base64::decode(&contents.content) {
                Ok(ref bytes) => {
                    match Plan::from_bytes(bytes) {
                        Ok(plan) => project.set_id(format!("{}/{}", origin.get_name(), plan.name)),
                        Err(_) => {
                            return Ok(Response::with((status::UnprocessableEntity, "rg:pc:3")))
                        }
                    }
                }
                Err(e) => {
                    error!("Base64 decode failure: {:?}", e);
                    return Ok(Response::with((status::UnprocessableEntity, "rg:pc:4")));
                }
            }
        }
        Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pc:2"))),
    }

    project.set_owner_id(session.get_id());
    request.set_project(project);
    match conn.route::<ProjectCreate, Project>(&request) {
        Ok(response) => {
            log_event!(req,
                       Event::ProjectCreate {
                           origin: origin.get_name().to_string(),
                           package: request.get_project().get_id().to_string(),
                           account: session.get_id().to_string(),
                       });
            Ok(render_json(status::Created, &response))
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Delete the given project
pub fn project_delete(req: &mut Request) -> IronResult<Response> {
    let mut project_del = ProjectDelete::new();
    let params = req.extensions.get::<Router>().unwrap();
    let session = req.extensions.get::<Authenticated>().unwrap();
    let mut conn = Broker::connect().unwrap();
    {
        let origin = params.find("origin").unwrap();
        if !try!(check_origin_access(&mut conn, session.get_id(), origin)) {
            return Ok(Response::with(status::Forbidden));
        }
        let name = params.find("name").unwrap();
        project_del.set_id(format!("{}/{}", origin, name));
    }
    project_del.set_requestor_id(session.get_id());
    match conn.route::<ProjectDelete, NetOk>(&project_del) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Update the given project
pub fn project_update(req: &mut Request) -> IronResult<Response> {
    let mut request = ProjectUpdate::new();
    let mut project = Project::new();
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    let (organization, repo) = match req.get::<bodyparser::Struct<ProjectCreateReq>>() {
        Ok(Some(body)) => {
            if body.plan_path.len() <= 0 {
                return Ok(Response::with((status::UnprocessableEntity,
                                          "Missing value for field: `plan_path`")));
            }
            if body.github.organization.len() <= 0 {
                return Ok(Response::with((status::UnprocessableEntity,
                                          "Missing value for field: `github.organization`")));
            }
            if body.github.repo.len() <= 0 {
                return Ok(Response::with((status::UnprocessableEntity,
                                          "Missing value for field: `github.repo`")));
            }
            let session = req.extensions.get::<Authenticated>().unwrap();
            let mut vcs = VCSGit::new();
            project.set_plan_path(body.plan_path);
            match github.repo(&session.get_token(),
                              &body.github.organization,
                              &body.github.repo) {
                Ok(repo) => vcs.set_url(repo.clone_url),
                Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pu:1"))),
            }
            project.set_git(vcs);
            (body.github.organization, body.github.repo)
        }
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    };
    let mut conn = Broker::connect().unwrap();
    let session = req.extensions.get::<Authenticated>().unwrap();
    match github.contents(&session.get_token(),
                          &organization,
                          &repo,
                          &project.get_plan_path()) {
        Ok(contents) => {
            match base64::decode(&contents.content) {
                Ok(ref bytes) => {
                    match Plan::from_bytes(bytes) {
                        Ok(plan) => {
                            let params = req.extensions.get::<Router>().unwrap();
                            let origin = params.find("origin").unwrap();
                            if !try!(check_origin_access(&mut conn, session.get_id(), origin)) {
                                return Ok(Response::with(status::Forbidden));
                            }
                            let name = params.find("name").unwrap();
                            if plan.name != params.find("name").unwrap() {
                                return Ok(Response::with((status::UnprocessableEntity, "rg:pu:2")));
                            }
                            project.set_id(format!("{}/{}", origin, name));
                        }
                        Err(_) => {
                            return Ok(Response::with((status::UnprocessableEntity, "rg:pu:3")))
                        }
                    }
                }
                Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pu:4"))),
            }
        }
        Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pu:5"))),
    }
    // JW TODO: owner_id should *not* be changing but we aren't using it just yet. FIXME before
    // making the project API public.
    project.set_owner_id(session.get_id());
    request.set_requestor_id(session.get_id());
    request.set_project(project);
    match conn.route::<ProjectUpdate, NetOk>(&request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Display the the given project's details
pub fn project_show(req: &mut Request) -> IronResult<Response> {
    let mut project_get = ProjectGet::new();
    let params = req.extensions.get::<Router>().unwrap();
    {
        let origin = params.find("origin").unwrap();
        let name = params.find("name").unwrap();
        project_get.set_id(format!("{}/{}", origin, name));
    }
    let mut conn = Broker::connect().unwrap();
    match conn.route::<ProjectGet, Project>(&project_get) {
        Ok(project) => Ok(render_json(status::Ok, &project)),
        Err(err) => Ok(render_net_error(&err)),
    }
}
