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

use std::path::PathBuf;

use linux_users;
use linux_users::os::unix::{GroupExt, UserExt};

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
