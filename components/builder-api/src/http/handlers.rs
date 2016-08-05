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

//! A collection of handlers for the HTTP server's router

use bodyparser;
use hab_core::package::Plan;
use hab_net;
use hab_net::http::controller::*;
use hab_net::routing::Broker;
use iron::prelude::*;
use iron::status;
use persistent;
use protocol::jobsrv::{Job, JobGet, JobSpec};
use protocol::sessionsrv::{OAuthProvider, Session, SessionCreate};
use protocol::vault::*;
use protocol::net::{self, NetOk, ErrCode};
use router::Router;
use rustc_serialize::base64::FromBase64;
use serde_json::Value;

pub fn session_create(req: &mut Request) -> IronResult<Response> {
    let code = {
        let params = req.extensions.get::<Router>().unwrap();
        params.find("code").unwrap().to_string()
    };
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    match github.authenticate(&code) {
        Ok(token) => {
            match github.user(&token) {
                Ok(user) => {
                    // Select primary email. If no primary email can be found, use any email. If
                    // no email is associated with account return an access denied error.
                    let email = match github.emails(&token) {
                        Ok(ref emails) => {
                            emails.iter().find(|e| e.primary).unwrap_or(&emails[0]).email.clone()
                        }
                        Err(_) => {
                            let err = net::err(ErrCode::ACCESS_DENIED, "rg:auth:0");
                            return Ok(render_net_error(&err));
                        }
                    };
                    let mut conn = Broker::connect().unwrap();
                    let mut request = SessionCreate::new();
                    request.set_token(token);
                    request.set_extern_id(user.id);
                    request.set_email(email);
                    request.set_name(user.login);
                    request.set_provider(OAuthProvider::GitHub);
                    match conn.route::<SessionCreate, Session>(&request) {
                        Ok(session) => Ok(render_json(status::Ok, &session)),
                        Err(err) => Ok(render_net_error(&err)),
                    }
                }
                Err(e @ hab_net::Error::JsonDecode(_)) => {
                    debug!("github user get, err={:?}", e);
                    let err = net::err(ErrCode::BAD_REMOTE_REPLY, "rg:auth:1");
                    Ok(render_net_error(&err))
                }
                Err(e) => {
                    debug!("github user get, err={:?}", e);
                    let err = net::err(ErrCode::BUG, "rg:auth:2");
                    Ok(render_net_error(&err))
                }
            }
        }
        Err(hab_net::Error::Auth(e)) => {
            debug!("github authentication, err={:?}", e);
            let err = net::err(ErrCode::REMOTE_REJECTED, e.error);
            Ok(render_net_error(&err))
        }
        Err(e @ hab_net::Error::JsonDecode(_)) => {
            debug!("github authentication, err={:?}", e);
            let err = net::err(ErrCode::BAD_REMOTE_REPLY, "rg:auth:1");
            Ok(render_net_error(&err))
        }
        Err(e) => {
            error!("github authentication, err={:?}", e);
            let err = net::err(ErrCode::BUG, "rg:auth:0");
            Ok(render_net_error(&err))
        }
    }
}

pub fn job_create(req: &mut Request) -> IronResult<Response> {
    let mut project_get = ProjectGet::new();
    {
        match req.get::<bodyparser::Json>() {
            Ok(Some(ref body)) => {
                match body.find("project_id") {
                    Some(&Value::String(ref val)) => project_get.set_id(val.to_string()),
                    _ => return Ok(Response::with(status::UnprocessableEntity)),
                }
            }
            _ => return Ok(Response::with(status::BadRequest)),
        }
    }
    let session = req.extensions.get::<Authenticated>().unwrap();
    let mut conn = Broker::connect().unwrap();
    let project = match conn.route::<ProjectGet, Project>(&project_get) {
        Ok(project) => project,
        Err(err) => return Ok(render_net_error(&err)),
    };
    let mut job_spec: JobSpec = JobSpec::new();
    job_spec.set_owner_id(session.get_id());
    job_spec.set_project(project);
    match conn.route::<JobSpec, Job>(&job_spec) {
        Ok(job) => Ok(render_json(status::Created, &job)),
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
    let session = req.extensions.get::<Authenticated>().unwrap();
    let params = &req.extensions.get::<Router>().unwrap();
    let invitation_id = match params.find("invitation_id").unwrap().parse::<u64>() {
        Ok(value) => value,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };

    // TODO: read the body to determine "ignore"
    let ignore_val = false;

    let mut conn = Broker::connect().unwrap();
    let mut request = OriginInvitationAcceptRequest::new();

    // make sure we're not trying to accept someone else's request
    request.set_account_accepting_request(session.get_id());
    request.set_invite_id(invitation_id);
    request.set_ignore(ignore_val);
    match conn.route::<OriginInvitationAcceptRequest, OriginInvitationAcceptResponse>(&request) {
        Ok(_invites) => Ok(Response::with(status::NoContent)),
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
    let (organization, repo): (String, String) = {
        match req.get::<bodyparser::Json>() {
            Ok(Some(body)) => {
                match body.find("origin") {
                    Some(&Value::String(ref val)) => {
                        // JW TODO: check to see if we are a member of the origin
                        origin_get.set_name(val.to_string())
                    }
                    _ => {
                        return Ok(Response::with((status::UnprocessableEntity,
                                                  "Missing required field: `origin`")))
                    }
                }
                match body.find("plan_path") {
                    Some(&Value::String(ref val)) => project.set_plan_path(val.to_string()),
                    _ => {
                        return Ok(Response::with((status::UnprocessableEntity,
                                                  "Missing required field: `plan_path`")))
                    }
                }
                match body.find("github") {
                    Some(&Value::Object(ref map)) => {
                        let mut vcs = VCSGit::new();
                        let organization = match map.get("organization") {
                            Some(&Value::String(ref val)) => val.to_string(),
                            _ => {
                                return Ok(Response::with((status::UnprocessableEntity,
                                                          "Missing required field: \
                                                           `github.organization`")))
                            }
                        };
                        let repo = match map.get("repo") {
                            Some(&Value::String(ref val)) => val.to_string(),
                            _ => {
                                return Ok(Response::with((status::UnprocessableEntity,
                                                          "Missing required field: `github.repo`")))
                            }
                        };
                        match github.repo(&session.get_token(), &organization, &repo) {
                            Ok(repo) => vcs.set_url(repo.clone_url),
                            Err(_) => {
                                return Ok(Response::with((status::UnprocessableEntity, "rg:pc:1")))
                            }
                        }
                        project.set_git(vcs);
                        (organization, repo)
                    }
                    _ => {
                        return Ok(Response::with((status::UnprocessableEntity,
                                                  "Missing required field: `github`")))
                    }
                }
            }
            _ => return Ok(Response::with(status::BadRequest)),
        }
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
            match contents.content.from_base64() {
                Ok(ref bytes) => {
                    match Plan::from_bytes(bytes) {
                        Ok(plan) => project.set_id(format!("{}/{}", origin.get_name(), plan.name)),
                        Err(_) => {
                            return Ok(Response::with((status::UnprocessableEntity, "rg:pc:3")))
                        }
                    }
                }
                Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pc:4"))),
            }
        }
        Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pc:2"))),
    }
    project.set_owner_id(session.get_id());
    request.set_project(project);
    match conn.route::<ProjectCreate, Project>(&request) {
        Ok(response) => Ok(render_json(status::Created, &response)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Delete the given project
pub fn project_delete(req: &mut Request) -> IronResult<Response> {
    let mut project_del = ProjectDelete::new();
    let params = req.extensions.get::<Router>().unwrap();
    {
        let origin = params.find("origin").unwrap();
        // JW TODO: check to see if we are a member of the origin before deleting.
        let name = params.find("name").unwrap();
        project_del.set_id(format!("{}/{}", origin, name));
    }
    let session = req.extensions.get::<Authenticated>().unwrap();
    project_del.set_requestor_id(session.get_id());
    let mut conn = Broker::connect().unwrap();
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
    let (organization, repo): (String, String) = {
        match req.get::<bodyparser::Json>() {
            Ok(Some(body)) => {
                match body.find("plan_path") {
                    Some(&Value::String(ref val)) => project.set_plan_path(val.to_string()),
                    _ => {
                        return Ok(Response::with((status::UnprocessableEntity,
                                                  "Missing required field: `plan_path`")))
                    }
                }
                match body.find("github") {
                    Some(&Value::Object(ref map)) => {
                        let mut vcs = VCSGit::new();
                        let organization = match map.get("organization") {
                            Some(&Value::String(ref val)) => val.to_string(),
                            _ => {
                                return Ok(Response::with((status::UnprocessableEntity,
                                                          "Missing required field: \
                                                           `github.organization`")))
                            }
                        };
                        let repo = match map.get("repo") {
                            Some(&Value::String(ref val)) => val.to_string(),
                            _ => {
                                return Ok(Response::with((status::UnprocessableEntity,
                                                          "Missing required field: \
                                                           `github.repo`")))
                            }
                        };
                        let session = req.extensions.get::<Authenticated>().unwrap();
                        match github.repo(&session.get_token(), &organization, &repo) {
                            Ok(repo) => vcs.set_url(repo.clone_url),
                            Err(_) => {
                                return Ok(Response::with((status::UnprocessableEntity, "rg:pu:1")))
                            }
                        }
                        project.set_git(vcs);
                        (organization, repo)
                    }
                    _ => {
                        return Ok(Response::with((status::UnprocessableEntity,
                                                  "Missing required field: `github`")))
                    }
                }
            }
            _ => return Ok(Response::with(status::BadRequest)),
        }
    };
    let mut conn = Broker::connect().unwrap();
    let session = req.extensions.get::<Authenticated>().unwrap();
    match github.contents(&session.get_token(),
                          &organization,
                          &repo,
                          &project.get_plan_path()) {
        Ok(contents) => {
            match contents.content.from_base64() {
                Ok(ref bytes) => {
                    match Plan::from_bytes(bytes) {
                        Ok(plan) => {
                            let params = req.extensions.get::<Router>().unwrap();
                            let origin = params.find("origin").unwrap();
                            // JW TODO: check to see if we are a member of the origin before
                            // allowing any changes to be made.
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
