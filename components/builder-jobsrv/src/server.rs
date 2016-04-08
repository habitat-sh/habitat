// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::thread;

use protobuf::{parse_from_bytes, Message};
use protocol::jobsrv;
use zmq;

use config::Config;
use error::Result;

pub struct Server {
    pub config: Config,
    ctx: zmq::Context,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server {
            config: config,
            ctx: zmq::Context::new(),
        }
    }

    pub fn reconfigure(config: Config) -> Result<()> {
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        // build request socket?
        let mut socket = try!(self.ctx.socket(zmq::REP));
        try!(socket.bind("tcp://127.0.0.1:9636"));
        let thread = try!(thread::Builder::new()
                              .name("server".to_string())
                              .spawn(move || Self::recv_loop(&mut socket)));
        thread.join().unwrap();
        Ok(())
    }

    fn recv_loop(socket: &mut zmq::Socket) -> Result<()> {
        let mut msg = try!(zmq::Message::new());
        loop {
            try!(socket.recv(&mut msg, 0));
            try!(Self::dispatch(socket, &msg));
        }
    }

    fn dispatch(socket: &mut zmq::Socket, msg: &zmq::Message) -> Result<()> {
        let mut request: jobsrv::JobCreate = try!(parse_from_bytes(&msg));
        println!("Received {:?}", request);
        let mut job: jobsrv::Job = jobsrv::Job::new();
        job.set_id("fakeid".to_string());
        try!(socket.send(&job.write_to_bytes().unwrap(), 0));
        Ok(())
    }
}

pub fn run(config: Config) -> Result<()> {
    Server::new(config).run()
}
