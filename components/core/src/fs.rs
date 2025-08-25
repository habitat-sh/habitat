#[cfg(not(windows))]
use crate::util::posix_perm::{self,
                              set_permissions};
#[cfg(windows)]
use crate::util::win_perm::{self,
                            set_permissions};
use log::{debug,
          warn};
#[cfg(windows)]
use std::{iter,
          os::windows::ffi::OsStrExt};
#[cfg(windows)]
use winapi::um::winbase::MoveFileExW;

use crate::{env as henv,
            error::{Error,
                    Result},
            os::{process,
                 users::{self,
                         assert_pkg_user_and_group}},
            package::{Identifiable,
                      PackageIdent,
                      PackageInstall}};
use std::{env,
          fmt,
          fs,
          io::{self,
               Write},
          path::{Path,
                 PathBuf},
          str::FromStr};

/// The default root path of the Habitat filesystem
pub const ROOT_PATH: &str = "hab";
/// The default cache path
pub const CACHE_PATH: &str = "hab/cache";
/// The default download root path for package artifacts, used on package installation
pub const CACHE_ARTIFACT_PATH: &str = "hab/cache/artifacts";
/// The default path where hab-plan-build scripts are written, used for native package builds
pub const CACHE_BUILD_PATH: &str = "hab/cache/build";
/// The default path where cryptographic keys are stored
pub const CACHE_KEY_PATH_POSTFIX: &str = "hab/cache/keys";
/// The default path for ctl gateway TLS certificate and keys
pub const HAB_CTL_KEYS_CACHE: &str = "/hab/cache/keys/ctl";
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
/// The environment variable pointing to the filesystem root. This exists for internal Habitat team
/// usage and is not intended to be used by Habitat consumers. Using this variable could lead to
/// broken Supervisor services and should be used with extreme caution. The services may break due
/// to absolute paths in package binaries and libraries. Valid use cases include limited  testing or
/// creating new self-contained root filesystems for tarballs or containers.
pub const FS_ROOT_ENVVAR: &str = "FS_ROOT";
pub const SYSTEMDRIVE_ENVVAR: &str = "SYSTEMDRIVE";
/// The file where user-defined configuration for each service is found.
pub const USER_CONFIG_FILE: &str = "user.toml";
/// Permissions that service-owned service directories should
/// have. The user and group will be `SVC_USER` / `SVC_GROUP`.
#[cfg(not(windows))]
const SVC_DIR_PERMISSIONS: u32 = 0o770;
/// Permissions applied to artifacts that are downloaded and/or
/// cached. On Unix platforms, they are world-readable because there's
/// no reason for them to be locked down any tighter.
#[cfg(not(windows))]
pub const DEFAULT_CACHED_ARTIFACT_PERMISSIONS: Permissions = Permissions::Explicit(0o644);
/// Permissions applied to artifacts that are downloaded and/or
/// cached. On Windows, we don't need do to anything particularly
/// special, since artifacts will generally inherit the permissions of
/// their containing directory.
#[cfg(windows)]
pub const DEFAULT_CACHED_ARTIFACT_PERMISSIONS: Permissions = Permissions::Standard;

/// Permissions applied to downloaded public keys.
#[cfg(not(windows))]
pub const DEFAULT_PUBLIC_KEY_PERMISSIONS: Permissions = Permissions::Explicit(0o444);
/// Permissions applied to downloaded public keys.
#[cfg(windows)]
pub const DEFAULT_PUBLIC_KEY_PERMISSIONS: Permissions = Permissions::Standard;

/// Permissions applied to downloaded secret keys.
#[cfg(not(windows))]
pub const DEFAULT_SECRET_KEY_PERMISSIONS: Permissions = Permissions::Explicit(0o400);
/// Permissions applied to downloaded secret keys.
#[cfg(windows)]
pub const DEFAULT_SECRET_KEY_PERMISSIONS: Permissions = Permissions::Standard;

/// An `Option`-like abstraction over platform-specific ways to model
/// file permissions.
#[derive(Default, PartialEq)]
pub enum Permissions {
    /// Don't take any special action to set permissions beyond what
    /// they are "normally" set to when they are created. Here,
    /// "normal" denotes the low-level programming library sense,
    /// rather than any particular domain-specific sense.
    ///
    /// Think of this as a more semantically-descriptive and
    /// permission-specific version of `Option::None`.
    #[default]
    Standard,
    /// Indicates that a file should be created with very specific
    /// permissions.
    ///
    /// Think of this as a more semantically-descriptive and
    /// permission-specific version of `Option::Some`.
    #[cfg(windows)]
    Explicit(Vec<win_perm::PermissionEntry>),
    #[cfg(not(windows))]
    Explicit(u32),
}

// Explicitly implementing this so we can get octal formatting on
// Linux. Otherwise, it would just be a regular decimal number, which
// isn't very helpful when dealing with permission bits.
impl fmt::Debug for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            #[cfg(windows)]
            Self::Explicit(permissions) => write!(f, "Explicit({:?})", permissions),
            #[cfg(not(windows))]
            Self::Explicit(permissions) => write!(f, "Explicit({:#o})", permissions),
        }
    }
}

lazy_static::lazy_static! {
    /// The default filesystem root path to base all commands from. This is lazily generated on
    /// first call and reflects on the presence and value of the environment variable keyed as
    /// `FS_ROOT_ENVVAR`. This should be the only use of `FS_ROOT_ENVAR`. The environment variable will not
    /// be referenced, exported, or consumed anywhere else in the system to ensure that it is **ONLY**
    /// used internally in test suites. See `FS_ROOT_ENVVAR` documentation.
    pub static ref FS_ROOT_PATH: PathBuf = {
        if cfg!(target_os = "windows") {
            match (henv::var(FS_ROOT_ENVVAR), henv::var(SYSTEMDRIVE_ENVVAR)) {
                (Ok(path), _) => PathBuf::from(path),
                (Err(_), Ok(system_drive)) => PathBuf::from(format!("{}{}", system_drive, "\\")),
                (Err(_), Err(_)) => unreachable!(
                    "Windows should always have a SYSTEMDRIVE \
                    environment variable."
                ),
            }
        } else if let Ok(root) = henv::var(FS_ROOT_ENVVAR) {
            PathBuf::from(root)
        } else {
            PathBuf::from("/")
        }
    };


    /// The root path containing all runtime service directories and files
    pub static ref SVC_ROOT: PathBuf = {
        Path::new(&*FS_ROOT_PATH).join("hab").join("svc")
    };

    pub static ref USER_ROOT: PathBuf = {
        Path::new(&*FS_ROOT_PATH).join("hab").join("user")
    };

    static ref EUID: u32 = users::get_effective_uid();

    static ref MY_CACHE_PATH: PathBuf = {
        if am_i_root() {
            PathBuf::from(CACHE_PATH)
        } else {
            match dirs::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_PATH)),
                None => PathBuf::from(CACHE_PATH),
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

    static ref MY_CACHE_BUILD_PATH: PathBuf = {
        if am_i_root() {
            PathBuf::from(CACHE_BUILD_PATH)
        } else {
            match dirs::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_BUILD_PATH)),
                None => PathBuf::from(CACHE_BUILD_PATH),
            }
        }
    };

    static ref MY_CACHE_KEY_PATH_POSTFIX: PathBuf = {
        if am_i_root() {
            PathBuf::from(CACHE_KEY_PATH_POSTFIX)
        } else {
            match dirs::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_KEY_PATH_POSTFIX)),
                None => PathBuf::from(CACHE_KEY_PATH_POSTFIX),
            }
        }
    };

    /// The path to the keys cache rooted at `FS_ROOT_PATH`
    pub static ref CACHE_KEY_PATH: PathBuf = cache_key_path(&*FS_ROOT_PATH);

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

pub fn cache_root_path<T>(fs_root_path: Option<T>) -> PathBuf
    where T: AsRef<Path>
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_PATH),
    }
}

/// Returns the path to the artifacts cache, optionally taking a custom filesystem root.
pub fn cache_artifact_path<T>(fs_root_path: Option<T>) -> PathBuf
    where T: AsRef<Path>
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_ARTIFACT_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_ARTIFACT_PATH),
    }
}

/// Returns the path to the hab-plan-build script files used for native plan builds, optionally
/// taking a custom filesystem root.
pub fn cache_build_path<T>(fs_root_path: Option<T>) -> PathBuf
    where T: AsRef<Path>
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_BUILD_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_BUILD_PATH),
    }
}

/// Returns the path to the keys cache with a custom filesystem root.
pub fn cache_key_path(root_path: impl AsRef<Path>) -> PathBuf {
    root_path.as_ref().join(&*MY_CACHE_KEY_PATH_POSTFIX)
}

/// Returns the path to the src cache, optionally taking a custom filesystem root.
pub fn cache_src_path<T>(fs_root_path: Option<T>) -> PathBuf
    where T: AsRef<Path>
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_SRC_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_SRC_PATH),
    }
}

/// Returns the path to the SSL cache, optionally taking a custom filesystem root.
pub fn cache_ssl_path<T>(fs_root_path: Option<T>) -> PathBuf
    where T: AsRef<Path>
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(&*MY_CACHE_SSL_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CACHE_SSL_PATH),
    }
}

pub fn pkg_root_path<T>(fs_root: Option<T>) -> PathBuf
    where T: AsRef<Path>
{
    let mut buf = fs_root.map_or_else(|| PathBuf::from("/"), |p| p.as_ref().into());
    buf.push(PKG_PATH);
    buf
}

pub fn pkg_install_path<T>(ident: &PackageIdent, fs_root: Option<T>) -> PathBuf
    where T: AsRef<Path>
{
    assert!(ident.fully_qualified(),
            "Cannot determine install path without fully qualified ident");
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
pub fn fs_rooted_path(path: &Path, fs_root: &Path) -> PathBuf {
    if path.starts_with("/") && cfg!(windows) {
        fs_root.join(path.strip_prefix("/").unwrap())
    } else {
        path.to_path_buf()
    }
}

/// Return the path to the root of the launcher runtime directory
pub fn launcher_root_path<T>(fs_root_path: Option<T>) -> PathBuf
    where T: AsRef<Path>
{
    match fs_root_path {
        Some(fs_root_path) => fs_root_path.as_ref().join(LAUNCHER_ROOT_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(LAUNCHER_ROOT_PATH),
    }
}

/// Returns the root path for a given service's configuration, files, and data.
pub fn svc_path<T: AsRef<Path>>(service_name: T) -> PathBuf { SVC_ROOT.join(service_name) }

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
pub fn user_path<T: AsRef<Path>>(service_name: T) -> PathBuf { USER_ROOT.join(service_name) }

/// Returns the path to a given service's user configuration
/// directory.
pub fn user_config_path<T: AsRef<Path>>(service_name: T) -> PathBuf {
    user_path(service_name).join("config")
}

/// This produces a list of the default windows system paths
/// that are set by default on any modern version of windows.
/// This also arranges them in the same order as they would be by
/// default. The ordering is likely not super important but we
/// might as well match how windows sets them just in case.
pub fn windows_system_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(sys_root) = env::var_os("SystemRoot") {
        let system32 = Path::new(&sys_root).join("system32");
        paths.push(system32.clone());
        paths.push(PathBuf::from(sys_root));
        paths.push(system32.join("wbem"));
        paths.push(system32.join("WindowsPowerShell").join("v1.0"));
    }
    paths
}

/// Represents the service directory for a given package.
pub struct SvcDir<'a> {
    service_name: &'a str,
    svc_user:     &'a str,
    svc_group:    &'a str,
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
        SvcDir { service_name: name,
                 svc_user:     user,
                 svc_group:    group, }
    }

    /// Create a service directory, including all necessary
    /// sub-directories. Ownership and permissions are handled as
    /// well.
    pub fn create(&self) -> Result<()> {
        if process::can_run_services_as_svc_user() {
            // The only reason we assert that these users exist is
            // because our `set_owner` calls will fail if they
            // don't. If we don't have the ability to to change
            // ownership, however, it doesn't really matter!
            assert_pkg_user_and_group(self.svc_user, self.svc_group)?;
        }

        self.create_svc_root()?;
        self.create_all_sup_owned_dirs()?;
        self.create_all_svc_owned_dirs()?;

        Ok(())
    }

    /// Remove all templated content (hooks and configuration) from a
    /// service directory.
    ///
    /// Useful for removing rendered files that may be from older
    /// versions of a service that have been removed from the current
    /// version.
    pub fn purge_templated_content(&self) -> Result<()> {
        for dir_path in &[svc_config_path(self.service_name),
                          svc_hooks_path(self.service_name)]
        {
            debug!("Purging any old templated content from {}",
                   dir_path.display());
            Self::purge_directory_content(dir_path)?;
        }
        Ok(())
    }

    /// Utility function that removes all files in `root`.
    fn purge_directory_content(root: &Path) -> Result<()> {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            match entry.file_type() {
                Ok(ft) => {
                    debug!("Purging {:?} {:?}", ft, entry);
                    if ft.is_file() || ft.is_symlink() {
                        fs::remove_file(entry.path())?;
                    } else if ft.is_dir() {
                        fs::remove_dir_all(entry.path())?;
                    } else {
                        debug!("Nothing to do for {:?}", ft);
                    }
                }
                Err(e) => {
                    warn!("Not purging {}; could not determine file type: {}",
                          entry.path().display(),
                          e);
                }
            }
        }
        Ok(())
    }

    fn create_svc_root(&self) -> Result<()> {
        let svc_path = svc_path(self.service_name);
        Self::create_dir_all(&svc_path)?;
        #[cfg(unix)]
        posix_perm::ensure_path_permissions(&svc_path, 0o755)?;
        Ok(())
    }

    /// Creates all to sub-directories in a service directory that are
    /// owned by the Supervisor (that is, the user the current thread
    /// is running as).
    fn create_all_sup_owned_dirs(&self) -> Result<()> {
        let svc_hooks_path = svc_hooks_path(self.service_name);
        Self::create_dir_all(&svc_hooks_path)?;

        let svc_logs_path = svc_logs_path(self.service_name);
        Self::create_dir_all(&svc_logs_path)?;

        #[cfg(unix)]
        {
            posix_perm::ensure_path_permissions(&svc_hooks_path, 0o755)?;
            posix_perm::ensure_path_permissions(&svc_logs_path, 0o755)?;
        }

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
        self.create_svc_owned_dir(svc_config_path(self.service_name))?;
        self.create_svc_owned_dir(svc_config_install_path(self.service_name))?;
        self.create_svc_owned_dir(svc_data_path(self.service_name))?;
        self.create_svc_owned_dir(svc_files_path(self.service_name))?;
        self.create_svc_owned_dir(svc_var_path(self.service_name))?;
        self.create_svc_owned_dir(svc_static_path(self.service_name))?;
        Ok(())
    }

    fn create_svc_owned_dir<P>(&self, path: P) -> Result<()>
        where P: AsRef<Path>
    {
        // We do not want to change the permissions of an already
        // existing directory
        // See https://github.com/habitat-sh/habitat/issues/4475
        if path.as_ref().exists() {
            return Ok(());
        }

        Self::create_dir_all(&path)?;
        self.set_permissions_and_ownership(&path)
    }

    fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
        debug!("Creating dir with subdirs: {:?}", &path.as_ref());
        if let Err(e) = fs::create_dir_all(&path) {
            Err(Error::PermissionFailed(format!("Can't create {:?}, {}",
                                                &path.as_ref(),
                                                e)))
        } else {
            Ok(())
        }
    }

    #[cfg(not(windows))]
    fn set_permissions_and_ownership<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        if process::can_run_services_as_svc_user() {
            posix_perm::set_owner(path.as_ref(), &self.svc_user, &self.svc_group)?;
        }
        posix_perm::set_permissions(path.as_ref(), SVC_DIR_PERMISSIONS)
    }

    #[cfg(windows)]
    fn set_permissions_and_ownership<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        win_perm::harden_path(path.as_ref())
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
/// use habitat_core::fs::find_command;
/// use std::{env,
///           path::PathBuf};
///
/// let first_path = PathBuf::from("tests/fixtures");
/// let second_path = PathBuf::from("tests/fixtures/bin");
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
/// use habitat_core::fs::find_command;
/// use std::{env,
///           path::PathBuf};
///
/// let first_path = PathBuf::from("tests/fixtures");
/// let second_path = PathBuf::from("tests/fixtures/bin");
/// let path_bufs = vec![first_path, second_path];
/// let new_path = env::join_paths(path_bufs).unwrap();
/// env::set_var("PATH", &new_path);
///
/// let result = find_command("missing");
/// assert_eq!(result.is_some(), false);
/// ```
pub fn find_command<T>(command: T) -> Option<PathBuf>
    where T: AsRef<Path>
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
                if let Some(result) = find_command_with_pathext(&candidate) {
                    return Some(result);
                } else if candidate.is_file() {
                    return Some(candidate);
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
pub fn find_command_in_pkg<T, U>(command: T,
                                 pkg_install: &PackageInstall,
                                 fs_root_path: U)
                                 -> Result<Option<PathBuf>>
    where T: AsRef<Path>,
          U: AsRef<Path>
{
    for path in pkg_install.paths()? {
        let stripped =
            path.strip_prefix("/")
                .unwrap_or_else(|_| {
                    panic!("Package path missing / prefix {}", path.to_string_lossy())
                });
        let candidate = fs_root_path.as_ref().join(stripped).join(command.as_ref());
        if let Some(result) = find_command_with_pathext(&candidate) {
            return Ok(Some(result));
        } else if candidate.is_file() {
            return Ok(Some(path.join(command.as_ref())));
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
                Ok(None) => {
                    panic!("Could not find '{}' in the '{}' package! This is required for the \
                            proper operation of this program.",
                           program, &ident)
                }
                Err(err) => {
                    panic!("Error finding '{}' in the '{}' package! This is required for the \
                            proper operation of this program. (Err: {:?})",
                           program, &ident, err)
                }
            }
        }
        Err(err) => {
            panic!("Package installation for '{}' not found on disk! This is required for the \
                    proper operation of this program (Err: {:?})",
                   &ident, err)
        }
    };
    debug!("resolved absolute path to program, program={}, ident={}, abs_path={}",
           program,
           &ident,
           abs_path.display());
    abs_path
}

// Windows relies on path extensions to resolve commands like `docker` to `docker.exe`
// Path extensions are found in the PATHEXT environment variable.
// We should only search with PATHEXT if the file does not already have an extension.
fn find_command_with_pathext(candidate: &Path) -> Option<PathBuf> {
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
pub fn am_i_root() -> bool { *EUID == 0u32 }

/// parent returns the parent directory of the given path, accounting
/// for the fact that a relative path with no directory separator
/// returns an empty parent. This function transforms it to return "."
/// so that it can more easily be used in functions that expect to
/// take a directory.
fn parent(p: &Path) -> io::Result<&Path> {
    match p.parent() {
        Some(parent_path) if parent_path.as_os_str().is_empty() => Ok(Path::new(".")),
        Some(nonempty_parent_path) => Ok(nonempty_parent_path),
        None => Err(io::Error::new(io::ErrorKind::InvalidInput, "path has no parent")),
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
pub struct AtomicWriter {
    dest:        PathBuf,
    tempfile:    tempfile::NamedTempFile,
    permissions: Permissions,
}

impl AtomicWriter {
    /// Create a new `AtomicWriter` that writes to a file at
    /// `dest_path` with default permissions.
    pub fn new(dest_path: &Path) -> io::Result<Self> {
        Self::new_with_permissions(dest_path, Permissions::default())
    }

    /// Create a new `AtomicWriter` that writes to a file at
    /// `dest_path` with the specified permissions.
    ///
    /// Note: On Unix platforms, permissions are set explicitly and
    /// not subject to the process umask.
    // NOTE: If we ever add another kind of configurable parameter to
    // `AtomicWriter`, we should take a look at creating a Builder for
    // it.
    pub fn new_with_permissions(dest_path: &Path, permissions: Permissions) -> io::Result<Self> {
        let parent = parent(dest_path)?;
        let tempfile = tempfile::NamedTempFile::new_in(parent)?;
        Ok(Self { dest: dest_path.to_path_buf(),
                  tempfile,
                  permissions })
    }

    pub fn with_writer<F, T, E>(mut self, op: F) -> std::result::Result<T, E>
        where F: FnOnce(&mut std::fs::File) -> std::result::Result<T, E>,
              E: From<std::io::Error>
    {
        let r = op(self.tempfile.as_file_mut())?;
        self.finish()?;
        Ok(r)
    }

    /// Completes the atomic write by calling sync on the temporary
    /// file to ensure all data is flushed to disk and then renaming
    /// the file into place.
    fn finish(self) -> io::Result<()> {
        // Note that we only set permissions if given explicit ones to
        // override whatever permissions the file was created with.
        if let Permissions::Explicit(ref permissions) = self.permissions {
            // This is not my proudest moment, but it does the trick
            // with a minimum amount of fuss :/
            #[cfg(not(windows))]
            let permissions = *permissions;

            set_permissions(self.tempfile.path(), permissions).map_err(|e| {
                                                                  io::Error::other(e.to_string())
                                                              })?;
        }
        self.tempfile.as_file().sync_all()?;

        atomic_rename(self.tempfile.into_temp_path(), self.dest.as_path())?;

        Ok(())
    }

    /// sync_parent syncs the parent directory. This is required on
    /// some filesystems to ensure that rename(), create(), and
    /// unlink() operations have been persisted to disk. sync_parent
    /// ensures the durability of AtomicWriter but is not required for
    /// the atomocity guarantee.
    #[cfg(unix)]
    fn sync_parent(dest: &Path) -> io::Result<()> {
        use log::info;

        let parent = parent(dest)?;
        let f = fs::File::open(parent)?;
        if let Err(e) = f.sync_all() {
            // sync_all() calls libc::fsync() which will return EINVAL
            // if the filesystem does not support calling fsync() on
            // directories. libc's EINVAL is mapped to InvalidInput.
            if e.kind() == std::io::ErrorKind::InvalidInput {
                info!("Ignoring InvalidInput from sync_all on {}",
                      parent.to_string_lossy());
                Ok(())
            } else {
                Err(e)
            }
        } else {
            Ok(())
        }
    }
}

// `fs::rename` calls `MoveFileExW` on Windows, however the underlying implementation only
// utilizes the `MOVEFILE_REPLACE_EXISTING` flag which allows for file overwrite but no
// guarantee on durability. For this, we additionally pass `MOVEFILE_WRITE_THROUGH`.
// This causes the function to not return until the file is actually moved on the disk.
// Setting this value guarantees that a move performed as a copy and delete operation is
// flushed to disk before the function returns. The flush occurs at the end of the copy
// operation.
#[cfg(windows)]
fn rename_windows<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    // Helper function to transform a Path to a `LPCWSTR` Windows string data type. It is
    // essentially a null-terminated string of 16-bit Unicode characters.
    fn windows_u16s(s: &Path) -> Vec<u16> {
        s.as_os_str().encode_wide().chain(iter::once(0)).collect()
    }
    unsafe {
        if MoveFileExW(windows_u16s(from.as_ref()).as_ptr(),
                       windows_u16s(to.as_ref()).as_ptr(),
                       winapi::um::winbase::MOVEFILE_WRITE_THROUGH
                       | winapi::um::winbase::MOVEFILE_REPLACE_EXISTING)
           == 0
        {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

#[cfg(unix)]
fn rename_unix<P: AsRef<Path>, Q: AsRef<Path> + std::convert::AsRef<std::path::Path> + Copy>(
    from: P,
    to: Q)
    -> io::Result<()> {
    fs::rename(from, to)?;
    AtomicWriter::sync_parent(&PathBuf::from(to.as_ref()))?;
    Ok(())
}

/// atomic_rename is a cross platform  helper function for renaming a file atomically with
/// durability guarantees.
pub fn atomic_rename<P: AsRef<Path>,
                     Q: AsRef<Path> + std::convert::AsRef<std::path::Path> + Copy>(
    from: P,
    to: Q)
    -> io::Result<()> {
    debug!("Renaming {} to {}",
           from.as_ref().display(),
           to.as_ref().display());
    #[cfg(windows)]
    return rename_windows(from, to);
    #[cfg(unix)]
    return rename_unix(from, to);
}

/// atomic_write is a helper function for the most common use of
/// AtomicWriter.
pub fn atomic_write(dest_path: &Path, data: impl AsRef<[u8]>) -> io::Result<()> {
    let w = AtomicWriter::new(dest_path)?;
    w.with_writer(|f| f.write_all(data.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod svc_dir {
        use super::*;
        use std::fs::{self,
                      File};
        use tempfile::tempdir;

        #[test]
        fn purge_directory_removes_contents() {
            let root = tempdir().expect("couldn't create tempdir");

            let file_1 = root.path().join("file_1");
            File::create(&file_1).expect("Couldn't create file");

            let file_2 = root.path().join("file_2");
            File::create(&file_2).expect("Couldn't create file");

            let sub_dir = root.path().join("test_dir");
            fs::create_dir(&sub_dir).expect("Couldn't create directory");

            let sub_file_1 = sub_dir.join("sub_file_1");
            File::create(&sub_file_1).expect("Couldn't create file");

            let sub_file_2 = sub_dir.join("sub_file_2");
            File::create(&sub_file_2).expect("Couldn't create file");

            assert!(root.as_ref().exists());
            assert!(file_1.exists());
            assert!(file_2.exists());
            assert!(sub_dir.exists());
            assert!(sub_file_1.exists());
            assert!(sub_file_2.exists());

            SvcDir::purge_directory_content(root.path()).expect("Couldn't purge!");

            assert!(root.as_ref().exists());
            assert!(!file_1.exists());
            assert!(!file_2.exists());
            assert!(!sub_dir.exists());
            assert!(!sub_file_1.exists());
            assert!(!sub_file_2.exists());
        }
    }
}

#[cfg(test)]
mod test_find_command {
    pub use super::find_command;
    use crate::locked_env_var::LockedEnvVar;
    use std::{env,
              path::PathBuf};

    crate::locked_env_var!(PATHEXT, lock_pathext);

    #[allow(dead_code)]
    fn setup_pathext(lock: &LockedEnvVar) {
        let path_bufs = vec![PathBuf::from(".BAT"),
                             PathBuf::from(".COM"),
                             PathBuf::from(".EXE")];
        let new_path = env::join_paths(path_bufs).unwrap();
        lock.set(new_path);
    }

    fn setup_path() {
        let orig_path = env::var_os("PATH").unwrap();
        let mut os_paths: Vec<PathBuf> = env::split_paths(&orig_path).collect();
        let first_path = PathBuf::from("tests/fixtures");
        let second_path = PathBuf::from("tests/fixtures/bin");
        let mut path_bufs = vec![first_path, second_path];
        path_bufs.append(&mut os_paths);
        let new_path = env::join_paths(path_bufs).unwrap();
        env::set_var("PATH", new_path);
    }

    mod without_pathext_set {
        pub use super::find_command;
        use super::setup_path;

        mod argument_without_extension {
            use super::{find_command,
                        setup_path};
            use crate::fs::test_find_command::lock_pathext;

            #[test]
            fn command_exists() {
                setup_path();
                let lock = lock_pathext();
                lock.unset();
                let result = find_command("bin_with_no_extension");
                assert!(result.is_some());
            }

            #[test]
            fn command_does_not_exist() {
                setup_path();
                let lock = lock_pathext();
                lock.unset();
                let result = find_command("missing");
                assert!(result.is_none());
            }

            #[test]
            fn command_exists_with_extension() {
                setup_path();
                let lock = lock_pathext();
                lock.unset();
                let result = find_command("win95_dominator");
                assert!(result.is_none());
            }
        }

        mod argument_with_extension {
            use super::{find_command,
                        setup_path};
            use crate::fs::test_find_command::lock_pathext;
            use std::path::PathBuf;

            #[test]
            fn command_exists() {
                setup_path();
                let lock = lock_pathext();
                lock.unset();
                let result = find_command("bin_with_extension.exe");
                assert!(result.is_some());
            }

            #[test]
            fn command_does_not_exist() {
                setup_path();
                let lock = lock_pathext();
                lock.unset();
                let result = find_command("missing.com");
                assert!(result.is_none());
            }

            #[test]
            fn command_different_extension_does_exist() {
                setup_path();
                let lock = lock_pathext();
                lock.unset();
                let result = find_command("bin_with_extension.com");
                assert!(result.is_none());
            }

            #[test]
            fn first_command_on_path_found() {
                setup_path();
                let lock = lock_pathext();
                lock.unset();
                let target_path = PathBuf::from("tests/fixtures/plan.sh");
                let result = find_command("plan.sh");
                let found_path = result.unwrap();
                assert_eq!(found_path, target_path);
            }
        }
    }

    #[cfg(target_os = "windows")]
    mod with_pathext_set {
        pub use super::find_command;
        use super::{setup_path,
                    setup_pathext};

        mod argument_without_extension {
            use super::{find_command,
                        setup_path,
                        setup_pathext};
            use crate::fs::test_find_command::lock_pathext;

            #[test]
            fn command_exists() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("bin_with_no_extension");
                assert!(result.is_some());
            }

            #[test]
            fn command_does_not_exist() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("missing");
                assert!(result.is_none());
            }

            #[test]
            #[allow(non_snake_case)]
            fn command_exists_with_extension_in_PATHEXT() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("bin_with_extension");
                assert!(result.is_some());
            }

            #[test]
            #[allow(non_snake_case)]
            fn command_exists_with_extension_not_in_PATHEXT() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("win95_dominator");
                assert!(result.is_none());
            }

            #[test]
            #[allow(non_snake_case)]
            fn command_exists_with_extension_in_PATHEXT_and_without_extension() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("binstub");
                assert_eq!(result.unwrap().file_name().unwrap(), "binstub.BAT");
            }
        }

        mod argument_with_extension {
            use super::{find_command,
                        setup_path,
                        setup_pathext};
            use crate::fs::test_find_command::lock_pathext;
            use std::path::PathBuf;

            #[test]
            fn command_exists() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("bin_with_extension.exe");
                assert!(result.is_some());
            }

            #[test]
            fn command_does_not_exist() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("missing.com");
                assert!(result.is_none());
            }

            #[test]
            fn command_different_extension_does_exist() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("bin_with_extension.com");
                assert!(result.is_none());
            }

            #[test]
            fn first_command_on_path_found() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let target_path = PathBuf::from("tests/fixtures/plan.sh");
                let result = find_command("plan.sh");
                let found_path = result.unwrap();
                assert_eq!(found_path, target_path);
            }

            #[test]
            #[allow(non_snake_case)]
            fn command_exists_with_extension_in_PATHEXT_and_without_extension() {
                setup_path();
                let lock = lock_pathext();
                setup_pathext(&lock);
                let result = find_command("binstub.bat");
                assert_eq!(result.unwrap().file_name().unwrap(), "binstub.bat");
            }
        }
    }
}

#[cfg(test)]
mod test_atomic_writer {
    use super::{atomic_write,
                AtomicWriter};
    use std::{fs::File,
              io::{Read,
                   Seek,
                   Write}};

    const EXPECTED_CONTENT: &str = "A very good file format";

    #[test]
    fn atomic_write_writes_file() {
        let temp_file_path = {
            let dest_file = tempfile::NamedTempFile::new().expect("could not create temp file");
            dest_file.path().to_string_lossy().into_owned()
        };
        let dest_file_path = std::path::Path::new(&temp_file_path);
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
        let temp_file_path = {
            let dest_file = tempfile::NamedTempFile::new().expect("could not create temp file");
            dest_file.path().to_string_lossy().into_owned()
        };
        let dest_file_path = std::path::Path::new(&temp_file_path);

        let w = AtomicWriter::new(dest_file_path).expect("could not create AtomicWriter");
        let res = w.with_writer(|writer| {
                       writer.write_all(b"not the right content")?;
                       writer.rewind()?;
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
