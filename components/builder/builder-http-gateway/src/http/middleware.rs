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

use base64;
use bldr_core;
use github_api_client::{GitHubCfg, GitHubClient, HubError};
use hab_net::{ErrCode, NetError};
use hab_net::conn::RouteClient;
use hab_net::privilege::FeatureFlags;
use hyper;
use iron::Handler;
use iron::headers::{self, Authorization, Bearer};
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status::Status;
use iron::typemap::Key;
use persistent;
use protocol::message;
use protocol::sessionsrv::*;
use segment_api_client::SegmentClient;
use serde_json;
use std::path::PathBuf;
use unicase::UniCase;

use super::net_err_to_http;
use conn::RouteBroker;

/// Wrapper around the standard `iron::Chain` to assist in adding middleware on a per-handler basis
pub struct XHandler(Chain);

impl XHandler {
    /// Create a new XHandler
    pub fn new<H>(handler: H) -> Self
    where
        H: Handler,
    {
        XHandler(Chain::new(handler))
    }

    /// Add one or more before-middleware to the handler's chain
    pub fn before<M>(mut self, middleware: M) -> Self
    where
        M: BeforeMiddleware,
    {
        self.0.link_before(middleware);
        self
    }

    /// Add one or more after-middleware to the handler's chain
    pub fn after<M>(mut self, middleware: M) -> Self
    where
        M: AfterMiddleware,
    {
        self.0.link_after(middleware);
        self
    }

    /// Ad one or more around-middleware to the handler's chain
    pub fn around<M>(mut self, middleware: M) -> Self
    where
        M: AroundMiddleware,
    {
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

pub struct SegmentCli;

impl Key for SegmentCli {
    type Value = SegmentClient;
}

pub struct XRouteClient;
impl Key for XRouteClient {
    type Value = RouteClient;
}

impl BeforeMiddleware for XRouteClient {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let conn = RouteBroker::connect().unwrap();
        req.extensions.insert::<XRouteClient>(conn);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Authenticated {
    github: GitHubClient,
    features: FeatureFlags,
    key_dir: PathBuf,
    optional: bool,
}

impl Authenticated {
    pub fn new(config: GitHubCfg, key_dir: PathBuf) -> Self {
        let github = GitHubClient::new(config);
        Authenticated {
            github: github,
            features: FeatureFlags::empty(),
            key_dir: key_dir,
            optional: false,
        }
    }

    pub fn require(mut self, flag: FeatureFlags) -> Self {
        self.features.insert(flag);
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    fn authenticate(&self, req: &mut Request, token: SessionToken) -> IronResult<Session> {
        let mut request = SessionGet::new();
        request.set_token(token);
        let conn = req.extensions.get_mut::<XRouteClient>().unwrap();
        match conn.route::<SessionGet, Session>(&request) {
            Ok(session) => {
                self.validate_session(&session)?;
                Ok(session)
            }
            Err(err) => {
                let status = net_err_to_http(err.get_code());
                let body = itry!(serde_json::to_string(&err));
                Err(IronError::new(err, (body, status)))
            }
        }
    }

    fn validate_session(&self, session: &Session) -> IronResult<()> {
        let flags = FeatureFlags::from_bits(session.get_flags()).unwrap();
        if !flags.contains(self.features) {
            let err = NetError::new(ErrCode::ACCESS_DENIED, "net:auth:2");
            return Err(IronError::new(err, Status::Forbidden));
        }
        Ok(())
    }
}

impl Key for Authenticated {
    type Value = Session;
}

impl BeforeMiddleware for Authenticated {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let token = match req.headers.get::<Authorization<Bearer>>() {
            Some(&Authorization(Bearer { ref token })) => token.to_owned(),
            _ => {
                if self.optional {
                    return Ok(());
                } else {
                    let err = NetError::new(ErrCode::ACCESS_DENIED, "net:auth:1");
                    return Err(IronError::new(err, Status::Unauthorized));
                }
            }
        };

        let session = {
            // Handle Builder token
            if bldr_core::keys::is_bldr_token(&token) {
                if bldr_core::keys::validate_bldr_token(&self.key_dir, &token).is_ok() {
                    match session_get_builder(req, token.clone()) {
                        Ok(Some(sess)) => sess,
                        _ => session_create_builder(req, token)?,
                    }
                } else {
                    let err = NetError::new(ErrCode::BAD_TOKEN, "net:auth:4");
                    return Err(IronError::new(err, Status::Forbidden));
                }
            } else {
                if let Ok(decoded_token) = base64::decode(&token) {
                    if let Ok(token) = message::decode(&decoded_token) {
                        self.authenticate(req, token)?
                    } else {
                        // TODO: Replace temporary auth workaround
                        // We got a bearer token that is not a valid session token.
                        // Check to see if this is a valid github token, and create (or
                        // update) a session. This is a temporary fix until we can roll out
                        // and migrate clients to our own personal access tokens.
                        session_create_github(req, token)?
                    }
                } else {
                    let err = NetError::new(ErrCode::BAD_TOKEN, "net:auth:3");
                    return Err(IronError::new(err, Status::Forbidden));
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
            vec![Method::Put, Method::Delete, Method::Patch],
        ));
        res.headers.set(headers::AccessControlExposeHeaders(
            vec![UniCase("content-disposition".to_string())],
        ));
        Ok(res)
    }
}

pub fn session_create_github(req: &mut Request, token: String) -> IronResult<Session> {
    let github = req.get::<persistent::Read<GitHubCli>>().unwrap();
    let conn = req.extensions.get_mut::<XRouteClient>().expect(
        "no XRouteClient extension in request",
    );
    match github.user(&token) {
        Ok(user) => {
            let mut request = SessionCreate::new();
            request.set_session_type(SessionType::User);
            request.set_token(token);
            request.set_extern_id(user.id);
            request.set_name(user.login);
            request.set_provider(OAuthProvider::GitHub);
            if let Some(email) = user.email {
                request.set_email(email);
            }
            match conn.route::<SessionCreate, Session>(&request) {
                Ok(session) => Ok(session),
                Err(err) => {
                    let body = itry!(serde_json::to_string(&err));
                    let status = net_err_to_http(err.get_code());
                    Err(IronError::new(err, (body, status)))
                }
            }
        }
        Err(HubError::ApiError(hyper::status::StatusCode::Unauthorized, _)) => {
            let err = NetError::new(ErrCode::ACCESS_DENIED, "net:session-create:1");
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
        Err(e @ HubError::ApiError(_, _)) => {
            warn!("Unexpected response from GitHub, {:?}", e);
            let err = NetError::new(ErrCode::BAD_REMOTE_REPLY, "net:session-create:2");
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
        Err(e @ HubError::Serialization(_)) => {
            warn!("Bad response body from GitHub, {:?}", e);
            let err = NetError::new(ErrCode::BAD_REMOTE_REPLY, "net:session-create:3");
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
        Err(e) => {
            error!("Unexpected error, err={:?}", e);
            let err = NetError::new(ErrCode::BUG, "net:session-create:4");
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
    }
}

pub fn session_get_builder(req: &mut Request, token: String) -> IronResult<Option<Session>> {
    let mut session_token = SessionToken::new();
    session_token.set_account_id(bldr_core::keys::BUILDER_ACCOUNT_ID);
    session_token.set_extern_id(bldr_core::keys::BUILDER_EXTERN_ID);
    session_token.set_provider(OAuthProvider::None);
    session_token.set_token(token.into_bytes());

    let mut request = SessionGet::new();
    request.set_token(session_token);

    let conn = req.extensions.get_mut::<XRouteClient>().unwrap();
    match conn.route::<SessionGet, Session>(&request) {
        Ok(session) => Ok(Some(session)),
        Err(err) => {
            let status = net_err_to_http(err.get_code());
            let body = itry!(serde_json::to_string(&err));
            Err(IronError::new(err, (body, status)))
        }
    }
}

pub fn session_create_builder(req: &mut Request, token: String) -> IronResult<Session> {
    let conn = req.extensions.get_mut::<XRouteClient>().expect(
        "no XRouteClient extension in request",
    );
    let request = {
        let mut request = SessionCreate::new();
        request.set_session_type(SessionType::Builder);
        request.set_token(token);
        request.set_email(bldr_core::keys::BUILDER_ACCOUNT_EMAIL.to_string());
        request.set_name(bldr_core::keys::BUILDER_ACCOUNT_NAME.to_string());
        request.set_provider(OAuthProvider::None);
        request.set_extern_id(0);
        request
    };

    match conn.route::<SessionCreate, Session>(&request) {
        Ok(session) => {
            return Ok(session);
        }
        Err(err) => {
            let body = itry!(serde_json::to_string(&err));
            let status = net_err_to_http(err.get_code());
            return Err(IronError::new(err, (body, status)));
        }
    }
}

pub fn session_create_short_circuit(req: &mut Request, token: &str) -> IronResult<Session> {
    let conn = req.extensions.get_mut::<XRouteClient>().expect(
        "no XRouteClient extension in request",
    );
    let request = match token.as_ref() {
        "bobo" => {
            let mut request = SessionCreate::new();
            request.set_session_type(SessionType::User);
            request.set_extern_id(0);
            request.set_email("bobo@example.com".to_string());
            request.set_name("bobo".to_string());
            request.set_provider(OAuthProvider::GitHub);
            request
        }
        "logan" => {
            let mut request = SessionCreate::new();
            request.set_session_type(SessionType::User);
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
    match conn.route::<SessionCreate, Session>(&request) {
        Ok(session) => return Ok(session),
        Err(err) => {
            let body = itry!(serde_json::to_string(&err));
            let status = net_err_to_http(err.get_code());
            return Err(IronError::new(err, (body, status)));
        }
    }
}
