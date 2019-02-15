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

use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    command::package::install::{self, InstallHookMode, InstallMode, LocalPackageUsage},
    error::{Error, Result},
    ui,
};
use habitat_core::{
    fs::{cache_artifact_path, find_command, FS_ROOT_PATH},
    package::{PackageIdent, PackageInstall, PackageTarget},
    url::default_bldr_url,
    ChannelIdent, PROGRAM_NAME,
};

/// The package identifier for the OS specific interpreter which the Supervisor is built with,
/// or which may be independently installed
#[cfg(any(target_os = "linux", target_os = "macos"))]
const INTERPRETER_IDENT: &str = "core/busybox-static";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const INTERPRETER_COMMAND: &str = "busybox";

#[cfg(target_os = "windows")]
const INTERPRETER_IDENT: &str = "core/powershell";
#[cfg(target_os = "windows")]
const INTERPRETER_COMMAND: &str = "pwsh";

const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// Returns a list of path entries, one of which should contain the interpreter binary.
///
/// The Supervisor provides a minimal userland of commands to the supervised process. This includes
/// binaries such as `chpst` which can be used by a package's `run` hook.
///
/// There is a series of fallback strategies used here in order to find a usable interpreter
/// installation. The general strategy is the following:
///
/// * Are we (the Supervisor) running inside a package?
///     * Yes: use the interpreter release describes in our `DEPS` metafile & return its `PATH`
///       entries
///     * No
///         * Can we find any installed interpreter package?
///             * Yes: use the latest installed interpreter release & return its `PATH` entries
///             * No
///                 * Is the interpreter binary present on `$PATH`?
///                     * Yes: return the parent directory which holds the interpreter binary
///                     * No: out of ideas, so return an error after warning the user we're done
///
/// # Errors
///
/// * If an installed package should exist, but cannot be loaded
/// * If a installed package's path metadata cannot be read or returned
/// * If a known-working package identifier string cannot be parsed
/// * If the parent directory of a located interpreter binary cannot be computed
/// * If the Supervisor is not executing inside a package, and if no interpreter package is
///   installed, and if no interpreter binary can be found on the `PATH`
pub fn interpreter_paths() -> Result<Vec<PathBuf>> {
    // First, we'll check if we're running inside a package. If we are, then we should  be able to
    // access the `../DEPS` metadata file and read it to get the specific version of the
    // interpreter.
    let my_interpreter_dep_ident = match env::current_exe() {
        Ok(p) => match p.parent() {
            Some(p) => {
                let metafile = p.join("DEPS");
                if metafile.is_file() {
                    interpreter_dep_from_metafile(metafile)
                } else {
                    None
                }
            }
            None => None,
        },
        Err(_) => None,
    };
    let interpreter_paths: Vec<PathBuf> = match my_interpreter_dep_ident {
        // We've found the specific release that our Supervisor was built with. Get its path
        // metadata.
        Some(ident) => {
            let pkg_install = PackageInstall::load(&ident, Some(FS_ROOT_PATH.as_ref()))?;
            pkg_install.paths()?
        }
        // If we're not running out of a package, then see if any package of the interpreter is
        // installed.
        None => {
            let ident = PackageIdent::from_str(INTERPRETER_IDENT)?;
            match PackageInstall::load(&ident, Some(FS_ROOT_PATH.as_ref())) {
                // We found a version of the interpreter. Get its path metadata.
                Ok(pkg_install) => pkg_install.paths()?,
                // Nope, no packages of the interpreter installed. Now we're going to see if the
                // interpreter command is present on `PATH`.
                Err(_) => {
                    match find_command(INTERPRETER_COMMAND) {
                        // We found the interpreter on `PATH`, so that its `dirname` and return
                        // that.
                        Some(bin) => match bin.parent() {
                            Some(dir) => vec![dir.to_path_buf()],
                            None => {
                                let path = bin.to_string_lossy().into_owned();
                                println!(
                                    "An unexpected error has occurred. {} was found at {}, yet \
                                     the parent directory could not be computed. Aborting...",
                                    INTERPRETER_COMMAND, &path
                                );
                                return Err(Error::FileNotFound(path));
                            }
                        },
                        None => {
                            install::start(
                                &mut ui::UI::default_with_env(),
                                &default_bldr_url(),
                                Some(&ChannelIdent::stable()),
                                &(ident.clone(), *PackageTarget::active_target()).into(),
                                &*PROGRAM_NAME,
                                VERSION,
                                FS_ROOT_PATH.as_path(),
                                &cache_artifact_path(None::<String>),
                                None,
                                &InstallMode::default(),
                                &LocalPackageUsage::default(),
                                InstallHookMode::default(),
                            )?;
                            let pkg_install =
                                PackageInstall::load(&ident, Some(FS_ROOT_PATH.as_ref()))?;
                            pkg_install.paths()?
                        }
                    }
                }
            }
        }
    };
    Ok(interpreter_paths)
}

pub fn append_interpreter_and_path(path_entries: &mut Vec<PathBuf>) -> Result<String> {
    let mut paths = interpreter_paths()?;
    path_entries.append(&mut paths);
    if let Some(val) = env::var_os("PATH") {
        let mut os_paths = env::split_paths(&val).collect();
        path_entries.append(&mut os_paths);
    }
    let joined = env::join_paths(path_entries)?;
    let path_str = joined
        .into_string()
        .expect("Unable to convert OsStr path to string!");
    Ok(path_str)
}

/// Returns a `PackageIdent` for a interpreter package, assuming it exists in the provided metafile.
fn interpreter_dep_from_metafile(metafile: PathBuf) -> Option<PackageIdent> {
    let f = match File::open(metafile) {
        Ok(f) => f,
        Err(_) => return None,
    };
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return None,
        };
        if line.contains(INTERPRETER_IDENT) {
            match PackageIdent::from_str(&line) {
                Ok(pi) => return Some(pi),
                Err(_) => return None,
            }
        }
    }
    None
}
