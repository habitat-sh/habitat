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

use std::ffi::CStr;
use std::mem;
use std::process::Command;

use errno::errno;
use libc;

use error::{Error, Result};

pub fn ip(path: Option<&str>) -> Result<String> {
    debug!("Shelling out to determine IP address");
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg("ip route get 8.8.8.8 | awk '{printf \"%s\", $NF; exit}'");
    if let Some(path) = path {
        cmd.env("PATH", path);
        debug!("Setting shell out PATH={}", path);
    }
    let output = try!(cmd.output());
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

#[derive(Debug)]
pub struct Uname {
    pub sys_name: String,
    pub node_name: String,
    pub release: String,
    pub version: String,
    pub machine: String,
}

pub fn uname() -> Result<Uname> {
    unsafe { uname_libc() }
}

unsafe fn uname_libc() -> Result<Uname> {
    let mut utsname: libc::utsname = mem::uninitialized();
    let rv = libc::uname(&mut utsname);
    if rv < 0 {
        let errno = errno();
        let code = errno.0 as i32;
        return Err(Error::UnameFailed(format!("Error {} when calling uname: {}", code, errno)));
    }
    Ok(Uname {
        sys_name: CStr::from_ptr(utsname.sysname.as_ptr()).to_string_lossy().into_owned(),
        node_name: CStr::from_ptr(utsname.nodename.as_ptr()).to_string_lossy().into_owned(),
        release: CStr::from_ptr(utsname.release.as_ptr()).to_string_lossy().into_owned(),
        version: CStr::from_ptr(utsname.version.as_ptr()).to_string_lossy().into_owned(),
        machine: CStr::from_ptr(utsname.machine.as_ptr()).to_string_lossy().into_owned(),
    })
}
