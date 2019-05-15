use crate::error::{Error,
                   Result};
use habitat_win_users::account::Account;
use std::path::Path;
use widestring::WideCString;
use winapi::{shared::{minwindef::DWORD,
                      ntdef::NULL,
                      winerror::ERROR_SUCCESS},
             um::{accctrl::SE_FILE_OBJECT,
                  aclapi::SetNamedSecurityInfoW,
                  winnt::{DACL_SECURITY_INFORMATION,
                          FILE_ALL_ACCESS,
                          PACL,
                          PROTECTED_DACL_SECURITY_INFORMATION,
                          PSID}}};
use windows_acl::{acl::ACL,
                  helper};

pub struct PermissionEntry {
    pub account:     Account,
    pub access_mask: DWORD,
}

pub fn set_permissions<T: AsRef<Path>>(path: T, entries: &[PermissionEntry]) -> Result<()> {
    let s_path = match path.as_ref().to_str() {
        Some(s) => s,
        None => {
            return Err(Error::PermissionFailed(format!("Invalid path {:?}", &path.as_ref())));
        }
    };

    let ret = unsafe {
        SetNamedSecurityInfoW(WideCString::from_str(s_path).unwrap().into_raw(),
                              SE_FILE_OBJECT,
                              DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
                              NULL as PSID,
                              NULL as PSID,
                              NULL as PACL,
                              NULL as PACL)
    };
    if ret != ERROR_SUCCESS {
        return Err(Error::PermissionFailed(format!("OS error resetting \
                                                    permissions {}",
                                                   ret)));
    }

    let mut acl = match ACL::from_file_path(s_path, false) {
        Ok(acl) => acl,
        Err(e) => {
            return Err(Error::PermissionFailed(format!("OS error {} retrieving \
                                                        ACLs from path path {:?}",
                                                       e,
                                                       &path.as_ref())));
        }
    };

    for entry in entries {
        if let Err(e) = acl.allow(entry.account.sid.raw.as_ptr() as PSID,
                                  true,
                                  entry.access_mask)
        {
            return Err(Error::PermissionFailed(format!("OS error {} setting \
                                                        permissions for {}",
                                                       e, entry.account.name)));
        }
    }
    Ok(())
}

/// This is a convevience function that will essentially apply the default
/// permissions to a path but remove entries for the Users and Authenticated_Users
/// resulting in FULL_CONTROL access for Administrators, SYSTEM and the current
/// user. In nearly all Supervisor scenarios where we need to adjust permissions,
/// this is the desired ACL state.
pub fn harden_path<T: AsRef<Path>>(path: T) -> Result<()> {
    let current_user = match helper::current_user() {
        Some(u) => u,
        None => {
            return Err(Error::CryptoError(format!("Unable to find current user \
                                                   setting permissions for {}",
                                                  path.as_ref().display())));
        }
    };

    let entries = vec![PermissionEntry { account:     Account::from_name(&current_user).unwrap(),
                                         access_mask: FILE_ALL_ACCESS, },
                       PermissionEntry { account:     Account::from_name("Administrators").unwrap(),
                                         access_mask: FILE_ALL_ACCESS, },
                       PermissionEntry { account:     Account::from_name("SYSTEM").unwrap(),
                                         access_mask: FILE_ALL_ACCESS, },];
    set_permissions(path.as_ref(), &entries)
}

#[cfg(test)]
mod tests {
    use std::{fs::File,
              io::Write,
              path::Path};

    use tempfile::Builder;
    use winapi::um::winnt::FILE_ALL_ACCESS;
    use windows_acl::helper;

    use habitat_win_users::account;

    use super::*;
    use crate::error::Error;

    #[test]
    fn set_permissions_ok_test() {
        let tmp_dir = Builder::new().prefix("foo")
                                    .tempdir()
                                    .expect("create temp dir");
        let file_path = tmp_dir.path().join("test.txt");
        let mut tmp_file = File::create(&file_path).expect("create temp file");
        writeln!(tmp_file, "foobar123").expect("write temp file");

        let current_user = helper::current_user().expect("find current user");
        let entries = vec![PermissionEntry { account:
                                                 account::Account::from_name(&current_user).unwrap(),
                                             access_mask: FILE_ALL_ACCESS, }];

        assert!(set_permissions(&file_path, &entries).is_ok());

        let acl = ACL::from_file_path(file_path.to_str().unwrap(), false).expect("obtain file ACL");
        let mut acl_entries = acl.all().expect("retrieve all acl entries");

        assert_eq!(acl_entries.len(), 1);
        let entry = acl_entries.remove(0);
        assert_eq!(entry.mask, entries[0].access_mask);
        assert_eq!(
            helper::sid_to_string(entry.sid.unwrap().as_ptr() as PSID).expect("name from sid"),
            entries[0].account.sid.to_string().expect("sid to string")
        );

        drop(tmp_file);
        tmp_dir.close().expect("delete temp dir");
    }

    #[test]
    fn set_permissions_fail_test() {
        let badpath = Path::new("this_file_should_never_exist_deadbeef");

        let current_user = helper::current_user().expect("find current user");
        let entries = vec![PermissionEntry { account:
                                                 account::Account::from_name(&current_user).unwrap(),
                                             access_mask: FILE_ALL_ACCESS, }];

        match set_permissions(badpath, &entries) {
            Ok(_) => {
                panic!("Shouldn't be able to set permissions on non-existent file, but did!");
            }
            Err(Error::PermissionFailed(_)) => { /* OK */ }
            Err(e) => {
                panic!("Got unexpected error setting permissions a non-existent file: {:?}",
                       e);
            }
        }
    }
}
