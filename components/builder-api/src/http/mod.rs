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

use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use depot;
use hab_net::oauth::github::GitHubClient;
use hab_net::routing::BrokerContext;
use iron::prelude::*;
use iron::AfterMiddleware;
use iron::headers;
use iron::method::Method;
use iron::Protocol;
use mount::Mount;
use staticfile::Static;
use unicase::UniCase;

use config::Config;
use error::Result;
use self::handlers::*;

// Iron defaults to a threadpool of size `8 * num_cpus`.
// See: http://172.16.2.131:9633/iron/prelude/struct.Iron.html#method.http
const HTTP_THREAD_COUNT: usize = 128;

/// Create a new `iron::Chain` containing a Router and it's required middleware
pub fn router(config: Arc<Config>, context: Arc<BrokerContext>) -> Result<Chain> {
    let github = GitHubClient::new(&*config);
    let ctx1 = context.clone();
    let ctx2 = context.clone();
    let ctx3 = context.clone();
    let ctx4 = context.clone();
    let ctx5 = context.clone();
    let ctx6 = context.clone();

    let router = router!(
        get "/status" => move |r: &mut Request| status(r),
        get "/authenticate/:code" => move |r: &mut Request| session_create(r, &github, &ctx1),

        post "/jobs" => move |r: &mut Request| job_create(r, &ctx2),
        get "/jobs/:id" => move |r: &mut Request| job_show(r, &ctx3),

        get "/user/invitations" => move |r: &mut Request| list_account_invitations(r, &ctx4),
        put "/user/invitations/:invitation_id" => move |r: &mut Request| accept_invitation(r, &ctx5),
        get "/user/origins" => move |r: &mut Request| list_user_origins(r, &ctx6),

    );
    let mut chain = Chain::new(router);
    chain.link_after(Cors);
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
pub fn run(config: Arc<Config>, context: Arc<BrokerContext>) -> Result<JoinHandle<()>> {
    let (tx, rx) = mpsc::sync_channel(1);

    let addr = config.http_addr.clone();
    let ctx = context.clone();
    let depot = try!(depot::Depot::new(config.depot.clone(), ctx));
    let depot_chain = try!(depot::server::router(depot));

    let mut mount = Mount::new();
    if let Some(ref path) = config.ui_root {
        debug!("Mounting UI at filepath {}", path);
        mount.mount("/", Static::new(path));
    }
    let chain = try!(router(config, context));
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

struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers
            .set(headers::AccessControlAllowHeaders(vec![UniCase("authorization".to_owned())]));
        res.headers
            .set(headers::AccessControlAllowMethods(vec![Method::Put]));
        Ok(res)
    }
}
