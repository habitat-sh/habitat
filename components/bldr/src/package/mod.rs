// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

pub mod hooks;
pub mod updater;

pub use self::updater::{PackageUpdater, PackageUpdaterActor, UpdaterMessage};
pub use self::hooks::HookType;

use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::fs::{self, DirEntry, File};
use std::os::unix;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;
use std::io::prelude::*;
use std::process::Command;
use std::env;

use core::fs::{PACKAGE_CACHE, PACKAGE_HOME, SERVICE_HOME};
use core::package::{version_sort, MetaFile, PackageIdent};
use core::util;
use depot_core::data_object;

use self::hooks::HookTable;
use error::{BldrResult, BldrError, ErrorKind};
use health_check::{self, CheckResult};
use service_config::ServiceConfig;

static LOGKEY: &'static str = "PK";
const INIT_FILENAME: &'static str = "init";
const HEALTHCHECK_FILENAME: &'static str = "health_check";
const RECONFIGURE_FILENAME: &'static str = "reconfigure";
const RUN_FILENAME: &'static str = "run";

pub enum Signal {
    Status,
    Up,
    Down,
    Once,
    Pause,
    Cont,
    Hup,
    Alarm,
    Interrupt,
    Quit,
    One,
    Two,
    Term,
    Kill,
    Exit,
    Start,
    Stop,
    Reload,
    Restart,
    Shutdown,
    ForceStop,
    ForceReload,
    ForceRestart,
    ForceShutdown,
    TryRestart,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Package {
    pub origin: String,
    pub name: String,
    pub version: String,
    pub release: String,
    pub deps: Vec<PackageIdent>,
    pub tdeps: Vec<PackageIdent>,
}

impl Package {
    pub fn deps<P: AsRef<Path>>(ident: &PackageIdent, home: P) -> BldrResult<Vec<PackageIdent>> {
        Self::read_deps(home.as_ref().join(ident.to_string()), MetaFile::Deps)
    }

    pub fn tdeps<P: AsRef<Path>>(ident: &PackageIdent, home: P) -> BldrResult<Vec<PackageIdent>> {
        Self::read_deps(home.as_ref().join(ident.to_string()), MetaFile::TDeps)
    }

    /// Helper function for reading metafiles containing dependencies represented by package
    /// identifiers separated by new lines
    ///
    /// # Failures
    ///
    /// * Metafile could not be found
    /// * Contents of the metafile could not be read
    /// * Contents of the metafile are unreadable or malformed
    fn read_deps<P: AsRef<Path>>(path: P, file: MetaFile) -> BldrResult<Vec<PackageIdent>> {
        let mut deps: Vec<PackageIdent> = vec![];
        match Self::read_metadata(path.as_ref(), file) {
            Ok(body) => {
                let ids: Vec<String> = body.split("\n").map(|d| d.to_string()).collect();
                for id in &ids {
                    let package = try!(PackageIdent::from_str(id));
                    if !package.fully_qualified() {
                        return Err(bldr_error!(ErrorKind::InvalidPackageIdent(package.to_string())));
                    }
                    deps.push(package);
                }
                Ok(deps)
            }
            Err(BldrError{err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(deps),
            Err(e) => Err(e),
        }
    }

    /// Read the contents of the given metafile from a package at the given filepath.
    ///
    /// # Failures
    ///
    /// * A metafile could not be found
    /// * Contents of the metafile could not be read
    /// * Contents of the metafile are unreadable or malformed
    fn read_metadata<P: AsRef<Path>>(path: P, file: MetaFile) -> BldrResult<String> {
        let filepath = path.as_ref().join(file.to_string());
        match fs::metadata(&filepath) {
            Ok(_) => {
                match File::open(&filepath) {
                    Ok(mut f) => {
                        let mut data = String::new();
                        if f.read_to_string(&mut data).is_err() {
                            return Err(bldr_error!(ErrorKind::MetaFileMalformed(file)));
                        }
                        Ok(data.trim().to_string())
                    }
                    Err(e) => Err(bldr_error!(ErrorKind::MetaFileIO(e))),
                }
            }
            Err(_) => Err(bldr_error!(ErrorKind::MetaFileNotFound(file))),
        }
    }

    /// Verifies a package is within the package home and returns a struct representing that
    /// package.
    ///
    /// Only the origin and name of a package are required - the latest version/release of a
    /// package will be returned if their optional value is not specified. If only a version is
    /// specified, the latest release of that package origin, name, and version is returned.
    ///
    /// An optional `home` path may be provided to search for a package in a non-default path.
    pub fn load(ident: &PackageIdent, home: Option<&str>) -> BldrResult<Package> {
        let path = home.unwrap_or(PACKAGE_HOME);
        let pl = try!(Self::package_list(path));
        if ident.fully_qualified() {
            if pl.iter().any(|ref p| p.satisfies(ident)) {
                Ok(Package {
                    origin: ident.origin.clone(),
                    name: ident.name.clone(),
                    version: ident.version.as_ref().unwrap().clone(),
                    release: ident.release.as_ref().unwrap().clone(),
                    deps: try!(Self::deps(ident, path)),
                    tdeps: try!(Self::tdeps(ident, path)),
                })
            } else {
                Err(bldr_error!(ErrorKind::PackageNotFound(ident.clone())))
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
                Ok(Package {
                    deps: try!(Self::deps(&id, path)),
                    tdeps: try!(Self::tdeps(&id, path)),
                    origin: id.origin,
                    name: id.name,
                    version: id.version.unwrap(),
                    release: id.release.unwrap(),
                })
            } else {
                Err(bldr_error!(ErrorKind::PackageNotFound(ident.clone())))
            }
        }
    }

    pub fn signal(&self, signal: Signal) -> BldrResult<String> {
        let runit_pkg = try!(Self::load(&PackageIdent::new("chef", "runit", None, None), None));
        let signal_arg = match signal {
            Signal::Status => "status",
            Signal::Up => "up",
            Signal::Down => "down",
            Signal::Once => "once",
            Signal::Pause => "pause",
            Signal::Cont => "cont",
            Signal::Hup => "hup",
            Signal::Alarm => "alarm",
            Signal::Interrupt => "interrupt",
            Signal::Quit => "quit",
            Signal::One => "1",
            Signal::Two => "2",
            Signal::Term => "term",
            Signal::Kill => "kill",
            Signal::Exit => "exit",
            Signal::Start => "start",
            Signal::Stop => "stop",
            Signal::Reload => "reload",
            Signal::Restart => "restart",
            Signal::Shutdown => "shutdown",
            Signal::ForceStop => "force-stop",
            Signal::ForceReload => "force-reload",
            Signal::ForceRestart => "force-restart",
            Signal::ForceShutdown => "force-shutdown",
            Signal::TryRestart => "try-restart",
        };
        let output = try!(Command::new(runit_pkg.join_path("bin/sv"))
                              .arg(signal_arg)
                              .arg(&format!("{}/{}", SERVICE_HOME, self.name))
                              .output());
        match output.status.success() {
            true => {
                let stdout = try!(String::from_utf8(output.stdout));
                return Ok(stdout);
            }
            false => {
                match signal {
                    Signal::ForceShutdown => {
                        let outstr = try!(String::from_utf8(output.stdout));
                        return Ok(outstr);
                    }
                    _ => {}
                }
                debug!("Failed to send signal to the process supervisor for {}",
                       self.name);
                let outstr = try!(String::from_utf8(output.stdout));
                let errstr = try!(String::from_utf8(output.stderr));
                debug!("Supervisor (O): {}", outstr);
                debug!("Supervisor (E): {}", errstr);
                debug!("Supervisor Code {:?}", output.status.code());
                return Err(bldr_error!(ErrorKind::SupervisorSignalFailed));
            }
        }
    }

    /// Get status of running package.
    pub fn status(&self) -> BldrResult<String> {
        self.signal(Signal::Status)
    }

    /// A vector of ports we expose
    pub fn exposes(&self) -> Vec<String> {
        match fs::metadata(self.join_path("EXPOSES")) {
            Ok(_) => {
                let mut exposed_file = File::open(self.join_path("EXPOSES")).unwrap();
                let mut exposed_string = String::new();
                exposed_file.read_to_string(&mut exposed_string).unwrap();
                let v: Vec<String> = exposed_string.split(' ')
                                                   .map(|x| {
                                                       String::from(x.trim_right_matches('\n'))
                                                   })
                                                   .collect();
                return v;
            }
            Err(_) => {
                let v: Vec<String> = Vec::new();
                return v;
            }
        }
    }

    /// Returns a string with the full run path for this package. This path is composed of any
    /// binary paths specified by this package, or its TDEPS, plus bldr or its TDEPS, plus the
    /// existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can call 'bldr', without having
    /// to worry much about context.
    pub fn run_path(&self) -> BldrResult<String> {
        let mut run_path = String::new();
        if let Some(path) = try!(self.path_meta()) {
            run_path = format!("{}", path);
        }
        let tdeps: Vec<Package> = try!(self.load_tdeps());
        for dep in tdeps.iter() {
            if let Some(path) = try!(dep.path_meta()) {
                run_path = format!("{}:{}", run_path, path);
            }
        }
        if self.name != "bldr" {
            let bldr_pkg = try!(Package::load(&PackageIdent::new("chef", "bldr", None, None),
                                              None));
            if let Some(path) = try!(bldr_pkg.path_meta()) {
                run_path = format!("{}:{}", run_path, path);
            }
            let tdeps: Vec<Package> = try!(bldr_pkg.load_tdeps());
            for dep in tdeps.iter() {
                if let Some(path) = try!(dep.path_meta()) {
                    run_path = format!("{}:{}", run_path, path);
                }
            }
        }
        match env::var_os("PATH") {
            Some(val) => {
                run_path = format!("{}:{}",
                                   run_path,
                                   String::from(val.to_string_lossy().into_owned()));
            }
            None => outputln!("PATH is not defined in the environment; good luck, cowboy!"),
        }
        Ok(run_path)
    }

    /// Return the PATH string from the package metadata, if it exists
    ///
    /// # Failures
    ///
    /// * The package contains a Path metafile but it could not be read or it was malformed
    pub fn path_meta(&self) -> BldrResult<Option<String>> {
        match Self::read_metadata(self.path(), MetaFile::Path) {
            Ok(data) => Ok(Some(data)),
            Err(BldrError {err: ErrorKind::MetaFileNotFound(_), ..}) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn hook_template_path(&self, hook_type: &HookType) -> PathBuf {
        let base = PathBuf::from(self.join_path("hooks"));
        match *hook_type {
            HookType::Init => base.join(INIT_FILENAME),
            HookType::HealthCheck => base.join(HEALTHCHECK_FILENAME),
            HookType::Reconfigure => base.join(RECONFIGURE_FILENAME),
            HookType::Run => base.join(RUN_FILENAME),
        }
    }

    pub fn hook_path(&self, hook_type: &HookType) -> PathBuf {
        let base = PathBuf::from(self.srvc_join_path("hooks"));
        match *hook_type {
            HookType::Init => base.join(INIT_FILENAME),
            HookType::HealthCheck => base.join(HEALTHCHECK_FILENAME),
            HookType::Reconfigure => base.join(RECONFIGURE_FILENAME),
            HookType::Run => base.join(RUN_FILENAME),
        }
    }

    /// The path to the package on disk.
    pub fn path(&self) -> String {
        format!("{}/{}/{}/{}/{}",
                PACKAGE_HOME,
                self.origin,
                self.name,
                self.version,
                self.release)
    }

    /// Join a string to the path on disk.
    pub fn join_path(&self, join: &str) -> String {
        format!("{}/{}/{}/{}/{}/{}",
                PACKAGE_HOME,
                self.origin,
                self.name,
                self.version,
                self.release,
                join)
    }

    /// The on disk srvc path for this package.
    pub fn srvc_path(&self) -> String {
        format!("{}/{}", SERVICE_HOME, self.name)
    }

    /// Join a string to the on disk srvc path for this package.
    pub fn srvc_join_path(&self, join: &str) -> String {
        format!("{}/{}/{}", SERVICE_HOME, self.name, join)
    }

    /// Create the service path for this package.
    pub fn create_srvc_path(&self) -> BldrResult<()> {
        debug!("Creating srvc paths");
        try!(fs::create_dir_all(self.srvc_join_path("config")));
        try!(fs::create_dir_all(self.srvc_join_path("hooks")));
        try!(fs::create_dir_all(self.srvc_join_path("toml")));
        try!(fs::create_dir_all(self.srvc_join_path("data")));
        try!(fs::create_dir_all(self.srvc_join_path("var")));
        try!(util::perm::set_permissions(&self.srvc_join_path("toml"), "0700"));
        try!(util::perm::set_owner(&self.srvc_join_path("data"), "bldr:bldr"));
        try!(util::perm::set_permissions(&self.srvc_join_path("data"), "0700"));
        try!(util::perm::set_owner(&self.srvc_join_path("var"), "bldr:bldr"));
        try!(util::perm::set_permissions(&self.srvc_join_path("var"), "0700"));
        Ok(())
    }

    /// Copy the "run" file to the srvc path.
    pub fn copy_run(&self, context: &ServiceConfig) -> BldrResult<()> {
        debug!("Copying the run file");
        let srvc_run = self.srvc_join_path(RUN_FILENAME);
        if let Some(hook) = self.hooks().run_hook {
            try!(hook.compile(Some(context)));
            match fs::read_link(&srvc_run) {
                Ok(path) => {
                    if path != hook.path {
                        try!(util::perm::set_permissions(hook.path.to_str().unwrap(), "0755"));
                        try!(fs::remove_file(&srvc_run));
                        try!(unix::fs::symlink(hook.path, &srvc_run));
                    }
                }
                Err(_) => try!(unix::fs::symlink(hook.path, &srvc_run)),
            }
        } else {
            let run = self.join_path(RUN_FILENAME);
            try!(util::perm::set_permissions(&run, "0755"));
            match fs::metadata(&srvc_run) {
                Ok(_) => try!(fs::remove_file(&srvc_run)),
                Err(_) => {}
            }
            try!(unix::fs::symlink(&run, &srvc_run));
        }
        Ok(())
    }

    pub fn topology_leader() -> BldrResult<()> {
        Ok(())
    }

    /// Return an iterator of the configuration file names to render.
    ///
    /// This does not return the full path, for convenience with the path
    /// helpers above.
    pub fn config_files(&self) -> BldrResult<Vec<String>> {
        let mut files: Vec<String> = Vec::new();
        for config in try!(fs::read_dir(self.join_path("config"))) {
            let config = try!(config);
            match config.path().file_name() {
                Some(filename) => {
                    files.push(filename.to_string_lossy().into_owned().to_string());
                }
                None => unreachable!(),
            }
        }
        Ok(files)
    }

    pub fn ident(&self) -> String {
        format!("{}/{}/{}/{}",
                self.origin,
                self.name,
                self.version,
                self.release)
    }

    /// Run iniitalization hook if present
    pub fn initialize(&self, context: &ServiceConfig) -> BldrResult<()> {
        if let Some(hook) = self.hooks().init_hook {
            match hook.run(Some(context)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    /// Run reconfigure hook if present
    pub fn reconfigure(&self, context: &ServiceConfig) -> BldrResult<()> {
        if let Some(hook) = self.hooks().reconfigure_hook {
            match hook.run(Some(context)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            match self.signal(Signal::Restart) {
                Ok(_) => Ok(()),
                Err(e) => {
                    let output = format!("failed to run default hook: {}", e);
                    Err(bldr_error!(ErrorKind::HookFailed(HookType::Reconfigure, -1, output)))
                }
            }
        }
    }

    pub fn supervisor_running(&self) -> bool {
        let res = self.signal(Signal::Status);
        match res {
            Ok(_) => return true,
            Err(e) => {
                debug!("Supervisor not running?: {:?}", e);
                return false;
            }
        }
    }

    pub fn health_check(&self, config: &ServiceConfig) -> BldrResult<CheckResult> {
        if let Some(hook) = self.hooks().health_check_hook {
            match hook.run(Some(config)) {
                Ok(output) => Ok(health_check::CheckResult::ok(output)),
                Err(BldrError{err: ErrorKind::HookFailed(_, 1, output), ..}) => {
                    Ok(health_check::CheckResult::warning(output))
                }
                Err(BldrError{err: ErrorKind::HookFailed(_, 2, output), ..}) => {
                    Ok(health_check::CheckResult::critical(output))
                }
                Err(BldrError{err: ErrorKind::HookFailed(_, 3, output), ..}) => {
                    Ok(health_check::CheckResult::unknown(output))
                }
                Err(BldrError{err: ErrorKind::HookFailed(_, code, output), ..}) => {
                    Err(bldr_error!(ErrorKind::HealthCheck(format!("hook exited code={}, \
                                                                    output={}",
                                                                   code,
                                                                   output))))
                }
                Err(e) => Err(BldrError::from(e)),
            }
        } else {
            let status_output = try!(self.signal(Signal::Status));
            let last_config = try!(self.last_config());
            Ok(health_check::CheckResult::ok(format!("{}\n{}", status_output, last_config)))
        }
    }

    pub fn hooks(&self) -> HookTable {
        let mut hooks = HookTable::new(&self);
        hooks.load_hooks();
        hooks
    }

    pub fn cache_file(&self) -> PathBuf {
        PathBuf::from(format!("{}/{}-{}-{}-{}.bldr",
                              PACKAGE_CACHE,
                              self.origin,
                              self.name,
                              self.version,
                              self.release))
    }

    pub fn last_config(&self) -> BldrResult<String> {
        let mut file = try!(File::open(self.srvc_join_path("config.toml")));
        let mut result = String::new();
        try!(file.read_to_string(&mut result));
        Ok(result)
    }

    /// Attempts to load the extracted package for each transitive dependency and returns a
    /// `Package` struct representation of each in the returned vector.
    ///
    /// # Failures
    ///
    /// * Any transitive dependency could not be located or it's contents could not be read
    ///   from disk
    fn load_tdeps(&self) -> BldrResult<Vec<Package>> {
        let mut deps = Vec::with_capacity(self.tdeps.len());
        for dep in self.tdeps.iter() {
            let package = try!(Package::load(dep, None));
            deps.push(package);
        }
        Ok(deps)
    }

    /// Returns a list of package structs built from the contents of the given directory.
    fn package_list(path: &str) -> BldrResult<Vec<PackageIdent>> {
        let mut package_list: Vec<PackageIdent> = vec![];
        if try!(fs::metadata(path)).is_dir() {
            try!(Self::walk_origins(&path, &mut package_list));
        }
        Ok(package_list)
    }

    /// Helper function for package_list. Walks the given path for origin directories
    /// and builds on the given package list by recursing into name, version, and release
    /// directories.
    fn walk_origins(path: &str, packages: &mut Vec<PackageIdent>) -> BldrResult<()> {
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
    fn walk_names(origin: &DirEntry, packages: &mut Vec<PackageIdent>) -> BldrResult<()> {
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
                     -> BldrResult<()> {
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
                     -> BldrResult<()> {
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

impl Into<PackageIdent> for Package {
    fn into(self) -> PackageIdent {
        PackageIdent::new(self.origin, self.name, Some(self.version), Some(self.release))
    }
}

impl From<data_object::Package> for Package {
    fn from(val: data_object::Package) -> Package {
        let ident: &PackageIdent = val.ident.as_ref();
        Package {
            origin: ident.origin.clone(),
            name: ident.name.clone(),
            version: ident.version.as_ref().unwrap().clone(),
            release: ident.release.as_ref().unwrap().clone(),
            deps: val.deps.into_iter().map(|d| d.into()).collect(),
            tdeps: val.tdeps.into_iter().map(|d| d.into()).collect(),
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident())
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Package) -> bool {
        if self.origin != other.origin {
            return false;
        } else if self.name != other.name {
            return false;
        } else if self.version != other.version {
            return false;
        } else if self.release != other.release {
            return false;
        } else {
            return true;
        }
    }
}

impl PartialOrd for Package {
    /// Packages can be compared according to the following:
    ///
    /// * origin is ignored in the comparison - my redis and
    ///   your redis compare the same.
    /// * If the names are not equal, they cannot be compared.
    /// * If the versions are greater/lesser, return that as
    ///   the ordering.
    /// * If the versions are equal, return the greater/lesser
    ///   for the release.
    fn partial_cmp(&self, other: &Package) -> Option<Ordering> {
        if self.name != other.name {
            return None;
        }
        let ord = match version_sort(&self.version, &other.version) {
            Ok(ord) => ord,
            Err(e) => {
                error!("This was a very bad version number: {:?}", e);
                return None;
            }
        };
        match ord {
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Equal => {
                return Some(self.release.cmp(&other.release));
            }
        }
    }
}
