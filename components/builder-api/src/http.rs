// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate bodyparser;

use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use depot;
use iron::prelude::*;
use iron::{status, AfterMiddleware};
use iron::headers::{self, Authorization, Bearer};
use mount::Mount;
use protobuf::{self, Message};
use protocol::sessionsrv::{Session, SessionGet, GitHubAuth};
use protocol::vault::{Origin, OriginCreate, OriginGet};
use protocol::net::{NetError, ErrCode};
use router::Router;
use rustc_serialize::json::{self, ToJson};
use unicase::UniCase;
use zmq;

use broker::{SessionSrv, VaultSrv};
use config::Config;
use error::Result;

struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers
           .set(headers::AccessControlAllowHeaders(vec![UniCase("authorization".to_owned())]));
        Ok(res)
    }
}

pub fn router(context: Arc<Mutex<zmq::Context>>) -> Result<Chain> {
    let ctx1 = context.clone();
    let ctx2 = context.clone();
    let ctx3 = context.clone();
    let router = router!(
        get "/authenticate/:code" => move |r: &mut Request| authenticate(r, &ctx1),

        get "/origins/:origin" => move |r: &mut Request| origin_show(r, &ctx2),
        post "/origins" => move |r: &mut Request| origin_create(r, &ctx3),
    );
    let mut chain = Chain::new(router);
    chain.link_after(Cors);
    Ok(chain)
}

pub fn run(config: Arc<Config>, context: Arc<Mutex<zmq::Context>>) -> Result<JoinHandle<()>> {
    let (tx, rx) = mpsc::sync_channel(1);
    let depot = try!(depot::server::router(config.depot.clone()));
    let chain = try!(router(context));
    let mut mount = Mount::new();
    mount.mount("/", chain).mount("/depot", depot);
    let handle = thread::Builder::new()
                     .name("http-srv".to_string())
                     .spawn(move || {
                         let _server = Iron::new(mount).http(config.http_addr).unwrap();
                         tx.send(()).unwrap();
                     })
                     .unwrap();
    match rx.recv() {
        Ok(()) => Ok(handle),
        Err(e) => panic!("http-srv thread startup error, err={}", e),
    }
}

fn authenticate(req: &mut Request, ctx: &Arc<Mutex<zmq::Context>>) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let code = match params.find("code") {
        Some(code) => code.to_string(),
        _ => return Ok(Response::with(status::BadRequest)),
    };
    let mut conn = SessionSrv::connect(&ctx).unwrap();
    let mut request = GitHubAuth::new();
    request.set_code(code.to_string());
    conn.send_str("SessionCreate", zmq::SNDMORE).unwrap();
    conn.send(&request.write_to_bytes().unwrap(), 0).unwrap();

    match conn.recv_msg(0) {
        Ok(rep) => {
            match rep.as_str() {
                Some("Session") => {
                    let msg = conn.recv_msg(0).unwrap();
                    let token: Session = protobuf::parse_from_bytes(&msg).unwrap();
                    let encoded = json::encode(&token.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                Some("NetError") => {
                    let msg = conn.recv_msg(0).unwrap();
                    let err: NetError = protobuf::parse_from_bytes(&msg).unwrap();
                    let encoded = json::encode(&err.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                _ => Ok(Response::with(status::InternalServerError)),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}

fn origin_show(req: &mut Request, ctx: &Arc<Mutex<zmq::Context>>) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let origin = match params.find("origin") {
        Some(origin) => origin.to_string(),
        _ => return Ok(Response::with(status::BadRequest)),
    };
    let mut conn = VaultSrv::connect(&ctx).unwrap();
    let mut request = OriginGet::new();
    request.set_name(origin);
    conn.send_str("OriginGet", zmq::SNDMORE).unwrap();
    conn.send(&request.write_to_bytes().unwrap(), 0).unwrap();
    match conn.recv_msg(0) {
        Ok(rep) => {
            match rep.as_str() {
                Some("Origin") => {
                    let msg = conn.recv_msg(0).unwrap();
                    let origin: Origin = protobuf::parse_from_bytes(&msg).unwrap();
                    let encoded = json::encode(&origin.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                Some("NetError") => {
                    let msg = conn.recv_msg(0).unwrap();
                    let err: NetError = protobuf::parse_from_bytes(&msg).unwrap();

                    match err.get_code() {
                        ErrCode::ENTITY_NOT_FOUND => Ok(Response::with(status::NotFound)),
                        _ => Ok(Response::with(status::InternalServerError)),
                    }
                }
                _ => Ok(Response::with(status::InternalServerError)),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}

fn origin_create(req: &mut Request, ctx: &Arc<Mutex<zmq::Context>>) -> IronResult<Response> {
    let session: Session = match req.headers.get::<Authorization<Bearer>>() {
        Some(&Authorization(Bearer { ref token })) => {
            // ask session server for owner
            let mut conn = SessionSrv::connect(&ctx).unwrap();
            let mut request = SessionGet::new();
            request.set_token(token.to_string());
            conn.send_str("SessionGet", zmq::SNDMORE).unwrap();
            conn.send(&request.write_to_bytes().unwrap(), 0).unwrap();
            match conn.recv_msg(0) {
                Ok(rep) => {
                    match rep.as_str() {
                        Some("Session") => {
                            let msg = conn.recv_msg(0).unwrap();
                            protobuf::parse_from_bytes(&msg).unwrap()
                        }
                        Some("NetError") => {
                            let msg = conn.recv_msg(0).unwrap();
                            let err: NetError = protobuf::parse_from_bytes(&msg).unwrap();
                            let encoded = json::encode(&err.to_json()).unwrap();
                            return Ok(Response::with((status::Ok, encoded)));
                        }
                        Some(msg) => {
                            warn!("unexpected msg: {:?}", msg);
                            return Ok(Response::with(status::InternalServerError));
                        }
                        None => return Ok(Response::with(status::Unauthorized)),
                    }
                }
                Err(e) => {
                    error!("session get, err={:?}", e);
                    return Ok(Response::with(status::InternalServerError));
                }
            }
        }
        _ => return Ok(Response::with(status::Unauthorized)),
    };

    let mut request = OriginCreate::new();

    match req.get::<bodyparser::Json>() {
        Ok(Some(body)) => {
            match body.find("name") {
                Some(origin) => request.set_name(origin.as_string().unwrap().to_owned()),
                _ => return Ok(Response::with(status::BadRequest)),
            }
        }
        _ => return Ok(Response::with(status::BadRequest)),
    };

    let mut conn = VaultSrv::connect(&ctx).unwrap();
    request.set_owner_id(session.get_id());
    conn.send_str("OriginCreate", zmq::SNDMORE).unwrap();
    conn.send(&request.write_to_bytes().unwrap(), 0).unwrap();
    match conn.recv_msg(0) {
        Ok(rep) => {
            match rep.as_str() {
                Some("Origin") => {
                    let msg = conn.recv_msg(0).unwrap();
                    let origin: Origin = protobuf::parse_from_bytes(&msg).unwrap();
                    let encoded = json::encode(&origin.to_json()).unwrap();
                    Ok(Response::with((status::Created, encoded)))
                }
                Some("NetError") => {
                    let msg = conn.recv_msg(0).unwrap();
                    let err: NetError = protobuf::parse_from_bytes(&msg).unwrap();
                    let encoded = json::encode(&err.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                _ => Ok(Response::with(status::InternalServerError)),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}
