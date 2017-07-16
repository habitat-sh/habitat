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

use hcore::os::users;

pub use sys::users::*;
use error::{Result, Error};

pub const DEFAULT_USER: &'static str = "hab";
pub const DEFAULT_GROUP: &'static str = "hab";
static LOGKEY: &'static str = "UR";

/// checks to see if hab/hab exists, if not, fall back to
/// current user/group. If that fails, then return an error.
pub fn default_user_and_group() -> Result<(String, String)> {
    let uid = users::get_uid_by_name(DEFAULT_USER);
    let gid = users::get_gid_by_name(DEFAULT_GROUP);
    match (uid, gid) {
        (Some(_), Some(_)) => return Ok((DEFAULT_USER.to_string(), DEFAULT_GROUP.to_string())),
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
                    return Err(sup_error!(Error::Permissions(
                        "Can't determine current user:group".to_string(),
                    )))
                }
            }
        }
    }
}
