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

mod listen_ctl_addr;

pub use self::listen_ctl_addr::ListenCtlAddr;

/// Bundles up information about the user and group that a supervised
/// service should be run as. If the Supervisor itself is running with
/// root-like permissions, then these will be for `SVC_USER` and
/// `SVC_GROUP` for a service. If not, it will be for the user the
/// Supervisor itself is running as.
///
/// On Windows, all but `username` will be `None`. On Linux,
/// `username` and `groupname` may legitimately be `None`, but `uid`
/// and `gid` should always be `Some`.
#[cfg(unix)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserInfo {
    pub username: Option<String>,
    /// Linux preferred
    pub uid: u32,
    pub groupname: Option<String>,
    /// Linux preferred
    pub gid: u32,
}

#[cfg(windows)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserInfo {
    pub username: String,
}
