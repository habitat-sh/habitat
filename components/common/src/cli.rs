// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! This file contains the string defaults values as well as environment variable strings
//! for use in the clap layer of the application. This is not the final form for defaults.
//! Eventually this will be composed of fully typed default values. But as a first step we
//! need a spot to consolidate those values and help simplify some of the logic around them.

use crate::types::ListenCtlAddr;
use clap::ArgMatches;

#[cfg(unix)]
use habitat_core::os::process::ShutdownSignal;
use habitat_core::{env as henv,
                   fs::{cache_key_path,
                        CACHE_KEY_PATH},
                   os::process::ShutdownTimeout};
use std::path::PathBuf;

pub const GOSSIP_DEFAULT_IP: &str = "0.0.0.0";
pub const GOSSIP_DEFAULT_PORT: u16 = 9638;
lazy_static! {
    pub static ref GOSSIP_DEFAULT_ADDR: String =
        { format!("{}:{}", GOSSIP_DEFAULT_IP, GOSSIP_DEFAULT_PORT) };
}
pub const GOSSIP_LISTEN_ADDRESS_ENVVAR: &str = "HAB_LISTEN_GOSSIP";
pub const RING_ENVVAR: &str = "HAB_RING";
pub const RING_KEY_ENVVAR: &str = "HAB_RING_KEY";

pub const LISTEN_HTTP_DEFAULT_PORT: u16 = 9631;
pub const LISTEN_HTTP_DEFAULT_IP: &str = "0.0.0.0";
lazy_static! {
    pub static ref LISTEN_HTTP_DEFAULT_ADDR: String =
        { format!("{}:{}", LISTEN_HTTP_DEFAULT_IP, LISTEN_HTTP_DEFAULT_PORT) };
}
pub const LISTEN_HTTP_ADDRESS_ENVVAR: &str = "HAB_LISTEN_HTTP";

pub const PACKAGE_TARGET_ENVVAR: &str = "HAB_PACKAGE_TARGET";
lazy_static! {
    pub static ref SHUTDOWN_TIMEOUT_DEFAULT: String = ShutdownTimeout::default().to_string();
}
#[cfg(unix)]
lazy_static! {
    pub static ref SHUTDOWN_SIGNAL_DEFAULT: String = ShutdownSignal::default().to_string();
}

const SYSTEMDRIVE_ENVVAR: &str = "SYSTEMDRIVE";

lazy_static! {
    pub static ref LISTEN_CTL_DEFAULT_ADDR_STRING: String =
        { ListenCtlAddr::default().to_string() };

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
