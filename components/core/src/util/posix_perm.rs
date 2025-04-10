use libc::{self,
           c_char,
           c_int,
           mode_t};
use log::debug;
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

    let uid = match users::get_uid_by_name(owner.as_ref())? {
        Some(user) => user,
        None => {
            let msg = format!("Can't change owner of {:?} to {:?}:{:?}, error getting user.",
                              &path.as_ref(),
                              &owner.as_ref(),
                              &group.as_ref());
            return Err(Error::PermissionFailed(msg));
        }
    };

    let gid = match users::get_gid_by_name(group.as_ref())? {
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

// This is required on machines where umask is set to a higher value like `0077`. (See CHEF-10987)
// This has a side effect of potentially changing the *mode* of directories not created by us but
// this is done in order to ensure the ability to execute in the face of the variety of scenarios
// that may be encounter "in the wild". This is as such not a huge problem as we are only changing
// the *mode* for `ancestors` we are owner of.
pub(crate) fn ensure_path_permissions(path: &Path, permissions: u32) -> Result<()> {
    let euid = users::get_effective_uid();
    let egid = users::get_effective_gid();
    for ancestor in path.ancestors() {
        if ancestor.ends_with(crate::fs::PKG_PATH) {
            break;
        }
        if euid_egid_matches(&euid, &egid, ancestor) {
            set_permissions(ancestor, permissions)?
        }
    }
    Ok(())
}

fn euid_egid_matches(euid: &u32, egid: &u32, path: &Path) -> bool {
    if let Ok(file_stat) = nix::sys::stat::stat(path) {
        *euid == file_stat.st_uid && *egid == file_stat.st_gid
    } else {
        false
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
    let r_path = validate_raw_path(path)?;

    unsafe {
        let res = libc::chown(r_path, uid, gid);
        let _ = CString::from_raw(r_path); // necessary to prevent leaks
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
        let _ = CString::from_raw(r_path); // necessary to prevent leaks
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
