// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! A module containing the HTTP server and handlers for servicing client requests

pub mod handlers;

use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use depot;
use hab_net::oauth::github::GitHubClient;
use iron::prelude::*;
use iron::AfterMiddleware;
use iron::headers;
use iron::method::Method;
use mount::Mount;
use unicase::UniCase;
use zmq;

use config::Config;
use error::Result;
use self::handlers::*;

/// Create a new `iron::Chain` containing a Router and it's required middleware
pub fn router(config: Arc<Config>, context: Arc<Mutex<zmq::Context>>) -> Result<Chain> {
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
pub fn run(config: Arc<Config>, context: Arc<Mutex<zmq::Context>>) -> Result<JoinHandle<()>> {
    let (tx, rx) = mpsc::sync_channel(1);

    let addr = config.http_addr.clone();
    let ctx = context.clone();
    let depot = try!(depot::Depot::new(config.depot.clone(), ctx));
    let depot_chain = try!(depot::server::router(depot));
    let chain = try!(router(config, context));

    let mut mount = Mount::new();
    mount.mount("/v1", chain).mount("/v1/depot", depot_chain);
    let handle = thread::Builder::new()
        .name("http-srv".to_string())
        .spawn(move || {
            let _server = Iron::new(mount).http(addr).unwrap();
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
