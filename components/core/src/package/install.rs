// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::cmp::{Ordering, PartialOrd};
use std::env;
use std::fs::{self, DirEntry, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use error::{Error, Result};
use fs::PACKAGE_HOME;
use package::{MetaFile, PackageIdent};

const SUP_PKG_ORIGIN: &'static str = "chef";
const SUP_PKG_NAME: &'static str = "bldr";

#[derive(Clone, Debug)]
pub struct PackageInstall {
    ident: PackageIdent,
    installed_path: PathBuf,
    package_root_path: PathBuf,
}

impl PackageInstall {
    /// Verifies an installation of a package is within the package home and returns a struct
    /// representing that package installation.
    ///
    /// Only the origin and name of a package are required - the latest version/release of a
    /// package will be returned if their optional value is not specified. If only a version is
    /// specified, the latest release of that package origin, name, and version is returned.
    ///
    /// An optional `home` path may be provided to search for a package in a non-default path.
    pub fn load(ident: &PackageIdent, home: Option<&Path>) -> Result<PackageInstall> {
        let path = home.unwrap_or(Path::new(PACKAGE_HOME));
        let pl = try!(Self::package_list(path));
        if ident.fully_qualified() {
            if pl.iter().any(|ref p| p.satisfies(ident)) {
                Ok(PackageInstall {
                    ident: ident.clone(),
                    installed_path: try!(Self::calc_installed_path(ident, path)),
                    package_root_path: PathBuf::from(path),
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
                                                                 Some(Ordering::Less) => {
                                                                     Some(b.clone())
                                                                 }
                                                                 None => Some(a),
                                                             }
                                                         }
                                                         None => Some(b.clone()),
                                                     }
                                                 });
            if let Some(id) = latest {
                Ok(PackageInstall {
                    ident: id.clone(),
                    installed_path: try!(Self::calc_installed_path(&id, path)),
                    package_root_path: PathBuf::from(path),
                })
            } else {
                Err(Error::PackageNotFound(ident.clone()))
            }
        }
    }

    pub fn new_from_parts(ident: PackageIdent,
                          installed_path: PathBuf,
                          package_root_path: PathBuf)
                          -> PackageInstall {
        PackageInstall {
            ident: ident,
            installed_path: installed_path,
            package_root_path: package_root_path,
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

    /// Returns a `String` with the full run path for this package. This path is composed of any
    /// binary paths specified by this package, or its TDEPS, plus the Supervisor or its TDEPS,
    /// plus the existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can invoke the Supervisor,
    /// without having to worry much about context.
    pub fn runtime_path(&self) -> Result<String> {
        let mut run_path = String::new();
        for path in try!(self.paths()) {
            run_path.push_str(&path.to_string_lossy());
        }
        let tdeps: Vec<PackageInstall> = try!(self.load_tdeps());
        for dep in tdeps.iter() {
            for path in try!(dep.paths()) {
                run_path.push(':');
                run_path.push_str(&path.to_string_lossy());
            }
        }
        if self.ident.name != SUP_PKG_NAME {
            let sup_pkg = try!(Self::load(&PackageIdent::new(SUP_PKG_ORIGIN,
                                                             SUP_PKG_NAME,
                                                             None,
                                                             None),
                                          Some(&self.package_root_path)));
            for path in try!(sup_pkg.paths()) {
                run_path.push(':');
                run_path.push_str(&path.to_string_lossy());
            }
            let tdeps: Vec<PackageInstall> = try!(sup_pkg.load_tdeps());
            for dep in tdeps.iter() {
                for path in try!(dep.paths()) {
                    run_path.push(':');
                    run_path.push_str(&path.to_string_lossy());
                }
            }
        }
        if let Some(val) = env::var_os("PATH") {
            run_path.push(':');
            run_path.push_str(&val.to_string_lossy());
        }
        Ok(run_path)
    }

    pub fn installed_path(&self) -> &PathBuf {
        &self.installed_path
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
        match fs::metadata(&filepath) {
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
                let ids: Vec<String> = body.split("\n").map(|d| d.to_string()).collect();
                for id in &ids {
                    let package = try!(PackageIdent::from_str(id));
                    if !package.fully_qualified() {
                        return Err(Error::InvalidPackageIdent(package.to_string()));
                    }
                    deps.push(package);
                }
                Ok(deps)
            }
            Err(Error::MetaFileNotFound(_)) => Ok(deps),
            Err(e) => Err(e),
        }
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
            let dep_install = try!(Self::load(dep, Some(&self.package_root_path)));
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
        if try!(fs::metadata(path)).is_dir() {
            try!(Self::walk_origins(&path, &mut package_list));
        }
        Ok(package_list)
    }

    /// Helper function for package_list. Walks the given path for origin directories
    /// and builds on the given package list by recursing into name, version, and release
    /// directories.
    fn walk_origins(path: &Path, packages: &mut Vec<PackageIdent>) -> Result<()> {
        for entry in try!(fs::read_dir(path)) {
            let origin = try!(entry);
            if try!(fs::metadata(origin.path())).is_dir() {
                try!(Self::walk_names(&origin, packages));
            }
        }
        Ok(())
    }

    /// Helper function for walk_origins. Walks the given origin DirEntry for name
    /// directories and recurses into them to find version and release directories.
    fn walk_names(origin: &DirEntry, packages: &mut Vec<PackageIdent>) -> Result<()> {
        for name in try!(fs::read_dir(origin.path())) {
            let name = try!(name);
            let origin = origin.file_name().to_string_lossy().into_owned().to_string();
            if try!(fs::metadata(name.path())).is_dir() {
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
        for version in try!(fs::read_dir(name.path())) {
            let version = try!(version);
            let name = name.file_name().to_string_lossy().into_owned().to_string();
            if try!(fs::metadata(version.path())).is_dir() {
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
        for release in try!(fs::read_dir(version.path())) {
            let release = try!(release).file_name().to_string_lossy().into_owned().to_string();
            let version = version.file_name().to_string_lossy().into_owned().to_string();
            let ident = PackageIdent::new(origin.clone(),
                                          name.clone(),
                                          Some(version),
                                          Some(release));
            packages.push(ident)
        }
        Ok(())
    }
}
