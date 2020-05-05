//! This file contains the string defaults values as well as environment variable strings
//! for use in the clap layer of the application. This is not the final form for defaults.
//! Eventually this will be composed of fully typed default values. But as a first step we
//! need a spot to consolidate those values and help simplify some of the logic around them.

use clap::{value_t,
           ArgMatches};

use habitat_core::{self,
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

pub const BINLINK_DIR_ENVVAR: &str = "HAB_BINLINK_DIR";

/// Default Binlink Dir
#[cfg(target_os = "windows")]
pub const DEFAULT_BINLINK_DIR: &str = "/hab/bin";
#[cfg(target_os = "linux")]
pub const DEFAULT_BINLINK_DIR: &str = "/bin";
#[cfg(target_os = "macos")]
pub const DEFAULT_BINLINK_DIR: &str = "/usr/local/bin";

pub fn cache_key_path_from_matches(matches: &ArgMatches<'_>) -> PathBuf {
    clap::value_t!(matches, "CACHE_KEY_PATH", PathBuf).expect("CACHE_KEY_PATH required")
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
    let trimmed = line.split('#').next().unwrap_or("").trim();
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
