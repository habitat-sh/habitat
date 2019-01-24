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

use crate::env;

pub const UNSTABLE_CHANNEL: &str = "unstable";
pub const STABLE_CHANNEL: &str = "stable";

/// Default Builder Channel environment variable
pub const BLDR_CHANNEL_ENVVAR: &str = "HAB_BLDR_CHANNEL";
const LEGACY_CHANNEL_ENVVAR: &str = "HAB_DEPOT_CHANNEL";

/// Helper function for Builder dynamic channels
pub fn bldr_channel_name(id: u64) -> String {
    format!("bldr-{}", id)
}

/// Return the default release channel to use
pub fn default() -> String {
    match env::var(BLDR_CHANNEL_ENVVAR) {
        Ok(value) => value.to_string(),
        Err(_) => legacy_default(),
    }
}

fn legacy_default() -> String {
    env::var(LEGACY_CHANNEL_ENVVAR)
        .ok()
        .and_then(|c| Some(c.to_string()))
        .unwrap_or_else(|| STABLE_CHANNEL.to_string())
}
