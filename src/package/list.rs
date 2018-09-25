// Copyright (c) 2016-2018 Chef Software Inc. and/or applicable contributors
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

use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::str::FromStr;

use super::metadata::{read_metafile, MetaFile};
use super::{PackageIdent, PackageTarget};

use error::Result;

pub const INSTALL_TMP_PREFIX: &'static str = ".hab-pkg-install";

/// Returns a list of package structs built from the contents of the given directory.
pub fn list(path: &Path) -> Result<Vec<PackageIdent>> {
    let mut package_list: Vec<PackageIdent> = vec![];
    if fs::metadata(path)?.is_dir() {
        walk_origins(&path, &mut package_list)?;
    }
    Ok(package_list)
}

/// Helper function for package_list. Walks the given path for origin directories
/// and builds on the given package list by recursing into name, version, and release
/// directories.
fn walk_origins(path: &Path, packages: &mut Vec<PackageIdent>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let origin = entry?;
        if fs::metadata(origin.path())?.is_dir() {
            walk_names(&origin, packages)?;
        }
    }
    Ok(())
}

/// Helper function for walk_origins. Walks the given origin DirEntry for name
/// directories and recurses into them to find version and release directories.
fn walk_names(origin: &DirEntry, packages: &mut Vec<PackageIdent>) -> Result<()> {
    for name in fs::read_dir(origin.path())? {
        let name = name?;
        let origin = origin
            .file_name()
            .to_string_lossy()
            .into_owned()
            .to_string();
        if fs::metadata(name.path())?.is_dir() {
            walk_versions(&origin, &name, packages)?;
        }
    }
    Ok(())
}

/// Helper function for walk_names. Walks the given name DirEntry for directories and recurses
/// into them to find release directories.
fn walk_versions(origin: &String, name: &DirEntry, packages: &mut Vec<PackageIdent>) -> Result<()> {
    for version in fs::read_dir(name.path())? {
        let version = version?;
        let name = name.file_name().to_string_lossy().into_owned().to_string();
        if fs::metadata(version.path())?.is_dir() {
            walk_releases(origin, &name, &version, packages)?;
        }
    }
    Ok(())
}

/// Helper function for walk_versions. Walks the given release DirEntry for directories and
/// recurses into them to find version directories. Finally, a Package struct is built and
/// concatenated onto the given packages vector with the origin, name, version, and release of
/// each.
fn walk_releases(
    origin: &String,
    name: &String,
    version: &DirEntry,
    packages: &mut Vec<PackageIdent>,
) -> Result<()> {
    let active_target = PackageTarget::active_target();

    for entry in fs::read_dir(version.path())? {
        let entry = entry?;
        if let Some(path) = entry.path().file_name().and_then(|f| f.to_str()) {
            if path.starts_with(INSTALL_TMP_PREFIX) {
                debug!(
                    "PackageInstall::walk_releases(): rejected PackageInstall candidate \
                     because it matches installation temporary directory prefix: {}",
                    path
                );
                continue;
            }
        }

        let metafile_content = read_metafile(entry.path(), &MetaFile::Target);
        // If there is an error reading the target metafile, then skip the candidate
        if let Err(e) = metafile_content {
            debug!(
                "PackageInstall::walk_releases(): rejected PackageInstall candidate \
                 due to error reading TARGET metafile, found={}, reason={:?}",
                entry.path().display(),
                e,
            );
            continue;
        }
        // Any errors have been cleared, so unwrap is safe
        let metafile_content = metafile_content.unwrap();
        let install_target = PackageTarget::from_str(&metafile_content);
        // If there is an error parsing the target as a valid PackageTarget, then skip the
        // candidate
        if let Err(e) = install_target {
            debug!(
                "PackageInstall::walk_releases(): rejected PackageInstall candidate \
                 due to error parsing TARGET metafile as a valid PackageTarget, \
                 found={}, reason={:?}",
                entry.path().display(),
                e,
            );
            continue;
        }
        // Any errors have been cleared, so unwrap is safe
        let install_target = install_target.unwrap();

        // Ensure that the installed package's target matches the active `PackageTarget`,
        // otherwise skip the candidate
        if active_target == &install_target {
            let release = entry.file_name().to_string_lossy().into_owned().to_string();
            let version = version
                .file_name()
                .to_string_lossy()
                .into_owned()
                .to_string();
            let ident =
                PackageIdent::new(origin.clone(), name.clone(), Some(version), Some(release));
            packages.push(ident)
        } else {
            debug!(
                "PackageInstall::walk_releases(): rejected PackageInstall candidate, \
                 found={}, installed_target={}, active_target={}",
                entry.path().display(),
                install_target,
                active_target,
            );
        }
    }
    Ok(())
}
