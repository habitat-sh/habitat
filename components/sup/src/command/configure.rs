// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Prints the configuration options for a service. Actually the `config` command.
//!
//! # Examples
//!
//! ```bash
//! $ bldr config redis
//! ```
//!
//! Will show the `default.toml`.

use std::io::prelude::*;
use std::fs::File;

use error::BldrResult;
use config::Config;
use package::Package;

/// Print the default.toml for a given package.
///
/// # Failures
///
/// * If the package cannot be found
/// * If the default.toml does not exist, or cannot be read
/// * If we can't read the file into a string
pub fn display(config: &Config) -> BldrResult<()> {
    let package = try!(Package::load(config.package(), None));
    let mut file = try!(File::open(package.join_path("default.toml")));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    println!("{}", s);
    Ok(())
}
