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

use caps::{self, CapSet, Capability};
use std::path::PathBuf;

use crate::error::{Error, Result};
use crate::linux_users;
use crate::linux_users::os::unix::{GroupExt, UserExt};

/// This is currently the "master check" for whether the Supervisor
/// can behave "as root".
///
/// All capabilities must be present. If we can run processes as other
/// users, but can't change ownership, then the processes won't be
/// able to access their files. Similar logic holds for the reverse.
pub fn can_run_services_as_svc_user() -> bool {
    has(Capability::CAP_SETUID) && has(Capability::CAP_SETGID) && has(Capability::CAP_CHOWN)
}

/// Helper function; does the current thread have `cap` in its
/// effective capability set?
fn has(cap: Capability) -> bool {
    caps::has_cap(None, CapSet::Effective, cap).unwrap_or(false)
}

pub fn get_uid_by_name(owner: &str) -> Option<u32> {
    linux_users::get_user_by_name(owner).map(|u| u.uid())
}

pub fn get_gid_by_name(group: &str) -> Option<u32> {
    linux_users::get_group_by_name(group).map(|g| g.gid())
}

/// Any members that fail conversion from OsString to string will be omitted
pub fn get_members_by_groupname(group: &str) -> Option<Vec<String>> {
    linux_users::get_group_by_name(group).map(|g| {
        g.members()
            .to_vec()
            .into_iter()
            .filter_map(|os_string| os_string.into_string().ok())
            .collect()
    })
}

pub fn get_current_username() -> Option<String> {
    linux_users::get_current_username().and_then(|os_string| os_string.into_string().ok())
}

pub fn get_current_groupname() -> Option<String> {
    linux_users::get_current_groupname().and_then(|os_string| os_string.into_string().ok())
}

pub fn get_effective_username() -> Option<String> {
    linux_users::get_effective_username().and_then(|os_string| os_string.into_string().ok())
}

pub fn get_effective_uid() -> u32 {
    linux_users::get_effective_uid()
}

pub fn get_effective_gid() -> u32 {
    linux_users::get_effective_gid()
}

pub fn get_effective_groupname() -> Option<String> {
    linux_users::get_effective_groupname().and_then(|os_string| os_string.into_string().ok())
}

pub fn get_home_for_user(username: &str) -> Option<PathBuf> {
    linux_users::get_user_by_name(username).map(|u| PathBuf::from(u.home_dir()))
}

pub fn root_level_account() -> String {
    "root".to_string()
}

/// This function checks to see if a user and group and if:
///     a) we are root
///     b) we are the specified user:group
///     c) fail otherwise
pub fn assert_pkg_user_and_group(user: &str, group: &str) -> Result<()> {
    if let None = get_uid_by_name(user) {
        return Err(Error::PermissionFailed(format!(
            "Package requires user {} to exist, but it doesn't",
            user
        )));
    }
    if let None = get_gid_by_name(&group) {
        return Err(Error::PermissionFailed(format!(
            "Package requires group {} to exist, but it doesn't",
            group
        )));
    }

    let current_user = get_current_username();
    let current_group = get_current_groupname();

    if let None = current_user {
        return Err(Error::PermissionFailed(
            "Can't determine current user".to_string(),
        ));
    }

    if let None = current_group {
        return Err(Error::PermissionFailed(
            "Can't determine current group".to_string(),
        ));
    }

    let current_user = current_user.unwrap();
    let current_group = current_group.unwrap();

    if current_user == root_level_account() {
        Ok(())
    } else {
        if current_user == user && current_group == group {
            // ok, sup is running as svc_user/svc_group already
            Ok(())
        } else {
            let msg = format!("Package must run as {}:{} or root", user, &group);
            return Err(Error::PermissionFailed(msg));
        }
    }
}
