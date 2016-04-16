// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
use config::Config;
use package::Package;

/// Print the default.toml for a given package.
///
/// # Failures
///
/// * If the package cannot be found
/// * If the default.toml does not exist, or cannot be read
/// * If we can't read the file into a string
pub fn display(config: &Config) -> Result<()> {
    let package = try!(Package::load(config.package(), None));
    let mut file = try!(File::open(package.path().join("default.toml")));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    println!("{}", s);
    Ok(())
}
