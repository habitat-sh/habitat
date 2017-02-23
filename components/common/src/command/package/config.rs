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

//! Prints the default configuration options for a service.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg config core/redis
//! ```
//!
//! Will show the `default.toml`.

use std::io::{self, Write};
use std::path::Path;

use hcore::package::{PackageIdent, PackageInstall};
use hcore::package::install::DEFAULT_CFG_FILE;
use toml;

use error::Result;

pub fn start<P>(ident: &PackageIdent, fs_root_path: P) -> Result<()>
    where P: AsRef<Path>
{
    let package = try!(PackageInstall::load(ident, Some(fs_root_path.as_ref())));
    match package.default_cfg() {
        Some(cfg) => println!("{}", try!(toml::ser::to_string(&cfg))),
        None => {
            writeln!(&mut io::stderr(),
                     "No '{}' found for {}",
                     DEFAULT_CFG_FILE,
                     &package.ident)
                .expect("Failed printing to stderr")
        }
    }
    Ok(())
}
