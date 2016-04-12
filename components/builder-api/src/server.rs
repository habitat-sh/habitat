// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::sync::{Arc, Mutex};

use zmq;

use broker;
use config::Config;
use error::Result;
use http;

pub struct Server {
    pub config: Arc<Config>,
    context: Arc<Mutex<zmq::Context>>,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let ctx = zmq::Context::new();
        Ok(Server {
            config: Arc::new(config),
            context: Arc::new(Mutex::new(ctx)),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let cfg1 = self.config.clone();
        let cfg2 = self.config.clone();
        let cfg3 = self.config.clone();
        let ctx1 = self.context.clone();
        let ctx2 = self.context.clone();
        let ctx3 = self.context.clone();
        try!(broker::SessionSrv::run(cfg1, ctx1));
        try!(broker::VaultSrv::run(cfg2, ctx2));
        let handle = try!(http::run(cfg3, ctx3));

        println!("Builder API listening on {}", &self.config.http_addr);
        handle.join().unwrap();
        Ok(())
    }
}

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
