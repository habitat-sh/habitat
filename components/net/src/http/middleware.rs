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

use std::env;

use hyper;
use iron::Handler;
use iron::headers::{self, Authorization, Bearer};
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status::Status;
use iron::typemap::Key;
use unicase::UniCase;
use protocol::sessionsrv::*;
use protocol::net::{self, ErrCode};
use serde_json;

use super::net_err_to_http;
use super::super::error::Error;
use super::super::routing::{Broker, BrokerConn};
use super::super::oauth::github::GitHubClient;
use config;
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

#[derive(Clone)]
pub struct Authenticated {
    github: GitHubClient,
    features: FeatureFlags,
}

impl Authenticated {
    pub fn new<T>(config: &T) -> Self
    where
        T: config::GitHubOAuth,
    {
        let github = GitHubClient::new(config);
        Authenticated {
            github: github,
            features: FeatureFlags::empty(),
        }
    }

    pub fn require(mut self, flag: FeatureFlags) -> Self {
        self.features.insert(flag);
        self
    }

    fn authenticate(&self, conn: &mut BrokerConn, token: &str) -> IronResult<Session> {
        let mut request = SessionGet::new();
        request.set_token(token.to_string());
        match conn.route::<SessionGet, Session>(&request) {
            Ok(session) => Ok(session),
            Err(err) => {
                if err.get_code() == ErrCode::SESSION_EXPIRED {
                    let session = session_create(&self.github, token)?;
                    let flags = FeatureFlags::from_bits(session.get_flags()).unwrap();
                    if !flags.contains(self.features) {
                        let err = net::err(ErrCode::ACCESS_DENIED, "net:auth:0");
                        return Err(IronError::new(err, Status::Forbidden));
                    }
                    Ok(session)
                } else {
                    let status = net_err_to_http(err.get_code());
                    let body = itry!(serde_json::to_string(&err));
                    Err(IronError::new(err, (body, status)))
                }
            }
        }
    }
}

impl Key for Authenticated {
    type Value = Session;
}

impl BeforeMiddleware for Authenticated {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let session = {
            match req.headers.get::<Authorization<Bearer>>() {
                Some(&Authorization(Bearer { ref token })) => {
                    match req.extensions.get_mut::<RouteBroker>() {
                        Some(broker) => self.authenticate(broker, token)?,
                        None => {
                            let mut broker = Broker::connect().unwrap();
                            self.authenticate(&mut broker, token)?
                        }
                    }
                }
                _ => {
                    let err = net::err(ErrCode::ACCESS_DENIED, "net:auth:1");
                    return Err(IronError::new(err, Status::Unauthorized));
                }
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
        res.headers.set(headers::AccessControlAllowHeaders(vec![
            UniCase("authorization".to_string()),
            UniCase("range".to_string()),
        ]));
        res.headers.set(headers::AccessControlAllowMethods(
            vec![Method::Put, Method::Delete],
        ));
        Ok(res)
    }
}

pub fn session_create(github: &GitHubClient, token: &str) -> IronResult<Session> {
    if env::var_os("HAB_FUNC_TEST").is_some() {
        let request = match token {
            "bobo" => {
                let mut request = SessionCreate::new();
                request.set_token(token.to_string());
                request.set_extern_id(0);
                request.set_email("bobo@example.com".to_string());
                request.set_name("bobo".to_string());
                request.set_provider(OAuthProvider::GitHub);
                request
            }
            "logan" => {
                let mut request = SessionCreate::new();
                request.set_token(token.to_string());
                request.set_extern_id(1);
                request.set_email("logan@example.com".to_string());
                request.set_name("logan".to_string());
                request.set_provider(OAuthProvider::GitHub);
                request
            }
            user => {
                panic!(
                    "You need to define the stub user {} during HAB_FUNC_TEST",
                    user
                )
            }
        };
        let mut conn = Broker::connect().unwrap();
        match conn.route::<SessionCreate, Session>(&request) {
            Ok(session) => return Ok(session),
            Err(err) => {
                let body = itry!(serde_json::to_string(&err));
                let status = net_err_to_http(err.get_code());
                return Err(IronError::new(err, (body, status)));
            }
        }
    }
    match github.user(&token) {
        Ok(user) => {
            // Select primary email. If no primary email can be found, use any email. If
            // no email is associated with account return an access denied error.
            let email = match github.emails(&token) {
                Ok(ref emails) => {
                    emails
                        .iter()
                        .find(|e| e.primary)
                        .unwrap_or(&emails[0])
                        .email
                        .clone()
                }
                Err(_) => {
                    let err = net::err(ErrCode::ACCESS_DENIED, "net:session-create:0");
                    let status = net_err_to_http(err.get_code());
                    let body = itry!(serde_json::to_string(&err));
                    return Err(IronError::new(err, (body, status)));
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
                    let body = itry!(serde_json::to_string(&err));
                    let status = net_err_to_http(err.get_code());
                    Err(IronError::new(err, (body, status)))
                }
            }
        }
        Err(Error::GitHubAPI(hyper::status::StatusCode::Unauthorized, _)) => {
            let err = net::err(ErrCode::ACCESS_DENIED, "net:session-create:1");
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
        Err(e @ Error::GitHubAPI(_, _)) => {
            warn!("Unexpected response from GitHub, {:?}", e);
            let err = net::err(ErrCode::BAD_REMOTE_REPLY, "net:session-create:2");
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
        Err(e @ Error::Json(_)) => {
            warn!("Bad response body from GitHub, {:?}", e);
            let err = net::err(ErrCode::BAD_REMOTE_REPLY, "net:session-create:3");
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
        Err(e) => {
            error!("Unexpected error, err={:?}", e);
            let err = net::err(ErrCode::BUG, "net:session-create:4");
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
    }
}
