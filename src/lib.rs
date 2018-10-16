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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate ansi_term;
extern crate base64;
#[cfg(windows)]
extern crate ctrlc;
extern crate dirs;
extern crate errno;
extern crate hex;
#[cfg(test)]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate libarchive;
extern crate libc;
extern crate libsodium_sys;
#[macro_use]
extern crate log;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate tempdir;
#[macro_use]
extern crate serde_derive;

// This is a little gross, but we only need the macros in tests right
// now.
#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(not(test))]
extern crate serde_json;

extern crate sodiumoxide;
extern crate time;
extern crate toml;
extern crate typemap;
extern crate url as extern_url;

#[cfg(not(windows))]
extern crate users as linux_users;

#[cfg(windows)]
extern crate habitat_win_users;
#[cfg(windows)]
extern crate widestring;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate windows_acl;

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

use std::path::PathBuf;

pub use os::filesystem;
pub use os::users;

pub const AUTH_TOKEN_ENVVAR: &'static str = "HAB_AUTH_TOKEN";

lazy_static! {
    pub static ref PROGRAM_NAME: String = {
        let arg0 = std::env::args().next().map(|p| PathBuf::from(p));
        arg0.as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|p| p.to_str())
            .unwrap()
            .to_string()
    };
}
