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

use env as henv;

/// Default Depot URL
pub const DEFAULT_DEPOT_URL: &'static str = "https://willem.habitat.sh/v1/depot";

/// Default Depot channel
pub const DEFAULT_DEPOT_CHANNEL: &'static str = "unstable";

/// Default Depot publishing
pub const DEFAULT_DEPOT_PUBLISH: &'static str = "false";

/// Default Depot URL environment variable
pub const DEPOT_URL_ENVVAR: &'static str = "HAB_DEPOT_URL";

/// Default Depot Channel environment variable
pub const DEPOT_CHANNEL_ENVVAR: &'static str = "HAB_DEPOT_CHANNEL";

/// Default Depot Builder publishing environment variable
pub const DEPOT_PUBLISH_ENVVAR: &'static str = "HAB_DEPOT_PUBLISH";

pub fn default_depot_url() -> String {
    match henv::var(DEPOT_URL_ENVVAR) {
        Ok(val) => val,
        Err(_) => DEFAULT_DEPOT_URL.to_string(),
    }
}

pub fn default_depot_channel() -> String {
    match henv::var(DEPOT_CHANNEL_ENVVAR) {
        Ok(val) => val,
        Err(_) => DEFAULT_DEPOT_CHANNEL.to_string(),
    }
}

pub fn default_depot_publish() -> String {
    match henv::var(DEPOT_PUBLISH_ENVVAR) {
        Ok(val) => val,
        Err(_) => DEFAULT_DEPOT_PUBLISH.to_string(),
    }
}
