// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use error::{BldrResult, ErrorKind};
use std::process::Command;

static LOGKEY: &'static str = "US";

pub fn ip() -> BldrResult<String> {
    debug!("Shelling out to determine IP address");
    let output = try!(Command::new("sh")
                          .arg("-c")
                          .arg("ip route get 8.8.8.8 | awk '{printf \"%s\", $NF; exit}'")
                          .output());
    match output.status.success() {
        true => {
            debug!("IP address is {}", String::from_utf8_lossy(&output.stdout));
            let ip = try!(String::from_utf8(output.stdout));
            Ok(ip)
        }
        false => {
            debug!("IP address command returned: OUT: {} ERR: {}",
                   String::from_utf8_lossy(&output.stdout),
                   String::from_utf8_lossy(&output.stderr));
            Err(bldr_error!(ErrorKind::IPFailed))
        }
    }
}

pub fn hostname() -> BldrResult<String> {
    debug!("Shelling out to determine IP address");
    let output = try!(Command::new("sh")
                          .arg("-c")
                          .arg("hostname | awk '{printf \"%s\", $NF; exit}'")
                          .output());
    match output.status.success() {
        true => {
            debug!("Hostname address is {}",
                   String::from_utf8_lossy(&output.stdout));
            let hostname = try!(String::from_utf8(output.stdout));
            Ok(hostname)
        }
        false => {
            debug!("Hostname address command returned: OUT: {} ERR: {}",
                   String::from_utf8_lossy(&output.stdout),
                   String::from_utf8_lossy(&output.stderr));
            Err(bldr_error!(ErrorKind::IPFailed))
        }
    }
}

pub fn to_toml() -> BldrResult<String> {
    let mut toml_string = String::from("[sys]\n");
    let ip = try!(ip());
    toml_string.push_str(&format!("ip = \"{}\"\n", ip));
    let hostname = try!(hostname());
    toml_string.push_str(&format!("hostname = \"{}\"\n", hostname));
    debug!("Sys Toml: {}", toml_string);
    Ok(toml_string)
}
