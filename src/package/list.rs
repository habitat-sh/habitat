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
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::metadata::{read_metafile, MetaFile};
use super::{PackageIdent, PackageTarget};

use error::{Error, Result};

use tempdir::TempDir;

pub const INSTALL_TMP_PREFIX: &'static str = ".hab-pkg-install";

/// Return a directory which can be used as a temp dir during package install/
/// uninstall
///
/// It returns a path which is in the same parent directory as `path`
/// but with TempDir style randomization
pub fn temp_package_directory(path: &Path) -> Result<TempDir> {
    let base = path.parent().ok_or(Error::PackageUnpackFailed(
        "Could not determine parent directory for temporary package directory".to_owned(),
    ))?;
    fs::create_dir_all(base)?;
    let temp_install_prefix = path
        .file_name()
        .and_then(|f| f.to_str())
        .and_then(|dirname| Some(format!("{}-{}", INSTALL_TMP_PREFIX, dirname)))
        .ok_or(Error::PackageUnpackFailed(
            "Could not generate prefix for temporary package directory".to_owned(),
        ))?;
    Ok(TempDir::new_in(base, &temp_install_prefix)?)
}

/// Returns a list of package structs built from the contents of the given directory.
pub fn all_packages(path: &Path) -> Result<Vec<PackageIdent>> {
    let mut package_list: Vec<PackageIdent> = vec![];
    if fs::metadata(path)?.is_dir() {
        walk_origins(&path, &mut package_list)?;
    }
    Ok(package_list)
}

/// Returns a vector of package structs built from the contents of
/// the given directory, using the given ident to restrict the
/// search.
///
/// The search is restricted by assuming the package directory
/// structure is:
///
///    /base/ORIGIN/NAME/VERSION/RELEASE/
///
pub fn package_list_for_ident(
    base_pkg_path: &Path,
    ident: &PackageIdent,
) -> Result<Vec<PackageIdent>> {
    let mut package_list: Vec<PackageIdent> = vec![];
    let mut package_path = PathBuf::from(base_pkg_path);
    package_path.push(&ident.origin);
    package_path.push(&ident.name);

    if !is_existing_dir(&package_path)? {
        return Ok(package_list);
    }

    match (&ident.version, &ident.release) {
        // origin/name
        (None, _) => walk_versions(&ident.origin, &ident.name, &package_path, &mut package_list)?,
        // origin/name/version
        (Some(version), None) => {
            package_path.push(version);
            if !is_existing_dir(&package_path)? {
                return Ok(package_list);
            }
            walk_releases(
                &ident.origin,
                &ident.name,
                &version,
                &package_path,
                &mut package_list,
            )?
        }
        // origin/name/version/release
        (Some(version), Some(release)) => {
            package_path.push(version);
            package_path.push(release);
            if !is_existing_dir(&package_path)? {
                return Ok(package_list);
            }

            let active_target = PackageTarget::active_target();
            if let Some(new_ident) = package_ident_from_dir(
                &ident.origin,
                &ident.name,
                &version,
                active_target,
                &package_path,
            ) {
                package_list.push(new_ident.clone())
            }
        }
    }
    Ok(package_list)
}

/// Helper function for all_packages. Walks the directory at the given
/// Path for origin directories and builds on the given package list
/// by recursing into name, version, and release directories.
fn walk_origins(path: &Path, packages: &mut Vec<PackageIdent>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let origin_dir = entry?;
        let origin_path = origin_dir.path();
        if fs::metadata(&origin_path)?.is_dir() {
            let origin = filename_from_entry(origin_dir);
            walk_names(&origin, &origin_path, packages)?;
        }
    }
    Ok(())
}

/// Helper function for walk_origins. Walks the direcotry at the given
/// Path for name directories and recurses into them to find version
/// and release directories.
fn walk_names(origin: &String, dir: &Path, packages: &mut Vec<PackageIdent>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let name_dir = entry?;
        let name_path = name_dir.path();
        if fs::metadata(&name_path)?.is_dir() {
            let name = filename_from_entry(name_dir);
            walk_versions(&origin, &name, &name_path, packages)?;
        }
    }
    Ok(())
}

/// Helper function for walk_names. Walks the directory at the given
/// Path and recurses into them to find release directories.
fn walk_versions(
    origin: &String,
    name: &String,
    dir: &Path,
    packages: &mut Vec<PackageIdent>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let version_dir = entry?;
        let version_path = version_dir.path();
        if fs::metadata(&version_path)?.is_dir() {
            let version = filename_from_entry(version_dir);
            walk_releases(origin, name, &version, &version_path, packages)?;
        }
    }
    Ok(())
}

/// Helper function for walk_versions. Walks the directory at the
/// given Path and constructs a Package struct if the directory is a
/// valid package directory. Any resulting packages are pushed onto
/// the given packages vector, assuming the given origin, name, and
/// version.
fn walk_releases(
    origin: &String,
    name: &String,
    version: &String,
    dir: &Path,
    packages: &mut Vec<PackageIdent>,
) -> Result<()> {
    let active_target = PackageTarget::active_target();
    for entry in fs::read_dir(dir)? {
        let release_dir = entry?;
        let release_path = release_dir.path();
        if fs::metadata(&release_path)?.is_dir() {
            if let Some(ident) =
                package_ident_from_dir(origin, name, version, active_target, &release_path)
            {
                packages.push(ident)
            }
        }
    }
    Ok(())
}

/// package_ident_from_dir returns a PackageIdent if the given
/// path contains a valid package for the given active_target.
///
/// Returns None when
///    - The directory is a temporary install directroy
///    - An error occurs reading the package metadata
///    - An error occurs reading the package target
///    - The package target doesn't match the given active target
fn package_ident_from_dir(
    origin: &String,
    name: &String,
    version: &String,
    active_target: &PackageTarget,
    dir: &Path,
) -> Option<PackageIdent> {
    let release = if let Some(rel) = dir.file_name().and_then(|f| f.to_str()) {
        rel
    } else {
        return None;
    };

    if release.starts_with(INSTALL_TMP_PREFIX) {
        debug!(
            "PackageInstall::package_ident_from_dir(): rejected PackageInstall candidate \
             because it matches installation temporary directory prefix: {}",
            dir.display()
        );
        return None;
    }

    let metafile_content = read_metafile(dir, &MetaFile::Target);
    // If there is an error reading the target metafile, then skip the candidate
    if let Err(e) = metafile_content {
        debug!(
            "PackageInstall::package_ident_from_dir(): rejected PackageInstall candidate \
             due to error reading TARGET metafile, found={}, reason={:?}",
            dir.display(),
            e,
        );
        return None;
    }

    // Any errors have been cleared, so unwrap is safe
    let metafile_content = metafile_content.unwrap();
    let install_target = PackageTarget::from_str(&metafile_content);
    // If there is an error parsing the target as a valid PackageTarget, then skip the
    // candidate
    if let Err(e) = install_target {
        debug!(
            "PackageInstall::package_ident_from_dir(): rejected PackageInstall candidate \
             due to error parsing TARGET metafile as a valid PackageTarget, \
             found={}, reason={:?}",
            dir.display(),
            e,
        );
        return None;
    }
    // Any errors have been cleared, so unwrap is safe
    let install_target = install_target.unwrap();

    // Ensure that the installed package's target matches the active `PackageTarget`,
    // otherwise skip the candidate
    if active_target == &install_target {
        return Some(PackageIdent::new(
            origin.clone(),
            name.clone(),
            Some(version.clone()),
            Some(release.to_owned()),
        ));
    } else {
        debug!(
            "PackageInstall::package_ident_from_dir(): rejected PackageInstall candidate, \
             found={}, installed_target={}, active_target={}",
            dir.display(),
            install_target,
            active_target,
        );
        return None;
    }
}

fn filename_from_entry(entry: fs::DirEntry) -> String {
    return entry.file_name().to_string_lossy().into_owned().to_string();
}

fn is_existing_dir(path: &Path) -> Result<bool> {
    match fs::metadata(&path) {
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                return Ok(false);
            }
            return Err(Error::from(err));
        }
        Ok(metadata) => return Ok(metadata.is_dir()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use package::test_support::testing_package_install;

    use fs;
    use std::fs::File;
    use tempdir::TempDir;

    #[test]
    fn empty_dir_gives_empty_list() {
        let package_root = TempDir::new("fs-root").unwrap();
        let packages = all_packages(&package_root.path()).unwrap();

        assert_eq!(0, packages.len());
    }

    #[test]
    fn not_a_dir_gives_empty_list() {
        let fs_root = TempDir::new("fs-root").unwrap();
        let file_path = fs_root.path().join("not-a-dir");
        let _tmp_file = File::create(&file_path).unwrap();

        let packages = all_packages(&file_path).unwrap();

        assert_eq!(0, packages.len());
    }
    #[test]
    fn can_read_single_package() {
        let fs_root = TempDir::new("fs-root").unwrap();
        let package_root = fs::pkg_root_path(Some(fs_root.path()));
        let package_install = testing_package_install("core/redis", fs_root.path());

        let packages = all_packages(&package_root).unwrap();

        assert_eq!(1, packages.len());
        assert_eq!(package_install.ident, packages[0]);
    }

    #[test]
    fn can_read_multiple_packages() {
        let fs_root = TempDir::new("fs-root").unwrap();
        let package_root = fs::pkg_root_path(Some(fs_root.path()));
        let expected = vec![
            testing_package_install("core/redis/1.0.0", fs_root.path()),
            testing_package_install("test/foobar", fs_root.path()),
            testing_package_install("core/redis/1.1.0", fs_root.path()),
        ];

        let packages = all_packages(&package_root).unwrap();

        assert_eq!(3, packages.len());
        for p in &expected {
            assert!(packages.contains(&p.ident));
        }
    }

    #[test]
    fn create_temp_package_directory_in_same_parentdir() {
        let p = Path::new("/tmp/foo");
        let temp_dir = temp_package_directory(&p).unwrap();
        let temp_path = temp_dir.path();

        assert_eq!(&p.parent(), &temp_path.parent());
    }

    #[test]
    fn temp_package_directory_starts_with_prefix() {
        let p = Path::new("/tmp/foo");
        let temp_dir = temp_package_directory(&p).unwrap();
        let temp_filename = temp_dir.path().file_name().unwrap().to_str().unwrap();

        assert!(&temp_filename.starts_with(INSTALL_TMP_PREFIX));
    }

    #[test]
    fn temp_package_directory_changes() {
        let p = Path::new("/tmp/foo");
        let temp_dir1 = temp_package_directory(&p).unwrap();
        let temp_dir2 = temp_package_directory(&p).unwrap();

        assert_ne!(&temp_dir1.path(), &temp_dir2.path());
    }
}
