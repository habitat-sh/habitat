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

use iron::Handler;
use iron::headers::{self, Authorization, Bearer};
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use unicase::UniCase;
use persistent;
use protocol::sessionsrv::*;
use protocol::net::{self, ErrCode};
use rustc_serialize::json::{self, ToJson};

use super::super::error::Error;
use super::super::routing::{Broker, BrokerConn};
use super::super::oauth::github::GitHubClient;
use super::HttpError;
use privilege::FeatureFlags;

/// Wrapper around the standard `iron::Chain` to assist in adding middleware on a per-handler basis
pub struct XHandler(Chain);

impl XHandler {
    /// Create a new XHandler
    pub fn new<H: Handler>(handler: H) -> Self {
        XHandler(Chain::new(handler))
    }

    /// Add one or more before-middleware to the handler's chain
    pub fn before<M: BeforeMiddleware>(mut self, middleware: M) -> Self {
        self.0.link_before(middleware);
        self
    }

    /// Add one or more after-middleware to the handler's chain
    pub fn after<M: AfterMiddleware>(mut self, middleware: M) -> Self {
        self.0.link_after(middleware);
        self
    }

    /// Ad one or more around-middleware to the handler's chain
    pub fn around<M: AroundMiddleware>(mut self, middleware: M) -> Self {
        self.0.link_around(middleware);
        self
    }
}

impl Handler for XHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.0.handle(req)
    }
}

pub struct GitHubCli;

impl Key for GitHubCli {
    type Value = GitHubClient;
}

pub struct RouteBroker;

impl Key for RouteBroker {
    type Value = BrokerConn;
}

impl BeforeMiddleware for RouteBroker {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let conn = Broker::connect().unwrap();
        req.extensions.insert::<RouteBroker>(conn);
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Authenticated(FeatureFlags);

impl Authenticated {
    pub fn require(mut self, flag: FeatureFlags) -> Self {
        self.0.insert(flag);
        self
    }
}

impl Key for Authenticated {
    type Value = Session;
}

impl Default for Authenticated {
    fn default() -> Self {
        Authenticated(FeatureFlags::empty())
    }
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
                                return Err(IronError::new(HttpError::Authorization,
                                                          (encoded, status::Unauthorized)));
                            }
                        }
                    }
                }
                _ => return Err(IronError::new(HttpError::Authorization, status::Unauthorized)),
            }
        };
        let flags = FeatureFlags::from_bits(session.get_flags()).unwrap();
        if !flags.contains(self.0) {
            return Err(IronError::new(HttpError::Authorization, status::Forbidden));
        }
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
                    return Err(IronError::new(HttpError::Authorization,
                                              (encoded, status::Unauthorized)));
                }
            };
            let mut conn = Broker::connect().unwrap();
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
                    Err(IronError::new(HttpError::Authorization, (encoded, status::Unauthorized)))
                }
            }
        }
        Err(e @ Error::JsonDecode(_)) => {
            debug!("github user get, err={:?}", e);
            let err = net::err(ErrCode::BAD_REMOTE_REPLY, "rg:auth:1");
            let encoded = json::encode(&err.to_json()).unwrap();
            Err(IronError::new(HttpError::Authorization, (encoded, status::Unauthorized)))
        }
        Err(e) => {
            debug!("github user get, err={:?}", e);
            let err = net::err(ErrCode::BUG, "rg:auth:2");
            let encoded = json::encode(&err.to_json()).unwrap();
            Err(IronError::new(HttpError::Authorization, (encoded, status::Unauthorized)))
        }
    }
}
