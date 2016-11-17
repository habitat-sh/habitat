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

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;

use hcore::fs::find_command;
use hcore::package::{PackageIdent, PackageInstall};

use error::{Error, Result};

static LOGKEY: &'static str = "PT";

/// The package identifier for BusyBox which the Supervisor is built with, or which may be
/// independently installed
const BUSYBOX_IDENT: &'static str = "core/busybox-static";

/// Returns a list of path entries, one of which should contain the BusyBox binary.
///
/// The Supervisor provides a minimal userland of commands to the supervised process. This includes
/// binaries such as `chpst` which can be used by a package's `run` hook.
///
/// There is a series of fallback strategies used here in order to find a usable BusyBox
/// installation. The general strategy is the following:
///
/// * Are we (the Supervisor) running inside a package?
///     * Yes: use the BusyBox release describes in our `DEPS` metafile & return its `PATH` entries
///     * No
///         * Can we find any installed BusyBox pacakge?
///             * Yes: use the latest installed BusyBox release & return its `PATH` entries
///             * No
///                 * Is the `busybox` binary present on `$PATH`?
///                     * Yes: return the parent directory which holds the `busybox` binary
///                     * No: out of ideas, so return an error after warning the user we're done
///
/// # Errors
///
/// * If an installed package should exist, but cannot be loaded
/// * If a installed package's path metadata cannot be read or returned
/// * If a known-working package identifier string cannot be parsed
/// * If the parent directory of a located `busybox` binary cannot be computed
/// * If the Supervisor is not executing inside a packge, and if no BusyBox package is installed,
///   and if no `busybox` binary can be found on the `PATH`
#[cfg(any(target_os="linux", target_os="macos"))]
pub fn interpreter_paths() -> Result<Vec<PathBuf>> {
    // First, we'll check if we're running inside a package. If we are, then we should  be able to
    // access the `../DEPS` metadata file and read it to get the specific version of BusyBox.
    let my_busybox_dep_ident = match env::current_exe() {
        Ok(p) => {
            match p.parent() {
                Some(p) => {
                    let metafile = p.join("DEPS");
                    if metafile.is_file() {
                        busybox_dep_from_metafile(metafile)
                    } else {
                        None
                    }
                }
                None => None,
            }
        }
        Err(_) => None,
    };
    let bb_paths: Vec<PathBuf> = match my_busybox_dep_ident {
        // We've found the specific release that our supervisor was built with. Get its path
        // metadata.
        Some(ident) => {
            let pkg_install = try!(PackageInstall::load(&ident, None));
            try!(pkg_install.paths())
        }
        // If we're not running out of a package, then see if any package of BusyBox is installed.
        None => {
            let ident = try!(PackageIdent::from_str(BUSYBOX_IDENT));
            match PackageInstall::load(&ident, None) {
                // We found a version of BusyBox. Get its path metadata.
                Ok(pkg_install) => try!(pkg_install.paths()),
                // Nope, no packages of BusyBox installed. Now we're going to see if the `busybox`
                // command is present on `PATH`.
                Err(_) => {
                    match find_command("busybox") {
                        // We found `busybox` on `PATH`, so that its `dirname` and return that.
                        Some(bin) => {
                            match bin.parent() {
                                Some(dir) => vec![dir.to_path_buf()],
                                None => {
                                    let path = bin.to_string_lossy().into_owned();
                                    outputln!("An unexpected error has occured. BusyBox was \
                                               found at {}, yet the parent directory could not \
                                               be computed. Aborting...",
                                              &path);
                                    return Err(sup_error!(Error::FileNotFound(path)));
                                }
                            }
                        }
                        // Well, we're not running out of a pacakge, there is no BusyBox package
                        // installed, it's not on `PATH`, what more can we do. Time to give up the
                        // chase. Too bad, we were really trying to be helpful here.
                        None => {
                            outputln!("A BusyBox installation is required but could not be \
                                       found. Please install 'core/busybox-static' or put the \
                                       'busybox' command on your $PATH. Aborting...");
                            return Err(sup_error!(Error::PackageNotFound(ident)));
                        }
                    }
                }
            }
        }
    };
    Ok(bb_paths)
}

#[cfg(target_os = "windows")]
pub fn interpreter_paths() -> Result<Vec<PathBuf>> {
    let empty: Vec<PathBuf> = Vec::new();
    Ok(empty)
}

/// Returns a `PackageIdent` for a BusyBox package, assuming it exists in the provided metafile.
fn busybox_dep_from_metafile(metafile: PathBuf) -> Option<PackageIdent> {
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
        if line.contains(BUSYBOX_IDENT) {
            match PackageIdent::from_str(&line) {
                Ok(pi) => return Some(pi),
                Err(_) => return None,
            }
        }
    }
    None
}
