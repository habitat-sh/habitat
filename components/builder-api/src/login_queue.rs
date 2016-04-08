// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::sync::{Arc, Mutex};
use std::thread;

use zmq;

use config::Config;
use error::Result;

pub struct Server {
    config: Arc<Config>,
    ctx: Arc<Mutex<zmq::Context>>,
    fe_sock: zmq::Socket,
    be_sock: zmq::Socket,
    rz_sock: zmq::Socket,
}

impl Server {
    fn new(config: Arc<Config>, ctx: Arc<Mutex<zmq::Context>>) -> Self {
        let (fe, be, rz) = {
            let mut ctx = ctx.lock().unwrap();
            let fe = ctx.socket(zmq::ROUTER).unwrap();
            let be = ctx.socket(zmq::DEALER).unwrap();
            let rz = ctx.socket(zmq::PAIR).unwrap();
            (fe, be, rz)
        };
        Server {
            config: config,
            ctx: ctx,
            fe_sock: fe,
            be_sock: be,
            rz_sock: rz,
        }
    }

    pub fn run(config: Arc<Config>, ctx: Arc<Mutex<zmq::Context>>) -> Result<()> {
        thread::Builder::new().name("login-queue-conn".to_string()).spawn(move || {
            let mut conn = Self::new(config, ctx);
            conn.start();
        });
        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        try!(self.fe_sock.bind("inproc://login-queue"));
        try!(self.be_sock.connect(&self.config.sessionsrv_addr()));
        try!(self.rz_sock.connect("inproc://rz-login-queue"));
        try!(self.rz_sock.send(&[], 0));
        try!(zmq::proxy(&mut self.fe_sock, &mut self.be_sock));
        Ok(())
    }
}
