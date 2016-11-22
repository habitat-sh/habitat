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
use std::net::IpAddr;
use std::str;

use libc;

use error::{Error, Result};
use hcore::util::sys;

static LOGKEY: &'static str = "SY";

pub fn ip() -> Result<IpAddr> {
    match sys::ip() {
        Ok(s) => Ok(s),
        Err(e) => Err(sup_error!(Error::HabitatCore(e))),
    }
}


extern "C" {
    pub fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}


pub fn hostname() -> Result<String> {
    debug!("Determining host name");
    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_slice().as_mut_ptr();
    let err = unsafe { gethostname(ptr as *mut libc::c_char, len as libc::size_t) };
    match err {
        0 => {
            let slice = unsafe { CStr::from_ptr(ptr as *const i8) };
            let s = try!(slice.to_str());
            debug!("Hostname = {}", &s);
            Ok(s.to_string())
        }
        n => {
            debug!("gethostname failure: {}", n);
            Err(sup_error!(Error::IPFailed))
        }
    }
}

pub fn to_toml() -> Result<String> {
    let mut toml_string = String::from("[sys]\n");
    let ip = try!(ip()).to_string();
    toml_string.push_str(&format!("ip = \"{}\"\n", ip));
    let hostname = try!(hostname());
    toml_string.push_str(&format!("hostname = \"{}\"\n", hostname));
    debug!("Sys Toml: {}", toml_string);
    Ok(toml_string)
}
