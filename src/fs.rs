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

use dirs;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tempfile;

use crate::env as henv;
use crate::error::{Error, Result};
use crate::os::users::{self, assert_pkg_user_and_group};
use crate::package::{Identifiable, PackageIdent, PackageInstall};

/// The default root path of the Habitat filesystem
pub const ROOT_PATH: &str = "hab";
/// The default path for any analytics related files
pub const CACHE_ANALYTICS_PATH: &str = "hab/cache/analytics";
/// The default download root path for package artifacts, used on package installation
pub const CACHE_ARTIFACT_PATH: &str = "hab/cache/artifacts";
/// The default path where cryptographic keys are stored
pub const CACHE_KEY_PATH: &str = "hab/cache/keys";
/// The default path where source artifacts are downloaded, extracted, & compiled
pub const CACHE_SRC_PATH: &str = "hab/cache/src";
/// The default path where SSL-related artifacts are placed
pub const CACHE_SSL_PATH: &str = "hab/cache/ssl";
/// The root path for the launcher runtime
pub const LAUNCHER_ROOT_PATH: &str = "hab/launcher";
/// The root path containing all locally installed packages
/// Because this value is used in template rendering, we
/// use native directory separator
#[cfg(not(target_os = "windows"))]
pub const PKG_PATH: &str = "hab/pkgs";
#[cfg(target_os = "windows")]
pub const PKG_PATH: &str = "hab\\pkgs";
/// The environment variable pointing to the filesystem root. This exists for internal
/// Habitat team usage and is not intended to be used by Habitat consumers.
/// Using this variable could lead to broken Supervisor services and it should
/// be used with extreme caution.
pub const FS_ROOT_ENVVAR: &str = "FS_ROOT";
pub const SYSTEMDRIVE_ENVVAR: &str = "SYSTEMDRIVE";
/// The file where user-defined configuration for each service is found.
pub const USER_CONFIG_FILE: &str = "user.toml";
/// Permissions that service-owned service directories should
/// have. The user and group will be `SVC_USER` / `SVC_GROUP`.
#[cfg(not(windows))]
const SVC_DIR_PERMISSIONS: u32 = 0o770;

lazy_static::lazy_static! {
    /// The default filesystem root path.
    ///
    /// WARNING: On Windows this variable mutates on first call if an environment variable with
    ///          the key of `FS_ROOT_ENVVAR` is set.
    pub static ref FS_ROOT_PATH: PathBuf = fs_root_path();

    /// The root path containing all runtime service directories and files
    pub static ref SVC_ROOT: PathBuf = {
        Path::new(&*FS_ROOT_PATH).join("hab").join("svc")
    };

    pub static ref USER_ROOT: PathBuf = {
        Path::new(&*FS_ROOT_PATH).join("hab").join("user")
    };

    static ref EUID: u32 = users::get_effective_uid();

    static ref MY_CACHE_ANALYTICS_PATH: PathBuf = {
        if am_i_root() {
            PathBuf::from(CACHE_ANALYTICS_PATH)
        } else {
            match dirs::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_ANALYTICS_PATH)),
                None => PathBuf::from(CACHE_ANALYTICS_PATH),
            }
        }
    };

    static ref MY_CACHE_ARTIFACT_PATH: PathBuf = {
        if am_i_root() {
            PathBuf::from(CACHE_ARTIFACT_PATH)
        } else {
            match dirs::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_ARTIFACT_PATH)),
                None => PathBuf::from(CACHE_ARTIFACT_PATH),
            }
        }
    };

    static ref MY_CACHE_KEY_PATH: PathBuf = {
        if am_i_root() {
            PathBuf::from(CACHE_KEY_PATH)
        } else {
            match dirs::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_KEY_PATH)),
                None => PathBuf::from(CACHE_KEY_PATH),
            }
        }
    };

    static ref MY_CACHE_SRC_PATH: PathBuf = {
        if am_i_root() {
            PathBuf::from(CACHE_SRC_PATH)
        } else {
            match dirs::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_SRC_PATH)),
                None => PathBuf::from(CACHE_SRC_PATH),
            }
        }
    };

    static ref MY_CACHE_SSL_PATH: PathBuf = {
        if am_i_root() {
            PathBuf::from(CACHE_SSL_PATH)
        } else {
            match dirs::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_SSL_PATH)),
                None => PathBuf::from(CACHE_SSL_PATH),
            }
        }
    };
}

/// Returns the path to the analytics cache, optionally taking a custom filesystem root.
pub fn cache_analytics_path<T>(fs_root_path: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_ANALYTICS_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_ANALYTICS_PATH),
    }
}

/// Returns the path to the artifacts cache, optionally taking a custom filesystem root.
pub fn cache_artifact_path<T>(fs_root_path: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_ARTIFACT_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_ARTIFACT_PATH),
    }
}

/// Returns the path to the keys cache, optionally taking a custom filesystem root.
pub fn cache_key_path<T>(fs_root_path: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_KEY_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_KEY_PATH),
    }
}

/// Returns the path to the src cache, optionally taking a custom filesystem root.
pub fn cache_src_path<T>(fs_root_path: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_SRC_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_SRC_PATH),
    }
}

/// Returns the path to the SSL cache, optionally taking a custom filesystem root.
pub fn cache_ssl_path<T>(fs_root_path: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_SSL_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_SSL_PATH),
    }
}

pub fn pkg_root_path<T>(fs_root: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    let mut buf = fs_root.map_or(PathBuf::from("/"), |p| p.as_ref().into());
    buf.push(PKG_PATH);
    buf
}

pub fn pkg_install_path<T>(ident: &PackageIdent, fs_root: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    assert!(
        ident.fully_qualified(),
        "Cannot determine install path without fully qualified ident"
    );
    let mut pkg_path = pkg_root_path(fs_root);
    pkg_path.push(&ident.origin);
    pkg_path.push(&ident.name);
    pkg_path.push(ident.version.as_ref().unwrap());
    pkg_path.push(ident.release.as_ref().unwrap());
    pkg_path
}

/// Given a linux style absolute path (prepended with '/') and a fs_root,
/// this will "re-root" the path just under the fs_root. Otherwise returns
/// the given path unchanged. Non-Windows platforms will always return the
/// unchanged path.
pub fn fs_rooted_path<T>(path: &PathBuf, fs_root: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    match fs_root {
        Some(ref root) if path.starts_with("/") && cfg!(windows) => {
            Path::new(root.as_ref()).join(path.strip_prefix("/").unwrap())
        }
        _ => path.to_path_buf(),
    }
}

/// Return the path to the root of the launcher runtime directory
pub fn launcher_root_path<T>(fs_root_path: Option<T>) -> PathBuf
where
    T: AsRef<Path>,
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(LAUNCHER_ROOT_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(LAUNCHER_ROOT_PATH),
    }
}

/// Returns the root path for a given service's configuration, files, and data.
pub fn svc_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    SVC_ROOT.join(service_name)
}

/// Returns the path to the configuration directory for a given service.
pub fn svc_config_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("config")
}

/// Returns the path to the install configuration directory for a given service.
pub fn svc_config_install_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("config_install")
}

/// Returns the path to a given service's data.
pub fn svc_data_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("data")
}

/// Returns the path to a given service's gossiped config files.
pub fn svc_files_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("files")
}

/// Returns the path to a given service's hooks.
pub fn svc_hooks_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("hooks")
}

/// Returns the path to a given service's static content.
pub fn svc_static_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("static")
}

/// Returns the path to a given service's variable state.
pub fn svc_var_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("var")
}

/// Returns the path to a given service's logs.
pub fn svc_logs_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("logs")
}

/// Returns the path to a given service's pid file.
pub fn svc_pid_file<T: AsRef<Path>>(service_name: T) -> PathBuf {
    svc_path(service_name).join("PID")
}

/// Returns the root path for a given service's user configuration,
/// files, and data.
pub fn user_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    USER_ROOT.join(service_name)
}

/// Returns the path to a given service's user configuration
/// directory.
pub fn user_config_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    user_path(service_name).join("config")
}

/// Represents the service directory for a given package.
pub struct SvcDir<'a> {
    service_name: &'a str,
    svc_user: &'a str,
    svc_group: &'a str,
}

impl<'a> SvcDir<'a> {
    // TODO (CM): When / if data intended solely for templated content
    // is separated out of Pkg, we could just wrap a &Pkg directly,
    // instead of extracting name, user, and group. Until then,
    // however, we're being explicit to avoid confusion and needless
    // intertwining of code.

    // The fact that all our references are coming from a single Pkg
    // (with a single lifetime) is why we only take a single lifetime
    // parameter; beyond that, there's no intrinsic requirement for
    // the lifetimes of the three struct members to be the same.
    //
    // (They could also be Strings and not references, but there's
    // really no need to make copies of that data.)
    pub fn new(name: &'a str, user: &'a str, group: &'a str) -> Self {
        SvcDir {
            service_name: name,
            svc_user: user,
            svc_group: group,
        }
    }

    /// Create a service directory, including all necessary
    /// sub-directories. Ownership and permissions are handled as
    /// well.
    pub fn create(&self) -> Result<()> {
        if users::can_run_services_as_svc_user() {
            // The only reason we assert that these users exist is
            // because our `set_owner` calls will fail if they
            // don't. If we don't have the ability to to change
            // ownership, however, it doesn't really matter!
            assert_pkg_user_and_group(&self.svc_user, &self.svc_group)?;
        }

        self.create_svc_root()?;
        self.create_all_sup_owned_dirs()?;
        self.create_all_svc_owned_dirs()?;

        Ok(())
    }

    fn create_svc_root(&self) -> Result<()> {
        Self::create_dir_all(svc_path(&self.service_name))
    }

    /// Creates all to sub-directories in a service directory that are
    /// owned by the Supervisor (that is, the user the current thread
    /// is running as).
    fn create_all_sup_owned_dirs(&self) -> Result<()> {
        Self::create_dir_all(svc_hooks_path(&self.service_name))?;
        Self::create_dir_all(svc_logs_path(&self.service_name))?;
        Ok(())
    }

    /// Creates all to sub-directories in a service directory that are
    /// owned by the service user by default.
    ///
    /// If the Supervisor (i.e., the current thread) is not running as
    /// a user that has the ability to change file and directory
    /// ownership, however, they will be owned by the Supervisor
    /// instead.
    fn create_all_svc_owned_dirs(&self) -> Result<()> {
        self.create_svc_owned_dir(svc_config_path(&self.service_name))?;
        self.create_svc_owned_dir(svc_config_install_path(&self.service_name))?;
        self.create_svc_owned_dir(svc_data_path(&self.service_name))?;
        self.create_svc_owned_dir(svc_files_path(&self.service_name))?;
        self.create_svc_owned_dir(svc_var_path(&self.service_name))?;
        self.create_svc_owned_dir(svc_static_path(&self.service_name))?;
        Ok(())
    }

    fn create_svc_owned_dir<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // We do not want to change the permissions of an already
        // existing directory
        // See https://github.com/habitat-sh/habitat/issues/4475
        if path.as_ref().exists() {
            return Ok(());
        }

        Self::create_dir_all(&path)?;
        self.set_permissions(&path)
    }

    fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
        debug!("Creating dir with subdirs: {:?}", &path.as_ref());
        if let Err(e) = fs::create_dir_all(&path) {
            Err(Error::PermissionFailed(format!(
                "Can't create {:?}, {}",
                &path.as_ref(),
                e
            )))
        } else {
            Ok(())
        }
    }

    #[cfg(not(windows))]
    fn set_permissions<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        use crate::util::posix_perm;

        if users::can_run_services_as_svc_user() {
            posix_perm::set_owner(path.as_ref(), &self.svc_user, &self.svc_group)?;
        }
        posix_perm::set_permissions(path.as_ref(), SVC_DIR_PERMISSIONS).map_err(From::from)
    }

    #[cfg(windows)]
    fn set_permissions<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        use crate::util::win_perm;

        win_perm::harden_path(path.as_ref()).map_err(From::from)
    }
}

/// Returns the absolute path for a given command, if it exists, by searching the `PATH`
/// environment variable.
///
/// If the command represents an absolute path, then the `PATH` searching will not be performed. If
/// no absolute path can be found for the command, then `None` is returned.
///
/// On Windows, the PATHEXT environment variable contains common extensions for commands,
/// for example allowing "docker.exe" to be found when searching for "docker".
///
/// # Examples
///
/// Behavior when the command exists on PATH:
///
/// ```
///
/// use std::env;
/// use std::fs;
/// use habitat_core::fs::find_command;
///
/// let first_path = fs::canonicalize("./tests/fixtures").unwrap();
/// let second_path = fs::canonicalize("./tests/fixtures/bin").unwrap();
/// let path_bufs = vec![first_path, second_path];
/// let new_path = env::join_paths(path_bufs).unwrap();
/// env::set_var("PATH", &new_path);
///
/// let result = find_command("bin_with_no_extension");
/// assert_eq!(result.is_some(), true);
/// ```
///
/// Behavior when the command does not exist on PATH:
///
/// ```
///
/// use std::env;
/// use std::fs;
/// use habitat_core::fs::find_command;
///
/// let first_path = fs::canonicalize("./tests/fixtures").unwrap();
/// let second_path = fs::canonicalize("./tests/fixtures/bin").unwrap();
/// let path_bufs = vec![first_path, second_path];
/// let new_path = env::join_paths(path_bufs).unwrap();
/// env::set_var("PATH", &new_path);
///
/// let result = find_command("missing");
/// assert_eq!(result.is_some(), false);
/// ```
///
pub fn find_command<T>(command: T) -> Option<PathBuf>
where
    T: AsRef<Path>,
{
    // If the command path is absolute and a file exists, then use that.
    if command.as_ref().is_absolute() && command.as_ref().is_file() {
        return Some(command.as_ref().to_path_buf());
    }
    // Find the command by checking each entry in `PATH`. If we still can't find it, give up and
    // return `None`.
    match henv::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let candidate = PathBuf::from(&path).join(command.as_ref());
                if candidate.is_file() {
                    return Some(candidate);
                } else if let Some(result) = find_command_with_pathext(&candidate) {
                    return Some(result);
                }
            }
            None
        }
        None => None,
    }
}

/// Returns the absolute path to the given command from a given package installation.
///
/// If the command is not found, then `None` is returned.
///
/// # Failures
///
/// * The path entries metadata cannot be loaded
pub fn find_command_in_pkg<T, U>(
    command: T,
    pkg_install: &PackageInstall,
    fs_root_path: U,
) -> Result<Option<PathBuf>>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    for path in pkg_install.paths()? {
        let stripped = path
            .strip_prefix("/")
            .unwrap_or_else(|_| panic!("Package path missing / prefix {}", path.to_string_lossy()));
        let candidate = fs_root_path.as_ref().join(stripped).join(command.as_ref());
        if candidate.is_file() {
            return Ok(Some(path.join(command.as_ref())));
        } else if let Some(result) = find_command_with_pathext(&candidate) {
            return Ok(Some(result));
        }
    }
    Ok(None)
}

/// Resolves the absolute path to a program in the given package identifier string.
///
/// Note: this function is designed to be callable in `lazy_static!` blocks, meaning that if it
/// can't make forward progress, it will panic and possibly termine the program. This is by design.
///
/// # Panics
///
/// * If the installed package can't be loaded off disk
/// * If the the program can't be found in the installed package
/// * If there is an error looking for the program in the installed package
pub fn resolve_cmd_in_pkg(program: &str, ident_str: &str) -> PathBuf {
    let ident = PackageIdent::from_str(ident_str).unwrap();
    let abs_path = match PackageInstall::load(&ident, None) {
        Ok(ref pkg_install) => {
            match find_command_in_pkg(program, pkg_install, Path::new(&*FS_ROOT_PATH)) {
                Ok(Some(p)) => p,
                Ok(None) => panic!(format!(
                    "Could not find '{}' in the '{}' package! This is required for the \
                     proper operation of this program.",
                    program, &ident
                )),
                Err(err) => panic!(format!(
                    "Error finding '{}' in the '{}' package! This is required for the \
                     proper operation of this program. (Err: {:?})",
                    program, &ident, err
                )),
            }
        }
        Err(err) => panic!(format!(
            "Package installation for '{}' not found on disk! This is required for the \
             proper operation of this program (Err: {:?})",
            &ident, err
        )),
    };
    debug!(
        "resolved absolute path to program, program={}, ident={}, abs_path={}",
        program,
        &ident,
        abs_path.display()
    );
    abs_path
}

// Windows relies on path extensions to resolve commands like `docker` to `docker.exe`
// Path extensions are found in the PATHEXT environment variable.
// We should only search with PATHEXT if the file does not already have an extension.
fn find_command_with_pathext(candidate: &PathBuf) -> Option<PathBuf> {
    if candidate.extension().is_none() {
        if let Some(pathexts) = henv::var_os("PATHEXT") {
            for pathext in env::split_paths(&pathexts) {
                let mut source_candidate = candidate.to_path_buf();
                let extension = pathext.to_str().unwrap().trim_matches('.');
                source_candidate.set_extension(extension);
                let current_candidate = source_candidate.to_path_buf();
                if current_candidate.is_file() {
                    return Some(current_candidate);
                }
            }
        };
    }
    None
}

/// Returns whether or not the current process is running with a root
/// effective user id or not.
///
/// NOTE: This should only be used if *identity* as the root user is
/// important. If you are instead using "root" as a proxy for "has the
/// ability to do X", then test for that specific ability
/// instead. Containerized workflows may require running as a non-root
/// user, while still granting specific abilities.
///
/// See, for example, Linux capabilities.
pub fn am_i_root() -> bool {
    *EUID == 0u32
}

/// Returns a `PathBuf` which represents the filesystem root for Habitat.
///
/// **Note** with the current exception of behavior on Windows (see below), an absolute default
/// path of `"/"` should always be returned. This function is used to populate a one-time static
/// value which cannot be altered for the execution length of a program. Packages in Habitat may
/// contain binaries and libraries having dependent libraries which are located in absolute paths
/// meaning that changing the value from this function will render existing packages un-runnable in
/// the Supervisor. Furthermore as a rule in this codebase, external environment variables should
/// *not* influence the behavior of inner libraries--any environment variables should be detected
/// in a program at CLI parsing time and explicitly passed to inner module functions.
///
/// There is one exception to this rule which is supported for testing only--primarily exercising
/// the Supervisor behavior. It allows setting a testing-only environment variable to influence the
/// file system root for the duration of a running program.  Note that when using such an
/// environment varible, any existing/actual Habitat packages may not run correctly due to the
/// presence of absolute paths in package binaries and libraries. The environment variable will not
/// be referenced, exported, or consumed anywhere else in the system to ensure that it is *only*
/// used internally in test suites.
///
/// Please contact a project maintainer or current owner with any questions. Thanks!
fn fs_root_path() -> PathBuf {
    // This behavior must never be expected, used, or counted on in production. This is explicitly
    // unsupported.
    if let Ok(path) = henv::var("TESTING_FS_ROOT") {
        writeln!(
            io::stderr(),
            "DEBUG: setting custom filesystem root for testing only (TESTING_FS_ROOT='{}')",
            &path
        )
        .expect("Could not write to stderr");
        return PathBuf::from(path);
    }

    // JW TODO: When Windows container studios are available the platform reflection should
    // be removed.
    if cfg!(target_os = "windows") {
        match (henv::var(FS_ROOT_ENVVAR), henv::var(SYSTEMDRIVE_ENVVAR)) {
            (Ok(path), _) => PathBuf::from(path),
            (Err(_), Ok(system_drive)) => PathBuf::from(format!("{}{}", system_drive, "\\")),
            (Err(_), Err(_)) => unreachable!(
                "Windows should always have a SYSTEMDRIVE \
                 environment variable."
            ),
        }
    } else {
        PathBuf::from("/")
    }
}

/// parent returns the parent directory of the given path, accounting
/// for the fact that a relative path with no directory separator
/// returns an empty parent. This function transforms it to return "."
/// so that it can more easily be used in functions that expect to
/// take a directory.
fn parent(p: &Path) -> io::Result<&Path> {
    match p.parent() {
        Some(parent_path) if parent_path.as_os_str().is_empty() => Ok(&Path::new(".")),
        Some(nonempty_parent_path) => Ok(nonempty_parent_path),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path has no parent",
        )),
    }
}

/// An AtomicWriter atomically writes content to a file at the
/// specified path using a tempfile+rename strategy to achieve
/// atomicity.
///
/// The goal of this function is to ensure that other observers can
/// only ever see:
///
/// - Any previous file at that location
/// - The new file with your content
///
/// without seeing any possible intermediate state.
///
/// Assumes that the parent directory of dest_path exists.
///
pub struct AtomicWriter {
    dest: PathBuf,
    tempfile: tempfile::NamedTempFile,
}

impl AtomicWriter {
    pub fn new(dest_path: &Path) -> io::Result<Self> {
        let parent = parent(dest_path)?;
        let tempfile = tempfile::NamedTempFile::new_in(parent)?;
        Ok(Self {
            dest: dest_path.to_path_buf(),
            tempfile: tempfile,
        })
    }

    pub fn with_writer<F, T, E>(mut self, op: F) -> std::result::Result<T, E>
    where
        F: FnOnce(&mut std::fs::File) -> std::result::Result<T, E>,
        E: From<std::io::Error>,
    {
        let r = op(&mut self.tempfile.as_file_mut())?;
        self.finish()?;
        Ok(r)
    }

    /// finish completes the atomic write by calling sync on the
    /// temporary file to ensure all data is flushed to disk and then
    /// renaming the file into place.
    fn finish(self) -> io::Result<()> {
        self.tempfile.as_file().sync_all()?;
        debug!(
            "Renaming {} to {}",
            self.tempfile.path().to_string_lossy(),
            &self.dest.to_string_lossy()
        );
        fs::rename(self.tempfile.path(), &self.dest)?;

        #[cfg(unix)]
        self.sync_parent()?;

        Ok(())
    }

    /// sync_parent syncs the parent directory. This is required on
    /// some filesystems to ensure that rename(), create(), and
    /// unlink() operations have been persisted to disk. sync_parent
    /// ensures the durability of AtomicWriter but is not required for
    /// the atomocity guarantee.
    fn sync_parent(&self) -> io::Result<()> {
        let parent = parent(&self.dest)?;
        let f = fs::File::open(parent)?;
        if let Err(e) = f.sync_all() {
            // sync_all() calls libc::fsync() which will return EINVAL
            // if the filesystem does not support calling fsync() on
            // directories. libc's EINVAL is mapped to InvalidInput.
            if e.kind() == std::io::ErrorKind::InvalidInput {
                info!(
                    "Ignoring InvalidInput from sync_all on {}",
                    parent.to_string_lossy()
                );
                Ok(())
            } else {
                Err(e)
            }
        } else {
            Ok(())
        }
    }
}

/// atomic_write is a helper function for the most common use of
/// AtomicWriter.
pub fn atomic_write(dest_path: &Path, data: impl AsRef<[u8]>) -> io::Result<()> {
    let w = AtomicWriter::new(dest_path)?;
    w.with_writer(|f| f.write_all(data.as_ref()))
}

#[cfg(test)]
mod test_find_command {

    pub use super::find_command;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    #[allow(dead_code)]
    fn setup_pathext() {
        let path_bufs = vec![PathBuf::from(".COM"), PathBuf::from(".EXE")];
        let new_path = env::join_paths(path_bufs).unwrap();
        env::set_var("PATHEXT", &new_path);
    }

    fn setup_empty_pathext() {
        if env::var("PATHEXT").is_ok() {
            env::remove_var("PATHEXT")
        }
    }

    fn setup_path() {
        let orig_path = env::var_os("PATH").unwrap();
        let mut os_paths: Vec<PathBuf> = env::split_paths(&orig_path).collect();
        let first_path = fs::canonicalize("./tests/fixtures").unwrap();
        let second_path = fs::canonicalize("./tests/fixtures/bin").unwrap();
        let mut path_bufs = vec![first_path, second_path];
        path_bufs.append(&mut os_paths);
        let new_path = env::join_paths(path_bufs).unwrap();
        env::set_var("PATH", &new_path);
    }

    mod without_pathext_set {
        pub use super::find_command;
        use super::{setup_empty_pathext, setup_path};

        fn setup_environment() {
            setup_path();
            setup_empty_pathext();
        }

        mod argument_without_extension {
            use super::{find_command, setup_environment};

            #[test]
            fn command_exists() {
                setup_environment();
                let result = find_command("bin_with_no_extension");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            fn command_does_not_exist() {
                setup_environment();
                let result = find_command("missing");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn command_exists_with_extension() {
                setup_environment();
                let result = find_command("win95_dominator");
                assert_eq!(result.is_some(), false);
            }
        }

        mod argument_with_extension {
            use super::{find_command, setup_environment};
            use std::fs::canonicalize;

            #[test]
            fn command_exists() {
                setup_environment();
                let result = find_command("bin_with_extension.exe");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            fn command_does_not_exist() {
                setup_environment();
                let result = find_command("missing.com");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn command_different_extension_does_exist() {
                setup_environment();
                let result = find_command("bin_with_extension.com");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn first_command_on_path_found() {
                setup_environment();
                let target_path = canonicalize("./tests/fixtures/plan.sh").unwrap();
                let result = find_command("plan.sh");
                let found_path = result.unwrap();
                assert_eq!(found_path, target_path);
            }
        }
    }

    #[cfg(target_os = "windows")]
    mod with_pathext_set {
        pub use super::find_command;
        use super::{setup_path, setup_pathext};

        fn setup_environment() {
            setup_path();
            setup_pathext();
        }

        mod argument_without_extension {
            use super::{find_command, setup_environment};

            #[test]
            fn command_exists() {
                setup_environment();
                let result = find_command("bin_with_no_extension");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            fn command_does_not_exist() {
                setup_environment();
                let result = find_command("missing");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            #[allow(non_snake_case)]
            fn command_exists_with_extension_in_PATHEXT() {
                setup_environment();
                let result = find_command("bin_with_extension");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            #[allow(non_snake_case)]
            fn command_exists_with_extension_not_in_PATHEXT() {
                setup_environment();
                let result = find_command("win95_dominator");
                assert_eq!(result.is_some(), false);
            }
        }

        mod argument_with_extension {
            use super::{find_command, setup_environment};
            use std::fs::canonicalize;

            #[test]
            fn command_exists() {
                setup_environment();
                let result = find_command("bin_with_extension.exe");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            fn command_does_not_exist() {
                setup_environment();
                let result = find_command("missing.com");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn command_different_extension_does_exist() {
                setup_environment();
                let result = find_command("bin_with_extension.com");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn first_command_on_path_found() {
                setup_environment();
                let target_path = canonicalize("./tests/fixtures/plan.sh").unwrap();
                let result = find_command("plan.sh");
                let found_path = result.unwrap();
                assert_eq!(found_path, target_path);
            }
        }
    }
}

#[cfg(test)]
mod test_atomic_writer {
    use super::{atomic_write, AtomicWriter};
    use std::fs::{remove_file, File};
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::panic;
    use tempfile;

    const EXPECTED_CONTENT: &str = "A very good file format";

    #[test]
    fn atomic_write_writes_file() {
        let dest_file = tempfile::NamedTempFile::new().expect("could not create temp file");
        let dest_file_path = dest_file.path();
        remove_file(dest_file_path).expect("could not delete temp file");

        let res = atomic_write(dest_file_path, EXPECTED_CONTENT);
        assert!(res.is_ok());
        let mut f = File::open(dest_file_path).expect("file not found");
        let mut actual_content = String::new();
        f.read_to_string(&mut actual_content)
            .expect("failed to read file");
        assert_eq!(EXPECTED_CONTENT, actual_content);
    }

    #[test]
    fn with_atomic_writer_writes_file() {
        let dest_file = tempfile::NamedTempFile::new().expect("could not create temp file");
        let dest_file_path = dest_file.path();
        remove_file(dest_file_path).expect("could not delete temp file");

        let w = AtomicWriter::new(dest_file_path).expect("could not create AtomicWriter");
        let res = w.with_writer(|writer| {
            writer.write_all(b"not the right content")?;
            writer.seek(SeekFrom::Start(0))?;
            writer.write_all(EXPECTED_CONTENT.as_bytes())?;
            writer.flush() // not needed, just making sure we can call it
        });
        assert!(res.is_ok());

        let mut f = File::open(dest_file_path).expect("file not found");
        let mut actual_content = String::new();
        f.read_to_string(&mut actual_content)
            .expect("failed to read file");
        assert_eq!(EXPECTED_CONTENT, actual_content);
    }
}
