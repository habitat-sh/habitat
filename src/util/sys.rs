// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use error::{Error, Result};
use std::process::Command;

pub fn ip() -> Result<String> {
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
            Err(Error::NoOutboundAddr)
        }
    }
}
