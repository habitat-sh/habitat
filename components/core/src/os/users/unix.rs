use crate::{error::{Error,
                    Result},
            ok_warn};
use nix::unistd::{Group,
                  User};
use std::path::PathBuf;

pub fn get_uid_by_name(owner: &str) -> Option<u32> {
    ok_warn!(User::from_name(owner)).flatten()
                                    .map(|u| u.uid.as_raw())
}

pub fn get_gid_by_name(group: &str) -> Option<u32> {
    ok_warn!(Group::from_name(group)).flatten()
                                     .map(|g| g.gid.as_raw())
}

/// Any members that fail conversion from OsString to string will be omitted
pub fn get_members_by_groupname(group: &str) -> Option<Vec<String>> {
    ok_warn!(Group::from_name(group)).flatten().map(|g| g.mem)
}

pub fn get_current_username() -> Option<String> {
    let uid = nix::unistd::getuid();
    ok_warn!(User::from_uid(uid)).flatten().map(|u| u.name)
}

pub fn get_current_groupname() -> Option<String> {
    let gid = nix::unistd::getgid();
    ok_warn!(Group::from_gid(gid)).flatten().map(|g| g.name)
}

pub fn get_effective_username() -> Option<String> {
    let euid = nix::unistd::geteuid();
    ok_warn!(User::from_uid(euid)).flatten().map(|u| u.name)
}

pub fn get_effective_uid() -> u32 { nix::unistd::geteuid().as_raw() }

pub fn get_effective_gid() -> u32 { nix::unistd::getegid().as_raw() }

pub fn get_effective_groupname() -> Option<String> {
    let egid = nix::unistd::getegid();
    ok_warn!(Group::from_gid(egid)).flatten().map(|g| g.name)
}

pub fn get_home_for_user(username: &str) -> Option<PathBuf> {
    ok_warn!(User::from_name(username)).flatten().map(|u| u.dir)
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
