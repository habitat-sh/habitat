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

use std::path::Path;

use filesystem;
use users;

use error::{Error, Result};

pub fn set_owner<T: AsRef<Path>, X: AsRef<str>>(path: T, owner: X, group: X) -> Result<()> {
    debug!("Attempting to set owner of {:?} to {:?}",
           &path.as_ref(),
           &owner.as_ref());

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
        None => return Err(Error::PermissionFailed(format!("Invalid path {:?}", &path.as_ref()))),

    };
    let result = filesystem::chown(s_path, uid, gid);

    match result {
        Err(err) => Err(err),
        Ok(0) => Ok(()),
        _ => {
            Err(Error::PermissionFailed(format!("Can't change owner of {:?} to {:?}",
                                                &path.as_ref(),
                                                &owner.as_ref())))
        }
    }
}

pub fn set_permissions<T: AsRef<Path>>(path: T, mode: u32) -> Result<()> {
    let s_path = match path.as_ref().to_str() {
        Some(s) => s,
        None => return Err(Error::PermissionFailed(format!("Invalid path {:?}", &path.as_ref()))),

    };

    let result = filesystem::chmod(s_path, mode);
    match result {
        Err(err) => Err(err),
        Ok(0) => Ok(()),
        _ => {
            Err(Error::PermissionFailed(format!("Can't set permissions on {:?} to {:?}",
                                                &path.as_ref(),
                                                &mode)))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use tempdir::TempDir;

    use error::Error;
    use super::*;

    #[test]
    fn chmod_ok_test() {
        let tmp_dir = TempDir::new("foo").expect("create temp dir");
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
        if let Err(Error::PermissionFailed(_)) = set_permissions(badpath, mode) {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
