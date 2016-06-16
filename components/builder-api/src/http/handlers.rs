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

use std::result;

use hab_net;
use hab_net::routing::Broker;
use hab_net::oauth::github::GitHubClient;
use iron::prelude::*;
use iron::status;
use iron::headers::{Authorization, Bearer};
use protobuf;
use protocol::jobsrv::{Job, JobCreate, JobGet};
use protocol::sessionsrv::{OAuthProvider, Session, SessionCreate, SessionGet};
use protocol::vault::*;
use protocol::net::{self, NetError, ErrCode};
use router::Router;
use rustc_serialize::json::{self, ToJson};

use super::super::server::ZMQ_CONTEXT;

pub fn authenticate(req: &mut Request) -> result::Result<Session, Response> {
    match req.headers.get::<Authorization<Bearer>>() {
        Some(&Authorization(Bearer { ref token })) => {
            let mut conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();
            let mut request = SessionGet::new();
            request.set_token(token.to_string());
            conn.route(&request).unwrap();
            match conn.recv() {
                Ok(rep) => {
                    match rep.get_message_id() {
                        "Session" => {
                            let session = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                            Ok(session)
                        }
                        "NetError" => {
                            let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                            Err(render_net_error(&err))
                        }

                        _ => unreachable!("unexpected msg: {:?}", rep),
                    }
                }
                Err(e) => {
                    error!("session get, err={:?}", e);
                    Err(Response::with(status::InternalServerError))
                }
            }
        }
        _ => Err(Response::with(status::Unauthorized)),
    }
}

pub fn session_create(req: &mut Request, github: &GitHubClient) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let code = match params.find("code") {
        Some(code) => code,
        _ => return Ok(Response::with(status::BadRequest)),
    };
    match github.authenticate(code) {
        Ok(token) => {
            match github.user(&token) {
                Ok(user) => {
                    let mut conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();
                    let mut request = SessionCreate::new();
                    request.set_token(token.to_string());
                    request.set_extern_id(user.id);
                    if let Some(email) = user.email {
                        request.set_email(email);
                    }
                    request.set_name(user.login);
                    request.set_provider(OAuthProvider::GitHub);
                    conn.route(&request).unwrap();
                    match conn.recv() {
                        Ok(rep) => {
                            match rep.get_message_id() {
                                "Session" => {
                                    let token: Session = protobuf::parse_from_bytes(rep.get_body())
                                        .unwrap();
                                    let encoded = json::encode(&token.to_json()).unwrap();
                                    Ok(Response::with((status::Ok, encoded)))
                                }
                                "NetError" => {
                                    let err: NetError = protobuf::parse_from_bytes(rep.get_body())
                                        .unwrap();
                                    Ok(render_net_error(&err))
                                }
                                _ => unreachable!("unexpected msg: {:?}", rep),
                            }
                        }
                        Err(e) => {
                            error!("{:?}", e);
                            Ok(Response::with(status::ServiceUnavailable))
                        }
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
    let session = match authenticate(req) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };
    let mut conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();
    let mut request = JobCreate::new();
    request.set_owner_id(session.get_id());
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "Job" => {
                    let job: Job = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    let encoded = json::encode(&job.to_json()).unwrap();
                    Ok(Response::with((status::Created, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}

pub fn job_show(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let id = match params.find("id") {
        Some(id) => {
            match id.parse() {
                Ok(id) => id,
                Err(_) => return Ok(Response::with(status::BadRequest)),
            }
        }
        _ => return Ok(Response::with(status::BadRequest)),
    };
    let mut conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();
    let mut request = JobGet::new();
    request.set_id(id);
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "Job" => {
                    let job: Job = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    let encoded = json::encode(&job.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}

/// Endpoint for determining availability of builder-api components.
///
/// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
pub fn status(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

/// Return an IronResult containing the body of a NetError and the appropriate HTTP response status
/// for the corresponding NetError.
///
/// For example, a NetError::ENTITY_NOT_FOUND will result in an HTTP response containing the body
/// of the NetError with an HTTP status of 404.
///
/// # Panics
///
/// * The given encoded message was not a NetError
/// * The given messsage could not be decoded
/// * The NetError could not be encoded to JSON
fn render_net_error(err: &NetError) -> Response {
    let encoded = json::encode(&err.to_json()).unwrap();
    let status = match err.get_code() {
        ErrCode::ENTITY_NOT_FOUND => status::NotFound,
        ErrCode::ENTITY_CONFLICT => status::Conflict,
        ErrCode::NO_SHARD => status::ServiceUnavailable,
        ErrCode::TIMEOUT => status::RequestTimeout,
        ErrCode::BAD_REMOTE_REPLY => status::BadGateway,
        ErrCode::SESSION_EXPIRED => status::Unauthorized,
        _ => status::InternalServerError,
    };
    Response::with((status, encoded))
}

pub fn list_account_invitations(req: &mut Request) -> IronResult<Response> {
    debug!("list_account_invitations");
    let session = match authenticate(req) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };

    let mut conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();
    let mut request = AccountInvitationListRequest::new();
    request.set_account_id(session.get_id());
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "AccountInvitationListResponse" => {
                    let invites: AccountInvitationListResponse =
                        protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    let encoded = json::encode(&invites.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}

pub fn list_user_origins(req: &mut Request) -> IronResult<Response> {
    debug!("list_user_origins");
    let session = match authenticate(req) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };

    let mut conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();

    let mut request = AccountOriginListRequest::new();
    request.set_account_id(session.get_id());
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "AccountOriginListResponse" => {
                    let invites: AccountOriginListResponse =
                        protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    let encoded = json::encode(&invites.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}

pub fn accept_invitation(req: &mut Request) -> IronResult<Response> {
    debug!("accept_invitation");
    let session = match authenticate(req) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };
    let params = &req.extensions.get::<Router>().unwrap();

    let invitation_id = match params.find("invitation_id") {
        Some(ref invitation_id) => {
            match invitation_id.parse::<u64>() {
                Ok(v) => v,
                Err(_) => return Ok(Response::with(status::BadRequest)),
            }
        }
        None => return Ok(Response::with(status::BadRequest)),
    };

    // TODO: read the body to determine "ignore"
    let ignore_val = false;

    let mut conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();
    let mut request = OriginInvitationAcceptRequest::new();

    // make sure we're not trying to accept someone else's request
    request.set_account_accepting_request(session.get_id());
    request.set_invite_id(invitation_id);
    request.set_ignore(ignore_val);

    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "OriginInvitationAcceptResponse" => {
                    let _invites: OriginInvitationAcceptResponse =
                        protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    // empty response
                    Ok(Response::with(status::Ok))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}
