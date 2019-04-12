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

use libc::{self,
           c_char,
           c_int,
           mode_t};
use std::{ffi::CString,
          path::Path};

use crate::users;

use crate::error::{Error,
                   Result};

pub fn set_owner<T: AsRef<Path>, X: AsRef<str>>(path: T, owner: X, group: X) -> Result<()> {
    debug!("Attempting to set owner of {:?} to {:?}:{:?}",
           &path.as_ref(),
           &owner.as_ref(),
           &group.as_ref());

    let uid = match users::get_uid_by_name(&owner.as_ref()) {
        Some(user) => user,
        None => {
            let msg = format!("Can't change owner of {:?} to {:?}:{:?}, error getting user.",
                              &path.as_ref(),
                              &owner.as_ref(),
                              &group.as_ref());
            return Err(Error::PermissionFailed(msg));
        }
    };

    let gid = match users::get_gid_by_name(&group.as_ref()) {
        Some(group) => group,
        None => {
            let msg = format!("Can't change owner of {:?} to {:?}:{:?}, error getting group.",
                              &path.as_ref(),
                              &owner.as_ref(),
                              &group.as_ref());
            return Err(Error::PermissionFailed(msg));
        }
    };

    let s_path = match path.as_ref().to_str() {
        Some(s) => s,
        None => {
            return Err(Error::PermissionFailed(format!("Invalid path {:?}", &path.as_ref())));
        }
    };
    let result = chown(s_path, uid, gid);

    match result {
        Err(err) => Err(err),
        Ok(0) => Ok(()),
        _ => {
            Err(Error::PermissionFailed(format!("Can't change owner of \
                                                 {:?} to {:?}:{:?}",
                                                &path.as_ref(),
                                                &owner.as_ref(),
                                                &group.as_ref())))
        }
    }
}

pub fn set_permissions<T: AsRef<Path>>(path: T, mode: u32) -> Result<()> {
    let s_path = match path.as_ref().to_str() {
        Some(s) => s,
        None => {
            return Err(Error::PermissionFailed(format!("Invalid path {:?}", &path.as_ref())));
        }
    };

    let result = chmod(s_path, mode);
    match result {
        Err(err) => Err(err),
        Ok(0) => Ok(()),
        _ => {
            Err(Error::PermissionFailed(format!("Can't set permissions \
                                                 on {:?} to {:?}",
                                                &path.as_ref(),
                                                &mode)))
        }
    }
}

fn validate_raw_path(path: &str) -> Result<*mut c_char> {
    let c_path = match CString::new(path) {
        Ok(c) => c,
        Err(e) => {
            return Err(Error::PermissionFailed(format!("Can't create string \
                                                        from path {:?}: {}",
                                                       path, e)));
        }
    };
    Ok(c_path.into_raw())
}

fn chown(path: &str, uid: u32, gid: u32) -> Result<c_int> {
    let r_path = match validate_raw_path(path) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    unsafe {
        let res = libc::chown(r_path, uid, gid);
        CString::from_raw(r_path); // necessary to prevent leaks
        Ok(res)
    }
}

fn chmod(path: &str, mode: u32) -> Result<c_int> {
    let c_path = match CString::new(path) {
        Ok(c) => c,
        Err(e) => {
            return Err(Error::PermissionFailed(format!("Can't create string \
                                                        from path {:?}: {}",
                                                       path, e)));
        }
    };
    let r_path = c_path.into_raw();

    unsafe {
        let res = libc::chmod(r_path, mode as mode_t);
        CString::from_raw(r_path); // necessary to prevent leaks
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File,
              io::Write,
              path::Path};

    use tempfile::Builder;

    use super::*;
    use crate::error::Error;

    #[test]
    fn chmod_ok_test() {
        let tmp_dir = Builder::new().prefix("foo")
                                    .tempdir()
                                    .expect("create temp dir");
        let file_path = tmp_dir.path().join("test.txt");
        let mut tmp_file = File::create(&file_path).expect("create temp file");
        writeln!(tmp_file, "foobar123").expect("write temp file");

        let mode = 0o745;
        assert!(set_permissions(file_path, mode).is_ok());
        drop(tmp_file);
        tmp_dir.close().expect("delete temp dir");
    }

    #[test]
    fn chmod_fail_test() {
        let mode = 0o745;
        let badpath = Path::new("this_file_should_never_exist_deadbeef");

        match set_permissions(badpath, mode) {
            Ok(_) => {
                panic!("Shouldn't be able to chmod on non-existent file, but did!");
            }
            Err(Error::PermissionFailed(_)) => { /* OK */ }
            Err(e) => {
                panic!("Got unexpected error chmodding a non-existent file: {:?}",
                       e);
            }
        }
    }
}
