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

use std::env;
use std::path::PathBuf;

use habitat_win_users::account::Account;

extern "C" {
    pub fn GetUserTokenStatus() -> u32;
}

fn get_sid_by_name(name: &str) -> Option<String> {
    match Account::from_name(name) {
        Some(acct) => {
            match acct.sid.to_string() {
                Ok(username) => Some(username),
                Err(_) => None,
            }
        }
        None => None,
    }
}

pub fn get_uid_by_name(owner: &str) -> Option<String> {
    get_sid_by_name(owner)
}

pub fn get_gid_by_name(group: &str) -> Option<String> {
    get_sid_by_name(group)
}

pub fn get_current_username() -> Option<String> {
    match env::var("USERNAME").ok() {
        Some(username) => Some(username.to_lowercase()),
        None => None,
    }
}

// this is a no-op on windows
pub fn get_current_groupname() -> Option<String> {
    Some(String::new())
}

pub fn get_effective_uid() -> u32 {
    unsafe { GetUserTokenStatus() }
}

pub fn get_home_for_user(username: &str) -> Option<PathBuf> {
    unimplemented!();
}

pub fn get_primary_gid_for_user(username: &str) -> Option<String> {
    None
}

pub fn root_level_account() -> String {
    env::var("COMPUTERNAME").unwrap().to_uppercase() + "$"
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn downcase_current_username() {
        env::set_var("USERNAME", "uSer");
        assert_eq!(get_current_username().unwrap(), "user")
    }

    #[test]
    fn return_none_when_no_user() {
        env::remove_var("USERNAME");
        assert_eq!(get_current_username(), None)
    }
}
