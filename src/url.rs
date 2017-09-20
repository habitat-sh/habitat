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

use env;

/// Default Builder URL environment variable
pub const BLDR_URL_ENVVAR: &'static str = "HAB_BLDR_URL";
/// Default Builder URL
pub const DEFAULT_BLDR_URL: &'static str = "https://bldr.habitat.sh";
/// Legacy environment variable for defining a default Builder endpoint
const LEGACY_BLDR_URL_ENVVAR: &'static str = "HAB_DEPOT_URL";

pub fn default_bldr_url() -> String {
    match env::var(BLDR_URL_ENVVAR) {
        Ok(val) => val,
        Err(_) => legacy_depot_url(),
    }
}

fn legacy_depot_url() -> String {
    match env::var(LEGACY_BLDR_URL_ENVVAR) {
        Ok(val) => val,
        Err(_) => DEFAULT_BLDR_URL.to_string(),
    }
}
