use crate::{command::package::install::{self,
                                        InstallHookMode,
                                        InstallMode,
                                        LocalPackageUsage},
            error::{Error,
                    Result},
            ui,
            PROGRAM_NAME};
use habitat_core::{self,
                   fs::{cache_artifact_path,
                        fs_rooted_path,
                        FS_ROOT_PATH},
                   package::{PackageIdent,
                             PackageInstall,
                             PackageTarget},
                   url::default_bldr_url,
                   ChannelIdent};
use std::{env,
          fs::File,
          io::{self,
               prelude::*,
               BufReader},
          path::PathBuf,
          str::FromStr};

// The package identifier for the OS specific interpreter which the Supervisor is built with,
// or which may be independently installed
#[cfg(any(target_os = "linux", target_os = "macos"))]
habitat_core::env_config_string!(InterpreterIdent,
                                 HAB_INTERPRETER_IDENT,
                                 "core/busybox-static");

#[cfg(target_os = "windows")]
habitat_core::env_config_string!(InterpreterIdent, HAB_INTERPRETER_IDENT, "core/powershell");

const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// Returns a list of path entries, one of which should contain the interpreter binary.
///
/// The Supervisor provides a minimal userland of commands to the supervised process. This includes
/// binaries such as `chpst` which can be used by a package's `run` hook.
///
/// There is a series of fallback strategies used here in order to find a usable interpreter
/// installation. The general strategy is the following:
///
/// * Are we (the Supervisor) running inside a package?
///     * Yes: use the interpreter release describes in our `DEPS` metafile & return its `PATH`
///       entries
///     * No
///         * Can we find any installed interpreter package?
///             * Yes: use the latest installed interpreter release & return its `PATH` entries
///             * No: install the interpreter package
///
/// # Errors
///
/// * If an installed package should exist, but cannot be loaded
/// * If a installed package's path metadata cannot be read or returned
/// * If a known-working package identifier string cannot be parsed
/// * If the Supervisor is not executing inside a package, and if no interpreter package is
///   installed
async fn interpreter_paths() -> Result<Vec<PathBuf>> {
    // First, we'll check if we're running inside a package. If we are, then we should  be able to
    // access the `../DEPS` metadata file and read it to get the specific version of the
    // interpreter.
    let my_interpreter_dep_ident = match env::current_exe() {
        Ok(p) => {
            match p.parent() {
                Some(p) => {
                    let metafile = p.join("../DEPS");
                    if metafile.is_file() {
                        interpreter_dep_from_metafile(metafile)
                    } else {
                        None
                    }
                }
                None => None,
            }
        }
        Err(_) => None,
    };
    let interpreter_paths: Vec<PathBuf> = match my_interpreter_dep_ident {
        // We've found the specific release that our Supervisor was built with. Get its path
        // metadata.
        Some(ident) => {
            let pkg_install = PackageInstall::load(&ident, Some(FS_ROOT_PATH.as_ref()))?;
            pkg_install.paths()?
        }
        // If we're not running out of a package, then see if any package of the interpreter is
        // installed.
        None => {
            let ident = PackageIdent::from_str(&InterpreterIdent::configured_value().0)?;
            match PackageInstall::load(&ident, Some(FS_ROOT_PATH.as_ref())) {
                // We found a version of the interpreter. Get its path metadata.
                Ok(pkg_install) => pkg_install.paths()?,
                // Nope, no packages of the interpreter installed. Now we're going to see if the
                // interpreter command is present on `PATH`.
                Err(_) => {
                    match install::type_erased_start(&mut ui::NullUi::new(),
                                                     &default_bldr_url(),
                                                     &ChannelIdent::lts(),
                                                     &(ident.clone(),
                                                       PackageTarget::active_target())
                                                                                      .into(),
                                                     &PROGRAM_NAME,
                                                     VERSION,
                                                     FS_ROOT_PATH.as_path(),
                                                     &cache_artifact_path(None::<String>),
                                                     None,
                                                     &InstallMode::default(),
                                                     &LocalPackageUsage::default(),
                                                     InstallHookMode::default()).await
                    {
                        Ok(pkg_install) => pkg_install.paths()?,
                        Err(err) => {
                            return Err(Error::PackageFailedToInstall(ident, Box::new(err)))
                        }
                    }
                }
            }
        }
    };
    Ok(interpreter_paths)
}

fn root_paths(paths: &mut [PathBuf]) {
    for path in &mut paths.iter_mut() {
        *path = fs_rooted_path(path, FS_ROOT_PATH.as_ref());
    }
}

/// Append the the interpreter path and environment PATH variable to the provided path entries
pub async fn append_interpreter_and_env_path(path_entries: &mut Vec<PathBuf>) -> Result<String> {
    let mut paths = interpreter_paths().await?;
    root_paths(&mut paths);
    path_entries.append(&mut paths);
    append_env_path(path_entries)
}

/// Append the the environment PATH variable to the provided path entries
pub fn append_env_path(path_entries: &mut Vec<PathBuf>) -> Result<String> {
    if let Some(val) = env::var_os("PATH") {
        let mut os_paths = env::split_paths(&val).collect();
        path_entries.append(&mut os_paths);
    }
    let joined = env::join_paths(path_entries)?;
    let path_str =
        joined.into_string()
              .map_err(|s| io::Error::new(io::ErrorKind::InvalidData, s.to_string_lossy()))?;
    Ok(path_str)
}

/// Returns a `PackageIdent` for a interpreter package, assuming it exists in the provided metafile.
fn interpreter_dep_from_metafile(metafile: PathBuf) -> Option<PackageIdent> {
    let f = match File::open(metafile) {
        Ok(f) => f,
        Err(_) => return None,
    };
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return None,
        };
        if line.contains(&InterpreterIdent::configured_value().0) {
            match PackageIdent::from_str(&line) {
                Ok(pi) => return Some(pi),
                Err(_) => return None,
            }
        }
    }
    None
}
