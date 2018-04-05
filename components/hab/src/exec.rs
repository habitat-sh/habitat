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

use common;
use common::ui::{Status, UI};
use common::command::package::install::InstallMode;
use hcore::{self, channel};
use hcore::env as henv;
use hcore::fs::{self, cache_artifact_path};
use hcore::package::{PackageIdent, PackageInstall};
use hcore::url::default_bldr_url;

use {PRODUCT, VERSION};
use error::{Error, Result};

const MAX_RETRIES: u8 = 4;
const INTERNAL_TOOLING_CHANNEL_ENVVAR: &'static str = "HAB_INTERNAL_BLDR_CHANNEL";

/// Returns the absolute path to the given command from a package no
/// older than the given package identifier.
///
/// If the package is not locally installed, the package will be
/// installed before recomputing.  There are a maximum number of times
/// a re-installation will be attempted before returning an error.
///
/// # Notes on Package Installation
///
/// By default, Habitat will install packages from the `stable`
/// channel. However, if you'd rather use unstable (particularly if
/// you're developing Habitat), you'll need to set the
/// `INTERNAL_TOOLING_CHANNEL_ENVVAR` appropriately.
///
/// Note that this environment variable *only* applies to packages
/// installed through this function. As a result, this function should
/// only be used to install Habitat packages (i.e. things Habitat
/// itself needs to run), and not arbitrary user packages. This allows
/// users to fine-tune where packages come from.
///
/// Also note that due to the "minimum package" logic, this overriding
/// of the internal tooling channel logic is really only called for
/// when first encountering a given package; thereafter, we can use
/// whatever is on disk already. This provides another mechanism by
/// which you can influence what packages are used: simply install a
/// newer one.
///
/// # Failures
///
/// * If the package is installed but the command cannot be found in
///   the package
/// * If an error occurs when loading the local package from disk
/// * If the maximum number of installation retries has been exceeded
pub fn command_from_min_pkg<T>(
    ui: &mut UI,
    command: T,
    ident: &PackageIdent,
    cache_key_path: &Path,
    retry: u8,
) -> Result<PathBuf>
where
    T: Into<PathBuf>,
{
    let command = command.into();
    if retry > MAX_RETRIES {
        return Err(Error::ExecCommandNotFound(command));
    }

    let fs_root_path = Path::new("/");
    match PackageInstall::load_at_least(ident, None) {
        Ok(pi) => {
            match fs::find_command_in_pkg(&command, &pi, fs_root_path)? {
                Some(cmd) => Ok(cmd),
                None => Err(Error::ExecCommandNotFound(command)),
            }
        }
        Err(hcore::Error::PackageNotFound(_)) => {
            ui.status(
                Status::Missing,
                format!("package for {}", &ident),
            )?;

            // JB TODO - Does an auth token need to be plumbed into here?  Not 100% sure.
            common::command::package::install::start(
                ui,
                &default_bldr_url(),
                Some(&internal_tooling_channel()),
                &ident.clone().into(),
                PRODUCT,
                VERSION,
                fs_root_path,
                &cache_artifact_path(None::<String>),
                None,
                // TODO fn: pass through and enable offline install mode
                &InstallMode::default(),
            )?;
            command_from_min_pkg(ui, &command, &ident, &cache_key_path, retry + 1)
        }
        Err(e) => Err(Error::from(e)),
    }
}

/// Determine the channel from which to install Habitat-specific
/// packages.
fn internal_tooling_channel() -> String {
    match henv::var(INTERNAL_TOOLING_CHANNEL_ENVVAR) {
        Ok(channel) => channel,
        Err(_) => channel::STABLE_CHANNEL.to_string(),
    }
}
