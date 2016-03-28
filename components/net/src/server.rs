// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use zmq;

use error::Result;

pub struct Connector {
    addr: String,
    socket_type: zmq::SocketType,
}

impl Connector {
    pub fn new(socket_type: zmq::SocketType, addr: &str) -> Self {
        Connector {
            addr: addr.to_string(),
            socket_type: socket_type,
        }
    }
}

pub trait Proxy {
    fn front_end(&self) -> &Connector;
    fn back_end(&self) -> &Connector;

    fn start(&self, ctx: &mut zmq::Context) -> Result<()> {
        let mut fe = try!(ctx.socket(self.front_end().socket_type));
        let mut be = try!(ctx.socket(self.back_end().socket_type));

        try!(fe.bind(&self.front_end().addr));
        try!(be.connect(&self.back_end().addr));
        try!(zmq::proxy(&mut fe, &mut be));
        Ok(())
    }
}
