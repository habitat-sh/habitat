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

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use errno;

use {Error, Result};

pub fn pivot_root<N, P>(new_root: N, put_old: P) -> Result<()>
where
    N: AsRef<Path>,
    P: AsRef<Path>,
{
    let c_new_root = CString::new(new_root.as_ref().as_os_str().as_bytes())?;
    let c_put_old = CString::new(put_old.as_ref().as_os_str().as_bytes())?;

    debug!(
        "calling pivot_root(), new_root={}, put_old={}",
        new_root.as_ref().display(),
        put_old.as_ref().display()
    );
    match unsafe { ffi::pivot_root(c_new_root.as_ptr(), c_put_old.as_ptr()) } {
        rc if rc < 0 => {
            Err(Error::PivotRoot(format!(
                "pivot_root({}, {}) returned: {} ({})",
                new_root.as_ref().display(),
                put_old.as_ref().display(),
                rc,
                errno::errno()
            )))
        }
        _ => Ok(()),
    }
}

mod ffi {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn pivot_root(new_root: *const c_char, put_old: *const c_char) -> c_int;
    }
}
