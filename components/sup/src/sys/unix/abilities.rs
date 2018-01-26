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

//! Provides functions for determining if the current user has certain
//! abilities based on querying the available Linux capabilities; the
//! functioning of the Supervisor may change depending on the answer.

use caps::{self, Capability, CapSet};

/// Returns true if the current thread is able to set both user and
/// group IDs of processes.
pub fn can_set_process_user_and_group() -> bool {
    has(Capability::CAP_SETUID) && has(Capability::CAP_SETGID)
}

/// Returns true if the current thread is able to change ownership of
/// files.
pub fn can_change_ownership() -> bool {
    has(Capability::CAP_CHOWN)
}

/// Helper function; does the current thread have `cap` in its
/// effective capability set?
fn has(cap: Capability) -> bool {
    caps::has_cap(None, CapSet::Effective, cap).unwrap_or(false)
}
