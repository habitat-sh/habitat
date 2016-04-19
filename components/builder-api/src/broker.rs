// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use zmq;

use config::Config;
use error::Result;

pub const SESSION_LISTEN_ADDR: &'static str = "inproc://sessionsrv-broker";
pub const VAULT_LISTEN_ADDR: &'static str = "inproc://vault-broker";

pub struct SessionSrv {
    config: Arc<Config>,
    #[allow(dead_code)]
    ctx: Arc<Mutex<zmq::Context>>,
    fe_sock: zmq::Socket,
    be_sock: zmq::Socket,
}

impl SessionSrv {
    fn new(config: Arc<Config>, ctx: Arc<Mutex<zmq::Context>>) -> Self {
        let (fe, be) = {
            let mut ctx = ctx.lock().unwrap();
            let fe = ctx.socket(zmq::ROUTER).unwrap();
            let be = ctx.socket(zmq::DEALER).unwrap();
            (fe, be)
        };
        SessionSrv {
            config: config,
            ctx: ctx,
            fe_sock: fe,
            be_sock: be,
        }
    }

    pub fn connect(ctx: &Arc<Mutex<zmq::Context>>) -> Result<zmq::Socket> {
        let mut socket = ctx.lock().unwrap().socket(zmq::REQ).unwrap();
        socket.connect(SESSION_LISTEN_ADDR).unwrap();
        Ok(socket)
    }

    pub fn run(config: Arc<Config>, ctx: Arc<Mutex<zmq::Context>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
                         .name("sessionsrv-broker".to_string())
                         .spawn(move || {
                             let mut broker = Self::new(config, ctx);
                             broker.start(tx).unwrap();
                         })
                         .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("sessionsrv-broker thread startup error, err={}", e),
        }
    }

    fn start(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        try!(self.fe_sock.bind(SESSION_LISTEN_ADDR));
        // JW TODO: connect to multiple session servers, not one
        try!(self.be_sock.connect(&self.config.sessionsrv_addr()));
        rz.send(()).unwrap();
        try!(zmq::proxy(&mut self.fe_sock, &mut self.be_sock));
        Ok(())
    }
}

pub struct VaultSrv {
    config: Arc<Config>,
    #[allow(dead_code)]
    ctx: Arc<Mutex<zmq::Context>>,
    fe_sock: zmq::Socket,
    be_sock: zmq::Socket,
}

impl VaultSrv {
    fn new(config: Arc<Config>, ctx: Arc<Mutex<zmq::Context>>) -> Self {
        let (fe, be) = {
            let mut ctx = ctx.lock().unwrap();
            let fe = ctx.socket(zmq::ROUTER).unwrap();
            let be = ctx.socket(zmq::DEALER).unwrap();
            (fe, be)
        };
        VaultSrv {
            config: config,
            ctx: ctx,
            fe_sock: fe,
            be_sock: be,
        }
    }

    pub fn connect(ctx: &Arc<Mutex<zmq::Context>>) -> Result<zmq::Socket> {
        let mut socket = ctx.lock().unwrap().socket(zmq::REQ).unwrap();
        socket.connect(VAULT_LISTEN_ADDR).unwrap();
        Ok(socket)
    }

    pub fn run(config: Arc<Config>, ctx: Arc<Mutex<zmq::Context>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
                         .name("vault-broker".to_string())
                         .spawn(move || {
                             let mut broker = Self::new(config, ctx);
                             broker.start(tx).unwrap();
                         })
                         .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("sessionsrv-broker thread startup error, err={}", e),
        }
    }

    fn start(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        try!(self.fe_sock.bind(VAULT_LISTEN_ADDR));
        // JW TODO: connect to multiple vault servers, not one
        try!(self.be_sock.connect(&self.config.vaultsrv_addr()));
        rz.send(()).unwrap();
        try!(zmq::proxy(&mut self.fe_sock, &mut self.be_sock));
        Ok(())
    }
}
