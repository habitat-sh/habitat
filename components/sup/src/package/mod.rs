// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod hooks;
pub mod updater;

pub use self::updater::{PackageUpdater, PackageUpdaterActor, UpdaterMessage};
pub use self::hooks::HookType;

use std::fmt;
use std::fs::{self, File};
use std::os::unix;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::io::prelude::*;
use std::os::unix::fs::MetadataExt;

use time;
use time::Timespec;

use hcore;
use hcore::fs::SERVICE_HOME;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::util;

use self::hooks::HookTable;
use error::{Error, Result, SupError};
use health_check::{self, CheckResult};
use service_config::ServiceConfig;

static LOGKEY: &'static str = "PK";
const INIT_FILENAME: &'static str = "init";
const HEALTHCHECK_FILENAME: &'static str = "health_check";
const FILEUPDATED_FILENAME: &'static str = "file_updated";
const RECONFIGURE_FILENAME: &'static str = "reconfigure";
const RUN_FILENAME: &'static str = "run";
const PIDFILE_NAME: &'static str = "PID";
const SERVICE_PATH_OWNER: &'static str = "bldr";
const SERVICE_PATH_GROUP: &'static str = "bldr";

#[derive(Debug, Clone)]
pub struct Package {
    pub origin: String,
    pub name: String,
    pub version: String,
    pub release: String,
    pub deps: Vec<PackageIdent>,
    pub tdeps: Vec<PackageIdent>,
    pub pkg_install: PackageInstall,
}


impl Package {
    /// Verifies a package is within the package home and returns a struct representing that
    /// package.
    ///
    /// Only the origin and name of a package are required - the latest version/release of a
    /// package will be returned if their optional value is not specified. If only a version is
    /// specified, the latest release of that package origin, name, and version is returned.
    ///
    /// An optional `home` path may be provided to search for a package in a non-default path.
    pub fn load(ident: &PackageIdent, home: Option<&str>) -> Result<Package> {
        // Shim in `PackageInstall` to provide the same original `Package` struct even though some
        // data is duplicated
        let home_path = match home {
            Some(p) => Some(Path::new(p)),
            None => None,
        };
        let pkg_install = try!(PackageInstall::load(ident, home_path));
        Ok(Package {
            origin: pkg_install.ident().origin.clone(),
            name: pkg_install.ident().name.clone(),
            version: pkg_install.ident().version.as_ref().unwrap().clone(),
            release: pkg_install.ident().release.as_ref().unwrap().clone(),
            deps: try!(pkg_install.deps()).clone(),
            tdeps: try!(pkg_install.tdeps()).clone(),
            pkg_install: pkg_install,
        })
    }

    /// Create a pid file for a package
    /// The existence of this file does not guarantee that a
    /// process exists at the PID contained within.
    pub fn create_pidfile(&self, pid: u32) -> Result<()> {
        let service_dir = format!("{}/{}", SERVICE_HOME, self.name);
        let pid_file = format!("{}/{}", service_dir, PIDFILE_NAME);
        debug!("Creating PID file for child {} -> {}", pid_file, pid);
        let mut f = try!(File::create(pid_file));
        try!(write!(f, "{}", pid));
        Ok(())
    }


    /// Remove a pidfile for this package if it exists.
    /// Do NOT fail if there is an error removing the PIDFILE
    pub fn cleanup_pidfile(&self) -> Result<()> {
        let service_dir = format!("{}/{}", SERVICE_HOME, self.name);
        let pid_file = format!("{}/{}", service_dir, PIDFILE_NAME);
        debug!("Attempting to clean up pid file {}", &pid_file);
        match fs::remove_file(pid_file) {
            Ok(_) => {
                debug!("Removed pid file");
            }
            Err(e) => {
                debug!("Error removing pidfile: {}, continuing", e);
            }
        };
        Ok(())
    }

    /// attempt to read the pidfile for this package.
    /// If the pidfile does not exist, then return None,
    /// otherwise, return Some(pid, uptime_seconds).
    pub fn read_pidfile(&self) -> Result<Option<(u32, Timespec)>> {
        let service_dir = format!("{}/{}", SERVICE_HOME, self.name);
        let pid_file = format!("{}/{}", service_dir, PIDFILE_NAME);
        debug!("Reading pidfile {}", &pid_file);

        // check to see if the file exists
        // if it does, we'll return the start_time
        let start_time = match fs::metadata(&pid_file) {
            Ok(metadata) => {
                if metadata.is_file() {
                    Timespec::new(metadata.ctime(), 0 /* nanos */)
                } else {
                    return Ok(None);
                }
            }
            Err(_) => {
                debug!("No pidfile detected");
                return Ok(None);
            }
        };

        let mut f = try!(File::open(pid_file));
        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));
        debug!("pidfile contents = {}", contents);
        let pid = match contents.parse::<u32>() {
            Ok(pid) => pid,
            Err(e) => {
                debug!("Error reading pidfile: {}", e);
                return Err(sup_error!(Error::InvalidPidFile));
            }
        };
        Ok(Some((pid, start_time)))
    }

    /// take a timestamp in seconds since epoch,
    /// calculate how many seconds before right now
    /// this timestamp occurs.
    fn seconds_before_now(t0: Timespec) -> i64 {
        let now = time::now().to_timespec();
        (now - t0).num_seconds()
    }

    /// return a "down" or "run" with uptime in seconds status message
    pub fn status_from_pid(&self, childinfo: Option<(u32, Timespec)>) -> Result<String> {
        match childinfo {
            Some((pid, start_time)) => {
                let diff = Self::seconds_before_now(start_time);
                let s = format!("run: (pid {}) {}s\n", pid, diff);
                Ok(s)
            }
            None => Ok("down".to_string()),
        }
    }

    /// read the pidfile to get a status
    pub fn status_via_pidfile(&self) -> Result<String> {
        let pidinfo = try!(self.read_pidfile());
        self.status_from_pid(pidinfo)
    }

    /// A vector of ports we expose
    pub fn exposes(&self) -> Vec<String> {
        // This function really should be returning a `Result` as it could fail for a gaggle of
        // IO-related reasons. However, in order to preseve the function contract (for now), we're
        // going to potentially swallow some stuff... - FIN
        match self.pkg_install.exposes() {
            Ok(vec) => vec,
            Err(_) => Vec::new(),
        }
    }

    /// Returns a string with the full run path for this package. This path is composed of any
    /// binary paths specified by this package, or its TDEPS, plus the Supervisor or its TDEPS,
    /// plus the existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can invoke the Supervisor,
    /// without having to worry much about context.
    pub fn run_path(&self) -> Result<String> {
        match self.pkg_install.runtime_path() {
            Ok(r) => Ok(r),
            Err(e) => Err(sup_error!(Error::HabitatCore(e))),
        }
    }

    pub fn hook_template_path(&self, hook_type: &HookType) -> PathBuf {
        let base = self.pkg_install.installed_path().join("hooks");
        match *hook_type {
            HookType::Init => base.join(INIT_FILENAME),
            HookType::HealthCheck => base.join(HEALTHCHECK_FILENAME),
            HookType::FileUpdated => base.join(FILEUPDATED_FILENAME),
            HookType::Reconfigure => base.join(RECONFIGURE_FILENAME),
            HookType::Run => base.join(RUN_FILENAME),
        }
    }

    pub fn hook_path(&self, hook_type: &HookType) -> PathBuf {
        let base = PathBuf::from(self.svc_join_path("hooks"));
        match *hook_type {
            HookType::Init => base.join(INIT_FILENAME),
            HookType::HealthCheck => base.join(HEALTHCHECK_FILENAME),
            HookType::FileUpdated => base.join(FILEUPDATED_FILENAME),
            HookType::Reconfigure => base.join(RECONFIGURE_FILENAME),
            HookType::Run => base.join(RUN_FILENAME),
        }
    }

    /// The path to the package on disk.
    pub fn path(&self) -> &Path {
        self.pkg_install.installed_path()
    }

    /// Join a string to the path on disk.
    pub fn join_path(&self, join: &str) -> String {
        self.pkg_install.installed_path().join(join).to_string_lossy().into_owned()
    }

    /// The on disk svc path for this package.
    pub fn svc_path(&self) -> PathBuf {
        hcore::fs::service_path(&self.name)
    }

    /// Join a string to the on disk svc path for this package.
    pub fn svc_join_path(&self, join: &str) -> String {
        format!("{}/{}/{}", SERVICE_HOME, self.name, join)
    }

    /// Create the service path for this package.
    pub fn create_svc_path(&self) -> Result<()> {
        debug!("Creating svc paths");
        try!(fs::create_dir_all(self.svc_join_path("config")));
        try!(fs::create_dir_all(self.svc_join_path("hooks")));
        try!(fs::create_dir_all(self.svc_join_path("toml")));
        try!(fs::create_dir_all(self.svc_join_path("data")));
        try!(fs::create_dir_all(self.svc_join_path("var")));
        try!(fs::create_dir_all(self.svc_join_path("files")));
        try!(util::perm::set_permissions(&self.svc_join_path("files"), "0700"));
        try!(util::perm::set_owner(&self.svc_join_path("files"),
                                   &format!("{}:{}", SERVICE_PATH_OWNER, SERVICE_PATH_GROUP)));
        try!(util::perm::set_permissions(&self.svc_join_path("toml"), "0700"));
        try!(util::perm::set_owner(&self.svc_join_path("data"),
                                   &format!("{}:{}", SERVICE_PATH_OWNER, SERVICE_PATH_GROUP)));
        try!(util::perm::set_permissions(&self.svc_join_path("data"), "0700"));
        try!(util::perm::set_owner(&self.svc_join_path("var"),
                                   &format!("{}:{}", SERVICE_PATH_OWNER, SERVICE_PATH_GROUP)));
        try!(util::perm::set_permissions(&self.svc_join_path("var"), "0700"));
        Ok(())
    }

    /// Copy the "run" file to the svc path.
    pub fn copy_run(&self, context: &ServiceConfig) -> Result<()> {
        debug!("Copying the run file");
        let svc_run = self.svc_join_path(RUN_FILENAME);
        if let Some(hook) = self.hooks().run_hook {
            try!(hook.compile(Some(context)));
            match fs::read_link(&svc_run) {
                Ok(path) => {
                    if path != hook.path {
                        try!(util::perm::set_permissions(hook.path.to_str().unwrap(), "0755"));
                        try!(fs::remove_file(&svc_run));
                        try!(unix::fs::symlink(hook.path, &svc_run));
                    }
                }
                Err(_) => try!(unix::fs::symlink(hook.path, &svc_run)),
            }
        } else {
            let run = self.join_path(RUN_FILENAME);
            try!(util::perm::set_permissions(&run, "0755"));
            match fs::metadata(&svc_run) {
                Ok(_) => try!(fs::remove_file(&svc_run)),
                Err(_) => {}
            }
            try!(unix::fs::symlink(&run, &svc_run));
        }
        Ok(())
    }

    pub fn topology_leader() -> Result<()> {
        Ok(())
    }

    /// Return an iterator of the configuration file names to render.
    ///
    /// This does not return the full path, for convenience with the path
    /// helpers above.
    pub fn config_files(&self) -> Result<Vec<String>> {
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

    pub fn ident(&self) -> &PackageIdent {
        self.pkg_install.ident()
    }

    /// Run initialization hook if present
    pub fn initialize(&self, context: &ServiceConfig) -> Result<()> {
        if let Some(hook) = self.hooks().init_hook {
            match hook.run(Some(context)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    /// Run reconfigure hook if present, DOES NOT restart the package
    pub fn reconfigure(&self, context: &ServiceConfig) -> Result<()> {
        if let Some(hook) = self.hooks().reconfigure_hook {
            match hook.run(Some(context)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    /// Run file_updated hook if present
    pub fn file_updated(&self, context: &ServiceConfig) -> Result<()> {
        if let Some(hook) = self.hooks().file_updated_hook {
            match hook.run(Some(context)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    /// called from outside a topo worker, this will hit the pidfile
    pub fn supervisor_running(&self) -> bool {
        let res = self.status_via_pidfile();
        match res {
            Ok(_) => return true,
            Err(e) => {
                debug!("Supervisor not running?: {:?}", e);
                return false;
            }
        }
    }

    pub fn health_check(&self, config: &ServiceConfig) -> Result<CheckResult> {
        if let Some(hook) = self.hooks().health_check_hook {
            match hook.run(Some(config)) {
                Ok(output) => Ok(health_check::CheckResult::ok(output)),
                Err(SupError { err: Error::HookFailed(_, 1, output), .. }) => {
                    Ok(health_check::CheckResult::warning(output))
                }
                Err(SupError { err: Error::HookFailed(_, 2, output), .. }) => {
                    Ok(health_check::CheckResult::critical(output))
                }
                Err(SupError { err: Error::HookFailed(_, 3, output), .. }) => {
                    Ok(health_check::CheckResult::unknown(output))
                }
                Err(SupError { err: Error::HookFailed(_, code, output), .. }) => {
                    Err(sup_error!(Error::HealthCheck(format!("hook exited code={}, \
                                                                    output={}",
                                                              code,
                                                              output))))
                }
                Err(e) => Err(SupError::from(e)),
            }
        } else {
            let status_output = try!(self.status_via_pidfile());
            let last_config = try!(self.last_config());
            Ok(health_check::CheckResult::ok(format!("{}\n{}", status_output, last_config)))
        }
    }

    pub fn hooks(&self) -> HookTable {
        let mut hooks = HookTable::new(&self);
        hooks.load_hooks();
        hooks
    }

    pub fn last_config(&self) -> Result<String> {
        let mut file = try!(File::open(self.svc_join_path("config.toml")));
        let mut result = String::new();
        try!(file.read_to_string(&mut result));
        Ok(result)
    }
}

impl Into<PackageIdent> for Package {
    fn into(self) -> PackageIdent {
        PackageIdent::new(self.origin,
                          self.name,
                          Some(self.version),
                          Some(self.release))
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident())
    }
}
