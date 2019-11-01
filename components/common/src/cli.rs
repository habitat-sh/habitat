//! This file contains the string defaults values as well as environment variable strings
//! for use in the clap layer of the application. This is not the final form for defaults.
//! Eventually this will be composed of fully typed default values. But as a first step we
//! need a spot to consolidate those values and help simplify some of the logic around them.

use clap::ArgMatches;

use habitat_core::{env as henv,
                   fs::{cache_key_path,
                        CACHE_KEY_PATH},
                   os::process::{ShutdownSignal,
                                 ShutdownTimeout},
                   package::PackageIdent};
use std::{ffi::OsStr,
          path::{Path,
                 PathBuf},
          str::FromStr};

pub const RING_ENVVAR: &str = "HAB_RING";
pub const RING_KEY_ENVVAR: &str = "HAB_RING_KEY";

pub const LISTEN_HTTP_DEFAULT_PORT: u16 = 9631;
pub const LISTEN_HTTP_DEFAULT_IP: &str = "0.0.0.0";
lazy_static! {
    pub static ref LISTEN_HTTP_DEFAULT_ADDR: String =
        { format!("{}:{}", LISTEN_HTTP_DEFAULT_IP, LISTEN_HTTP_DEFAULT_PORT) };
}

pub const PACKAGE_TARGET_ENVVAR: &str = "HAB_PACKAGE_TARGET";
lazy_static! {
    pub static ref SHUTDOWN_TIMEOUT_DEFAULT: String = ShutdownTimeout::default().to_string();
}

// We allow this on Windows as well as Unix, even though we don't use
// ShutdownSignal on Windows, because we want to allow Windows CLI
// users to interact with Unix Supervisors.
lazy_static! {
    pub static ref SHUTDOWN_SIGNAL_DEFAULT: String = ShutdownSignal::default().to_string();
}

const SYSTEMDRIVE_ENVVAR: &str = "SYSTEMDRIVE";

lazy_static! {
    /// The default filesystem root path to base all commands from. This is lazily generated on
    /// first call and reflects on the presence and value of the environment variable keyed as
    /// `FS_ROOT_ENVVAR`.
    pub static ref FS_ROOT: PathBuf = {
        use crate::hcore::fs::FS_ROOT_ENVVAR;

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
}

pub const BINLINK_DIR_ENVVAR: &str = "HAB_BINLINK_DIR";

/// Default Binlink Dir
#[cfg(target_os = "windows")]
pub const DEFAULT_BINLINK_DIR: &str = "/hab/bin";
#[cfg(target_os = "linux")]
pub const DEFAULT_BINLINK_DIR: &str = "/bin";
#[cfg(target_os = "macos")]
pub const DEFAULT_BINLINK_DIR: &str = "/usr/local/bin";

/// We require the value at the clap layer (see cli::arg_cache_key_path),
/// so we can safely unwrap, but we need some additional logic to calculate
// the dynamic "default" value if the argument has the default signifier value:
/// CACHE_KEY_PATH. An empty value can't stand for default since it is invalid.
pub fn cache_key_path_from_matches(matches: &ArgMatches<'_>) -> PathBuf {
    match matches.value_of("CACHE_KEY_PATH")
                 .expect("CACHE_KEY_PATH required")
    {
        CACHE_KEY_PATH => cache_key_path(Some(&*FS_ROOT)),
        val => PathBuf::from(val),
    }
}

pub fn is_toml_file(val: &str) -> bool {
    let extension = Path::new(&val).extension().and_then(OsStr::to_str);
    match extension {
        // We could do some more validation (parse the whole toml file and check it) but that seems
        // excessive.
        Some("toml") => true,
        _ => false,
    }
}

pub fn file_into_idents(path: &str) -> Result<Vec<PackageIdent>, habitat_core::error::Error> {
    let s = std::fs::read_to_string(&path).map_err(|_| {
                habitat_core::error::Error::FileNotFound(format!("Could not open file {}", path))
            })?;

    s.lines().filter_map(line_to_ident).collect()
}

fn line_to_ident(line: &str) -> Option<Result<PackageIdent, habitat_core::error::Error>> {
    let trimmed = line.split('#').nth(0).unwrap_or("").trim();
    match trimmed.len() {
        0 => None,
        _ => Some(PackageIdent::from_str(trimmed)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // These tests aren't super pretty because we can't do PartialEq for the Option<Error<X>> stuff,
    // so lots of unwrap
    fn test_line_to_ident() {
        assert!(line_to_ident("").is_none());
        assert!(line_to_ident("# foo").is_none());
        assert!(line_to_ident("   # foo").is_none());
        assert!(line_to_ident("\n\r# foo").is_none());

        assert_eq!(line_to_ident("core/gzip").unwrap().unwrap(),
                   PackageIdent::from_str("core/gzip").unwrap());
        assert_eq!(line_to_ident("core/gzip #foo").unwrap().unwrap(),
                   PackageIdent::from_str("core/gzip").unwrap());

        assert!(line_to_ident("core").unwrap().is_err());

        assert!(line_to_ident("core # not").unwrap().is_err());
    }
}
