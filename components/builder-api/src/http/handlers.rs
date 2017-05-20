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

use std::env;

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
use params::{Params, Value, FromValue};
use persistent;
use protocol::jobsrv::{Job, JobGet, JobLogGet, JobLog, JobSpec, ProjectJobsGet,
                       ProjectJobsGetResponse};
use protocol::originsrv::*;
use protocol::sessionsrv;
use protocol::net::{self, NetOk, ErrCode};
use router::Router;

// For the initial release, Builder will only be enabled on the "core"
// origin. Later, we'll roll it out to other origins; at that point,
// we should consider other options, such as configurable middleware.
const BUILDER_ENABLED_ORIGIN: &'static str = "core";

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

    if env::var_os("HAB_FUNC_TEST").is_some() {
        let session = try!(session_create(&github, &code));

        log_event!(req,
                   Event::GithubAuthenticate {
                       user: session.get_name().to_string(),
                       account: session.get_id().to_string(),
                   });

        return Ok(render_json(status::Ok, &session));
    }

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
    let mut project_get = OriginProjectGet::new();
    {
        match req.get::<bodyparser::Struct<JobCreateReq>>() {
            Ok(Some(body)) => project_get.set_name(body.project_id),
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    // TODO: SA - Eliminate need to clone the session
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let mut conn = Broker::connect().unwrap();
    let project = match conn.route::<OriginProjectGet, OriginProject>(&project_get) {
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

pub fn job_log(req: &mut Request) -> IronResult<Response> {
    let start = {
        let params = req.get_ref::<Params>().unwrap();
        match params.find(&["start"]) {
            Some(&Value::String(ref val)) => {
                match val.parse::<u64>() {
                    Ok(num) => num,
                    Err(e) => {
                        debug!("Tried to parse 'start' parameter as a number but failed: {:?}",
                               e);
                        return Ok(Response::with(status::BadRequest));
                    }
                }
            }
            _ => 0,
        }
    };

    let include_color = req.get_ref::<Params>()
        .unwrap()
        .find(&["color"])
        .and_then(FromValue::from_value)
        .unwrap_or(false);

    let params = req.extensions.get::<Router>().unwrap();
    let id = match params.find("id").unwrap().parse::<u64>() {
        Ok(id) => id,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    let mut conn = Broker::connect().unwrap();

    let mut request = JobLogGet::new();
    request.set_id(id);
    request.set_start(start);

    match conn.route::<JobLogGet, JobLog>(&request) {
        Ok(mut log) => {
            if !include_color {
                log.strip_ansi();
            }
            Ok(render_json(status::Ok, &log))
        }
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
    let mut request = sessionsrv::AccountInvitationListRequest::new();
    request.set_account_id(session.get_id());
    match conn.route::<sessionsrv::AccountInvitationListRequest, sessionsrv::AccountInvitationListResponse>(&request) {
        Ok(invites) => Ok(render_json(status::Ok, &invites)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn list_user_origins(req: &mut Request) -> IronResult<Response> {
    let session = req.extensions.get::<Authenticated>().unwrap();
    let mut conn = Broker::connect().unwrap();
    let mut request = sessionsrv::AccountOriginListRequest::new();
    request.set_account_id(session.get_id());
    match conn.route::<sessionsrv::AccountOriginListRequest, sessionsrv::AccountOriginListResponse>(&request) {
        Ok(invites) => Ok(render_json(status::Ok, &invites)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Create a new project as the authenticated user and associated to
/// the given origin.
///
/// NOTE: This currently only allows creation of projects in the
/// "core" origin.
pub fn project_create(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginProjectCreate::new();
    let mut project = OriginProject::new();
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
            origin_get.set_name(body.origin);
            project.set_plan_path(body.plan_path);
            project.set_vcs_type(String::from("git"));
            match github.repo(&session.get_token(),
                              &body.github.organization,
                              &body.github.repo) {
                Ok(repo) => project.set_vcs_data(repo.clone_url),
                Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pc:1"))),
            }
            (body.github.organization, body.github.repo)
        }
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    };
    let mut conn = Broker::connect().unwrap();
    let origin = match conn.route::<OriginGet, Origin>(&origin_get) {
        Ok(response) => response,
        Err(err) => return Ok(render_net_error(&err)),
    };

    // Only allow projects to be created for the core origin initially.
    if origin.get_name() != BUILDER_ENABLED_ORIGIN {
        return Ok(Response::with((status::UnprocessableEntity, "rg:pc:5")));
    }

    match github.contents(&session.get_token(),
                          &organization,
                          &repo,
                          &project.get_plan_path()) {
        Ok(contents) => {
            match base64::decode(&contents.content) {
                Ok(ref bytes) => {
                    match Plan::from_bytes(bytes) {
                        Ok(plan) => {
                            project.set_origin_name(String::from(origin.get_name()));
                            project.set_origin_id(origin.get_id());
                            project.set_package_name(String::from(plan.name));
                        }
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
    match conn.route::<OriginProjectCreate, OriginProject>(&request) {
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
    let mut project_del = OriginProjectDelete::new();

    let (session_id, origin) = {
        let session = req.extensions.get::<Authenticated>().unwrap();
        let session_id = session.get_id();

        let params = req.extensions.get::<Router>().unwrap();
        let origin = params.find("origin").unwrap().to_owned();
        let name = params.find("name").unwrap();

        // We're only allowing projects to be created for the core
        // origin initially. Thus, if we try to delete a project for any
        // other origin, we can safely short-circuit processing.
        if origin != BUILDER_ENABLED_ORIGIN {
            return Ok(Response::with((status::NotFound, "rg:pd:1")));
        }

        project_del.set_name(format!("{}/{}", origin, name));
        (session_id, origin)
    };

    if !try!(check_origin_access(req, session_id, origin)) {
        return Ok(Response::with(status::Forbidden));
    }

    project_del.set_requestor_id(session_id);
    let mut conn = Broker::connect().unwrap();
    match conn.route::<OriginProjectDelete, NetOk>(&project_del) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Update the given project
pub fn project_update(req: &mut Request) -> IronResult<Response> {

    let (name, origin) = {
        let params = req.extensions.get::<Router>().unwrap();
        let origin = params.find("origin").unwrap().to_owned();
        let name = params.find("name").unwrap().to_owned();

        // We're only allowing projects to be created for the core
        // origin initially. Thus, if we try to update a project for
        // any other origin, we can safely short-circuit processing.
        if origin != BUILDER_ENABLED_ORIGIN {
            return Ok(Response::with((status::NotFound, "rg:pu:6")));
        }
        (name, origin)
    };

    let mut request = OriginProjectUpdate::new();
    let mut project = OriginProject::new();
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();

    let (session_token, session_id) = {
        let session = req.extensions.get::<Authenticated>().unwrap();
        let session_id = session.get_id();
        let session_token = session.get_token().to_string();

        (session_token, session_id)
    };

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
            project.set_vcs_type(String::from("git"));
            project.set_plan_path(body.plan_path);
            match github.repo(&session_token, &body.github.organization, &body.github.repo) {
                Ok(repo) => project.set_vcs_data(repo.clone_url),
                Err(_) => return Ok(Response::with((status::UnprocessableEntity, "rg:pu:1"))),
            }
            (body.github.organization, body.github.repo)
        }
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    };
    let mut conn = Broker::connect().unwrap();
    match github.contents(&session_token,
                          &organization,
                          &repo,
                          &project.get_plan_path()) {
        Ok(contents) => {
            match base64::decode(&contents.content) {
                Ok(ref bytes) => {
                    match Plan::from_bytes(bytes) {
                        Ok(plan) => {
                            if !try!(check_origin_access(req, session_id, &origin)) {
                                return Ok(Response::with(status::Forbidden));
                            }
                            if plan.name != name {
                                return Ok(Response::with((status::UnprocessableEntity, "rg:pu:2")));
                            }
                            project.set_origin_name(String::from(origin));
                            project.set_package_name(String::from(name));
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
    project.set_owner_id(session_id);
    request.set_requestor_id(session_id);
    request.set_project(project);
    match conn.route::<OriginProjectUpdate, NetOk>(&request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Display the the given project's details
pub fn project_show(req: &mut Request) -> IronResult<Response> {
    let mut project_get = OriginProjectGet::new();
    let params = req.extensions.get::<Router>().unwrap();
    {
        let origin = params.find("origin").unwrap();

        // We're only allowing projects to be created for the core
        // origin initially. Thus, if we try to get a project for any
        // other origin, we can safely short-circuit processing.
        if origin != BUILDER_ENABLED_ORIGIN {
            return Ok(Response::with((status::NotFound, "rg:ps:1")));
        }

        let name = params.find("name").unwrap();
        project_get.set_name(format!("{}/{}", origin, name));
    }
    let mut conn = Broker::connect().unwrap();
    match conn.route::<OriginProjectGet, OriginProject>(&project_get) {
        Ok(project) => Ok(render_json(status::Ok, &project)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

/// Retrieve the most recent 50 jobs for a project.
///
/// Later, we'll add more options for pagination, sorting, filtering,
/// etc.
pub fn project_jobs(req: &mut Request) -> IronResult<Response> {
    let mut jobs_get = ProjectJobsGet::new();
    let params = req.extensions.get::<Router>().unwrap();
    {
        let origin = params.find("origin").unwrap();

        // We're only allowing projects to be created for the core
        // origin initially. Thus, if we try to get jobs for any
        // project in another, we can safely short-circuit processing.
        if origin != BUILDER_ENABLED_ORIGIN {
            return Ok(Response::with((status::NotFound, "rg:pj:1")));
        }

        let name = params.find("name").unwrap();
        jobs_get.set_name(format!("{}/{}", origin, name));
    }
    let mut conn = Broker::connect().unwrap();
    match conn.route::<ProjectJobsGet, ProjectJobsGetResponse>(&jobs_get) {
        Ok(jobs) => Ok(render_json(status::Ok, &jobs)),
        Err(err) => Ok(render_net_error(&err)),
    }
}
