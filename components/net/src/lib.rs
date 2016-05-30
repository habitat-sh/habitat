// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate fnv;
extern crate habitat_builder_protocol as protocol;
extern crate libc;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate time;
extern crate zmq;

pub mod config;
pub mod error;
pub mod routing;
pub mod server;

use std::process::Command;

pub use self::error::{Error, Result};
pub use self::server::{Application, ServerReg, Supervisor, Supervisable};

pub fn hostname() -> Result<String> {
    let output = try!(Command::new("sh")
        .arg("-c")
        .arg("hostname | awk '{printf \"%s\", $NF; exit}'")
        .output());
    match output.status.success() {
        true => {
            debug!("Hostname address is {}",
                   String::from_utf8_lossy(&output.stdout));
            let hostname = try!(String::from_utf8(output.stdout).or(Err(Error::Sys)));
            Ok(hostname)
        }
        false => {
            debug!("Hostname address command returned: OUT: {} ERR: {}",
                   String::from_utf8_lossy(&output.stdout),
                   String::from_utf8_lossy(&output.stderr));
            Err(Error::Sys)
        }
    }
}
