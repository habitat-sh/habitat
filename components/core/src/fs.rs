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

use std::env;
use std::path::{Path, PathBuf};

use users;

use env as henv;

/// The default filesystem root path
#[cfg(not(target_os="windows"))]
pub const FS_ROOT_PATH: &'static str = "/";
#[cfg(target_os="windows")]
pub const FS_ROOT_PATH: &'static str = concat!(env!("SYSTEMDRIVE"), "/");
/// The default root path of the Habitat filesytem
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
/// The root path containing all runtime service directories and files
const SVC_PATH: &'static str = "hab/svc";

lazy_static! {
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
pub fn cache_analytics_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_CACHE_ANALYTICS_PATH),
        None => Path::new(FS_ROOT_PATH).join(&*MY_CACHE_ANALYTICS_PATH),
    }
}

/// Returns the path to the artifacts cache, optionally taking a custom filesystem root.
pub fn cache_artifact_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_CACHE_ARTIFACT_PATH),
        None => Path::new(FS_ROOT_PATH).join(&*MY_CACHE_ARTIFACT_PATH),
    }
}

/// Returns the path to the keys cache, optionally taking a custom filesystem root.
pub fn cache_key_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_CACHE_KEY_PATH),
        None => Path::new(FS_ROOT_PATH).join(&*MY_CACHE_KEY_PATH),
    }
}

/// Returns the path to the src cache, optionally taking a custom filesystem root.
pub fn cache_src_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_CACHE_SRC_PATH),
        None => Path::new(FS_ROOT_PATH).join(&*MY_CACHE_SRC_PATH),
    }
}

/// Returns the path to the SSL cache, optionally taking a custom filesystem root.
pub fn cache_ssl_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_CACHE_SSL_PATH),
        None => Path::new(FS_ROOT_PATH).join(&*MY_CACHE_SSL_PATH),
    }
}

/// Returns the root path containing all runtime service directories and files
pub fn svc_root() -> PathBuf {
    Path::new("/").join(SVC_PATH).to_path_buf()
}

/// Returns the root path for a given service's configuration, files, and data.
pub fn svc_path(service_name: &str) -> PathBuf {
    Path::new("/").join(SVC_PATH).join(service_name)
}

/// Returns the path to a given service's configuration.
pub fn svc_config_path(service_name: &str) -> PathBuf {
    svc_path(service_name).join("config")
}

/// Returns the path to a given service's data.
pub fn svc_data_path(service_name: &str) -> PathBuf {
    svc_path(service_name).join("data")
}

/// Returns the path to a given service's gossiped config files.
pub fn svc_files_path(service_name: &str) -> PathBuf {
    svc_path(service_name).join("files")
}

/// Returns the path to a given service's hooks.
///
/// Note that this path is internal to the Supervisor and should not be directly accessed under
/// normal circumstances.
pub fn svc_hooks_path(service_name: &str) -> PathBuf {
    svc_path(service_name).join("hooks")
}

/// Returns the path to a given service's static content.
pub fn svc_static_path(service_name: &str) -> PathBuf {
    svc_path(service_name).join("static")
}

/// Returns the path to a given service's variable state.
pub fn svc_var_path(service_name: &str) -> PathBuf {
    svc_path(service_name).join("var")
}

/// Returns the absolute path for a given command, if it exists, by searching the `PATH`
/// environment variable.
///
/// If the command represents an absolute path, then the `PATH` seaching will not be performed. If
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
pub fn find_command(command: &str) -> Option<PathBuf> {
    // If the command path is absolute and a file exists, then use that.
    let candidate = PathBuf::from(command);
    if candidate.is_absolute() && candidate.is_file() {
        return Some(candidate);
    }

    // Find the command by checking each entry in `PATH`. If we still can't find it, give up and
    // return `None`.
    match henv::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let candidate = PathBuf::from(&path).join(command);
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

    #[cfg(target_os="windows")]
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
