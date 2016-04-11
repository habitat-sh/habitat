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

use std;
use std::fmt;
use std::fs::File;
use std::os::unix;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::io::prelude::*;

use hcore::package::{PackageIdent, PackageInstall};
use hcore::util;

use self::hooks::HookTable;
use error::{Error, Result, SupError};
use health_check::{self, CheckResult};
use service_config::ServiceConfig;
use supervisor::Supervisor;

static LOGKEY: &'static str = "PK";
const INIT_FILENAME: &'static str = "init";
const HEALTHCHECK_FILENAME: &'static str = "health_check";
const FILEUPDATED_FILENAME: &'static str = "file_updated";
const RECONFIGURE_FILENAME: &'static str = "reconfigure";
const RUN_FILENAME: &'static str = "run";
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

    /// Create the service path for this package.
    pub fn create_svc_path(&self) -> Result<()> {
        let runas = format!("{}:{}", SERVICE_PATH_OWNER, SERVICE_PATH_GROUP);
        debug!("Creating svc paths");
        try!(std::fs::create_dir_all(self.pkg_install.svc_config_path()));
        try!(std::fs::create_dir_all(self.pkg_install.svc_data_path()));
        try!(util::perm::set_owner(self.pkg_install.svc_data_path(), &runas));
        try!(util::perm::set_permissions(self.pkg_install.svc_data_path(), "0700"));
        try!(std::fs::create_dir_all(self.pkg_install.svc_files_path()));
        try!(util::perm::set_owner(self.pkg_install.svc_files_path(), &runas));
        try!(util::perm::set_permissions(self.pkg_install.svc_files_path(), "0700"));
        try!(std::fs::create_dir_all(self.pkg_install.svc_hooks_path()));
        try!(std::fs::create_dir_all(self.pkg_install.svc_var_path()));
        try!(util::perm::set_owner(self.pkg_install.svc_var_path(), &runas));
        try!(util::perm::set_permissions(self.pkg_install.svc_var_path(), "0700"));
        // TODO: Not 100% if this directory is still needed, but for the moment it's still here -
        // FIN
        try!(std::fs::create_dir_all(self.pkg_install.svc_path().join("toml")));
        try!(util::perm::set_permissions(self.pkg_install.svc_path().join("toml"), "0700"));
        Ok(())
    }

    /// Copy the "run" file to the svc path.
    pub fn copy_run(&self, context: &ServiceConfig) -> Result<()> {
        debug!("Copying the run file");
        let svc_run = self.pkg_install.svc_path().join(RUN_FILENAME);
        if let Some(hook) = self.hooks().run_hook {
            try!(hook.compile(Some(context)));
            match std::fs::read_link(&svc_run) {
                Ok(path) => {
                    if path != hook.path {
                        try!(util::perm::set_permissions(hook.path.to_str().unwrap(), "0755"));
                        try!(std::fs::remove_file(&svc_run));
                        try!(unix::fs::symlink(hook.path, &svc_run));
                    }
                }
                Err(_) => try!(unix::fs::symlink(hook.path, &svc_run)),
            }
        } else {
            let run = self.path().join(RUN_FILENAME);
            try!(util::perm::set_permissions(&run, "0755"));
            match std::fs::metadata(&svc_run) {
                Ok(_) => try!(std::fs::remove_file(&svc_run)),
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
        for config in try!(std::fs::read_dir(self.path().join("config"))) {
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

    /// Run reconfigure hook if present. Return false if it is not present, to trigger default
    /// restart behavior.
    pub fn reconfigure(&self, context: &ServiceConfig) -> Result<bool> {
        if let Some(hook) = self.hooks().reconfigure_hook {
            match hook.run(Some(context)) {
                Ok(_) => Ok(true),
                Err(e) => Err(e),
            }
        } else {
            Ok(false)
        }
    }

    /// Run file_updated hook if present
    pub fn file_updated(&self, context: &ServiceConfig) -> Result<bool> {
        if let Some(hook) = self.hooks().file_updated_hook {
            match hook.run(Some(context)) {
                Ok(_) => Ok(true),
                Err(e) => Err(e),
            }
        } else {
            Ok(false)
        }
    }

    pub fn health_check(&self,
                        config: &ServiceConfig,
                        supervisor: &Supervisor)
                        -> Result<CheckResult> {
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
