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

use std;
use std::collections::HashSet;
use std::cmp::{Ordering, PartialOrd};
use std::env;
use std::fs::{DirEntry, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use error::{Error, Result};
use fs::{self, PKG_PATH};
use package::{Identifiable, MetaFile, PackageIdent, Target, PackageTarget};

#[derive(Clone, Debug)]
pub struct PackageInstall {
    ident: PackageIdent,
    fs_root_path: PathBuf,
    package_root_path: PathBuf,
    installed_path: PathBuf,
}

impl PackageInstall {
    /// Verifies an installation of a package is within the package path and returns a struct
    /// representing that package installation.
    ///
    /// Only the origin and name of a package are required - the latest version/release of a
    /// package will be returned if their optional value is not specified. If only a version is
    /// specified, the latest release of that package origin, name, and version is returned.
    ///
    /// An optional `fs_root` path may be provided to search for a package that is mounted on a
    /// filesystem not currently rooted at `/`.
    pub fn load(ident: &PackageIdent, fs_root_path: Option<&Path>) -> Result<PackageInstall> {
        let package_install = try!(Self::resolve_package_install(ident, fs_root_path));
        let package_target = try!(package_install.target());
        match package_target.validate() {
            Ok(()) => Ok(package_install),
            Err(e) => Err(e),
        }
    }

    fn resolve_package_install(ident: &PackageIdent,
                               fs_root_path: Option<&Path>)
                               -> Result<PackageInstall> {
        let fs_root_path = fs_root_path.unwrap_or(Path::new("/"));
        let package_root_path = fs_root_path.join(PKG_PATH);
        if !package_root_path.exists() {
            return Err(Error::PackageNotFound(ident.clone()));
        }
        let pl = try!(Self::package_list(&package_root_path));
        if ident.fully_qualified() {
            if pl.iter().any(|ref p| p.satisfies(ident)) {
                Ok(PackageInstall {
                    ident: ident.clone(),
                    fs_root_path: PathBuf::from(fs_root_path),
                    package_root_path: package_root_path.clone(),
                    installed_path: try!(Self::calc_installed_path(ident, &package_root_path)),
                })
            } else {
                Err(Error::PackageNotFound(ident.clone()))
            }
        } else {
            let latest: Option<PackageIdent> = pl.iter()
                .filter(|&p| p.satisfies(ident))
                .fold(None, |winner, b| {
                    match winner {
                        Some(a) => {
                            match a.partial_cmp(&b) {
                                Some(Ordering::Greater) => Some(a),
                                Some(Ordering::Equal) => Some(a),
                                Some(Ordering::Less) => Some(b.clone()),
                                None => Some(a),
                            }
                        }
                        None => Some(b.clone()),
                    }
                });
            if let Some(id) = latest {
                Ok(PackageInstall {
                    ident: id.clone(),
                    fs_root_path: PathBuf::from(fs_root_path),
                    package_root_path: package_root_path.clone(),
                    installed_path: try!(Self::calc_installed_path(&id, &package_root_path)),
                })
            } else {
                Err(Error::PackageNotFound(ident.clone()))
            }
        }
    }

    pub fn new_from_parts(ident: PackageIdent,
                          fs_root_path: PathBuf,
                          package_root_path: PathBuf,
                          installed_path: PathBuf)
                          -> PackageInstall {
        PackageInstall {
            ident: ident,
            fs_root_path: fs_root_path,
            package_root_path: package_root_path,
            installed_path: installed_path,
        }
    }

    pub fn deps(&self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::Deps)
    }

    pub fn tdeps(&self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::TDeps)
    }

    /// A vector of ports we expose
    pub fn exposes(&self) -> Result<Vec<String>> {
        match self.read_metafile(MetaFile::Exposes) {
            Ok(body) => {
                let v: Vec<String> = body.split(' ')
                    .map(|x| String::from(x.trim_right_matches('\n')))
                    .collect();
                Ok(v)
            }
            Err(Error::MetaFileNotFound(MetaFile::Exposes)) => {
                let v: Vec<String> = Vec::new();
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    pub fn ident(&self) -> &PackageIdent {
        &self.ident
    }

    /// Return the PATH string from the package metadata, if it exists
    ///
    /// # Failures
    ///
    /// * The package contains a Path metafile but it could not be read or it was malformed
    pub fn paths(&self) -> Result<Vec<PathBuf>> {
        match self.read_metafile(MetaFile::Path) {
            Ok(body) => {
                let v = env::split_paths(&body).map(|p| PathBuf::from(&p)).collect();
                Ok(v)
            }
            Err(Error::MetaFileNotFound(MetaFile::Path)) => {
                let v: Vec<PathBuf> = Vec::new();
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns a `String` with the full run path for this package. The `PATH` string will be
    /// constructed by add all `PATH` metadata entries from the *direct* dependencies first (in
    /// declared order) and then from any remaining transitive dependencies last (in lexically
    /// sorted order).
    pub fn runtime_path(&self) -> Result<String> {
        let mut idents = HashSet::new();
        let mut run_paths: Vec<PathBuf> = Vec::new();

        let mut p = try!(self.paths());
        run_paths.append(&mut p);
        idents.insert(self.ident().clone());
        let deps: Vec<PackageInstall> = try!(self.load_deps());
        for dep in deps.iter() {
            let mut p = try!(dep.paths());
            run_paths.append(&mut p);
            idents.insert(dep.ident().clone());
        }
        let tdeps: Vec<PackageInstall> = try!(self.load_tdeps());
        for dep in tdeps.iter() {
            if idents.contains(dep.ident()) {
                continue;
            }
            let mut p = try!(dep.paths());
            run_paths.append(&mut p);
            idents.insert(dep.ident().clone());
        }

        let p = env::join_paths(&run_paths).expect("Failed to build path string");
        Ok(p.into_string().expect("Failed to convert path to utf8 string"))
    }

    pub fn installed_path(&self) -> &PathBuf {
        &self.installed_path
    }

    /// Returns the root path for service configuration, files, and data.
    pub fn svc_path(&self) -> PathBuf {
        fs::svc_path(&self.ident.name)
    }

    /// Returns the path to the service configuration.
    pub fn svc_config_path(&self) -> PathBuf {
        fs::svc_config_path(&self.ident.name)
    }

    /// Returns the path to the service data.
    pub fn svc_data_path(&self) -> PathBuf {
        fs::svc_data_path(&self.ident.name)
    }

    /// Returns the path to the service's gossiped config files.
    pub fn svc_files_path(&self) -> PathBuf {
        fs::svc_files_path(&self.ident.name)
    }

    /// Returns the path to the service hooks.
    pub fn svc_hooks_path(&self) -> PathBuf {
        fs::svc_hooks_path(&self.ident.name)
    }

    /// Returns the path to the service static content.
    pub fn svc_static_path(&self) -> PathBuf {
        fs::svc_static_path(&self.ident.name)
    }

    /// Returns the path to the service variable state.
    pub fn svc_var_path(&self) -> PathBuf {
        fs::svc_var_path(&self.ident.name)
    }

    /// Returns the user that the package is specified to run as
    /// or None if the package doesn't contain a SVC_USER Metafile
    pub fn svc_user(&self) -> Result<Option<String>> {
        match self.read_metafile(MetaFile::SvcUser) {
            Ok(body) => Ok(Some(body)),
            Err(Error::MetaFileNotFound(MetaFile::SvcUser)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Returns the group that the package is specified to run as
    /// or None if the package doesn't contain a SVC_GROUP Metafile
    pub fn svc_group(&self) -> Result<Option<String>> {
        match self.read_metafile(MetaFile::SvcGroup) {
            Ok(body) => Ok(Some(body)),
            Err(Error::MetaFileNotFound(MetaFile::SvcGroup)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn target(&self) -> Result<PackageTarget> {
        match self.read_metafile(MetaFile::Target) {
            Ok(body) => PackageTarget::from_str(&body),
            Err(e) => Err(e),
        }
    }

    /// Read the contents of a given metafile.
    ///
    /// # Failures
    ///
    /// * A metafile could not be found
    /// * Contents of the metafile could not be read
    /// * Contents of the metafile are unreadable or malformed
    fn read_metafile(&self, file: MetaFile) -> Result<String> {
        let filepath = self.installed_path.join(file.to_string());
        match std::fs::metadata(&filepath) {
            Ok(_) => {
                match File::open(&filepath) {
                    Ok(mut f) => {
                        let mut data = String::new();
                        if f.read_to_string(&mut data).is_err() {
                            return Err(Error::MetaFileMalformed(file));
                        }
                        Ok(data.trim().to_string())
                    }
                    Err(e) => Err(Error::MetaFileIO(e)),
                }
            }
            Err(_) => Err(Error::MetaFileNotFound(file)),
        }
    }

    /// Reads metafiles containing dependencies represented by package identifiers separated by new
    /// lines.
    ///
    /// # Failures
    ///
    /// * Metafile could not be found
    /// * Contents of the metafile could not be read
    /// * Contents of the metafile are unreadable or malformed
    fn read_deps(&self, file: MetaFile) -> Result<Vec<PackageIdent>> {
        let mut deps: Vec<PackageIdent> = vec![];
        match self.read_metafile(file) {
            Ok(body) => {
                if body.len() > 0 {
                    let ids: Vec<String> = body.split("\n").map(|d| d.to_string()).collect();
                    for id in &ids {
                        let package = try!(PackageIdent::from_str(id));
                        if !package.fully_qualified() {
                            return Err(Error::InvalidPackageIdent(package.to_string()));
                        }
                        deps.push(package);
                    }
                }
                Ok(deps)
            }
            Err(Error::MetaFileNotFound(_)) => Ok(deps),
            Err(e) => Err(e),
        }
    }

    /// Attempts to load the extracted package for each direct dependency and returns a
    /// `Package` struct representation of each in the returned vector.
    ///
    /// # Failures
    ///
    /// * Any direct dependency could not be located or it's contents could not be read
    ///   from disk
    fn load_deps(&self) -> Result<Vec<PackageInstall>> {
        let ddeps = try!(self.deps());
        let mut deps = Vec::with_capacity(ddeps.len());
        for dep in ddeps.iter() {
            let dep_install = try!(Self::load(dep, Some(&self.fs_root_path)));
            deps.push(dep_install);
        }
        Ok(deps)
    }

    /// Attempts to load the extracted package for each transitive dependency and returns a
    /// `Package` struct representation of each in the returned vector.
    ///
    /// # Failures
    ///
    /// * Any transitive dependency could not be located or it's contents could not be read
    ///   from disk
    fn load_tdeps(&self) -> Result<Vec<PackageInstall>> {
        let tdeps = try!(self.tdeps());
        let mut deps = Vec::with_capacity(tdeps.len());
        for dep in tdeps.iter() {
            let dep_install = try!(Self::load(dep, Some(&self.fs_root_path)));
            deps.push(dep_install);
        }
        Ok(deps)
    }

    fn calc_installed_path(ident: &PackageIdent, pkg_root: &Path) -> Result<PathBuf> {
        if ident.fully_qualified() {
            Ok(pkg_root.join(&ident.origin)
                .join(&ident.name)
                .join(ident.version.as_ref().unwrap())
                .join(ident.release.as_ref().unwrap()))
        } else {
            Err(Error::PackageNotFound(ident.clone()))
        }
    }

    /// Returns a list of package structs built from the contents of the given directory.
    fn package_list(path: &Path) -> Result<Vec<PackageIdent>> {
        let mut package_list: Vec<PackageIdent> = vec![];
        if try!(std::fs::metadata(path)).is_dir() {
            try!(Self::walk_origins(&path, &mut package_list));
        }
        Ok(package_list)
    }

    /// Helper function for package_list. Walks the given path for origin directories
    /// and builds on the given package list by recursing into name, version, and release
    /// directories.
    fn walk_origins(path: &Path, packages: &mut Vec<PackageIdent>) -> Result<()> {
        for entry in try!(std::fs::read_dir(path)) {
            let origin = try!(entry);
            if try!(std::fs::metadata(origin.path())).is_dir() {
                try!(Self::walk_names(&origin, packages));
            }
        }
        Ok(())
    }

    /// Helper function for walk_origins. Walks the given origin DirEntry for name
    /// directories and recurses into them to find version and release directories.
    fn walk_names(origin: &DirEntry, packages: &mut Vec<PackageIdent>) -> Result<()> {
        for name in try!(std::fs::read_dir(origin.path())) {
            let name = try!(name);
            let origin = origin.file_name().to_string_lossy().into_owned().to_string();
            if try!(std::fs::metadata(name.path())).is_dir() {
                try!(Self::walk_versions(&origin, &name, packages));
            }
        }
        Ok(())
    }

    /// Helper fuction for walk_names. Walks the given name DirEntry for directories and recurses
    /// into them to find release directories.
    fn walk_versions(origin: &String,
                     name: &DirEntry,
                     packages: &mut Vec<PackageIdent>)
                     -> Result<()> {
        for version in try!(std::fs::read_dir(name.path())) {
            let version = try!(version);
            let name = name.file_name().to_string_lossy().into_owned().to_string();
            if try!(std::fs::metadata(version.path())).is_dir() {
                try!(Self::walk_releases(origin, &name, &version, packages));
            }
        }
        Ok(())
    }

    /// Helper function for walk_versions. Walks the given release DirEntry for directories and recurses
    /// into them to find version directories. Finally, a Package struct is built and concatenated onto
    /// the given packages vector with the origin, name, version, and release of each.
    fn walk_releases(origin: &String,
                     name: &String,
                     version: &DirEntry,
                     packages: &mut Vec<PackageIdent>)
                     -> Result<()> {
        for release in try!(std::fs::read_dir(version.path())) {
            let release = try!(release).file_name().to_string_lossy().into_owned().to_string();
            let version = version.file_name().to_string_lossy().into_owned().to_string();
            let ident =
                PackageIdent::new(origin.clone(), name.clone(), Some(version), Some(release));
            packages.push(ident)
        }
        Ok(())
    }
}
