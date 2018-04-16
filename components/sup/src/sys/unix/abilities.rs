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

pub use self::imp::*;

#[cfg(target_os = "linux")]
mod imp {
    use caps::{self, CapSet, Capability};

    /// This is currently the "master check" for whether the Supervisor
    /// can behave "as root".
    ///
    /// All capabilities must be present. If we can run processes as other
    /// users, but can't change ownership, then the processes won't be
    /// able to access their files. Similar logic holds for the reverse.
    pub fn can_run_services_as_svc_user() -> bool {
        has(Capability::CAP_SETUID) && has(Capability::CAP_SETGID) && has(Capability::CAP_CHOWN)
    }

    /// Helper function; does the current thread have `cap` in its
    /// effective capability set?
    fn has(cap: Capability) -> bool {
        caps::has_cap(None, CapSet::Effective, cap).unwrap_or(false)
    }
}

#[cfg(target_os = "macos")]
mod imp {
    pub fn can_run_services_as_svc_user() -> bool {
        true
    }
}
