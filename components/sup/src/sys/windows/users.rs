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

use crate::error::{Error, Result};

static LOGKEY: &'static str = "UR";

/// Windows does not have a concept of "group" in a Linux sense
/// So we just validate the user
pub fn assert_pkg_user_and_group(user: &str, _group: &str) -> Result<()> {
    match users::get_uid_by_name(user) {
        Some(_) => Ok(()),
        None => Err(sup_error!(Error::Permissions(format!(
            "Package requires user {} to exist, but it doesn't",
            user
        )))),
    }
}
