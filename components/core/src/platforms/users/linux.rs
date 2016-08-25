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

use linux_users;

pub fn get_uid_by_name(owner: &str) -> Option<u32> {
    linux_users::get_user_by_name(owner) .map(|u| u.uid())
}

pub fn get_gid_by_name(group: &str) -> Option<u32> {
    linux_users::get_group_by_name(&group.as_ref()) .map(|g| g.gid())
}

pub fn get_effective_uid() -> u32 {
    linux_users::get_effective_uid()
}