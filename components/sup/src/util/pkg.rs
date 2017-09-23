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
use hcore::fs::{self, FS_ROOT_PATH};
use hcore::package::PackageInstall;

use {PRODUCT, VERSION};
use error::{Result, SupError};

// NOOOO, this is just for the sup crate... but we also do this in
// hab, too! AAAAAAAA
/// Helper function for use in the Supervisor to handle lower-level
/// arguments needed for installing a package.
pub fn install(
    ui: &mut UI,
    url: &str,
    install_source: &InstallSource,
    channel: &str,
) -> Result<PackageInstall> {
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
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
    ).map_err(SupError::from)
}
