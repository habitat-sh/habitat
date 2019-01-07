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

use crate::hcore::os::users;

use crate::error::{Error, Result};

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
