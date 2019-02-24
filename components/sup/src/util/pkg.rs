// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

use crate::{common::{self,
                     command::package::install::{InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     ui::UIWriter},
            hcore::{env as henv,
                    fs::{self,
                         FS_ROOT_PATH},
                    package::{PackageIdent,
                              PackageInstall},
                    ChannelIdent,
                    AUTH_TOKEN_ENVVAR}};

use crate::{error::{Error,
                    Result,
                    SupError},
            PRODUCT,
            VERSION};

static LOGKEY: &'static str = "UT";

/// Helper function for use in the Supervisor to handle lower-level
/// arguments needed for installing a package.
pub fn install<T>(
    ui: &mut T,
    url: &str,
    install_source: &InstallSource,
    channel: &ChannelIdent,
) -> Result<PackageInstall>
where
    T: UIWriter,
{
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
        channel,
        install_source,
        PRODUCT,
        VERSION,
        fs_root_path,
        &fs::cache_artifact_path(None::<String>),
        auth_token.as_ref().map(String::as_str),
        // TODO fn: pass through and enable offline install mode
        &InstallMode::default(),
        // TODO (CM): pass through and enable ignore-local mode
        &LocalPackageUsage::default(),
        // Install hooks are run when the supervisor loads the package
        // in add_service so it is repetitive to run them here
        InstallHookMode::Ignore,
    )
    .map_err(SupError::from)
}

/// Given an InstallSource, install a new package only if an existing
/// one that can satisfy the package identifier is not already
/// present.
///
/// Return the PackageInstall corresponding to the package that was
/// installed, or was pre-existing.
pub fn satisfy_or_install<T>(
    ui: &mut T,
    install_source: &InstallSource,
    bldr_url: &str,
    channel: &ChannelIdent,
) -> Result<PackageInstall>
where
    T: UIWriter,
{
    match installed(install_source) {
        Some(package) => Ok(package),
        None => install(ui, bldr_url, install_source, channel),
    }
    .and_then(|installed| {
        if installed.is_runnable() {
            Ok(installed)
        } else {
            outputln!("Can't start non-runnable service: {}", installed.ident());
            Err(sup_error!(Error::PackageNotRunnable(
                installed.ident().clone()
            )))
        }
    })
}

/// Returns an installed package for the given ident, if one is present.
pub fn installed<T>(ident: T) -> Option<PackageInstall>
where
    T: AsRef<PackageIdent>,
{
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
    PackageInstall::load(ident.as_ref(), Some(fs_root_path)).ok()
}
