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

use hab_net;
use hab_net::routing::{Broker, BrokerConn};
use hab_net::oauth::github::GitHubClient;
use iron::headers::{self, Authorization, Bearer};
use iron::method::Method;
use iron::middleware::{AfterMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use unicase::UniCase;
use persistent;
use protocol::sessionsrv::*;
use protocol::net::{self, ErrCode};
use rustc_serialize::json::{self, ToJson};

use super::super::server::ZMQ_CONTEXT;
use super::GitHubCli;
use error::Error;

pub struct RouteBroker;

impl Key for RouteBroker {
    type Value = BrokerConn;
}

impl BeforeMiddleware for RouteBroker {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();
        req.extensions.insert::<RouteBroker>(conn);
        Ok(())
    }
}

pub struct Authenticated;

impl Key for Authenticated {
    type Value = Session;
}

impl BeforeMiddleware for Authenticated {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
        let session = {
            match req.headers.get::<Authorization<Bearer>>() {
                Some(&Authorization(Bearer { ref token })) => {
                    let mut conn = req.extensions.get_mut::<RouteBroker>().unwrap();
                    let mut request = SessionGet::new();
                    request.set_token(token.to_string());
                    match conn.route::<SessionGet, Session>(&request) {
                        Ok(session) => session,
                        Err(err) => {
                            if err.get_code() == ErrCode::SESSION_EXPIRED {
                                try!(session_create(&github, token))
                            } else {
                                let encoded = json::encode(&err.to_json()).unwrap();
                                return Err(IronError::new(Error::Authorization,
                                                          (encoded, status::Unauthorized)));
                            }
                        }
                    }
                }
                _ => return Err(IronError::new(Error::Authorization, status::Unauthorized)),
            }
        };
        req.extensions.insert::<Self>(session);
        Ok(())
    }
}

pub struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers
            .set(headers::AccessControlAllowHeaders(vec![UniCase("authorization".to_owned())]));
        res.headers
            .set(headers::AccessControlAllowMethods(vec![Method::Put, Method::Delete]));
        Ok(res)
    }
}

fn session_create(github: &GitHubClient, token: &str) -> IronResult<Session> {
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
                    let encoded = json::encode(&err.to_json()).unwrap();
                    return Err(IronError::new(Error::Authorization,
                                              (encoded, status::Unauthorized)));
                }
            };
            let mut conn = Broker::connect(&**ZMQ_CONTEXT).unwrap();
            let mut request = SessionCreate::new();
            request.set_token(token.to_string());
            request.set_extern_id(user.id);
            request.set_email(email);
            request.set_name(user.login);
            request.set_provider(OAuthProvider::GitHub);
            match conn.route::<SessionCreate, Session>(&request) {
                Ok(session) => Ok(session),
                Err(err) => {
                    let encoded = json::encode(&err.to_json()).unwrap();
                    Err(IronError::new(Error::Authorization, (encoded, status::Unauthorized)))
                }
            }
        }
        Err(e @ hab_net::Error::JsonDecode(_)) => {
            debug!("github user get, err={:?}", e);
            let err = net::err(ErrCode::BAD_REMOTE_REPLY, "rg:auth:1");
            let encoded = json::encode(&err.to_json()).unwrap();
            Err(IronError::new(Error::Authorization, (encoded, status::Unauthorized)))
        }
        Err(e) => {
            debug!("github user get, err={:?}", e);
            let err = net::err(ErrCode::BUG, "rg:auth:2");
            let encoded = json::encode(&err.to_json()).unwrap();
            Err(IronError::new(Error::Authorization, (encoded, status::Unauthorized)))
        }
    }
}
