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

pub mod hooks;
pub mod updater;

pub use self::updater::{PackageUpdater, PackageUpdaterActor, UpdaterMessage};
pub use self::hooks::HookType;

use std;
use std::env;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::io::prelude::*;

use hcore::package::{PackageIdent, PackageInstall};
use hcore::util;

use self::hooks::{HookTable, HOOK_PERMISSIONS};
use config::gconfig;
use error::{Error, Result, SupError};
use health_check::{self, CheckResult};
use manager::service::config::ServiceConfig;
use supervisor::Supervisor;
use util::path::busybox_paths;
use util::users as hab_users;

static LOGKEY: &'static str = "PK";
const INIT_FILENAME: &'static str = "init";
const HEALTHCHECK_FILENAME: &'static str = "health_check";
const FILEUPDATED_FILENAME: &'static str = "file_updated";
const RECONFIGURE_FILENAME: &'static str = "reconfigure";
const RUN_FILENAME: &'static str = "run";

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
    /// binary paths specified by this package, or its TDEPS, plus a path to a BusyBox,
    /// plus the existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can invoke the Supervisor,
    /// without having to worry much about context.
    pub fn run_path(&self) -> Result<String> {
        let mut paths = match self.pkg_install.runtime_path() {
            Ok(r) => r,
            Err(e) => return Err(sup_error!(Error::HabitatCore(e))),
        };
        for path in try!(busybox_paths()).iter() {
            paths.push(':');
            paths.push_str(&path.to_string_lossy());
        }
        if let Some(val) = env::var_os("PATH") {
            paths.push(':');
            paths.push_str(&val.to_string_lossy());
        }
        Ok(paths)
    }

    pub fn hook_template_path(&self, hook_type: &HookType) -> PathBuf {
        let base = self.config_from().join("hooks");
        match *hook_type {
            HookType::Init => base.join(INIT_FILENAME),
            HookType::HealthCheck => base.join(HEALTHCHECK_FILENAME),
            HookType::FileUpdated => base.join(FILEUPDATED_FILENAME),
            HookType::Reconfigure => base.join(RECONFIGURE_FILENAME),
            HookType::Run => base.join(RUN_FILENAME),
        }
    }

    pub fn hook_path(&self, hook_type: &HookType) -> PathBuf {
        let base = self.pkg_install.svc_hooks_path();
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

    /// The on disk svc path for this package.
    pub fn svc_path(&self) -> PathBuf {
        self.pkg_install.svc_path()
    }

    /// this function wraps create_dir_all so we can give friendly error
    /// messages to the user.
    fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
        debug!("Creating dir with subdirs: {:?}", &path.as_ref());
        if let Err(e) = std::fs::create_dir_all(&path) {
            Err(sup_error!(Error::Permissions(format!("Can't create {:?}, {}", &path.as_ref(), e))))
        } else {
            Ok(())
        }
    }

    /// Create the service path for this package.
    pub fn create_svc_path(&self) -> Result<()> {
        let (user, group) = try!(hab_users::get_user_and_group(&self.pkg_install));

        debug!("Creating svc paths");

        if let Err(e) = Self::create_dir_all(self.pkg_install.svc_path()) {
            outputln!("Can't create directory {}",
                      &self.pkg_install.svc_path().to_str().unwrap());
            outputln!("If this service is running as non-root, you'll need to create \
                       {} and give the current user write access to it",
                      self.pkg_install.svc_path().to_str().unwrap());
            return Err(e);
        }

        try!(Self::create_dir_all(self.pkg_install.svc_config_path()));
        try!(util::perm::set_owner(self.pkg_install.svc_config_path(), &user, &group));
        try!(util::perm::set_permissions(self.pkg_install.svc_config_path(), 0o700));
        try!(Self::create_dir_all(self.pkg_install.svc_data_path()));
        try!(util::perm::set_owner(self.pkg_install.svc_data_path(), &user, &group));
        try!(util::perm::set_permissions(self.pkg_install.svc_data_path(), 0o700));
        try!(Self::create_dir_all(self.pkg_install.svc_files_path()));
        try!(util::perm::set_owner(self.pkg_install.svc_files_path(), &user, &group));
        try!(util::perm::set_permissions(self.pkg_install.svc_files_path(), 0o700));
        try!(Self::create_dir_all(self.pkg_install.svc_hooks_path()));
        try!(Self::create_dir_all(self.pkg_install.svc_var_path()));
        try!(util::perm::set_owner(self.pkg_install.svc_var_path(), &user, &group));
        try!(util::perm::set_permissions(self.pkg_install.svc_var_path(), 0o700));
        try!(Self::remove_symlink(self.pkg_install.svc_static_path()));
        try!(Self::create_dir_all(self.pkg_install.svc_static_path()));
        try!(util::perm::set_owner(self.pkg_install.svc_static_path(), &user, &group));
        try!(util::perm::set_permissions(self.pkg_install.svc_static_path(), 0o700));
        // TODO: Not 100% if this directory is still needed, but for the moment it's still here -
        // FIN
        try!(Self::create_dir_all(self.pkg_install.svc_path().join("toml")));
        try!(util::perm::set_permissions(self.pkg_install.svc_path().join("toml"), 0o700));
        Ok(())
    }

    /// attempt to remove a symlink in the /svc/run/foo/ directory if
    /// the link exists.
    fn remove_symlink<P: AsRef<Path>>(p: P) -> Result<()> {
        let p = p.as_ref();
        if !p.exists() {
            return Ok(());
        }
        // note: we're NOT using p.metadata() here as that will follow the
        // symlink, which returns smd.file_type().is_symlink() == false in all cases.
        let smd = try!(p.symlink_metadata());
        if smd.file_type().is_symlink() {
            try!(std::fs::remove_file(p));
        }
        Ok(())
    }

    /// Copy the "run" file to the svc path.
    pub fn copy_run(&self, context: &ServiceConfig) -> Result<()> {
        debug!("Copying the run file");
        let svc_run = self.pkg_install.svc_path().join(RUN_FILENAME);
        debug!("svc_run = {}", &svc_run.to_str().unwrap());
        if let Some(hook) = self.hooks().run_hook {
            debug!("Compiling hook");
            try!(hook.compile(Some(context)));
            try!(std::fs::copy(hook.path, &svc_run));
            try!(util::perm::set_permissions(&svc_run.to_str().unwrap(), HOOK_PERMISSIONS));
        } else {
            let run = self.path().join(RUN_FILENAME);
            match std::fs::metadata(&run) {
                Ok(_) => {
                    debug!("run file = {}", &run.to_str().unwrap());
                    debug!("svc_run file = {}", &svc_run.to_str().unwrap());
                    try!(Self::remove_symlink(&svc_run));
                    try!(std::fs::copy(&run, &svc_run));
                    try!(util::perm::set_permissions(&svc_run, HOOK_PERMISSIONS));
                }
                Err(e) => {
                    outputln!("Error finding the run file: {}", e);
                    return Err(sup_error!(Error::NoRunFile));
                }
            }
        }
        Ok(())
    }

    pub fn topology_leader() -> Result<()> {
        Ok(())
    }

    pub fn config_from(&self) -> PathBuf {
        gconfig().config_from().as_ref().map_or(self.pkg_install.installed_path().clone(),
                                                |p| PathBuf::from(p))
    }

    /// Return an iterator of the configuration file names to render.
    ///
    /// This does not return the full path, for convenience with the path
    /// helpers above.
    pub fn config_files(&self) -> Result<Vec<String>> {
        let mut files: Vec<String> = Vec::new();
        let config_dir = self.config_from().join("config");
        debug!("Loading configuration from {:?}", config_dir);
        match std::fs::read_dir(config_dir) {
            Ok(config_dir) => {
                for config in config_dir {
                    let config = try!(config);
                    match config.path().file_name() {
                        Some(filename) => {
                            debug!("Looking in {:?}", filename);
                            files.push(filename.to_string_lossy().into_owned().to_string());
                        }
                        None => unreachable!(),
                    }
                }
            }
            Err(e) => {
                debug!("No config directory in package: {}", e);
            }
        }
        Ok(files)
    }

    pub fn ident(&self) -> &PackageIdent {
        self.pkg_install.ident()
    }

    /// Run initialization hook if present
    pub fn initialize(&self) -> Result<()> {
        if let Some(hook) = self.hooks().init_hook {
            match hook.run() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    /// Run reconfigure hook if present. Return false if it is not present, to trigger default
    /// restart behavior.
    pub fn reconfigure(&self) -> Result<bool> {
        if let Some(hook) = self.hooks().reconfigure_hook {
            match hook.run() {
                Ok(_) => Ok(true),
                Err(e) => Err(e),
            }
        } else {
            Ok(false)
        }
    }

    /// Run file_updated hook if present
    pub fn file_updated(&self) -> Result<bool> {
        if let Some(hook) = self.hooks().file_updated_hook {
            match hook.run() {
                Ok(_) => Ok(true),
                Err(e) => Err(e),
            }
        } else {
            Ok(false)
        }
    }

    pub fn health_check(&self, supervisor: &Supervisor) -> Result<CheckResult> {
        if let Some(hook) = self.hooks().health_check_hook {
            match hook.run() {
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
            let (health, status) = supervisor.status();
            let last_config = try!(self.last_config());
            if health {
                Ok(health_check::CheckResult::ok(format!("{}\n{}", status, last_config)))
            } else {
                Ok(health_check::CheckResult::critical(format!("{}\n{}", status, last_config)))
            }
        }
    }

    pub fn hooks(&self) -> HookTable {
        let mut hooks = HookTable::new(&self);
        hooks.load_hooks();
        hooks
    }

    pub fn last_config(&self) -> Result<String> {
        let mut file = try!(File::open(self.pkg_install.svc_path().join("config.toml")));
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
