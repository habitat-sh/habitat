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

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::hcore::package::{PackageIdent, PackageInstall};

use crate::error::Result;

const BIN_PATH: &'static str = "/bin";

/// Returns the `bin` path used for symlinking programs.
pub fn bin_path() -> &'static Path {
    Path::new(BIN_PATH)
}

/// Returns the Package Identifier for a Busybox package.
#[cfg(unix)]
pub fn busybox_ident() -> Result<PackageIdent> {
    use super::BUSYBOX_IDENT;
    use std::str::FromStr;

    Ok(PackageIdent::from_str(BUSYBOX_IDENT)?)
}

/// Returns the path to a package prefix for the provided Package Identifier in a root file system.
///
/// # Errors
///
/// * If a package cannot be loaded from in the root file system
pub fn pkg_path_for<P: AsRef<Path>>(ident: &PackageIdent, rootfs: P) -> Result<PathBuf> {
    let pkg_install = PackageInstall::load(ident, Some(rootfs.as_ref()))?;
    Ok(Path::new("/").join(
        pkg_install
            .installed_path()
            .strip_prefix(rootfs.as_ref())
            .expect("installed path contains rootfs path"),
    ))
}

/// Writes a truncated/new file at the provided path with the provided content.
///
/// # Errors
///
/// * If an `IO` error occurs while creating, tuncating, writing, or closing the file
pub fn write_file<T>(file: T, content: &str) -> Result<()>
where
    T: AsRef<Path>,
{
    fs::create_dir_all(file.as_ref().parent().expect("Parent directory exists"))?;
    let mut f = File::create(file)?;
    f.write_all(content.as_bytes())?;
    Ok(())
}
