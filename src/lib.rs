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

// Convenience importing of `debug!`/`info!` macros for entire crate.
#[macro_use]
extern crate log;

pub use self::error::{Error, Result};

pub mod binlink;
pub mod channel;
pub mod config;
pub mod crypto;
pub mod env;
pub mod error;
pub mod event;
pub mod fs;
pub mod os;
pub mod output;
pub mod package;
pub mod service;
pub mod url;
pub mod util;

use std::env::VarError;
use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;

pub use crate::os::filesystem;
pub use crate::os::users;

pub const AUTH_TOKEN_ENVVAR: &str = "HAB_AUTH_TOKEN";

lazy_static::lazy_static! {
    pub static ref PROGRAM_NAME: String = {
        let arg0 = std::env::args().next().map(PathBuf::from);
        arg0.as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|p| p.to_str())
            .unwrap()
            .to_string()
    };
}

/// A Builder channel
#[derive(Clone, Debug, PartialEq)]
pub struct ChannelIdent(String);

impl ChannelIdent {
    const LEGACY_ENVVAR: &'static str = "HAB_DEPOT_CHANNEL";
    pub const BLDR_ENVVAR: &'static str = "HAB_BLDR_CHANNEL";
    const UNSTABLE: &'static str = "unstable";
    const STABLE: &'static str = "stable";

    pub fn from_env_var(key: impl AsRef<OsStr>) -> std::result::Result<Self, VarError> {
        crate::env::var(key).map(ChannelIdent)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn stable() -> Self {
        Self::from(Self::STABLE)
    }

    pub fn unstable() -> Self {
        Self::from(Self::UNSTABLE)
    }

    fn legacy_default() -> Self {
        ChannelIdent(
            env::var(Self::LEGACY_ENVVAR)
                .ok()
                .and_then(|c| Some(c.to_string()))
                .unwrap_or_else(|| Self::STABLE.to_string()),
        )
    }
}

impl From<&str> for ChannelIdent {
    fn from(s: &str) -> Self {
        ChannelIdent(s.to_string())
    }
}

impl From<String> for ChannelIdent {
    fn from(s: String) -> Self {
        ChannelIdent(s)
    }
}

impl fmt::Display for ChannelIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ChannelIdent {
    fn default() -> Self {
        match env::var(ChannelIdent::BLDR_ENVVAR) {
            Ok(value) => ChannelIdent(value.to_string()),
            Err(_) => ChannelIdent::legacy_default(),
        }
    }
}
