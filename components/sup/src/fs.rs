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

use crate::hcore::fs::FS_ROOT_PATH;
use std::path::{Path, PathBuf};

lazy_static! {
    /// The root path containing all runtime service directories and files
    pub static ref SVC_ROOT: PathBuf = {
        Path::new(&*FS_ROOT_PATH).join("hab").join("svc")
    };

    pub static ref USER_ROOT: PathBuf = {
        Path::new(&*FS_ROOT_PATH).join("hab").join("user")
    };
}

/// Returns the root path for a given service's configuration, files, and data.
pub fn svc_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    SVC_ROOT.join(service_name)
}

/// Returns the path to the configuration directory for a given service.
pub fn svc_config_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("config")
}

/// Returns the path to a given service's data.
pub fn svc_data_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("data")
}

/// Returns the path to a given service's gossiped config files.
pub fn svc_files_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("files")
}

/// Returns the path to a given service's hooks.
pub fn svc_hooks_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("hooks")
}

/// Returns the path to a given service's static content.
pub fn svc_static_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("static")
}

/// Returns the path to a given service's variable state.
pub fn svc_var_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("var")
}

/// Returns the path to a given service's logs.
pub fn svc_logs_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("logs")
}

/// Returns the path to a given service's pid file.
pub fn svc_pid_file<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("PID")
}

/// Returns the root path for a given service's user configuration,
/// files, and data.
pub fn user_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    USER_ROOT.join(service_name)
}

/// Returns the path to a given service's user configuration
/// directory.
pub fn user_config_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    user_path(service_name).join("config")
}
