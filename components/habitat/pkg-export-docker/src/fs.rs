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

use std::path::{Path, PathBuf};
use hcore::fs::FS_ROOT_PATH;

// TODO fn: This implementation is vendored from `components/sup/src/fs.rs` in a effort to forego a
// dependency on the Supervisor codebase. Perhaps we should move the `sup::fs` module back into
// `core::fs` for some reusability?

lazy_static! {
    /// The root path containing all runtime service directories and files
    pub static ref SVC_ROOT: PathBuf = {
        Path::new(&*FS_ROOT_PATH).join("hab/svc")
    };
}

/// Returns the root path for a given service's configuration, files, and data.
pub fn svc_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    SVC_ROOT.join(service_name)
}

/// Returns the path to a given service's data.
pub fn svc_data_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("data")
}

/// Returns the path to the configuration directory for a given service.
pub fn svc_config_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("config")
}
