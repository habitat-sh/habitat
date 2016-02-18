// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

pub mod archive;
pub mod hooks;
pub mod updater;
pub use self::archive::{PackageArchive, MetaFile};
pub use self::updater::{PackageUpdater, PackageUpdaterActor, UpdaterMessage};
pub use self::hooks::HookType;

use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::fs::{self, DirEntry, File};
use std::os::unix;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::process::Command;
use std::io::BufReader;
use std::env;
use regex::Regex;

use self::hooks::HookTable;
use fs::{PACKAGE_CACHE, PACKAGE_HOME, SERVICE_HOME};
use error::{BldrResult, BldrError, ErrorKind};
use health_check::{self, CheckResult};
use service_config::ServiceConfig;
use util;

static LOGKEY: &'static str = "PK";
const INIT_FILENAME: &'static str = "init";
const HEALTHCHECK_FILENAME: &'static str = "health_check";
const RECONFIGURE_FILENAME: &'static str = "reconfigure";
const RUN_FILENAME: &'static str = "run";

#[derive(Debug, Clone, Eq, RustcDecodable, RustcEncodable)]
pub struct Package {
    pub derivation: String,
    pub name: String,
    pub version: String,
    pub release: String,
    pub deps: Option<Vec<Package>>,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}/{}/{}/{}",
               self.derivation,
               self.name,
               self.version,
               self.release)
    }
}

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

impl Package {
    pub fn new(deriv: String, name: String, version: String, release: String) -> Self {
        Package {
            derivation: deriv,
            name: name,
            version: version,
            release: release,
            deps: None,
        }
    }

    pub fn add_dep(&mut self, dep: Package) -> &mut Self {
        if let Some(ref mut deps) = self.deps {
            deps.push(dep);
        } else {
            self.deps = Some(vec![dep]);
        }
        self
    }

    pub fn from_ident(id: &str) -> BldrResult<Package> {
        let items: Vec<&str> = id.split("/").collect();
        match items.len() {
            4 => {
                Ok(Self::new(items[0].trim().to_string(),
                             items[1].trim().to_string(),
                             items[2].trim().to_string(),
                             items[3].trim().to_string()))
            }
            _ => Err(bldr_error!(ErrorKind::InvalidPackageIdent(id.to_string()))),
        }
    }

    pub fn from_path(spath: &str) -> BldrResult<Package> {
        if Path::new(spath).starts_with(PACKAGE_HOME) {
            let items: Vec<&str> = spath.split("/").collect();
            Ok(Self::new(items[3].trim().to_string(),
                         items[4].trim().to_string(),
                         items[5].trim().to_string(),
                         items[6].trim().to_string()))
        } else {
            Err(bldr_error!(ErrorKind::PackageLoad(spath.to_string())))
        }
    }

    /// Verifies a package is within the package home and returns a struct representing that
    /// package.
    ///
    /// Only the derivation and name of a package are required - the latest version/release of a
    /// package will be returned if their optional value is not specified. If only a version is
    /// specified, the latest release of that package derivation, name, and version is returned.
    ///
    /// An optional `home` path may be provided to search for a package in the non-default path.
    pub fn load(deriv: &str,
                pkg: &str,
                ver: Option<String>,
                rel: Option<String>,
                home: Option<&str>)
                -> BldrResult<Package> {
        let path = home.unwrap_or(PACKAGE_HOME);
        let pl = try!(Self::package_list(path));
        let latest: Option<Package> = pl.iter()
                                        .filter(|&p| {
                                            if ver.is_some() && rel.is_some() {
                                                p.name == pkg && p.derivation == deriv &&
                                                p.version == *ver.as_ref().unwrap() &&
                                                p.release == *rel.as_ref().unwrap()
                                            } else if ver.is_some() {
                                                p.name == pkg && p.derivation == deriv &&
                                                p.version == *ver.as_ref().unwrap()
                                            } else {
                                                p.name == pkg && p.derivation == deriv
                                            }
                                        })
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
        latest.ok_or(bldr_error!(ErrorKind::PackageNotFound(deriv.to_string(),
                                                            pkg.to_string(),
                                                            ver,
                                                            rel)))
    }

    pub fn signal(&self, signal: Signal) -> BldrResult<String> {
        let runit_pkg = try!(Self::load("chef", "runit", None, None, None));
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

    /// Creates a Package for every TDEP
    pub fn tdeps(&self) -> BldrResult<Vec<Package>> {
        let mut tdeps = Vec::new();
        match fs::metadata(self.join_path("TDEPS")) {
            Ok(_) => {
                let tdep_file = File::open(self.join_path("TDEPS")).unwrap();
                let reader = BufReader::new(tdep_file);
                for line in reader.lines() {
                    match line {
                        Ok(tdep) => {
                            let pkg = try!(Package::from_ident(&tdep));
                            tdeps.push(pkg);
                        }
                        Err(e) => {
                            outputln!("Package {} has malformed TDEPS: {}", self, e);
                        }
                    }
                }
            }
            Err(_) => {}
        }
        Ok(tdeps)
    }

    /// Returns a string with the full run path for this package. This path is composed of any
    /// binary paths specified by this package, or its TDEPS, plus bldr or its TDEPS, plus the
    /// existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can call 'bldr', without having
    /// to worry much about context.
    pub fn run_path(&self) -> BldrResult<String> {
        let mut run_path = String::new();
        if let Some(path) = self.path_meta() {
            run_path = format!("{}", path);
        }
        let tdeps = try!(self.tdeps());
        for dep in tdeps.iter() {
            if let Some(path) = dep.path_meta() {
                run_path = format!("{}:{}", run_path, path);
            }
        }
        if self.name != "bldr" {
            let bldr_pkg = try!(Package::load("chef", "bldr", None, None, None));
            if let Some(path) = bldr_pkg.path_meta() {
                run_path = format!("{}:{}", run_path, path);
            }
            let tdeps = try!(bldr_pkg.tdeps());
            for dep in tdeps.iter() {
                if let Some(path) = dep.path_meta() {
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
    pub fn path_meta(&self) -> Option<String> {
        match fs::metadata(self.join_path("PATH")) {
            Ok(_) => {
                let mut path_file = File::open(self.join_path("PATH")).unwrap();
                let mut path_string = String::new();
                path_file.read_to_string(&mut path_string).unwrap();
                Some(String::from(path_string.trim_right_matches("\n")))
            }
            Err(_) => {
                None
            }
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
                self.derivation,
                self.name,
                self.version,
                self.release)
    }

    /// Join a string to the path on disk.
    pub fn join_path(&self, join: &str) -> String {
        format!("{}/{}/{}/{}/{}/{}",
                PACKAGE_HOME,
                self.derivation,
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
        let config_dir = self.join_path("config");
        {
            let p = Path::new(&config_dir);
            if !p.exists() {
                return Ok(files);
            }
        }
        for config in try!(fs::read_dir(config_dir)) {
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
                              self.derivation,
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

    /// Returns a list of package structs built from the contents of the given directory.
    fn package_list(path: &str) -> BldrResult<Vec<Package>> {
        let mut package_list: Vec<Package> = Vec::new();
        if try!(fs::metadata(path)).is_dir() {
            try!(Self::walk_derivations(&path, &mut package_list));
        }
        Ok(package_list)
    }

    /// Helper function for package_list. Walks the given path for derivation directories
    /// and builds on the given package list by recursing into name, version, and release
    /// directories.
    fn walk_derivations(path: &str, packages: &mut Vec<Package>) -> BldrResult<()> {
        for entry in try!(fs::read_dir(path)) {
            let derivation = try!(entry);
            if try!(fs::metadata(derivation.path())).is_dir() {
                try!(Self::walk_names(&derivation, packages));
            }
        }
        Ok(())
    }

    /// Helper function for walk_derivations. Walks the given derivation DirEntry for name
    /// directories and recurses into them to find version and release directories.
    fn walk_names(derivation: &DirEntry, packages: &mut Vec<Package>) -> BldrResult<()> {
        for name in try!(fs::read_dir(derivation.path())) {
            let name = try!(name);
            let derivation = derivation.file_name().to_string_lossy().into_owned().to_string();
            if try!(fs::metadata(name.path())).is_dir() {
                try!(Self::walk_versions(&derivation, &name, packages));
            }
        }
        Ok(())
    }

    /// Helper fuction for walk_names. Walks the given name DirEntry for directories and recurses
    /// into them to find release directories.
    fn walk_versions(derivation: &String,
                     name: &DirEntry,
                     packages: &mut Vec<Package>)
                     -> BldrResult<()> {
        for version in try!(fs::read_dir(name.path())) {
            let version = try!(version);
            let name = name.file_name().to_string_lossy().into_owned().to_string();
            if try!(fs::metadata(version.path())).is_dir() {
                try!(Self::walk_releases(derivation, &name, &version, packages));
            }
        }
        Ok(())
    }

    /// Helper function for walk_versions. Walks the given release DirEntry for directories and recurses
    /// into them to find version directories. Finally, a Package struct is built and concatenated onto
    /// the given packages vector with the derivation, name, version, and release of each.
    fn walk_releases(derivation: &String,
                     name: &String,
                     version: &DirEntry,
                     packages: &mut Vec<Package>)
                     -> BldrResult<()> {
        for release in try!(fs::read_dir(version.path())) {
            let release = try!(release).file_name().to_string_lossy().into_owned().to_string();
            let version = version.file_name().to_string_lossy().into_owned().to_string();
            let package = Package::new(derivation.clone(), name.clone(), version, release);
            packages.push(package)
        }
        Ok(())
    }
}

/// Sorts two packages according to their version.
///
/// We are a bit more strict than your average package management solution on versioning.
/// What we support is the "some number of digits or dots" (the version number),
/// followed by an optional "-" and any alphanumeric string (the extension). When determining sort order, we:
///
/// * Separate the version numbers from the extensions
/// * Split the version numbers into an array of digits on any '.' characters. Digits are convered
///   into <u64>.
/// * Compare the version numbers by iterating over them. If 'a' is greater or lesser than 'b', we
///   return that as the result. If it is equal, we move to the next digit and repeat. If one of
///   the version numbers is exhausted before the other, it gains 0's for the missing slot.
/// * If the version numbers are equal, but either A or B has an extension (but not both) than the
///   version without the extension is greater. (1.0.0 is greater than 1.0.0-alpha6)
/// * If both have an extension, it is compared lexicographically, with the result as the final
///   ordering.
///
/// Returns a BldrError if we fail to match for any reason.
pub fn version_sort(a_version: &str, b_version: &str) -> BldrResult<Ordering> {
    let (a_parts, a_extension) = try!(split_version(a_version));
    let (b_parts, b_extension) = try!(split_version(b_version));
    let mut a_iter = a_parts.iter();
    let mut b_iter = b_parts.iter();
    loop {
        let mut a_exhausted = false;
        let mut b_exhausted = false;
        let a_num = match a_iter.next() {
            Some(i) => try!(i.parse::<u64>()),
            None => {
                a_exhausted = true;
                0u64
            }
        };
        let b_num = match b_iter.next() {
            Some(i) => try!(i.parse::<u64>()),
            None => {
                b_exhausted = true;
                0u64
            }
        };
        if a_exhausted && b_exhausted {
            break;
        }
        match a_num.cmp(&b_num) {
            Ordering::Greater => {
                return Ok(Ordering::Greater);
            }
            Ordering::Equal => {
                continue;
            }
            Ordering::Less => {
                return Ok(Ordering::Less);
            }
        }
    }

    // If you have equal digits, and one has an extension, it is
    // the plain digits who win.
    // 1.0.0-alpha1 vs 1.0.0
    if a_extension.is_some() && b_extension.is_none() {
        return Ok(Ordering::Less);
    } else if a_extension.is_none() && b_extension.is_some() {
        return Ok(Ordering::Greater);
    } else if a_extension.is_none() && b_extension.is_none() {
        return Ok(Ordering::Equal);
    } else {
        let a = match a_extension {
            Some(a) => a,
            None => String::new(),
        };
        let b = match b_extension {
            Some(b) => b,
            None => String::new(),
        };
        return Ok(a.cmp(&b));
    }
}

fn split_version(version: &str) -> BldrResult<(Vec<&str>, Option<String>)> {
    let re = try!(Regex::new(r"([\d\.]+)(-.+)?"));
    let caps = match re.captures(version) {
        Some(caps) => caps,
        None => return Err(bldr_error!(ErrorKind::BadVersion)),
    };
    let version_number = caps.at(1).unwrap();
    let extension = match caps.at(2) {
        Some(e) => {
            let mut estr: String = e.to_string();
            estr.remove(0);
            Some(estr)
        }
        None => None,
    };
    let version_parts: Vec<&str> = version_number.split('.').collect();
    Ok((version_parts, extension))
}

impl PartialEq for Package {
    fn eq(&self, other: &Package) -> bool {
        if self.derivation != other.derivation {
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
    /// * Derivation is ignored in the comparison - my redis and
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

#[cfg(test)]
mod tests {
    use super::{Package, split_version, version_sort};
    use std::cmp::Ordering;
    use std::cmp::PartialOrd;

    #[test]
    fn package_partial_eq() {
        let a = Package::new("bldr".to_string(),
                             "bldr".to_string(),
                             "1.0.0".to_string(),
                             "20150521131555".to_string());
        let b = Package::new("bldr".to_string(),
                             "bldr".to_string(),
                             "1.0.0".to_string(),
                             "20150521131555".to_string());
        assert_eq!(a, b);
    }

    #[test]
    fn package_partial_ord() {
        let a = Package::new("bldr".to_string(),
                             "bldr".to_string(),
                             "1.0.1".to_string(),
                             "20150521131555".to_string());
        let b = Package::new("bldr".to_string(),
                             "bldr".to_string(),
                             "1.0.0".to_string(),
                             "20150521131555".to_string());
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("Ordering should be greater"),
        }
    }

    #[test]
    fn package_partial_ord_bad_name() {
        let a = Package::new("bldr".to_string(),
                             "snoopy".to_string(),
                             "1.0.1".to_string(),
                             "20150521131555".to_string());
        let b = Package::new("bldr".to_string(),
                             "bldr".to_string(),
                             "1.0.0".to_string(),
                             "20150521131555".to_string());
        match a.partial_cmp(&b) {
            Some(_) => panic!("We tried to return an order"),
            None => assert!(true),
        }
    }

    #[test]
    fn package_partial_ord_different_derivation() {
        let a = Package::new("adam".to_string(),
                             "bldr".to_string(),
                             "1.0.0".to_string(),
                             "20150521131555".to_string());
        let b = Package::new("bldr".to_string(),
                             "bldr".to_string(),
                             "1.0.0".to_string(),
                             "20150521131555".to_string());
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Equal),
            None => panic!("We failed to return an order"),
        }
    }

    #[test]
    fn package_partial_ord_release() {
        let a = Package::new("adam".to_string(),
                             "bldr".to_string(),
                             "1.0.0".to_string(),
                             "20150521131556".to_string());
        let b = Package::new("bldr".to_string(),
                             "bldr".to_string(),
                             "1.0.0".to_string(),
                             "20150521131555".to_string());
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("We failed to return an order"),
        }
    }

    #[test]
    fn split_version_returns_both_parts() {
        let svr = split_version("1.2.3-beta16");
        match svr {
            Ok((version_parts, Some(extension))) => {
                assert_eq!(vec!["1", "2", "3"], version_parts);
                assert_eq!("beta16", extension);
            }
            Ok((_, None)) => panic!("Has an extension"),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn version_sort_simple() {
        match version_sort("1.0.0", "2.0.0") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2.0.1", "2.0.0") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2.1.1", "2.1.1") {
            Ok(compare) => assert_eq!(compare, Ordering::Equal),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("20150521131347", "20150521131346") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn version_sort_complex() {
        match version_sort("1.0.0-alpha2", "1.0.0-alpha1") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("1.0.0-alpha1", "1.0.0-alpha2") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("1.0.0-beta1", "1.0.0-alpha1000") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2.1.1", "2.1.1-alpha2") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2.1.1-alpha2", "2.1.1") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
    }
}
