// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use zmq;

use hab_net::Conn;
use protobuf::{self, Message};
use protocol::{jobsrv, sessionsrv};

use config::Config;
use error::Result;
use login_queue;
use http;

pub struct Server {
    pub config: Arc<Config>,
    context: Arc<Mutex<zmq::Context>>,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let mut ctx = zmq::Context::new();
        Ok(Server {
            config: Arc::new(config),
            context: Arc::new(Mutex::new(ctx)),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        try!(self.start_conn());
        try!(self.start_http());

        loop {
            thread::sleep(Duration::from_millis(5000));
        }
        Ok(())
    }

    fn start_conn(&mut self) -> Result<()> {
        let mut receiver = {
            let mut receiver = try!(self.context.lock().unwrap().socket(zmq::PAIR));
            try!(receiver.bind("inproc://rz-login-queue"));
            receiver
        };
        let cfg1 = self.config.clone();
        let ctx1 = self.context.clone();
        try!(login_queue::Server::run(cfg1, ctx1));
        try!(receiver.recv_msg(0).map(|msg| {
            match msg.as_str() {
                Some(_) => println!("LoginQ conn ready"),
                _ => panic!("error starting LoginQ conn"),
            }
        }));
        Ok(())
    }

    fn start_http(&mut self) -> Result<()> {
        let mut receiver = {
            let mut receiver = try!(self.context.lock().unwrap().socket(zmq::PAIR));
            try!(receiver.bind("inproc://rz-http"));
            receiver
        };
        let cfg1 = self.config.clone();
        let ctx1 = self.context.clone();
        try!(http::run(cfg1, ctx1));
        try!(receiver.recv_msg(0).map(|msg| {
            match msg.as_str() {
                Some(_) => println!("Builder API listening on {}", &self.config.http_addr),
                _ => panic!("error starting http-srv"),
            }
        }));
        Ok(())
    }
}

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
