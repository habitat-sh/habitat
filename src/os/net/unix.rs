// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::{ffi::CStr,
          io};

use libc;

pub fn hostname() -> io::Result<String> {
    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_slice().as_mut_ptr();
    match unsafe { gethostname(ptr as *mut libc::c_char, len as libc::size_t) } {
        0 => {
            let c_str = unsafe { CStr::from_ptr(ptr as *const libc::c_char) };
            Ok(c_str.to_string_lossy().into_owned())
        }
        code => Err(io::Error::from_raw_os_error(code)),
    }
}

extern "C" {
    pub fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}
