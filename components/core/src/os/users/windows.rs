use crate::error::{Error,
                   Result};
use habitat_win_users::account::Account;
use std::path::PathBuf;
use windows_acl::helper;

extern "C" {
    pub fn GetUserTokenStatus() -> u32;
}

fn get_sid_by_name(name: &str) -> Option<String> {
    match Account::from_name(name) {
        Some(acct) => acct.sid.to_string().ok(),
        None => None,
    }
}

pub fn get_uid_by_name(owner: &str) -> Result<Option<String>> { Ok(get_sid_by_name(owner)) }

// this is a no-op on windows
pub fn get_gid_by_name(group: &str) -> Result<Option<String>> { Ok(Some(String::new())) }

pub fn get_current_username() -> Result<Option<String>> {
    match helper::current_user() {
        Some(username) => Ok(Some(username.to_lowercase())),
        None => Ok(None),
    }
}

// this is a no-op on windows
pub fn get_current_groupname() -> Result<Option<String>> { Ok(Some(String::new())) }

pub fn get_effective_uid() -> u32 { unsafe { GetUserTokenStatus() } }

pub fn get_home_for_user(username: &str) -> Option<PathBuf> {
    unimplemented!();
}

/// Windows does not have a concept of "group" in a Linux sense
/// So we just validate the user
pub fn assert_pkg_user_and_group(user: &str, _group: &str) -> Result<()> {
    match get_uid_by_name(user)? {
        Some(_) => Ok(()),
        None => {
            Err(Error::PermissionFailed(format!("Package requires user \
                                                 {} to exist, but it \
                                                 doesn't",
                                                user)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_uid_of_current_user() {
        assert!(get_current_username().unwrap()
                                      .map(|s| get_uid_by_name(&s))
                                      .is_some())
    }
}
