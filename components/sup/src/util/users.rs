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

#[cfg(any(target_os="linux", target_os="macos"))]
extern crate users;

use error::{Result, Error};
use hcore::package::PackageInstall;

static LOGKEY: &'static str = "UR";

const DEFAULT_USER: &'static str = "hab";
const DEFAULT_GROUP: &'static str = "hab";

/// This function checks to see if a custom SVC_USER and SVC_GROUP has
/// been specified as part of the package metadata.
/// If a pkg_svc_user and pkg_svc_group have been defined, check if:
///     a) we are root
///     b) we are the specified user:group
///     c) fail otherwise
/// If pkg_svc_user and pkg_svc_group have NOT been defined, return None.
#[cfg(any(target_os="linux", target_os="macos"))]
fn check_pkg_user_and_group(pkg_install: &PackageInstall) -> Result<Option<(String, String)>> {
    let svc_user = try!(pkg_install.svc_user());
    let svc_group = try!(pkg_install.svc_group());
    match (svc_user, svc_group) {
        (Some(user), Some(group)) => {
            // a package has a SVC_USER and SVC_GROUP defined,
            // these MUST exist in order to continue
            debug!("SVC_USER = {}", &user);
            debug!("SVC_GROUP = {}", &group);
            if let None = users::get_user_by_name(&user) {
                return Err(sup_error!(Error::Permissions(format!("Package requires user {} to \
                                                                  exist, but it doesn't",
                                                                 user))));
            }
            if let None = users::get_group_by_name(&group) {
                return Err(sup_error!(Error::Permissions(format!("Package requires group {} \
                                                                  to exist, but it doesn't",
                                                                 group))));
            }

            let current_user = users::get_current_username();
            let current_group = users::get_current_groupname();

            if let None = current_user {
                return Err(sup_error!(Error::Permissions("Can't determine current user"
                    .to_string())));
            }

            if let None = current_group {
                return Err(sup_error!(Error::Permissions("Can't determine current group"
                    .to_string())));
            }

            let current_user = current_user.unwrap();
            let current_group = current_group.unwrap();

            if current_user == "root" {
                Ok(Some((user, group)))
            } else {
                if current_user == user && current_group == group {
                    // ok, sup is running as svc_user/svc_group already
                    Ok(Some((user, group)))
                } else {
                    let msg = format!("Package must run as {}:{} or root", &user, &group);
                    return Err(sup_error!(Error::Permissions(msg)));
                }
            }
        }
        _ => {
            debug!("User/group not specified in package, running with default");
            Ok(None)
        }
    }
}

/// checks to see if hab/hab exists, if not, fall back to
/// current user/group. If that fails, then return an error.
#[cfg(any(target_os="linux", target_os="macos"))]
fn get_default_user_and_group() -> Result<(String, String)> {
    let user = users::get_user_by_name(DEFAULT_USER);
    let group = users::get_group_by_name(DEFAULT_GROUP);
    match (user, group) {
        (Some(user), Some(group)) => return Ok((user.name().to_string(), group.name().to_string())),
        _ => {
            debug!("hab:hab does NOT exist");
            let user = users::get_current_username();
            let group = users::get_current_groupname();
            match (user, group) {
                (Some(user), Some(group)) => {
                    debug!("Running as {}/{}", user, group);
                    return Ok((user, group));
                }
                _ => {
                    return Err(sup_error!(Error::Permissions("Can't determine current user:group"
                        .to_string())))
                }
            }
        }
    }
}

/// check and see if a user/group is specified in package metadata.
/// if not, we'll try and use hab/hab.
/// If hab/hab doesn't exist, try to use (current username, current group).
/// If that doesn't work, then give up.
#[cfg(any(target_os="linux", target_os="macos"))]
pub fn get_user_and_group(pkg_install: &PackageInstall) -> Result<(String, String)> {
    if let Some((user, group)) = try!(check_pkg_user_and_group(&pkg_install)) {
        Ok((user, group))
    } else {
        let defaults = try!(get_default_user_and_group());
        Ok(defaults)
    }
}

#[cfg(any(target_os="linux", target_os="macos"))]
pub fn user_name_to_uid(user: &str) -> Option<u32> {
    users::get_user_by_name(user).map(|u| u.uid())
}

#[cfg(any(target_os="linux", target_os="macos"))]
pub fn group_name_to_gid(group: &str) -> Option<u32> {
    users::get_group_by_name(group).map(|g| g.gid())
}

#[cfg(target_os = "windows")]
pub fn get_user_and_group(pkg_install: &PackageInstall) -> Result<(String, String)> {
    unimplemented!();
}

