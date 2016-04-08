// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::sync::{Arc, Mutex};
use std::thread;

use iron::prelude::*;
use iron::{headers, status, AfterMiddleware};
use protobuf::{self, Message};
use protocol::sessionsrv::{AuthToken, GitHubAuth};
use protocol::net::NetError;
use router::Router;
use rustc_serialize::json::{self, ToJson};
use zmq;

use config::Config;
use error::Result;

struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        Ok(res)
    }
}

pub fn run(config: Arc<Config>, context: Arc<Mutex<zmq::Context>>) -> Result<()> {
    let ctx1 = context.clone();
    let router = router!(
        get "/authenticate/:code" => move |r: &mut Request| authenticate(r, &ctx1)
    );
    let mut chain = Chain::new(router);
    chain.link_after(Cors);
    thread::Builder::new().name("http-srv".to_string()).spawn(move || {
        let mut xmitter = context.lock().unwrap().socket(zmq::PAIR).unwrap();
        xmitter.connect("inproc://rz-http").unwrap();
        let server = Iron::new(chain).http(config.http_addr).unwrap();
        xmitter.send(&[], 0).unwrap();
    });
    Ok(())
}

fn authenticate(req: &mut Request, ctx: &Arc<Mutex<zmq::Context>>) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let code = match params.find("code") {
        Some(code) => code.to_string(),
        _ => return Ok(Response::with(status::BadRequest)),
    };
    let mut xmitter = {
        ctx.lock().unwrap().socket(zmq::REQ).unwrap()
    };
    xmitter.connect("inproc://login-queue").unwrap();
    let mut request = GitHubAuth::new();
    request.set_code(code.to_string());
    xmitter.send_str("LOGIN", zmq::SNDMORE).unwrap();
    xmitter.send(&request.write_to_bytes().unwrap(), 0).unwrap();

    match xmitter.recv_msg(0) {
        Ok(rep) => {
            match rep.as_str() {
                Some("AuthToken") => {
                    let msg = xmitter.recv_msg(0).unwrap();
                    let token: AuthToken = protobuf::parse_from_bytes(&msg).unwrap();
                    let encoded = json::encode(&token.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                Some("NetError") => {
                    let msg = xmitter.recv_msg(0).unwrap();
                    let err: NetError = protobuf::parse_from_bytes(&msg).unwrap();
                    let encoded = json::encode(&err.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                _ => Ok(Response::with(status::InternalServerError)),
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}
