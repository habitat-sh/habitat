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

use users;
use users::os::unix::UserExt;

use {Error, Result};

pub fn check_running_user_is_root() -> Result<()> {
    if users::get_effective_uid() != 0 {
        return Err(Error::RootUserRequired);
    }
    Ok(())
}

pub fn gid_for_groupname(groupname: &str) -> Result<u32> {
    Ok(
        users::get_group_by_name(groupname)
            .ok_or(Error::GroupnameNotFound(String::from(groupname)))?
            .gid(),
    )
}

pub fn uid_for_username(username: &str) -> Result<u32> {
    Ok(user_by_username(username)?.uid())
}

pub fn primary_gid_for_username(username: &str) -> Result<u32> {
    Ok(user_by_username(username)?.primary_group_id())
}

// pub fn primary_groupname_for(username: &str) -> Result<String> {
//     let gid = primary_gid_for_username(username)?;
//     let group = users::get_group_by_gid(gid).ok_or(Error::GidNotFound(gid))?;

//     Ok(String::from(group.name()))
// }

pub fn home_dir_for_username(username: &str) -> Result<PathBuf> {
    Ok(PathBuf::from(user_by_username(username)?.home_dir()))
}

pub fn my_username() -> Result<String> {
    users::get_effective_username().ok_or(Error::EffectiveUsernameNotFound)
}

pub fn my_groupname() -> Result<String> {
    users::get_effective_groupname().ok_or(Error::EffectiveGroupnameNotFound)
}

fn user_by_username(username: &str) -> Result<users::User> {
    users::get_user_by_name(username).ok_or(Error::UsernameNotFound(String::from(username)))
}
