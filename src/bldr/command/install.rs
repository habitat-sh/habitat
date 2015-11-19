//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Installs a bldr package from a [repo](../repo).
//!
//! # Examples
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633
//! ```
//!
//! Will install the `redis` package from the repository at `http://bldr.co:9633`.
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633 -d adam
//! ```
//!
//! Will do the same, but choose the `adam` derivation, rather than the default `bldr`.
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633 -v 3.0.1
//! ```
//!
//! Will install the `3.0.1` version of redis.
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633 -v 3.0.1 -r 20150911204047
//! ```
//!
//! Will install the `20150911204047` release of the `3.0.1` version of `redis.
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633 -d adam -v 3.0.1 -r 20150911204047
//! ```
//!
//! The same as the last, but from the `adam` derivation as well.
//!
//! # Internals
//!
//! * Download the artifact
//! * Verify it is un-altered
//! * Unpack it
//!

use std::process::Command;
use std::fs;

use super::super::{PACKAGE_CACHE, GPG_CACHE};
use error::{BldrResult, BldrError};
use util::gpg;
use repo;

/// Given a package name and a base url, downloads the package
/// to `/opt/bldr/cache/pkgs`. Returns the filename in the cache as a String
///
/// # Failures
///
/// * Fails if it cannot create `/opt/bldr/cache/pkgs`
/// * Fails if it cannot download the package from the upstream
pub fn latest_from_url(deriv: &str, package: &str, url: &str) -> BldrResult<String> {
    try!(fs::create_dir_all(PACKAGE_CACHE));
    repo::client::fetch_package_latest(url, deriv, package, PACKAGE_CACHE)
}

/// Given a package name and a path to a file as an `&str`, verify
/// the files gpg signature.
///
/// # Failures
///
/// * Fails if it cannot verify the GPG signature for any reason
pub fn verify(package: &str, file: &str) -> BldrResult<()> {
    try!(gpg::verify(package, file));
    Ok(())
}

/// Given a package name and a path to a file as an `&str`, unpack
/// the package.
///
/// # Failures
///
/// * If the package cannot be unpacked via gpg
pub fn unpack(package: &str, file: &str) -> BldrResult<()> {
    let output = try!(Command::new("sh")
                          .arg("-c")
                          .arg(format!("gpg --homedir {} --decrypt {} | tar -C / -x",
                                       GPG_CACHE,
                                       file))
                          .output());
    match output.status.success() {
        true => println!("   {}: Installed", package),
        false => {
            println!("   {}: Failed to install", package);
            return Err(BldrError::UnpackFailed);
        }
    }
    Ok(())
}
