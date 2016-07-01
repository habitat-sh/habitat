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

//! A module containing the HTTP server and handlers for servicing client requests

pub mod handlers;
pub mod middleware;

use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use depot;
use hab_net::oauth::github::GitHubClient;
use iron::Handler;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::Protocol;
use iron::typemap;
use mount::Mount;
use persistent;
use staticfile::Static;

use super::server::ZMQ_CONTEXT;
use config::Config;
use error::Result;
use self::handlers::*;
use self::middleware::*;

// Iron defaults to a threadpool of size `8 * num_cpus`.
// See: http://172.16.2.131:9633/iron/prelude/struct.Iron.html#method.http
const HTTP_THREAD_COUNT: usize = 128;

/// Wrapper around the standard `iron::Chain` to assist in adding middleware on a per-handler basis
pub struct XHandler(Chain);

impl XHandler {
    /// Create a new XHandler
    pub fn new<H: Handler>(handler: H) -> Self {
        XHandler(Chain::new(handler))
    }

    /// Add one or more before-middleware to the handler's chain
    pub fn before<M: BeforeMiddleware>(mut self, middleware: Vec<M>) -> Self {
        for m in middleware.into_iter() {
            self.0.link_before(m);
        }
        self
    }

    /// Add one or more after-middleware to the handler's chain
    pub fn after<M: AfterMiddleware>(mut self, middleware: Vec<M>) -> Self {
        for m in middleware.into_iter() {
            self.0.link_after(m);
        }
        self
    }

    /// Ad one or more around-middleware to the handler's chain
    pub fn around<M: AroundMiddleware>(mut self, middleware: Vec<M>) -> Self {
        for m in middleware.into_iter() {
            self.0.link_around(m);
        }
        self
    }
}

impl Handler for XHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.0.handle(req)
    }
}

pub struct GitHubCli;

impl typemap::Key for GitHubCli {
    type Value = GitHubClient;
}

/// Create a new `iron::Chain` containing a Router and it's required middleware
pub fn router(config: Arc<Config>) -> Result<Chain> {
    let router = router!(
        get "/status" => status,
        get "/authenticate/:code" => session_create,

        post "/jobs" => XHandler::new(job_create).before(vec![Authenticated]),
        get "/jobs/:id" => job_show,

        get "/user/invitations" => {
            XHandler::new(list_account_invitations).before(vec![Authenticated])
        },
        put "/user/invitations/:invitation_id" => {
            XHandler::new(accept_invitation).before(vec![Authenticated])
        },
        get "/user/origins" => XHandler::new(list_user_origins).before(vec![Authenticated]),

        post "/projects" => XHandler::new(project_create).before(vec![Authenticated]),
        get "/projects/:origin/:name" => project_show,
        put "/projects/:origin/:name" => XHandler::new(project_update).before(vec![Authenticated]),
        delete "/projects/:origin/:name" => XHandler::new(project_delete).before(vec![Authenticated]),
    );
    let mut chain = Chain::new(router);
    chain.link(persistent::Read::<GitHubCli>::both(GitHubClient::new(&*config)));
    chain.link_before(middleware::RouteBroker);
    chain.link_after(middleware::Cors);
    Ok(chain)
}

/// Create a new HTTP listener and run it in a separate thread. This function will block the calling
/// thread until the new listener has successfully started.
///
/// # Errors
///
/// * Depot could not be started
/// * Couldn't create Router or it's middleware
///
/// # Panics
///
/// * Listener crashed during startup
pub fn run(config: Arc<Config>) -> Result<JoinHandle<()>> {
    let (tx, rx) = mpsc::sync_channel(1);

    let addr = config.http_addr.clone();
    let ctx1 = ZMQ_CONTEXT.clone();
    let depot = try!(depot::Depot::new(config.depot.clone(), ctx1));
    let depot_chain = try!(depot::server::router(depot));

    let mut mount = Mount::new();
    if let Some(ref path) = config.ui_root {
        debug!("Mounting UI at filepath {}", path);
        mount.mount("/", Static::new(path));
    }
    let chain = try!(router(config));
    mount.mount("/v1", chain).mount("/v1/depot", depot_chain);

    let handle = thread::Builder::new()
        .name("http-srv".to_string())
        .spawn(move || {
            let _server = Iron::new(mount)
                .listen_with(addr, HTTP_THREAD_COUNT, Protocol::Http, None)
                .unwrap();
            tx.send(()).unwrap();
        })
        .unwrap();
    match rx.recv() {
        Ok(()) => Ok(handle),
        Err(e) => panic!("http-srv thread startup error, err={}", e),
    }
}
