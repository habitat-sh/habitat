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

use libc::{c_int, c_char};
use std::ffi::CStr;
use std::path::Path;
use std::io;

pub fn chown(r_path: *const c_char, uid: u32, gid: u32) -> c_int {
    unimplemented!();
}

pub fn chmod(r_path: *const c_char, mode: u32) -> c_int {
    unsafe {
        let path = CStr::from_ptr(r_path).to_str().unwrap();
        match Path::new(path).exists() {
            false => 1,
            true => 0,
        }
    }
}

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    unimplemented!();
}
