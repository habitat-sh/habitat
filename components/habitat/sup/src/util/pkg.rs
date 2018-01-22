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

use std::path::Path;

use common;
use common::command::package::install::InstallSource;
use common::ui::UI;
use hcore::env as henv;
use hcore::AUTH_TOKEN_ENVVAR;
use hcore::fs::{self, FS_ROOT_PATH};
use hcore::package::{PackageIdent, PackageInstall};

use {PRODUCT, VERSION};
use error::{Result, SupError};

/// Helper function for use in the Supervisor to handle lower-level
/// arguments needed for installing a package.
pub fn install(
    ui: &mut UI,
    url: &str,
    install_source: &InstallSource,
    channel: &str,
) -> Result<PackageInstall> {
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
    let auth_token = match henv::var(AUTH_TOKEN_ENVVAR) {
        Ok(v) => Some(v),
        Err(_) => None,
    };

    common::command::package::install::start(
        ui,
        url,
        // We currently need this to be an option due to how the depot
        // client is written. Anything that calls the current
        // function, though, should always have a channel. We should
        // push this "Option-ness" as far down the stack as we can,
        // with the ultimate goal of eliminating it altogether.
        Some(channel),
        install_source,
        PRODUCT,
        VERSION,
        fs_root_path,
        &fs::cache_artifact_path(None::<String>),
        auth_token.as_ref().map(String::as_str),
    ).map_err(SupError::from)
}

/// Returns an installed package for the given ident, if one is present.
pub fn installed(ident: &PackageIdent) -> Option<PackageInstall> {
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
    PackageInstall::load(ident, Some(fs_root_path)).ok()
}
