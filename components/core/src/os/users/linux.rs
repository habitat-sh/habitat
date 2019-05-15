use std::path::PathBuf;

use crate::error::{Error,
                   Result};
use users::{self,
            os::unix::{GroupExt,
                       UserExt}};

/// This is currently the "master check" for whether the Supervisor
/// can behave "as root".
///
/// All capabilities must be present. If we can run processes as other
/// users, but can't change ownership, then the processes won't be
/// able to access their files. Similar logic holds for the reverse.
#[cfg(target_os = "linux")]
pub fn can_run_services_as_svc_user() -> bool {
    use caps::{CapSet,
               Capability};

    fn has(cap: Capability) -> bool { caps::has_cap(None, CapSet::Effective, cap).unwrap_or(false) }

    has(Capability::CAP_SETUID) && has(Capability::CAP_SETGID) && has(Capability::CAP_CHOWN)
}

#[cfg(target_os = "macos")]
pub fn can_run_services_as_svc_user() -> bool { true }

pub fn get_uid_by_name(owner: &str) -> Option<u32> {
    users::get_user_by_name(owner).map(|u| u.uid())
}

pub fn get_gid_by_name(group: &str) -> Option<u32> {
    users::get_group_by_name(group).map(|g| g.gid())
}

/// Any members that fail conversion from OsString to string will be omitted
pub fn get_members_by_groupname(group: &str) -> Option<Vec<String>> {
    users::get_group_by_name(group).map(|g| {
                                       g.members()
                                        .to_vec()
                                        .into_iter()
                                        .filter_map(|os_string| os_string.into_string().ok())
                                        .collect()
                                   })
}

pub fn get_current_username() -> Option<String> {
    users::get_current_username().and_then(|os_string| os_string.into_string().ok())
}

pub fn get_current_groupname() -> Option<String> {
    users::get_current_groupname().and_then(|os_string| os_string.into_string().ok())
}

pub fn get_effective_username() -> Option<String> {
    users::get_effective_username().and_then(|os_string| os_string.into_string().ok())
}

pub fn get_effective_uid() -> u32 { users::get_effective_uid() }

pub fn get_effective_gid() -> u32 { users::get_effective_gid() }

pub fn get_effective_groupname() -> Option<String> {
    users::get_effective_groupname().and_then(|os_string| os_string.into_string().ok())
}

pub fn get_home_for_user(username: &str) -> Option<PathBuf> {
    users::get_user_by_name(username).map(|u| PathBuf::from(u.home_dir()))
}

pub fn root_level_account() -> String { "root".to_string() }

/// This function checks to see if a user and group and if:
///     a) we are root
///     b) we are the specified user:group
///     c) fail otherwise
pub fn assert_pkg_user_and_group(user: &str, group: &str) -> Result<()> {
    if get_uid_by_name(user).is_none() {
        return Err(Error::PermissionFailed(format!("Package requires user \
                                                    {} to exist, but it \
                                                    doesn't",
                                                   user)));
    }
    if get_gid_by_name(&group).is_none() {
        return Err(Error::PermissionFailed(format!("Package requires group \
                                                    {} to exist, but it \
                                                    doesn't",
                                                   group)));
    }

    let current_user = get_current_username();
    let current_group = get_current_groupname();

    if current_user.is_none() {
        return Err(Error::PermissionFailed("Can't determine current user".to_string()));
    }

    if current_group.is_none() {
        return Err(Error::PermissionFailed("Can't determine current group".to_string()));
    }

    let current_user = current_user.unwrap();
    let current_group = current_group.unwrap();

    if current_user == root_level_account() || (current_user == user && current_group == group) {
        Ok(())
    } else {
        let msg = format!("Package must run as {}:{} or root", user, &group);
        Err(Error::PermissionFailed(msg))
    }
}
