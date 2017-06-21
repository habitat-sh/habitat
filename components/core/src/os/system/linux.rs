// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use libc;

use os::system::Uname;
use errno::errno;
use error::{Error, Result};

pub fn uname() -> Result<Uname> {
    unsafe { uname_libc() }
}

unsafe fn uname_libc() -> Result<Uname> {
    let mut utsname: libc::utsname = mem::uninitialized();
    let rv = libc::uname(&mut utsname);
    if rv < 0 {
        let errno = errno();
        let code = errno.0 as i32;
        return Err(Error::UnameFailed(
            format!("Error {} when calling uname: {}", code, errno),
        ));
    }
    Ok(Uname {
        sys_name: CStr::from_ptr(utsname.sysname.as_ptr())
            .to_string_lossy()
            .into_owned(),
        node_name: CStr::from_ptr(utsname.nodename.as_ptr())
            .to_string_lossy()
            .into_owned(),
        release: CStr::from_ptr(utsname.release.as_ptr())
            .to_string_lossy()
            .into_owned(),
        version: CStr::from_ptr(utsname.version.as_ptr())
            .to_string_lossy()
            .into_owned(),
        machine: CStr::from_ptr(utsname.machine.as_ptr())
            .to_string_lossy()
            .into_owned(),
    })
}
