// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::process::Command;

use error::{Error, Result};
use hcore::util::sys;

static LOGKEY: &'static str = "SY";

pub fn ip(path: Option<&str>) -> Result<String> {
    match sys::ip(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(sup_error!(Error::HabitatCore(e))),
    }
}

pub fn hostname(path: Option<&str>) -> Result<String> {
    debug!("Shelling out to determine IP address");
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg("hostname | awk '{printf \"%s\", $NF; exit}'");
    if let Some(path) = path {
        cmd.env("PATH", path);
        debug!("Setting shell out PATH={}", path);
    }
    let output = try!(cmd.output());
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
            Err(sup_error!(Error::IPFailed))
        }
    }
}

pub fn to_toml() -> Result<String> {
    let mut toml_string = String::from("[sys]\n");
    let ip = try!(ip(None));
    toml_string.push_str(&format!("ip = \"{}\"\n", ip));
    let hostname = try!(hostname(None));
    toml_string.push_str(&format!("hostname = \"{}\"\n", hostname));
    debug!("Sys Toml: {}", toml_string);
    Ok(toml_string)
}
