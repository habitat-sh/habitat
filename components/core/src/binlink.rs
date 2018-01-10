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

use env;

/// Default Binlink Dirs
pub const DEFAULT_BINLINK_DIR_1: &'static str = "/usr/local/bin";
pub const DEFAULT_BINLINK_DIR_2: &'static str = "/usr/bin";
pub const DEFAULT_BINLINK_DIR_3: &'static str = "/bin";

/// Binlink Dir Environment variable
pub const BINLINK_DIR_ENVVAR: &'static str = "HAB_BINLINK_DIR";

pub fn default_binlink_dir() -> String {
    match env::var(BINLINK_DIR_ENVVAR) {
        Ok(val) => val,
        Err(_) => fallback_binlib_dir(),
    }
}

fn fallback_binlib_dir() -> String {
    match env::var("PATH") {
        Ok(path) => {
            let path_members: Vec<&str> = path.split(':').collect();
            if path_members.contains(&DEFAULT_BINLINK_DIR_1) {
                DEFAULT_BINLINK_DIR_1.to_string()
            } else if path_members.contains(&DEFAULT_BINLINK_DIR_2) {
                DEFAULT_BINLINK_DIR_2.to_string()
            } else {
                DEFAULT_BINLINK_DIR_3.to_string()
            }
        }
        Err(_) => DEFAULT_BINLINK_DIR_3.to_string(),
    }
}
