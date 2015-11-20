//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::os::unix;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::process::{self, Command};
use std::sync::{Arc, RwLock};

use mustache;
use regex::Regex;
use wonder;
use wonder::actor::{GenServer, InitResult, HandleResult, ActorSender, ActorResult};

use fs::{PACKAGE_CACHE, PACKAGE_HOME, SERVICE_HOME};
use command;
use error::{BldrResult, BldrError};
use health_check::{self, CheckResult};
use service_config::ServiceConfig;
use util::{self, convert};
use repo;

const INIT_FILENAME: &'static str = "init";
const HEALTHCHECK_FILENAME: &'static str = "health_check";
const RECONFIGURE_FILENAME: &'static str = "reconfigure";
const RUN_FILENAME: &'static str = "run";

const TIMEOUT_MS: u64 = 60_000;

pub type PackageUpdaterActor = wonder::actor::Actor<UpdaterMessage>;

#[derive(Debug, Clone, Eq, RustcDecodable, RustcEncodable)]
pub struct Package {
    pub derivation: String,
    pub name: String,
    pub version: String,
    pub release: String,
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

#[derive(Debug, Clone)]
pub enum HookType {
    HealthCheck,
    Reconfigure,
    Run,
    Init,
}

impl fmt::Display for HookType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HookType::Init => write!(f, "init"),
            &HookType::HealthCheck => write!(f, "health_check"),
            &HookType::Reconfigure => write!(f, "reconfigure"),
            &HookType::Run => write!(f, "run"),
        }
    }
}

pub struct Hook {
    pub htype: HookType,
    pub template: PathBuf,
    path: PathBuf,
}

impl Hook {
    pub fn new(htype: HookType, template: PathBuf, path: PathBuf) -> Self {
        Hook {
            htype: htype,
            template: template,
            path: path,
        }
    }

    pub fn run(&self, context: Option<&ServiceConfig>) -> BldrResult<String> {
        try!(self.compile(context));
        match Command::new(&self.path).output() {
            Ok(result) => {
                let output = Self::format_output(&result);
                match result.status.code() {
                    Some(0) => Ok(output),
                    Some(code) => Err(BldrError::HookFailed(self.htype.clone(), code, output)),
                    None => Err(BldrError::HookFailed(self.htype.clone(), -1, output)),
                }
            }
            Err(_) => {
                let err = format!("couldn't run hook: {}", &self.path.to_string_lossy());
                Err(BldrError::HookFailed(self.htype.clone(), -1, err))
            }
        }
    }

    pub fn compile(&self, context: Option<&ServiceConfig>) -> BldrResult<()> {
        if let Some(ctx) = context {
            let template = try!(mustache::compile_path(&self.template));
            let mut out = Vec::new();
            let toml = try!(ctx.compile_toml());
            let data = convert::toml_table_to_mustache(toml);
            template.render_data(&mut out, &data);
            let data = try!(String::from_utf8(out));
            let mut file = try!(OpenOptions::new()
                                    .write(true)
                                    .truncate(true)
                                    .create(true)
                                    .read(true)
                                    .mode(0o770)
                                    .open(&self.path));
            try!(write!(&mut file, "{}", data));
            Ok(())
        } else {
            try!(fs::copy(&self.template, &self.path));
            Ok(())
        }
    }

    fn format_output(output: &process::Output) -> String {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("{}\n{}", stdout, stderr)
    }
}

pub struct HookTable<'a> {
    pub package: &'a Package,
    pub init_hook: Option<Hook>,
    pub health_check_hook: Option<Hook>,
    pub reconfigure_hook: Option<Hook>,
    pub run_hook: Option<Hook>,
}

impl<'a> HookTable<'a> {
    pub fn new(package: &'a Package) -> Self {
        HookTable {
            package: package,
            init_hook: None,
            health_check_hook: None,
            reconfigure_hook: None,
            run_hook: None,
        }
    }

    pub fn load_hooks(&mut self) -> &mut Self {
        let hook_path = self.package.join_path("hooks");
        let path = Path::new(&hook_path);
        match fs::metadata(path) {
            Ok(meta) => {
                if meta.is_dir() {
                    self.init_hook = self.load_hook(HookType::Init);
                    self.reconfigure_hook = self.load_hook(HookType::Reconfigure);
                    self.health_check_hook = self.load_hook(HookType::HealthCheck);
                    self.run_hook = self.load_hook(HookType::Run);
                }
            }
            Err(_) => {}
        }
        self
    }

    fn load_hook(&self, hook_type: HookType) -> Option<Hook> {
        let template = self.package.hook_template_path(&hook_type);
        let concrete = self.package.hook_path(&hook_type);
        match fs::metadata(&template) {
            Ok(_) => Some(Hook::new(hook_type, template, concrete)),
            Err(_) => None,
        }
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
    pub fn new(deriv: String, name: String, version: String, release: String) -> Package {
        Package {
            derivation: deriv,
            name: name,
            version: version,
            release: release,
        }
    }

    pub fn from_path(spath: &str) -> BldrResult<Package> {
        if Path::new(spath).starts_with(PACKAGE_HOME) {
            let items: Vec<&str> = spath.split("/").collect();
            Ok(Self::new(items[3].to_string(),
                         items[4].to_string(),
                         items[5].to_string(),
                         items[6].to_string()))
        } else {
            Err(BldrError::PackageLoad(spath.to_string()))
        }
    }

    pub fn latest(deriv: &str,
                  pkg: &str,
                  ver: Option<&str>,
                  opt_path: Option<&str>)
                  -> BldrResult<Package> {
        let path = opt_path.unwrap_or(PACKAGE_HOME);
        let pl = try!(Self::package_list(path));
        let latest: Option<Package> = pl.iter()
                                        .filter(|&p| {
                                            if ver.is_some() {
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
        let ver = if ver.is_some() {
            Some(String::from(ver.unwrap()))
        } else {
            None
        };
        latest.ok_or(BldrError::PackageNotFound(deriv.to_string(), pkg.to_string(), ver))
    }

    pub fn signal(&self, signal: Signal) -> BldrResult<String> {
        let runit_pkg = try!(Self::latest("chef", "runit", None, None));
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
                return Err(BldrError::SupervisorSignalFailed);
            }
        }
    }

    /// Get status of running package.
    pub fn status(&self) -> BldrResult<String> {
        self.signal(Signal::Status)
    }

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
            try!(fs::remove_file(&srvc_run));
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
                    Err(BldrError::HookFailed(HookType::Reconfigure, -1, output))
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
                Err(BldrError::HookFailed(_, 1, output)) =>
                    Ok(health_check::CheckResult::warning(output)),
                Err(BldrError::HookFailed(_, 2, output)) =>
                    Ok(health_check::CheckResult::critical(output)),
                Err(BldrError::HookFailed(_, 3, output)) =>
                    Ok(health_check::CheckResult::unknown(output)),
                Err(BldrError::HookFailed(_, code, output)) => {
                    Err(BldrError::HealthCheck(format!("hook exited code={}, output={}",
                                                       code,
                                                       output)))
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

pub struct PackageUpdater;

impl PackageUpdater {
    pub fn start(url: &str, package: Arc<RwLock<Package>>) -> PackageUpdaterActor {
        let state = UpdaterState::new(url.to_string(), package);
        wonder::actor::Builder::new(PackageUpdater)
            .name("package-updater".to_string())
            .start(state)
            .unwrap()
    }

    /// Signal a package updater to transition it's status from `stopped` to `running`. An updater
    /// has the `stopped` status after it has found and notified the main thread of an updated
    /// package.
    pub fn run(actor: &PackageUpdaterActor) -> ActorResult<()> {
        actor.cast(UpdaterMessage::Run)
    }
}

pub struct UpdaterState {
    pub repo: String,
    pub package: Arc<RwLock<Package>>,
    pub status: UpdaterStatus,
}

impl UpdaterState {
    pub fn new(repo: String, package: Arc<RwLock<Package>>) -> Self {
        UpdaterState {
            repo: repo,
            package: package,
            status: UpdaterStatus::Stopped,
        }
    }
}

#[derive(Debug)]
pub enum UpdaterMessage {
    Ok,
    Run,
    Stop,
    Update(Package),
}

pub enum UpdaterStatus {
    Running,
    Stopped,
}

impl GenServer for PackageUpdater {
    type T = UpdaterMessage;
    type S = UpdaterState;
    type E = BldrError;

    fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
        state.status = UpdaterStatus::Running;
        Ok(Some(TIMEOUT_MS))
    }

    fn handle_timeout(&self,
                      tx: &ActorSender<Self::T>,
                      _me: &ActorSender<Self::T>,
                      state: &mut Self::S)
                      -> HandleResult<Self::T> {
        let package = state.package.read().unwrap();
        match repo::client::show_package_latest(&state.repo,
                                                &package.derivation,
                                                &package.name,
                                                None) {
            Ok(latest) => {
                if latest > *package {
                    match repo::client::fetch_package_exact(&state.repo, &latest, PACKAGE_CACHE) {
                        Ok(path) => {
                            debug!("Updater downloaded new package to {:?}", path);
                            // JW TODO: actually handle verify and unpack results
                            command::install::verify(&package.name, &path).unwrap();
                            command::install::unpack(&package.name, &path).unwrap();
                            state.status = UpdaterStatus::Stopped;
                            let msg = wonder::actor::Message::Cast(UpdaterMessage::Update(latest));
                            tx.send(msg).unwrap();
                            HandleResult::NoReply(None)
                        }
                        Err(e) => {
                            debug!("Failed to download package: {:?}", e);
                            HandleResult::NoReply(Some(TIMEOUT_MS))
                        }
                    }
                } else {
                    debug!("Package found is not newer than ours");
                    HandleResult::NoReply(Some(TIMEOUT_MS))
                }
            }
            Err(e) => {
                debug!("Updater failed to get latest package: {:?}", e);
                HandleResult::NoReply(Some(TIMEOUT_MS))
            }
        }
    }

    fn handle_cast(&self,
                   msg: Self::T,
                   _tx: &ActorSender<Self::T>,
                   _me: &ActorSender<Self::T>,
                   state: &mut Self::S)
                   -> HandleResult<Self::T> {
        match msg {
            UpdaterMessage::Run => HandleResult::NoReply(Some(TIMEOUT_MS)),
            _ => {
                match state.status {
                    UpdaterStatus::Running => HandleResult::NoReply(Some(TIMEOUT_MS)),
                    UpdaterStatus::Stopped => HandleResult::NoReply(None),
                }
            }
        }
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
        None => return Err(BldrError::BadVersion),
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
        let a = Package {
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
        let b = Package {
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn package_partial_ord() {
        let a = Package {
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.1".to_string(),
            release: "20150521131555".to_string(),
        };
        let b = Package {
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("Ordering should be greater"),
        }
    }

    #[test]
    fn package_partial_ord_bad_name() {
        let a = Package {
            derivation: "bldr".to_string(),
            name: "snoopy".to_string(),
            version: "1.0.1".to_string(),
            release: "20150521131555".to_string(),
        };
        let b = Package {
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
        match a.partial_cmp(&b) {
            Some(_) => panic!("We tried to return an order"),
            None => assert!(true),
        }
    }

    #[test]
    fn package_partial_ord_different_derivation() {
        let a = Package {
            derivation: "adam".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
        let b = Package {
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Equal),
            None => panic!("We failed to return an order"),
        }
    }

    #[test]
    fn package_partial_ord_release() {
        let a = Package {
            derivation: "adam".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131556".to_string(),
        };
        let b = Package {
            derivation: "bldr".to_string(),
            name: "bldr".to_string(),
            version: "1.0.0".to_string(),
            release: "20150521131555".to_string(),
        };
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
