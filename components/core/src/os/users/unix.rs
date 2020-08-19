use crate::error::{Error,
                   Result};
use nix::unistd::{Group,
                  User};
use std::path::PathBuf;

pub fn get_uid_by_name(owner: &str) -> Result<Option<u32>> {
    Ok(User::from_name(owner)?.map(|u| u.uid.as_raw()))
}

pub fn get_gid_by_name(group: &str) -> Result<Option<u32>> {
    Ok(Group::from_name(group)?.map(|g| g.gid.as_raw()))
}

/// Any members that fail conversion from OsString to string will be omitted
pub fn get_members_by_groupname(group: &str) -> Result<Option<Vec<String>>> {
    Ok(Group::from_name(group)?.map(|g| g.mem))
}

pub fn get_current_username() -> Result<Option<String>> {
    let uid = nix::unistd::getuid();
    Ok(User::from_uid(uid)?.map(|u| u.name))
}

pub fn get_current_groupname() -> Result<Option<String>> {
    let gid = nix::unistd::getgid();
    Ok(Group::from_gid(gid)?.map(|g| g.name))
}

pub fn get_effective_username() -> Result<Option<String>> {
    let euid = nix::unistd::geteuid();
    Ok(User::from_uid(euid)?.map(|u| u.name))
}

pub fn get_effective_uid() -> u32 { nix::unistd::geteuid().as_raw() }

pub fn get_effective_gid() -> u32 { nix::unistd::getegid().as_raw() }

pub fn get_effective_groupname() -> Result<Option<String>> {
    let egid = nix::unistd::getegid();
    Ok(Group::from_gid(egid)?.map(|g| g.name))
}

pub fn get_home_for_user(username: &str) -> Result<Option<PathBuf>> {
    Ok(User::from_name(username)?.map(|u| u.dir))
}

/// This function checks to see if a user and group and if:
///     a) we are root
///     b) we are the specified user:group
///     c) fail otherwise
pub fn assert_pkg_user_and_group(user: &str, group: &str) -> Result<()> {
    if get_uid_by_name(user)?.is_none() {
        return Err(Error::PermissionFailed(format!("Package requires user \
                                                    {} to exist, but it \
                                                    doesn't",
                                                   user)));
    }
    if get_gid_by_name(&group)?.is_none() {
        return Err(Error::PermissionFailed(format!("Package requires group \
                                                    {} to exist, but it \
                                                    doesn't",
                                                   group)));
    }

    let current_user = get_current_username()?;
    let current_group = get_current_groupname()?;

    if current_user.is_none() {
        return Err(Error::PermissionFailed("Can't determine current user".to_string()));
    }

    if current_group.is_none() {
        return Err(Error::PermissionFailed("Can't determine current group".to_string()));
    }

    let current_user = current_user.unwrap();
    let current_group = current_group.unwrap();

    if current_user == "root" || (current_user == user && current_group == group) {
        Ok(())
    } else {
        let msg = format!("Package must run as {}:{} or root", user, &group);
        Err(Error::PermissionFailed(msg))
    }
}
