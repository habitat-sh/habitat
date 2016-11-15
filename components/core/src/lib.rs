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

extern crate errno;
#[cfg(test)]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate libarchive;
#[macro_use]
extern crate log;
extern crate regex;
extern crate rustc_serialize;
extern crate sodiumoxide;
extern crate libsodium_sys;
#[cfg(test)]
extern crate tempdir;
extern crate time;
extern crate toml;
extern crate url as extern_url;

#[cfg(not(windows))]
extern crate users as linux_users;

#[cfg(windows)]
extern crate habitat_win_users;

pub use self::error::{Error, Result};

pub mod config;
pub mod env;
pub mod error;
pub mod fs;
pub mod package;
pub mod service;
pub mod url;
pub mod util;
pub mod crypto;
pub mod os;

pub use os::filesystem;
pub use os::users;
