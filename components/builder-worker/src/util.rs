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

use std::path::{Path, PathBuf};
use std::str::FromStr;

use hab_core::fs::{self, FS_ROOT_PATH};
use hab_core::package::{PackageIdent, PackageInstall};

/// Resolves the absolute path to a program in the given package identifier string.
///
/// Note: this function is designed to be callable in `lazy_static!` blocks, meaning that if it
/// can't make forward progress, it will panic and possibly termine the program. This is by design.
///
/// # Panics
///
/// * If the installed package can't be loaded off disk
/// * If the the program can't be found in the installed package
/// * If there is an error looking for the program in the installed package
pub fn resolve_cmd_in_pkg(program: &str, ident_str: &str) -> PathBuf {
    let ident = PackageIdent::from_str(ident_str).unwrap();
    let abs_path = match PackageInstall::load(&ident, None) {
        Ok(ref pkg_install) => {
            match fs::find_command_in_pkg(program, pkg_install, Path::new(&*FS_ROOT_PATH)) {
                Ok(Some(p)) => p,
                Ok(None) => {
                    panic!(format!(
                        "Could not find '{}' in the '{}' package! This is required for the \
                        proper operation of this program.",
                        program,
                        &ident
                    ))
                }
                Err(err) => {
                    panic!(format!(
                        "Error finding '{}' in the '{}' package! This is required for the \
                        proper operation of this program. (Err: {:?})",
                        program,
                        &ident,
                        err
                    ))
                }
            }
        }
        Err(err) => {
            panic!(format!(
                "Habitat Studio package '{}' installation not found! This must be available \
                as it is a runtime dependency in the worker's package. (Err: {:?})",
                &ident,
                err
            ))
        }
    };
    debug!(
        "resolved absolute path to program, program={}, ident={}, abs_path={}",
        program,
        &ident,
        abs_path.display()
    );
    abs_path
}
