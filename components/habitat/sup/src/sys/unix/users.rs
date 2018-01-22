// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use hcore::os::users;
use hcore::package::PackageInstall;

use error::{Result, Error};
use util::users::default_user_and_group;

static LOGKEY: &'static str = "UR";

/// This function checks to see if a user and group and if:
///     a) we are root
///     b) we are the specified user:group
///     c) fail otherwise
pub fn assert_pkg_user_and_group(user: &str, group: &str) -> Result<()> {
    if let None = users::get_uid_by_name(user) {
        return Err(sup_error!(Error::Permissions(format!(
            "Package requires user {} to exist, but it doesn't",
            user
        ))));
    }
    if let None = users::get_gid_by_name(&group) {
        return Err(sup_error!(Error::Permissions(format!(
            "Package requires group {} to exist, but it doesn't",
            group
        ))));
    }

    let current_user = users::get_current_username();
    let current_group = users::get_current_groupname();

    if let None = current_user {
        return Err(sup_error!(Error::Permissions(
            "Can't determine current user".to_string(),
        )));
    }

    if let None = current_group {
        return Err(sup_error!(Error::Permissions(
            "Can't determine current group".to_string(),
        )));
    }

    let current_user = current_user.unwrap();
    let current_group = current_group.unwrap();

    if current_user == users::root_level_account() {
        Ok(())
    } else {
        if current_user == user && current_group == group {
            // ok, sup is running as svc_user/svc_group already
            Ok(())
        } else {
            let msg = format!("Package must run as {}:{} or root", user, &group);
            return Err(sup_error!(Error::Permissions(msg)));
        }
    }
}

/// check and see if a user/group is specified in package metadata.
/// if not, we'll try and use hab/hab.
/// If hab/hab doesn't exist, try to use (current username, current group).
/// If that doesn't work, then give up.
pub fn get_user_and_group(pkg_install: &PackageInstall) -> Result<(String, String)> {
    if let Some((user, group)) = get_pkg_user_and_group(&pkg_install)? {
        Ok((user, group))
    } else {
        let defaults = default_user_and_group()?;
        Ok(defaults)
    }
}

/// This function checks to see if a custom SVC_USER and SVC_GROUP has
/// been specified as part of the package metadata.
/// If pkg_svc_user and pkg_svc_group have NOT been defined, return None.
fn get_pkg_user_and_group(pkg_install: &PackageInstall) -> Result<Option<(String, String)>> {
    let svc_user = pkg_install.svc_user()?;
    let svc_group = pkg_install.svc_group()?;
    match (svc_user, svc_group) {
        (Some(user), Some(group)) => Ok(Some((user, group))),
        _ => {
            debug!("User/group not specified in package, running with default");
            Ok(None)
        }
    }
}
