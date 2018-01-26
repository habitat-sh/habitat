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

//! Windows-equivalent for determining if the current user has certain
//! abilities.

// The Linux version uses Capabilities. Until we sort out the
// equivalent implementation on Windows, we assume that the current
// process has the abilities. This was the implicit behavior prior to
// adding this abstraction, so Windows supervisor behavior will remain
// unchanged (i.e., it will still require "root"-like abilities to
// run).

pub fn can_set_process_user_and_group() -> bool {
    true
}

pub fn can_change_ownership() -> bool {
    true
}
