// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Types and functions for representing the contents of Linux account files
//! (`/etc/passwd` and `/etc/group`) that need to be manipulated in
//! the process of creating a Docker export of a Habitat package.

use std::fmt;

/// Represents an entry for a user in `/etc/passwd`
#[derive(Debug)]
pub struct EtcPasswdEntry {
    pub name: String,
    pub uid: u32,
    // Primary GID
    pub gid: u32,
}

impl EtcPasswdEntry {
    pub fn new<S>(name: S, uid: u32, gid: u32) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            uid: uid,
            gid: gid,
        }
    }
}

impl fmt::Display for EtcPasswdEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{name}:x:{uid}:{gid}:{name} User:/:/bin/false",
            name = self.name,
            uid = self.uid,
            gid = self.gid
        )
    }
}

/// Represents an entry for a group in `/etc/group`
#[derive(Debug)]
pub struct EtcGroupEntry {
    pub name: String,
    pub gid: u32,
    pub users: Vec<String>,
}

impl EtcGroupEntry {
    pub fn empty_group<S>(name: S, gid: u32) -> Self
    where
        S: ToString,
    {
        let users: Vec<String> = vec![];
        Self::group_with_users(name, gid, users)
    }
    pub fn group_with_users<S, U>(name: S, gid: u32, users: Vec<U>) -> Self
    where
        S: ToString,
        U: ToString,
    {
        Self {
            name: name.to_string(),
            gid: gid,
            users: users.iter().map(|u| u.to_string()).collect(),
        }
    }
}

impl fmt::Display for EtcGroupEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{name}:x:{gid}:{users}",
            name = self.name,
            gid = self.gid,
            users = self.users.join(",")
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn etc_passwd_entry_renders_correctly() {
        let entry = EtcPasswdEntry::new("my_user", 123, 456);
        let rendered = format!("{}", entry);

        assert_eq!(rendered, "my_user:x:123:456:my_user User:/:/bin/false");
    }

    #[test]
    fn etc_group_entry_with_users_renders_correctly() {
        let entry = EtcGroupEntry::group_with_users("my_group", 456, vec!["larry", "moe", "curly"]);
        let rendered = format!("{}", entry);

        assert_eq!(rendered, "my_group:x:456:larry,moe,curly");
    }

    #[test]
    fn empty_etc_group_entry_renders_correctly() {
        let entry = EtcGroupEntry::empty_group("my_group", 567);
        let rendered = format!("{}", entry);

        assert_eq!(rendered, "my_group:x:567:");
    }
}
