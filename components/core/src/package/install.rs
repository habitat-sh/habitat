#[cfg(test)]
use super::PackageTarget;
use super::{list::package_list_for_ident,
            metadata::{parse_key_value,
                       read_metafile,
                       Bind,
                       MetaFile,
                       PackageType},
            Identifiable,
            PackageIdent};
use crate::{error::{Error,
                    Result},
            fs,
            os::process::{ShutdownSignal,
                          ShutdownTimeout}};
use log::debug;
use serde::{Deserialize,
            Serialize};
use std::{cmp::{Ordering,
                PartialOrd},
          collections::{BTreeMap,
                        HashSet},
          env,
          fmt,
          fs::File,
          io::Read,
          path::{Path,
                 PathBuf},
          str::FromStr};
use toml::{self,
           Value};

pub const DEFAULT_CFG_FILE: &str = "default.toml";
const PATH_KEY: &str = "PATH";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PackageInstall {
    pub ident:          PackageIdent,
    fs_root_path:       PathBuf,
    package_root_path:  PathBuf,
    pub installed_path: PathBuf,
}

// The docs recommend implementing `From` instead, but that feels a
// bit odd here.
#[allow(clippy::from_over_into)]
impl Into<PackageIdent> for PackageInstall {
    fn into(self) -> PackageIdent { self.ident }
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
        Ok(package_install)
    }

    /// Verifies an installation of a package that is equal or newer to a given ident and returns
    /// a Result of a `PackageIdent` if one exists.
    ///
    /// An optional `fs_root` path may be provided to search for a package that is mounted on a
    /// filesystem not currently rooted at `/`.
    pub fn load_at_least(ident: &PackageIdent,
                         fs_root_path: Option<&Path>)
                         -> Result<PackageInstall> {
        let package_install = Self::resolve_package_install_min(ident, fs_root_path)?;
        Ok(package_install)
    }

    fn resolve_package_install<T>(ident: &PackageIdent,
                                  fs_root_path: Option<T>)
                                  -> Result<PackageInstall>
        where T: AsRef<Path>
    {
        let fs_root_path = fs_root_path.map_or_else(|| PathBuf::from("/"), |p| p.as_ref().into());
        let package_root_path = fs::pkg_root_path(Some(&fs_root_path));
        if !package_root_path.exists() {
            return Err(Error::PackageNotFound(Box::new(ident.clone())));
        }

        let pl = package_list_for_ident(&package_root_path, ident)?;
        if ident.fully_qualified() {
            if pl.iter().any(|p| p.satisfies(ident)) {
                Ok(PackageInstall { installed_path: fs::pkg_install_path(ident,
                                                                         Some(&fs_root_path)),
                                    fs_root_path,
                                    package_root_path,
                                    ident: ident.clone() })
            } else {
                Err(Error::PackageNotFound(Box::new(ident.clone())))
            }
        } else {
            let latest: Option<PackageIdent> =
                pl.iter()
                  .filter(|&p| p.satisfies(ident))
                  .fold(None, |winner, b| {
                      match winner {
                          Some(a) => {
                              match a.partial_cmp(b) {
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
                Ok(PackageInstall { installed_path: fs::pkg_install_path(&id,
                                                                         Some(&fs_root_path)),
                                    fs_root_path,
                                    package_root_path,
                                    ident: id.clone() })
            } else {
                Err(Error::PackageNotFound(Box::new(ident.clone())))
            }
        }
    }

    /// Find an installed package that is at minimum the version of the given ident.
    fn resolve_package_install_min<T>(ident: &PackageIdent,
                                      fs_root_path: Option<T>)
                                      -> Result<PackageInstall>
        where T: AsRef<Path>
    {
        let original_ident = ident;
        // If the PackageIndent is does not have a version, use a reasonable minimum version that
        // will be satisfied by any installed package with the same origin/name
        let ident = if ident.version.is_none() {
            PackageIdent::new(ident.origin.clone(),
                              ident.name.clone(),
                              Some("0".into()),
                              Some("0".into()))
        } else {
            ident.clone()
        };
        let fs_root_path = fs_root_path.map_or_else(|| PathBuf::from("/"), |p| p.as_ref().into());
        let package_root_path = fs::pkg_root_path(Some(&fs_root_path));
        if !package_root_path.exists() {
            return Err(Error::PackageNotFound(Box::new(original_ident.clone())));
        }

        let pl = package_list_for_ident(&package_root_path, original_ident)?;
        let latest: Option<PackageIdent> =
            pl.iter()
              .filter(|p| p.origin == ident.origin && p.name == ident.name)
              .fold(None, |winner, b| {
                  match winner {
                      Some(a) => {
                          match a.cmp(b) {
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
                  }
              });
        match latest {
            Some(id) => {
                Ok(PackageInstall { installed_path: fs::pkg_install_path(&id,
                                                                         Some(&fs_root_path)),
                                    fs_root_path,
                                    package_root_path,
                                    ident: id.clone() })
            }
            None => Err(Error::PackageNotFound(Box::new(original_ident.clone()))),
        }
    }

    pub fn new_from_parts(ident: PackageIdent,
                          fs_root_path: PathBuf,
                          package_root_path: PathBuf,
                          installed_path: PathBuf)
                          -> PackageInstall {
        PackageInstall { ident,
                         fs_root_path,
                         package_root_path,
                         installed_path }
    }

    /// Determines whether or not this package has a runnable service.
    pub fn is_runnable(&self) -> bool {
        // Currently, a runnable package can be determined by checking if a `run` hook exists in
        // package's hooks directory or directly in the package prefix.
        self.installed_path.join("hooks").join("run").is_file()
        || self.installed_path.join("run").is_file()
    }

    /// Determine what kind of package this is.
    pub fn package_type(&self) -> Result<PackageType> {
        match self.read_metafile(MetaFile::PackageType) {
            Ok(body) => body.parse(),
            Err(Error::MetaFileNotFound(MetaFile::PackageType)) => Ok(PackageType::Standard),
            Err(e) => Err(e),
        }
    }

    /// Constructs and returns a `HashMap` of environment variable/value key pairs of all
    /// environment variables needed to properly run a command from the context of this package.
    pub fn environment_for_command(&self) -> Result<BTreeMap<String, String>> {
        let mut env = self.runtime_environment_file_map(MetaFile::RuntimeEnvironment)?;
        // Remove any pre-existing PATH key as this is either from an older package or is
        // present for backwards compatibility with older Habitat releases.
        env.remove(PATH_KEY);

        let mut paths = self.runtime_paths()?;

        // Let's join the paths to the FS_ROOT
        // In most cases, this does nothing and should only mutate
        // the paths in a windows studio where FS_ROOT_PATH will
        // be the studio root path (ie c:\hab\studios\...)
        let rooted_path = self.root_paths(&mut paths)?;

        // Only insert a PATH entry if the resulting path string is non-empty
        if !rooted_path.is_empty() {
            if cfg!(windows) {
                // On Windows we want to make sure to append the system paths.
                // While this is not critical for every application, any windows
                // app may expect these to be on the path to access system utilities
                // and libraries and could possibly fail if they are absent.
                let paths = env::split_paths(&rooted_path).chain(fs::windows_system_paths());
                let joined = env::join_paths(paths)?;
                env.insert(PATH_KEY.to_string(), joined.to_string_lossy().to_string());
            } else {
                env.insert(PATH_KEY.to_string(), rooted_path);
            }
        }

        // release 0.85.0 introduces the RUNTIME_ENVIRONMENT_PATHS metadata
        // that are environment variables containing file paths that are to be
        // rooted under FS_ROOT. For backwards compatibility the key/value
        // pairs are duplicated in RUNTIME_ENVIRONMENT so we will remove previously
        // stored values with the same key.
        let environment_paths =
            self.runtime_environment_file_map(MetaFile::RuntimeEnvironmentPaths)?;
        for (key, value) in environment_paths.into_iter() {
            let mut split_path: Vec<_> = env::split_paths(&value).collect();
            let rooted_path = self.root_paths(&mut split_path)?;
            env.insert(key, rooted_path);
        }

        Ok(env)
    }

    /// Returns all the package's binds, required and then optional
    pub fn all_binds(&self) -> Result<Vec<Bind>> {
        let mut all_binds = self.binds()?;
        let mut optional = self.binds_optional()?;
        all_binds.append(&mut optional);
        Ok(all_binds)
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

    /// Read and return the decoded contents of the packages default configuration.
    pub fn default_cfg(&self) -> Option<toml::value::Value> {
        match File::open(self.installed_path.join(DEFAULT_CFG_FILE)) {
            Ok(mut file) => {
                let mut raw = String::new();
                #[allow(clippy::question_mark)]
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

    /// Return the direct dependencies of the package
    pub fn deps(&self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::Deps) }

    /// Return all transitive dependencies of the package
    pub fn tdeps(&self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::TDeps) }

    /// Return all build dependencies of the package
    pub fn build_deps(&self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::BuildDeps) }

    /// Return all transitive build dependencies of the package
    pub fn build_tdeps(&self) -> Result<Vec<PackageIdent>> { self.read_deps(MetaFile::BuildTDeps) }

    /// Returns a Rust representation of the mappings defined by the `pkg_exports` plan variable.
    ///
    /// These mappings are used as a filter-map to generate a public configuration when the package
    /// is started as a service. This public configuration can be retrieved by peers to assist in
    /// configuration of themselves.
    pub fn exports(&self) -> Result<BTreeMap<String, String>> {
        match self.read_metafile(MetaFile::Exports) {
            Ok(body) => {
                let parsed_value = parse_key_value(&body);
                let result = parsed_value.map_err(|_| Error::MetaFileMalformed(MetaFile::Exports))?;
                Ok(result)
            }
            Err(Error::MetaFileNotFound(MetaFile::Exports)) => Ok(BTreeMap::new()),
            Err(e) => Err(e),
        }
    }

    /// A vector of ports we expose
    pub fn exposes(&self) -> Result<Vec<String>> {
        match self.read_metafile(MetaFile::Exposes) {
            Ok(body) => {
                let v: Vec<String> = body.split_whitespace()
                                         .map(|x| String::from(x.trim_end_matches('\n')))
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

    pub fn ident(&self) -> &PackageIdent { &self.ident }

    /// Returns the path elements of the package's `PATH` metafile if it exists, or an empty `Vec`
    /// if not found.
    ///
    /// If no value for `PATH` can be found, return an empty `Vec`.
    pub fn paths(&self) -> Result<Vec<PathBuf>> {
        match self.read_metafile(MetaFile::Path) {
            Ok(body) => {
                if body.is_empty() {
                    return Ok(vec![]);
                }
                // The `filter()` in this chain is to reject any path entries that do not start
                // with the package's `installed_path` (aka pkg_prefix). This check is for any
                // packages built after
                // https://github.com/habitat-sh/habitat/commit/13344a679155e5210dd58ecb9d94654f5ae676d3
                // was merged (in https://github.com/habitat-sh/habitat/pull/4067, released in
                // Habitat 0.50.0, 2017-11-30) which produced `PATH` metafiles containing extra
                // path entries.
                let pkg_prefix = fs::pkg_install_path(self.ident(), None::<&Path>);
                let v = env::split_paths(&body).filter(|p| p.starts_with(&pkg_prefix))
                                               .collect();
                Ok(v)
            }
            Err(Error::MetaFileNotFound(MetaFile::Path)) => {
                if cfg!(windows) {
                    // This check is for any packages built after
                    // https://github.com/habitat-sh/habitat/commit/cc1f35e4bd9f7a8d881a602380730488e6ad055a
                    // was merged (in https://github.com/habitat-sh/habitat/pull/4478, released in
                    // Habitat 0.53.0, 2018-02-05) which stopped producing `PATH` metafiles. This
                    // workaround attempts to fallback to the `RUNTIME_ENVIRONMENT` metafile and
                    // use the value of the `PATH` key as a stand-in for the `PATH` metafile.
                    let pkg_prefix = fs::pkg_install_path(self.ident(), None::<&Path>);
                    match self.read_metafile(MetaFile::RuntimeEnvironment) {
                        Ok(ref body) => {
                            match Self::parse_runtime_environment_metafile(body)?.get(PATH_KEY) {
                                Some(env_path) => {
                                    let v = env::split_paths(env_path).filter(|p| {
                                                                          p.starts_with(&pkg_prefix)
                                                                      })
                                                                      .collect();
                                    Ok(v)
                                }
                                None => Ok(vec![]),
                            }
                        }
                        Err(Error::MetaFileNotFound(MetaFile::RuntimeEnvironment)) => Ok(vec![]),
                        Err(e) => Err(e),
                    }
                } else {
                    Ok(vec![])
                }
            }
            Err(e) => Err(e),
        }
    }

    fn root_paths(&self, paths: &mut Vec<PathBuf>) -> Result<String> {
        for path in &mut paths.iter_mut() {
            *path = fs::fs_rooted_path(path, &self.fs_root_path);
        }
        env::join_paths(paths)?.into_string()
                               .map_err(Error::InvalidPathString)
    }

    /// Attempts to load the extracted package for each direct dependency and returns a
    /// `Package` struct representation of each in the returned vector.
    ///
    /// # Failures
    ///
    /// * Any direct dependency could not be located or it's contents could not be read from disk
    fn load_deps(&self) -> Result<Vec<PackageInstall>> {
        let ddeps = self.deps()?;
        let mut deps = Vec::with_capacity(ddeps.len());
        for dep in ddeps.iter() {
            let dep_install = Self::load(dep, Some(&*self.fs_root_path))?;
            deps.push(dep_install);
        }
        Ok(deps)
    }

    /// Attempts to load the extracted package for each transitive dependency and returns a
    /// `Package` struct representation of each in the returned vector.
    ///
    /// # Failures
    ///
    /// * Any transitive dependency could not be located or it's contents could not be read from
    ///   disk
    fn load_tdeps(&self) -> Result<Vec<PackageInstall>> {
        let tdeps = self.tdeps()?;
        let mut deps = Vec::with_capacity(tdeps.len());
        for dep in tdeps.iter() {
            let dep_install = Self::load(dep, Some(&*self.fs_root_path))?;
            deps.push(dep_install);
        }
        Ok(deps)
    }

    /// Returns an ordered `Vec` of path entries which are read from the package's `RUNTIME_PATH`
    /// metafile if it exists, or calcuated using `PATH` metafiles if the package is older.
    /// Otherwise, an empty `Vec` is returned.
    ///
    /// # Errors
    ///
    /// * If a metafile exists but cannot be properly parsed
    fn runtime_paths(&self) -> Result<Vec<PathBuf>> {
        match self.read_metafile(MetaFile::RuntimePath) {
            Ok(body) => {
                if body.is_empty() {
                    return Ok(vec![]);
                }

                Ok(env::split_paths(&body).collect())
            }
            Err(Error::MetaFileNotFound(MetaFile::RuntimePath)) => self.legacy_runtime_paths(),
            Err(e) => Err(e),
        }
    }

    /// Returns an ordered `Vec` of path entries which can be used to create a runtime `PATH` value
    /// when an older package is missing a `RUNTIME_PATH` metafile.
    ///
    /// The path is constructed by taking all `PATH` metafile entries from the current package,
    /// followed by entries from the *direct* dependencies first (in declared order), and then from
    /// any remaining transitive dependencies last (in lexically sorted order). All entries are
    /// present once in the order of their first appearance.
    ///
    /// Preserved reference implementation:
    /// https://github.com/habitat-sh/habitat/blob/333b75d6234db0531cf4a5bdcb859f7d4adc2478/components/core/src/package/install.rs#L321-L350
    fn legacy_runtime_paths(&self) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();
        let mut seen = HashSet::new();

        for p in self.paths()? {
            if seen.contains(&p) {
                continue;
            }
            seen.insert(p.clone());
            paths.push(p);
        }

        let ordered_pkgs = self.load_deps()?.into_iter().chain(self.load_tdeps()?);
        for pkg in ordered_pkgs {
            for p in pkg.paths()? {
                if seen.contains(&p) {
                    continue;
                }
                seen.insert(p.clone());
                paths.push(p);
            }
        }

        Ok(paths)
    }

    fn parse_runtime_environment_metafile(body: &str) -> Result<BTreeMap<String, String>> {
        let mut env = BTreeMap::new();
        for line in body.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(Error::MetaFileMalformed(MetaFile::RuntimeEnvironment));
            }
            let key = parts[0].to_string();
            let value = parts[1].to_string();
            env.insert(key, value);
        }
        Ok(env)
    }

    /// Return the parsed contents of the package's environment` metafile as a `HashMap`,
    /// or an empty `HashMap` if not found.
    ///
    /// If no value of file is found, return an empty `HashMap`.
    fn runtime_environment_file_map(&self, metafile: MetaFile) -> Result<BTreeMap<String, String>> {
        match self.read_metafile(metafile) {
            Ok(ref body) => Self::parse_runtime_environment_metafile(body),
            Err(Error::MetaFileNotFound(_)) => Ok(BTreeMap::new()),
            Err(e) => Err(e),
        }
    }

    pub fn installed_path(&self) -> &Path { &self.installed_path }

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

    /// Returns the shutdown signal that the package is specified to shutdown with
    /// or None if the package doesn't contain a SHUTDOWN_SIGNAL Metafile
    pub fn shutdown_signal(&self) -> Result<Option<ShutdownSignal>> {
        match self.read_metafile(MetaFile::ShutdownSignal) {
            Ok(body) => Ok(Some(body.parse()?)),
            Err(Error::MetaFileNotFound(MetaFile::ShutdownSignal)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Returns the shutdown timeout that the package is specified to shutdown with
    /// or None if the package doesn't contain a SHUTDOWN_TIMEOUT Metafile
    pub fn shutdown_timeout(&self) -> Result<Option<ShutdownTimeout>> {
        match self.read_metafile(MetaFile::ShutdownTimeout) {
            Ok(body) => Ok(Some(body.parse()?)),
            Err(Error::MetaFileNotFound(MetaFile::ShutdownTimeout)) => Ok(None),
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
        read_metafile(&self.installed_path, file)
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
        match self.read_metafile(file) {
            Ok(body) => {
                if !body.is_empty() {
                    for id in body.lines() {
                        let package = PackageIdent::from_str(id)?;
                        if !package.fully_qualified() {
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

    #[cfg(test)]
    fn target(&self) -> Result<PackageTarget> {
        match self.read_metafile(MetaFile::Target) {
            Ok(body) => PackageTarget::from_str(&body),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for PackageInstall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.ident) }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::package::test_support::{fixture_path,
                                       testing_package_install};
    use std::{fmt::Write as FmtWrite,
              fs::File,
              io::Write as IoWrite};
    use tempfile::Builder;

    /// Write the given contents into the specified metadata file for
    /// the package.
    fn write_metafile(pkg_install: &PackageInstall, metafile: MetaFile, content: &str) {
        let path = pkg_install.installed_path().join(metafile.to_string());
        let mut f = File::create(path).expect("Could not create metafile");
        f.write_all(content.as_bytes())
         .expect("Could not write metafile contents");
    }

    /// Creates a `PATH` metafile with path entries all prefixed with the package's `pkg_prefix`.
    fn set_path_for(pkg_install: &PackageInstall, paths: &[&str]) {
        write_metafile(
            pkg_install,
            MetaFile::Path,
            env::join_paths(paths.iter().map(|p| pkg_prefix_for(pkg_install).join(p)))
                .unwrap()
                .to_string_lossy()
                .as_ref(),
        );
    }

    /// Creates a `RUNTIME_PATH` metafile with path entries in the order of the `Vec` of
    /// `PackageInstall`s. Note that this implementation uses the `PATH` metafile of each
    /// `PackageInstall`, including the target `pkg_install`.
    fn set_runtime_path_for(pkg_install: &PackageInstall, installs: Vec<&PackageInstall>) {
        let mut paths = Vec::new();
        for install in installs {
            for path in install.paths()
                               .expect("Could not read or parse PATH metafile")
            {
                paths.push(path)
            }
        }

        write_metafile(pkg_install,
                       MetaFile::RuntimePath,
                       env::join_paths(paths).unwrap().to_string_lossy().as_ref());
    }

    /// Creates a `DEPS` metafile for the given `PackageInstall` populated with the provided deps.
    fn set_deps_for(pkg_install: &PackageInstall, deps: &[&PackageInstall]) {
        let mut content = String::new();
        for dep in deps.iter().map(|d| d.ident()) {
            let _ = writeln!(content, "{}", dep);
        }
        write_metafile(pkg_install, MetaFile::Deps, &content);
    }

    /// Creates a `TDEPS` metafile for the given `PackageInstall` populated with the provided
    /// tdeps.
    fn set_tdeps_for(pkg_install: &PackageInstall, tdeps: &[&PackageInstall]) {
        let mut content = String::new();
        for tdep in tdeps.iter().map(|d| d.ident()) {
            let _ = writeln!(content, "{}", tdep);
        }
        write_metafile(pkg_install, MetaFile::TDeps, &content);
    }

    /// Returns the prefix path for a `PackageInstall`, making sure to not include any `FS_ROOT`.
    fn pkg_prefix_for(pkg_install: &PackageInstall) -> PathBuf {
        fs::pkg_install_path(pkg_install.ident(), None::<&Path>)
    }

    /// Returns a `PackageTarget` that does not match the active target of this system.
    fn wrong_package_target() -> &'static PackageTarget {
        let active = PackageTarget::active_target();
        match PackageTarget::targets().find(|&&target| target != active) {
            Some(wrong) => wrong,
            None => panic!("Should be able to find an unsupported package type"),
        }
    }

    #[test]
    fn can_serialize_default_config() {
        let package_ident = PackageIdent::from_str("just/nothing").unwrap();
        let fixture_path = fixture_path("test_package");
        let package_install = PackageInstall { ident:             package_ident,
                                               fs_root_path:      PathBuf::from(""),
                                               package_root_path: PathBuf::from(""),
                                               installed_path:    fixture_path, };

        let cfg = package_install.default_cfg().unwrap();

        if let Err(e) = toml::ser::to_string(&cfg) {
            panic!("{:?}", e);
        }
    }

    #[test]
    fn load_with_fully_qualified_ident_matching_target() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let active_target = PackageTarget::active_target();
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, &active_target);

        let loaded = PackageInstall::load(&PackageIdent::from_str(ident_s).unwrap(),
                                          Some(fs_root.path())).unwrap();
        assert_eq!(pkg_install, loaded);
        assert_eq!(active_target, loaded.target().unwrap());
    }

    #[test]
    fn load_with_fully_qualified_ident_with_wrong_target_returns_package_not_found_err() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let active_target = PackageTarget::active_target();
        let wrong_target = wrong_package_target();
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, wrong_target);
        let ident = PackageIdent::from_str(ident_s).unwrap();

        match PackageInstall::load(&ident, Some(fs_root.path())) {
            Err(Error::PackageNotFound(err_ident)) => {
                assert_eq!(Box::new(ident), err_ident);
            }
            Err(e) => panic!("Wrong error returned, error={:?}", e),
            Ok(i) => {
                panic!("Should not load successfully, install_ident={}, install_target={}, \
                        active_target={}",
                       &i,
                       i.target().unwrap(),
                       active_target,)
            }
        }
    }

    #[test]
    fn load_with_fuzzy_ident_matching_target() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let active_target = PackageTarget::active_target();
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, &active_target);

        let loaded = PackageInstall::load(&PackageIdent::from_str("dream-theater/\
                                                                   systematic-chaos").unwrap(),
                                          Some(fs_root.path())).unwrap();
        assert_eq!(pkg_install, loaded);
        assert_eq!(active_target, loaded.target().unwrap());
    }

    #[test]
    fn load_with_fuzzy_ident_with_wrong_target_returns_package_not_found_err() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let active_target = PackageTarget::active_target();
        let wrong_target = wrong_package_target();
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, wrong_target);
        let ident = PackageIdent::from_str("dream-theater/systematic-chaos").unwrap();

        match PackageInstall::load(&ident, Some(fs_root.path())) {
            Err(Error::PackageNotFound(err_ident)) => {
                assert_eq!(Box::new(ident), err_ident);
            }
            Err(e) => panic!("Wrong error returned, error={:?}", e),
            Ok(i) => {
                panic!("Should not load successfully, install_ident={}, install_target={}, \
                        active_target={}",
                       &i,
                       i.target().unwrap(),
                       active_target,)
            }
        }
    }

    #[test]
    fn load_with_fuzzy_ident_with_multiple_packages_only_one_matching_target() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let active_target = PackageTarget::active_target();
        let wrong_target = wrong_package_target();

        // This installed package is older but matching the active package target
        let matching_ident_s = "dream-theater/systematic-chaos/1.1.1/20180704142702";
        let matching_pkg_install = testing_package_install(matching_ident_s, fs_root.path());
        write_metafile(&matching_pkg_install, MetaFile::Target, &active_target);

        // This installed package is newer but does not match the active package target
        let wrong_ident_s = "dream-theater/systematic-chaos/5.5.5/20180704142702";
        let wrong_pkg_install = testing_package_install(wrong_ident_s, fs_root.path());
        write_metafile(&wrong_pkg_install, MetaFile::Target, wrong_target);

        let loaded = PackageInstall::load(&PackageIdent::from_str("dream-theater/\
                                                                   systematic-chaos").unwrap(),
                                          Some(fs_root.path())).unwrap();
        assert_eq!(matching_pkg_install, loaded);
        assert_eq!(active_target, loaded.target().unwrap());
    }

    #[test]
    fn load_with_missing_target_returns_package_not_found_err() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        std::fs::remove_file(pkg_install.installed_path()
                                        .join(MetaFile::Target.to_string())).unwrap();
        let ident = PackageIdent::from_str(ident_s).unwrap();

        match PackageInstall::load(&ident, Some(fs_root.path())) {
            Err(Error::PackageNotFound(err_ident)) => {
                assert_eq!(Box::new(ident), err_ident);
            }
            Err(e) => panic!("Wrong error returned, error={:?}", e),
            Ok(i) => {
                panic!("Should not load successfully, install_ident={}, install_target=missing",
                       &i,)
            }
        }
    }

    #[test]
    fn load_with_malformed_target_returns_package_not_found_err() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, "NOT_A_TARGET_EVER");
        let ident = PackageIdent::from_str(ident_s).unwrap();

        match PackageInstall::load(&ident, Some(fs_root.path())) {
            Err(Error::PackageNotFound(err_ident)) => {
                assert_eq!(Box::new(ident), err_ident);
            }
            Err(e) => panic!("Wrong error returned, error={:?}", e),
            Ok(i) => {
                panic!("Should not load successfully, install_ident={}, install_target=missing",
                       &i,)
            }
        }
    }

    #[test]
    fn load_at_least_with_fully_qualified_ident_matching_target() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let active_target = PackageTarget::active_target();
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, &active_target);

        let loaded = PackageInstall::load_at_least(&PackageIdent::from_str(ident_s).unwrap(),
                                                   Some(fs_root.path())).unwrap();
        assert_eq!(pkg_install, loaded);
        assert_eq!(active_target, loaded.target().unwrap());
    }

    #[test]
    fn load_at_least_with_fully_qualified_ident_with_wrong_target_returns_package_not_found_err() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let active_target = PackageTarget::active_target();
        let wrong_target = wrong_package_target();
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, wrong_target);
        let ident = PackageIdent::from_str(ident_s).unwrap();

        match PackageInstall::load_at_least(&ident, Some(fs_root.path())) {
            Err(Error::PackageNotFound(err_ident)) => {
                assert_eq!(Box::new(ident), err_ident);
            }
            Err(e) => panic!("Wrong error returned, error={:?}", e),
            Ok(i) => {
                panic!("Should not load successfully, install_ident={}, install_target={}, \
                        active_target={}",
                       &i,
                       i.target().unwrap(),
                       active_target,)
            }
        }
    }

    #[test]
    fn load_at_least_with_fuzzy_ident_matching_target() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let active_target = PackageTarget::active_target();
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, &active_target);

        let loaded =
            PackageInstall::load_at_least(&PackageIdent::from_str("dream-theater/\
                                                                   systematic-chaos").unwrap(),
                                          Some(fs_root.path())).unwrap();
        assert_eq!(pkg_install, loaded);
        assert_eq!(active_target, loaded.target().unwrap());
    }

    #[test]
    fn load_at_least_with_fuzzy_ident_with_wrong_target_returns_package_not_found_err() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let active_target = PackageTarget::active_target();
        let wrong_target = wrong_package_target();
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, wrong_target);
        let ident = PackageIdent::from_str("dream-theater/systematic-chaos").unwrap();

        match PackageInstall::load_at_least(&ident, Some(fs_root.path())) {
            Err(Error::PackageNotFound(err_ident)) => {
                assert_eq!(Box::new(ident), err_ident);
            }
            Err(e) => panic!("Wrong error returned, error={:?}", e),
            Ok(i) => {
                panic!("Should not load successfully, install_ident={}, install_target={}, \
                        active_target={}",
                       &i,
                       i.target().unwrap(),
                       active_target,)
            }
        }
    }

    #[test]
    fn load_at_least_with_fuzzy_ident_with_multiple_packages_only_one_matching_target() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let active_target = PackageTarget::active_target();
        let wrong_target = wrong_package_target();

        // This installed package is older but matching the active package target
        let matching_ident_s = "dream-theater/systematic-chaos/1.1.1/20180704142702";
        let matching_pkg_install = testing_package_install(matching_ident_s, fs_root.path());
        write_metafile(&matching_pkg_install, MetaFile::Target, &active_target);

        // This installed package is newer but does not match the active package target
        let wrong_ident_s = "dream-theater/systematic-chaos/5.5.5/20180704142702";
        let wrong_pkg_install = testing_package_install(wrong_ident_s, fs_root.path());
        write_metafile(&wrong_pkg_install, MetaFile::Target, wrong_target);

        let loaded =
            PackageInstall::load_at_least(&PackageIdent::from_str("dream-theater/\
                                                                   systematic-chaos").unwrap(),
                                          Some(fs_root.path())).unwrap();
        assert_eq!(matching_pkg_install, loaded);
        assert_eq!(active_target, loaded.target().unwrap());
    }

    #[test]
    fn load_at_least_with_missing_target_returns_package_not_found_err() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        std::fs::remove_file(pkg_install.installed_path()
                                        .join(MetaFile::Target.to_string())).unwrap();
        let ident = PackageIdent::from_str(ident_s).unwrap();

        match PackageInstall::load_at_least(&ident, Some(fs_root.path())) {
            Err(Error::PackageNotFound(err_ident)) => {
                assert_eq!(Box::new(ident), err_ident);
            }
            Err(e) => panic!("Wrong error returned, error={:?}", e),
            Ok(i) => {
                panic!("Should not load successfully, install_ident={}, install_target=missing",
                       &i,)
            }
        }
    }

    #[test]
    fn load_at_least_with_malformed_target_returns_package_not_found_err() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let ident_s = "dream-theater/systematic-chaos/1.2.3/20180704142702";
        let pkg_install = testing_package_install(ident_s, fs_root.path());
        write_metafile(&pkg_install, MetaFile::Target, "NOT_A_TARGET_EVER");
        let ident = PackageIdent::from_str(ident_s).unwrap();

        match PackageInstall::load_at_least(&ident, Some(fs_root.path())) {
            Err(Error::PackageNotFound(err_ident)) => {
                assert_eq!(Box::new(ident), err_ident);
            }
            Err(e) => panic!("Wrong error returned, error={:?}", e),
            Ok(i) => {
                panic!("Should not load successfully, install_ident={}, install_target=missing",
                       &i,)
            }
        }
    }

    #[test]
    fn paths_metafile_single() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        set_path_for(&pkg_install, &["bin"]);

        assert_eq!(vec![pkg_prefix_for(&pkg_install).join("bin")],
                   pkg_install.paths().unwrap());
    }

    #[test]
    fn paths_metafile_multiple() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        set_path_for(&pkg_install, &["bin", "sbin", ".gem/bin"]);

        let pkg_prefix = pkg_prefix_for(&pkg_install);

        assert_eq!(vec![pkg_prefix.join("bin"),
                        pkg_prefix.join("sbin"),
                        pkg_prefix.join(".gem/bin"),],
                   pkg_install.paths().unwrap());
    }

    #[test]
    fn paths_metafile_missing() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());

        assert_eq!(Vec::<PathBuf>::new(), pkg_install.paths().unwrap());
    }

    #[test]
    fn paths_metafile_empty() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());

        // Create a zero-sizd `PATH` metafile
        let _ = File::create(pkg_install.installed_path.join(MetaFile::Path.to_string())).unwrap();

        assert_eq!(Vec::<PathBuf>::new(), pkg_install.paths().unwrap());
    }

    #[test]
    fn paths_metafile_drop_extra_entries() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        let other_pkg_install = testing_package_install("acme/prophets-of-rage", fs_root.path());

        // Create `PATH` metafile which has path entries from another package to replicate certain
        // older packages
        write_metafile(
            &pkg_install,
            MetaFile::Path,
            env::join_paths(
                [
                    pkg_prefix_for(&pkg_install).join("bin"),
                    pkg_prefix_for(&other_pkg_install).join("bin"),
                    pkg_prefix_for(&other_pkg_install).join("sbin"),
                ]
                .iter(),
            )
            .unwrap()
            .to_string_lossy()
            .as_ref(),
        );

        assert_eq!(vec![pkg_prefix_for(&pkg_install).join("bin")],
                   pkg_install.paths().unwrap());
    }

    #[cfg(windows)]
    #[test]
    fn win_legacy_paths_metafile_missing_with_runtime_metafile() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        let other_pkg_install = testing_package_install("acme/prophets-of-rage", fs_root.path());

        // Create `RUNTIME_ENVIROMENT` metafile which has path entries from another package to
        // replicate certain older packages
        let path_val =
            env::join_paths([pkg_prefix_for(&pkg_install).join("bin"),
                             pkg_prefix_for(&other_pkg_install).join("bin"),
                             pkg_prefix_for(&other_pkg_install).join("sbin")].iter()).unwrap();
        write_metafile(&pkg_install,
                       MetaFile::RuntimeEnvironment,
                       &format!("PATH={}\n", path_val.to_string_lossy().as_ref()));

        assert_eq!(vec![pkg_prefix_for(&pkg_install).join("bin")],
                   pkg_install.paths().unwrap());
    }

    #[test]
    fn runtime_paths_single_package_single_path() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        set_path_for(&pkg_install, &["bin"]);
        set_runtime_path_for(&pkg_install, vec![&pkg_install]);

        assert_eq!(vec![pkg_prefix_for(&pkg_install).join("bin")],
                   pkg_install.runtime_paths().unwrap());
    }

    #[test]
    fn runtime_paths_single_package_multiple_paths() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        set_path_for(&pkg_install, &["sbin", ".gem/bin", "bin"]);
        set_runtime_path_for(&pkg_install, vec![&pkg_install]);

        let pkg_prefix = pkg_prefix_for(&pkg_install);

        assert_eq!(vec![pkg_prefix.join("sbin"),
                        pkg_prefix.join(".gem/bin"),
                        pkg_prefix.join("bin"),],
                   pkg_install.runtime_paths().unwrap());
    }

    #[test]
    fn runtime_paths_multiple_packages() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();

        let other_pkg_install = testing_package_install("acme/ty-tabor", fs_root.path());
        set_path_for(&other_pkg_install, &["sbin"]);

        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        set_path_for(&pkg_install, &["bin"]);
        set_runtime_path_for(&pkg_install, vec![&pkg_install, &other_pkg_install]);

        assert_eq!(vec![pkg_prefix_for(&pkg_install).join("bin"),
                        pkg_prefix_for(&other_pkg_install).join("sbin"),],
                   pkg_install.runtime_paths().unwrap());
    }

    // This test uses the legacy/fallback implementation of determining the runtime path
    #[test]
    fn runtime_paths_metafile_missing_with_path_metafiles() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();

        let other_pkg_install = testing_package_install("acme/ty-tabor", fs_root.path());
        set_path_for(&other_pkg_install, &["sbin"]);

        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        set_path_for(&pkg_install, &["bin"]);
        set_deps_for(&pkg_install, &[&other_pkg_install]);
        set_tdeps_for(&pkg_install, &[&other_pkg_install]);

        assert_eq!(vec![pkg_prefix_for(&pkg_install).join("bin"),
                        pkg_prefix_for(&other_pkg_install).join("sbin"),],
                   pkg_install.runtime_paths().unwrap());
    }

    #[test]
    fn runtime_paths_metafile_empty() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        // A `PATH` metafile should *not* influence this test
        set_path_for(&pkg_install, &["nope"]);

        // Create a zero-sizd `RUNTIME_PATH` metafile
        let _ = File::create(pkg_install.installed_path
                                        .join(MetaFile::RuntimePath.to_string())).unwrap();

        assert_eq!(Vec::<PathBuf>::new(), pkg_install.runtime_paths().unwrap());
    }

    // This test ensures the correct ordering of runtime `PATH` entries for legacy packages which
    // lack a `RUNTIME_PATH` metafile.
    #[test]
    fn legacy_runtime_paths() {
        fn paths_for(pkg_install: &PackageInstall) -> Vec<PathBuf> { pkg_install.paths().unwrap() }

        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();

        let hotel = testing_package_install("acme/hotel", fs_root.path());
        set_path_for(&hotel, &["bin"]);

        let golf = testing_package_install("acme/golf", fs_root.path());
        set_path_for(&golf, &["bin"]);

        let foxtrot = testing_package_install("acme/foxtrot", fs_root.path());
        set_path_for(&foxtrot, &["bin"]);

        let echo = testing_package_install("acme/echo", fs_root.path());
        set_deps_for(&echo, &[&foxtrot]);
        set_tdeps_for(&echo, &[&foxtrot]);

        let delta = testing_package_install("acme/delta", fs_root.path());
        set_deps_for(&delta, &[&echo]);
        set_tdeps_for(&delta, &[&echo, &foxtrot]);

        let charlie = testing_package_install("acme/charlie", fs_root.path());
        set_path_for(&charlie, &["sbin"]);
        set_deps_for(&charlie, &[&golf, &delta]);
        set_tdeps_for(&charlie, &[&delta, &echo, &foxtrot, &golf]);

        let beta = testing_package_install("acme/beta", fs_root.path());
        set_path_for(&beta, &["bin"]);
        set_deps_for(&beta, &[&delta]);
        set_tdeps_for(&beta, &[&delta, &echo, &foxtrot]);

        let alpha = testing_package_install("acme/alpha", fs_root.path());
        set_path_for(&alpha, &["sbin", ".gem/bin", "bin"]);
        set_deps_for(&alpha, &[&charlie, &hotel, &beta]);
        set_tdeps_for(&alpha,
                      &[&beta, &charlie, &delta, &echo, &foxtrot, &golf, &hotel]);

        let mut expected = Vec::new();
        expected.append(&mut paths_for(&alpha));
        expected.append(&mut paths_for(&charlie));
        expected.append(&mut paths_for(&hotel));
        expected.append(&mut paths_for(&beta));
        expected.append(&mut paths_for(&foxtrot));
        expected.append(&mut paths_for(&golf));

        assert_eq!(expected, alpha.legacy_runtime_paths().unwrap());
    }

    #[test]
    fn environment_for_command_missing_all_metafiles() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());

        assert_eq!(BTreeMap::<String, String>::new(),
                   pkg_install.environment_for_command().unwrap());
    }

    #[cfg(windows)]
    #[test]
    fn environment_for_command_with_runtime_environment_paths() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());

        write_metafile(&pkg_install,
                       MetaFile::RuntimeEnvironment,
                       "PSModulePath=/should/be/ignored\nJAVA_HOME=/my/java/home\nFOO=bar\n");
        write_metafile(&pkg_install,
                       MetaFile::RuntimeEnvironmentPaths,
                       "PSModulePath=/should/not/be/ignored;c:/my/dir;/should/really/not/be/\
                        ignored\n");

        let mut expected = BTreeMap::new();
        let fs_root_path = fs_root.into_path();
        expected.insert("FOO".to_string(), "bar".to_string());
        expected.insert("JAVA_HOME".to_string(), "/my/java/home".to_string());
        expected.insert(
                        "PSModulePath".to_string(),
                        env::join_paths(vec![
            fs::fs_rooted_path(&pkg_prefix_for(&pkg_install).join("/should/not/be/ignored"), &fs_root_path),
            PathBuf::from("c:/my/dir"),
            fs::fs_rooted_path(
                &pkg_prefix_for(&pkg_install).join("/should/really/not/be/ignored"),
                &fs_root_path,
            ),
        ]).unwrap()
                        .to_string_lossy()
                        .into_owned(),
        );

        assert_eq!(expected, pkg_install.environment_for_command().unwrap());
    }

    #[test]
    fn environment_for_command_with_runtime_environment_with_no_path() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();
        let pkg_install = testing_package_install("acme/pathy", fs_root.path());

        // Create a `RUNTIME_ENVIRONMENT` metafile including a `PATH` key which should be ignored
        write_metafile(&pkg_install,
                       MetaFile::RuntimeEnvironment,
                       "PATH=/should/be/ignored\nJAVA_HOME=/my/java/home\nFOO=bar\n");

        let mut expected = BTreeMap::new();
        expected.insert("FOO".to_string(), "bar".to_string());
        expected.insert("JAVA_HOME".to_string(), "/my/java/home".to_string());

        assert_eq!(expected, pkg_install.environment_for_command().unwrap());
    }

    #[test]
    fn environment_for_command_with_runtime_environment_with_path() {
        let fs_root = Builder::new().prefix("fs-root").tempdir().unwrap();

        let other_pkg_install = testing_package_install("acme/ty-tabor", fs_root.path());
        set_path_for(&other_pkg_install, &["sbin"]);

        let pkg_install = testing_package_install("acme/pathy", fs_root.path());
        set_path_for(&pkg_install, &["bin"]);
        set_runtime_path_for(&pkg_install, vec![&pkg_install, &other_pkg_install]);

        // Create a `RUNTIME_ENVIRONMENT` metafile including a `PATH` key which should be ignored
        write_metafile(&pkg_install,
                       MetaFile::RuntimeEnvironment,
                       "PATH=/should/be/ignored\nJAVA_HOME=/my/java/home\nFOO=bar\n");

        let mut expected = BTreeMap::new();
        let fs_root_path = fs_root.into_path();
        let mut paths = vec![fs::fs_rooted_path(&pkg_prefix_for(&pkg_install).join("bin"),
                                                &fs_root_path),
                             fs::fs_rooted_path(&pkg_prefix_for(&other_pkg_install).join("sbin"),
                                                &fs_root_path),];
        if cfg!(windows) {
            paths.append(&mut fs::windows_system_paths());
        }
        expected.insert("FOO".to_string(), "bar".to_string());
        expected.insert("JAVA_HOME".to_string(), "/my/java/home".to_string());
        expected.insert("PATH".to_string(),
                        env::join_paths(paths).unwrap()
                                              .to_string_lossy()
                                              .into_owned());

        assert_eq!(expected, pkg_install.environment_for_command().unwrap());
    }
}
