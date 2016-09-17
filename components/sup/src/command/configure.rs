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

//! Prints the configuration options for a service. Actually the `config` command.
//!
//! # Examples
//!
//! ```bash
//! $ hab-sup config redis
//! ```
//!
//! Will show the `default.toml`.

use std::io::prelude::*;
use std::fs::File;

use error::Result;
use config::gconfig;
use package::Package;

/// Print the default.toml for a given package.
///
/// # Failures
///
/// * If the package cannot be found
/// * If the default.toml does not exist, or cannot be read
/// * If we can't read the file into a string
pub fn display() -> Result<()> {
    let package = try!(Package::load(gconfig().package(), None));
    let mut file = try!(File::open(package.path().join("default.toml")));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    println!("{}", s);
    Ok(())
}
