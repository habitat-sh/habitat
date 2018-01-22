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

use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use users;

use package::{Identifiable, PackageInstall, PackageIdent};
use env as henv;
use error::Result;

/// The default root path of the Habitat filesystem
pub const ROOT_PATH: &'static str = "hab";
/// The default path for any analytics related files
pub const CACHE_ANALYTICS_PATH: &'static str = "hab/cache/analytics";
/// The default download root path for package artifacts, used on package installation
pub const CACHE_ARTIFACT_PATH: &'static str = "hab/cache/artifacts";
/// The default path where cryptographic keys are stored
pub const CACHE_KEY_PATH: &'static str = "hab/cache/keys";
/// The default path where source artifacts are downloaded, extracted, & compiled
pub const CACHE_SRC_PATH: &'static str = "hab/cache/src";
/// The default path where SSL-related artifacts are placed
pub const CACHE_SSL_PATH: &'static str = "hab/cache/ssl";
/// The root path containing all locally installed packages
pub const PKG_PATH: &'static str = "hab/pkgs";
/// The environment variable pointing to the filesystem root. This exists for internal
/// Habitat team usage and is not intended to be used by Habitat consumers.
/// Using this variable could lead to broken Supervisor services and it should
/// be used with extreme caution.
pub const FS_ROOT_ENVVAR: &'static str = "FS_ROOT";
pub const SYSTEMDRIVE_ENVVAR: &'static str = "SYSTEMDRIVE";
/// The file where user-defined configuration for each service is found.
pub const USER_CONFIG_FILE: &'static str = "user.toml";

lazy_static! {
    /// The default filesystem root path.
    ///
    /// WARNING: On Windows this variable mutates on first call if an environment variable with
    ///          the key of `FS_ROOT_ENVVAR` is set.
    pub static ref FS_ROOT_PATH: PathBuf = fs_root_path();

    static ref EUID: u32 = users::get_effective_uid();

    static ref MY_CACHE_ANALYTICS_PATH: PathBuf = {
        if *EUID == 0u32 {
            PathBuf::from(CACHE_ANALYTICS_PATH)
        } else {
            match env::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_ANALYTICS_PATH)),
                None => PathBuf::from(CACHE_ANALYTICS_PATH),
            }
        }
    };

    static ref MY_CACHE_ARTIFACT_PATH: PathBuf = {
        if *EUID == 0u32 {
            PathBuf::from(CACHE_ARTIFACT_PATH)
        } else {
            match env::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_ARTIFACT_PATH)),
                None => PathBuf::from(CACHE_ARTIFACT_PATH),
            }
        }
    };

    static ref MY_CACHE_KEY_PATH: PathBuf = {
        if *EUID == 0u32 {
            PathBuf::from(CACHE_KEY_PATH)
        } else {
            match env::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_KEY_PATH)),
                None => PathBuf::from(CACHE_KEY_PATH),
            }
        }
    };

    static ref MY_CACHE_SRC_PATH: PathBuf = {
        if *EUID == 0u32 {
            PathBuf::from(CACHE_SRC_PATH)
        } else {
            match env::home_dir() {
                Some(home) => home.join(format!(".{}", CACHE_SRC_PATH)),
                None => PathBuf::from(CACHE_SRC_PATH),
            }
        }
    };

    static ref MY_CACHE_SSL_PATH: PathBuf = {
        if *EUID == 0u32 {
            PathBuf::from(CACHE_SSL_PATH)
        } else {
            match env::home_dir() {
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
                } else {
                    match find_command_with_pathext(&candidate) {
                        Some(result) => return Some(result),
                        None => {}
                    }
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
        let stripped = path.strip_prefix("/").expect(&format!(
            "Package path missing / prefix {}",
            path.to_string_lossy()
        ));
        let candidate = fs_root_path.as_ref().join(stripped).join(command.as_ref());
        if candidate.is_file() {
            return Ok(Some(path.join(command.as_ref())));
        } else {
            match find_command_with_pathext(&candidate) {
                Some(result) => return Ok(Some(result)),
                None => {}
            }
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
                    panic!(format!(
                        "Could not find '{}' in the '{}' package! This is required for the \
                        proper operation of this program.",
                        program,
                        &ident
                    ))
                }
                Err(err) => {
                    panic!(format!(
                        "Error finding '{}' in the '{}' package! This is required for the \
                        proper operation of this program. (Err: {:?})",
                        program,
                        &ident,
                        err
                    ))
                }
            }
        }
        Err(err) => {
            panic!(format!(
                "Package installation for '{}' not found on disk! This is required for the \
                proper operation of this program (Err: {:?})",
                &ident,
                err
            ))
        }
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
        match henv::var_os("PATHEXT") {
            Some(pathexts) => {
                for pathext in env::split_paths(&pathexts) {
                    let mut source_candidate = candidate.to_path_buf();
                    let extension = pathext.to_str().unwrap().trim_matches('.');
                    source_candidate.set_extension(extension);
                    let current_candidate = source_candidate.to_path_buf();
                    if current_candidate.is_file() {
                        return Some(current_candidate);
                    }
                }
            }
            None => {}
        };
    }
    None
}

/// Returns whether or not the current process is running with a root effective user id or not.
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
        ).expect("Could not write to stderr");
        return PathBuf::from(path);
    }

    // JW TODO: When Windows container studios are available the platform reflection should
    // be removed.
    if cfg!(target_os = "windows") {
        match (henv::var(FS_ROOT_ENVVAR), henv::var(SYSTEMDRIVE_ENVVAR)) {
            (Ok(path), _) => PathBuf::from(path),
            (Err(_), Ok(system_drive)) => PathBuf::from(format!("{}{}", system_drive, "/")),
            (Err(_), Err(_)) => {
                unreachable!(
                    "Windows should always have a SYSTEMDRIVE \
                    environment variable."
                )
            }
        }
    } else {
        PathBuf::from("/")
    }
}

#[cfg(test)]
mod test_find_command {

    use std::env;
    use std::fs;
    use std::path::PathBuf;
    pub use super::find_command;

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
        let first_path = fs::canonicalize("./tests/fixtures").unwrap();
        let second_path = fs::canonicalize("./tests/fixtures/bin").unwrap();
        let path_bufs = vec![first_path, second_path];
        let new_path = env::join_paths(path_bufs).unwrap();
        env::set_var("PATH", &new_path);
    }

    mod without_pathext_set {
        use super::{setup_path, setup_empty_pathext};
        pub use super::find_command;

        fn setup_environment() {
            setup_path();
            setup_empty_pathext();
        }

        mod argument_without_extension {
            use super::{setup_environment, find_command};

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
            use std::fs::canonicalize;
            use super::{setup_environment, find_command};

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
        use super::{setup_path, setup_pathext};
        pub use super::find_command;

        fn setup_environment() {
            setup_path();
            setup_pathext();
        }

        mod argument_without_extension {
            use super::{setup_environment, find_command};

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
            use std::fs::canonicalize;
            use super::{setup_environment, find_command};

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
