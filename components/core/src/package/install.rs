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

use std;
use std::collections::HashMap;
use std::cmp::{Ordering, PartialOrd};
use std::env;
use std::fmt;
use std::fs::{DirEntry, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use toml;
use toml::Value;

use super::{Identifiable, PackageIdent, Target, PackageTarget};
use super::metadata::{Bind, BindMapping, MetaFile, PackageType, parse_key_value};
use error::{Error, Result};
use fs;

pub const DEFAULT_CFG_FILE: &'static str = "default.toml";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PackageInstall {
    pub ident: PackageIdent,
    fs_root_path: PathBuf,
    package_root_path: PathBuf,
    pub installed_path: PathBuf,
}

// The docs recommend implementing `From` instead, but that feels a
// bit odd here.
impl Into<PackageIdent> for PackageInstall {
    fn into(self) -> PackageIdent {
        self.ident
    }
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
        let package_install = Self::resolve_package_install(ident, fs_root_path)?;
        let package_target = package_install.target()?;
        match package_target.validate() {
            Ok(()) => Ok(package_install),
            Err(e) => Err(e),
        }
    }

    /// Verifies an installation of a package that is equal or newer to a given ident and returns
    /// a Result of a `PackageIdent` if one exists.
    ///
    /// An optional `fs_root` path may be provided to search for a package that is mounted on a
    /// filesystem not currently rooted at `/`.
    pub fn load_at_least(
        ident: &PackageIdent,
        fs_root_path: Option<&Path>,
    ) -> Result<PackageInstall> {
        let package_install = Self::resolve_package_install_min(ident, fs_root_path)?;
        let package_target = package_install.target()?;
        match package_target.validate() {
            Ok(()) => Ok(package_install),
            Err(e) => Err(e),
        }
    }

    fn resolve_package_install<T>(
        ident: &PackageIdent,
        fs_root_path: Option<T>,
    ) -> Result<PackageInstall>
    where
        T: AsRef<Path>,
    {
        let fs_root_path = fs_root_path.map_or(PathBuf::from("/"), |p| p.as_ref().into());
        let package_root_path = fs::pkg_root_path(Some(&fs_root_path));
        if !package_root_path.exists() {
            return Err(Error::PackageNotFound(ident.clone()));
        }
        let pl = Self::package_list(&package_root_path)?;
        if ident.fully_qualified() {
            if pl.iter().any(|ref p| p.satisfies(ident)) {
                Ok(PackageInstall {
                    installed_path: fs::pkg_install_path(&ident, Some(&fs_root_path)),
                    fs_root_path: fs_root_path,
                    package_root_path: package_root_path,
                    ident: ident.clone(),
                })
            } else {
                Err(Error::PackageNotFound(ident.clone()))
            }
        } else {
            let latest: Option<PackageIdent> = pl.iter().filter(|&p| p.satisfies(ident)).fold(
                None,
                |winner,
                 b| {
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
                },
            );
            if let Some(id) = latest {
                Ok(PackageInstall {
                    installed_path: fs::pkg_install_path(&id, Some(&fs_root_path)),
                    fs_root_path: PathBuf::from(fs_root_path),
                    package_root_path: package_root_path,
                    ident: id.clone(),
                })
            } else {
                Err(Error::PackageNotFound(ident.clone()))
            }
        }
    }

    /// Find an installed package that is at minimum the version of the given ident.
    fn resolve_package_install_min<T>(
        ident: &PackageIdent,
        fs_root_path: Option<T>,
    ) -> Result<PackageInstall>
    where
        T: AsRef<Path>,
    {
        // If the PackageIndent is does not have a version, use a reasonable minimum version that
        // will be satisfied by any installed package with the same origin/name
        let ident = if None == ident.version {
            PackageIdent::new(
                ident.origin.clone(),
                ident.name.clone(),
                Some("0".into()),
                Some("0".into()),
            )
        } else {
            ident.clone()
        };
        let fs_root_path = fs_root_path.map_or(PathBuf::from("/"), |p| p.as_ref().into());
        let package_root_path = fs::pkg_root_path(Some(&fs_root_path));
        if !package_root_path.exists() {
            return Err(Error::PackageNotFound(ident.clone()));
        }

        let pl = Self::package_list(&package_root_path)?;
        let latest: Option<PackageIdent> = pl.iter()
            .filter(|ref p| p.origin == ident.origin && p.name == ident.name)
            .fold(None, |winner, b| match winner {
                Some(a) => {
                    match a.cmp(&b) {
                        Ordering::Greater | Ordering::Equal => Some(a),
                        Ordering::Less => Some(b.clone()),
                    }
                }
                None => {
                    match b.cmp(&ident) {
                        Ordering::Greater | Ordering::Equal => Some(b.clone()),
                        Ordering::Less => None,
                    }
                }
            });
        match latest {
            Some(id) => {
                Ok(PackageInstall {
                    installed_path: fs::pkg_install_path(&id, Some(&fs_root_path)),
                    fs_root_path: fs_root_path,
                    package_root_path: package_root_path,
                    ident: id.clone(),
                })
            }
            None => Err(Error::PackageNotFound(ident.clone())),
        }
    }

    pub fn new_from_parts(
        ident: PackageIdent,
        fs_root_path: PathBuf,
        package_root_path: PathBuf,
        installed_path: PathBuf,
    ) -> PackageInstall {
        PackageInstall {
            ident: ident,
            fs_root_path: fs_root_path,
            package_root_path: package_root_path,
            installed_path: installed_path,
        }
    }

    /// Determines whether or not this package has a runnable service.
    pub fn is_runnable(&self) -> bool {
        // Currently, a runnable package can be determined by checking if a `run` hook exists in
        // package's hooks directory or directly in the package prefix.
        if self.installed_path.join("hooks").join("run").is_file() ||
            self.installed_path.join("run").is_file()
        {
            true
        } else {
            false
        }
    }

    /// Determine what kind of package this is.
    pub fn pkg_type(&self) -> Result<PackageType> {
        match self.read_metafile(MetaFile::Type) {
            Ok(body) => body.parse(),
            Err(Error::MetaFileNotFound(MetaFile::Type)) => Ok(PackageType::Standalone),
            Err(e) => Err(e),
        }
    }

    /// Which services are contained in a composite package? Note that
    /// these identifiers are *as given* in the initial `plan.sh` of
    /// the composite, and not the fully-resolved identifiers you
    /// would get from other "dependency" metadata files.
    pub fn pkg_services(&self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::Services)
    }

    pub fn binds(&self) -> Result<Vec<Bind>> {
        match self.read_metafile(MetaFile::Binds) {
            Ok(body) => {
                let mut binds = Vec::new();
                for line in body.lines() {
                    match Bind::from_str(line) {
                        Ok(bind) => binds.push(bind),
                        Err(_) => return Err(Error::MetaFileMalformed(MetaFile::Binds)),
                    }
                }
                Ok(binds)
            }
            Err(Error::MetaFileNotFound(MetaFile::Binds)) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    pub fn binds_optional(&self) -> Result<Vec<Bind>> {
        match self.read_metafile(MetaFile::BindsOptional) {
            Ok(body) => {
                let mut binds = Vec::new();
                for line in body.lines() {
                    match Bind::from_str(line) {
                        Ok(bind) => binds.push(bind),
                        Err(_) => return Err(Error::MetaFileMalformed(MetaFile::BindsOptional)),
                    }
                }
                Ok(binds)
            }
            Err(Error::MetaFileNotFound(MetaFile::BindsOptional)) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    /// Returns the bind mappings for a composite package.
    pub fn bind_map(&self) -> Result<HashMap<PackageIdent, Vec<BindMapping>>> {
        match self.read_metafile(MetaFile::BindMap) {
            Ok(body) => {
                let mut bind_map = HashMap::new();
                for line in body.lines() {
                    let mut parts = line.split("=");
                    let package = match parts.next() {
                        Some(ident) => ident.parse()?,
                        None => return Err(Error::MetaFileBadBind),
                    };
                    let binds: Result<Vec<BindMapping>> = match parts.next() {
                        Some(binds) => binds.split(" ").map(|b| b.parse()).collect(),
                        None => Err(Error::MetaFileBadBind),
                    };
                    bind_map.insert(package, binds?);
                }
                Ok(bind_map)
            }
            Err(Error::MetaFileNotFound(MetaFile::BindMap)) => Ok(HashMap::new()),
            Err(e) => Err(e),
        }
    }

    /// Read and return the decoded contents of the packages default configuration.
    pub fn default_cfg(&self) -> Option<toml::value::Value> {
        match File::open(self.installed_path.join(DEFAULT_CFG_FILE)) {
            Ok(mut file) => {
                let mut raw = String::new();
                if file.read_to_string(&mut raw).is_err() {
                    return None;
                };

                match raw.parse::<Value>() {
                    Ok(v) => Some(v),
                    Err(e) => {
                        debug!("Failed to parse toml, error: {:?}", e);
                        None
                    }
                }
            }
            Err(_) => None,
        }
    }

    pub fn tdeps(&self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::TDeps)
    }

    /// Returns a Rust representation of the mappings defined by the `pkg_exports` plan variable.
    ///
    /// These mappings are used as a filter-map to generate a public configuration when the package
    /// is started as a service. This public configuration can be retrieved by peers to assist in
    /// configuration of themselves.
    pub fn exports(&self) -> Result<HashMap<String, String>> {
        match self.read_metafile(MetaFile::Exports) {
            Ok(body) => {
                Ok(parse_key_value(&body).map_err(|_| {
                    Error::MetaFileMalformed(MetaFile::Exports)
                })?)
            }
            Err(Error::MetaFileNotFound(MetaFile::Exports)) => Ok(HashMap::new()),
            Err(e) => Err(e),
        }
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

    /// Returns the contents of a package's runtime PATH environment
    /// variable (if set), split into individual entries.
    ///
    /// Preferentially looks in the `RUNTIME_ENVIRONMENT` metadata file
    /// to find a `PATH` variable. If this metadata file is not
    /// present, we're dealing with an older Habitat package, and will
    /// instead look in the `PATH` metadata file.
    ///
    /// If no value for `PATH` can be found, return an empty `Vec`.
    pub fn paths(&self) -> Result<Vec<PathBuf>> {
        match self.existing_metafile(MetaFile::RuntimeEnvironment) {
            Some(_) => {
                // A RUNTIME_ENVIRONMENT file exists!
                let env = self.runtime_environment()?;
                match env.get("PATH") {
                    Some(path) => {
                        let v = env::split_paths(&path).map(|p| PathBuf::from(&p)).collect();
                        Ok(v)
                    }
                    None => Ok(vec![]),
                }
            }
            None => {
                // No RUNTIME_ENVIRONMENT file exists; fall back to PATH
                match self.read_metafile(MetaFile::Path) {
                    Ok(body) => {
                        let v = env::split_paths(&body).map(|p| PathBuf::from(&p)).collect();
                        Ok(v)
                    }
                    Err(Error::MetaFileNotFound(MetaFile::Path)) => Ok(vec![]),
                    Err(e) => Err(e),
                }
            }
        }
    }

    /// Return the embedded runtime environment specification for a
    /// package.
    pub fn runtime_environment(&self) -> Result<HashMap<String, String>> {
        match self.read_metafile(MetaFile::RuntimeEnvironment) {
            Ok(body) => {
                let mut env = HashMap::new();
                for line in body.lines() {
                    let parts: Vec<&str> = line.splitn(2, "=").collect();
                    if parts.len() != 2 {
                        return Err(Error::MetaFileMalformed(MetaFile::RuntimeEnvironment));
                    }
                    let key = parts[0].to_string();
                    let value = parts[1].to_string();
                    env.insert(key, value);
                }
                Ok(env)
            }
            Err(Error::MetaFileNotFound(MetaFile::RuntimeEnvironment)) => Ok(HashMap::new()),
            Err(e) => Err(e),
        }
    }

    pub fn installed_path(&self) -> &Path {
        &*self.installed_path
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

    fn target(&self) -> Result<PackageTarget> {
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
        match self.existing_metafile(file.clone()) {
            Some(filepath) => {
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
            None => Err(Error::MetaFileNotFound(file)),
        }
    }

    /// Returns the path to a package's specified MetaFile if it exists.
    ///
    /// Useful for fallback logic for dealing with older Habitat
    /// packages.
    fn existing_metafile(&self, file: MetaFile) -> Option<PathBuf> {
        let filepath = self.installed_path.join(file.to_string());
        match std::fs::metadata(&filepath) {
            Ok(_) => Some(filepath),
            Err(_) => None,
        }
    }

    /// Reads metafiles containing dependencies represented by package identifiers separated by new
    /// lines.
    ///
    /// In most cases, we want the identifiers to be fully qualified,
    /// but in some cases (notably reading SERVICES from a composite
    /// package), they do NOT need to be fully qualified.
    ///
    /// # Failures
    ///
    /// * Contents of the metafile could not be read
    /// * Contents of the metafile are unreadable or malformed
    fn read_deps(&self, file: MetaFile) -> Result<Vec<PackageIdent>> {
        let mut deps: Vec<PackageIdent> = vec![];

        // For now, all deps files but SERVICES need fully-qualified
        // package identifiers
        let must_be_fully_qualified = {
            file != MetaFile::Services
        };

        match self.read_metafile(file) {
            Ok(body) => {
                if body.len() > 0 {
                    for id in body.lines() {
                        let package = PackageIdent::from_str(id)?;
                        if !package.fully_qualified() && must_be_fully_qualified {
                            return Err(Error::FullyQualifiedPackageIdentRequired(
                                package.to_string(),
                            ));
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

    /// Returns a list of package structs built from the contents of the given directory.
    fn package_list(path: &Path) -> Result<Vec<PackageIdent>> {
        let mut package_list: Vec<PackageIdent> = vec![];
        if std::fs::metadata(path)?.is_dir() {
            Self::walk_origins(&path, &mut package_list)?;
        }
        Ok(package_list)
    }

    /// Helper function for package_list. Walks the given path for origin directories
    /// and builds on the given package list by recursing into name, version, and release
    /// directories.
    fn walk_origins(path: &Path, packages: &mut Vec<PackageIdent>) -> Result<()> {
        for entry in std::fs::read_dir(path)? {
            let origin = entry?;
            if std::fs::metadata(origin.path())?.is_dir() {
                Self::walk_names(&origin, packages)?;
            }
        }
        Ok(())
    }

    /// Helper function for walk_origins. Walks the given origin DirEntry for name
    /// directories and recurses into them to find version and release directories.
    fn walk_names(origin: &DirEntry, packages: &mut Vec<PackageIdent>) -> Result<()> {
        for name in std::fs::read_dir(origin.path())? {
            let name = name?;
            let origin = origin
                .file_name()
                .to_string_lossy()
                .into_owned()
                .to_string();
            if std::fs::metadata(name.path())?.is_dir() {
                Self::walk_versions(&origin, &name, packages)?;
            }
        }
        Ok(())
    }

    /// Helper function for walk_names. Walks the given name DirEntry for directories and recurses
    /// into them to find release directories.
    fn walk_versions(
        origin: &String,
        name: &DirEntry,
        packages: &mut Vec<PackageIdent>,
    ) -> Result<()> {
        for version in std::fs::read_dir(name.path())? {
            let version = version?;
            let name = name.file_name().to_string_lossy().into_owned().to_string();
            if std::fs::metadata(version.path())?.is_dir() {
                Self::walk_releases(origin, &name, &version, packages)?;
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
        for release in std::fs::read_dir(version.path())? {
            let release = release?
                .file_name()
                .to_string_lossy()
                .into_owned()
                .to_string();
            let version = version
                .file_name()
                .to_string_lossy()
                .into_owned()
                .to_string();
            let ident =
                PackageIdent::new(origin.clone(), name.clone(), Some(version), Some(release));
            packages.push(ident)
        }
        Ok(())
    }
}

impl fmt::Display for PackageInstall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[cfg(test)]
mod test {
    use package::metadata::{BindMapping, MetaFile};
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::str::FromStr;
    use super::PackageInstall;
    use super::super::PackageIdent;
    use super::super::test_support::*;
    use tempdir::TempDir;
    use toml;

    #[test]
    fn can_serialize_default_config() {
        let package_ident = PackageIdent::from_str("just/nothing").unwrap();
        let fixture_path = fixture_path("test_package");
        let package_install = PackageInstall {
            ident: package_ident,
            fs_root_path: PathBuf::from(""),
            package_root_path: PathBuf::from(""),
            installed_path: fixture_path,
        };

        let cfg = package_install.default_cfg().unwrap();

        match toml::ser::to_string(&cfg) {
            Ok(_) => (),
            Err(e) => assert!(false, format!("{:?}", e)),
        }
    }

    /// Create a `PackageInstall` struct for the explicit purpose of
    /// testing metadata file interpretation. This exists to point to
    /// a directory of metadata files, and that's it.
    ///
    /// You should pass in the path to a temporary directory for
    /// `installed_path`.
    fn fake_package_install(ident: &str, installed_path: PathBuf) -> PackageInstall {
        PackageInstall {
            ident: ident.parse().unwrap(),
            fs_root_path: PathBuf::from(""),
            package_root_path: PathBuf::from(""),
            installed_path: installed_path,
        }
    }

    /// Write the given contents into the specified metadata file for
    /// the package.
    fn write_metadata_file(pkg_install: &PackageInstall, metafile: MetaFile, content: &str) {
        let path = pkg_install.installed_path().join(metafile.to_string());
        let mut f = File::create(path).expect("Could not create metafile");
        f.write_all(content.as_bytes()).expect(
            "Could not write metafile contents",
        );
    }

    #[test]
    fn reading_a_valid_bind_map_file_works() {
        // Create a testing package
        let installed_path = TempDir::new("valid_bind_map").expect(
            "Could not create installed_path temporary directory",
        );
        let package_install =
            fake_package_install("core/composite", installed_path.path().to_path_buf());

        // Create a BIND_MAP file for that package
        let bind_map_contents = r#"
core/foo=db:core/database fe:core/front-end be:core/back-end
core/bar=pub:core/publish sub:core/subscribe
        "#;
        write_metadata_file(&package_install, MetaFile::BindMap, bind_map_contents);

        // Grab the bind map from that package
        let bind_map = package_install.bind_map().unwrap();

        // Assert that it was interpreted correctly
        let mut expected: HashMap<PackageIdent, Vec<BindMapping>> = HashMap::new();
        expected.insert(
            "core/foo".parse().unwrap(),
            vec![
                "db:core/database".parse().unwrap(),
                "fe:core/front-end".parse().unwrap(),
                "be:core/back-end".parse().unwrap(),
            ],
        );
        expected.insert(
            "core/bar".parse().unwrap(),
            vec![
                "pub:core/publish".parse().unwrap(),
                "sub:core/subscribe".parse().unwrap(),
            ],
        );

        assert_eq!(expected, bind_map);
    }

    #[test]
    fn reading_a_bad_bind_map_file_results_in_an_error() {
        // Create a testing package
        let installed_path = TempDir::new("invalid_bind_map").expect(
            "Could not create installed_path temporary directory",
        );
        let package_install = fake_package_install("core/dud", installed_path.path().to_path_buf());

        // Create a BIND_MAP directory for that package
        let bind_map_contents = "core/foo=db:this-is-not-an-identifier";
        write_metadata_file(&package_install, MetaFile::BindMap, bind_map_contents);

        // Grab the bind map from that package
        let bind_map = package_install.bind_map();
        assert!(bind_map.is_err());
    }

    /// Composite packages don't need to have a BIND_MAP file, and
    /// standalone packages will never have them. This is OK.
    #[test]
    fn missing_bind_map_files_are_ok() {
        let installed_path = TempDir::new("missing_bind_map").expect(
            "Could not create installed_path temporary directory",
        );
        let package_install =
            fake_package_install("core/no-binds", installed_path.path().to_path_buf());

        // Grab the bind map from that package
        let bind_map = package_install.bind_map().unwrap();
        assert!(bind_map.is_empty());

    }


}
