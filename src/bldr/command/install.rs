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

use std::fs;

use fs::PACKAGE_CACHE;
use error::BldrResult;
use pkg::Package;
use repo;

static LOGKEY: &'static str = "CI";

/// Given a package name and a base url, downloads the package
/// to `/opt/bldr/cache/pkgs`. Returns the filename in the cache as a String
///
/// # Failures
///
/// * Fails if it cannot create `/opt/bldr/cache/pkgs`
/// * Fails if it cannot download the package from the upstream
pub fn from_url(repo: &str,
                deriv: &str,
                name: &str,
                version: Option<String>,
                release: Option<String>)
                -> BldrResult<Package> {
    let package = try!(repo::client::show_package(repo, deriv, name, version, release));
    try!(fs::create_dir_all(PACKAGE_CACHE));
    let mut installed: Vec<Package> = vec![];
    if let Some(ref pkgs) = package.deps {
        for pkg in pkgs {
            try!(install(repo, &pkg, &mut installed));
        }
    }
    try!(install(repo, &package, &mut installed));
    Ok(package)
}

fn install(repo: &str, package: &Package, acc: &mut Vec<Package>) -> BldrResult<()> {
    if acc.contains(&package) {
        return Ok(());
    }
    let archive = try!(repo::client::fetch_package_exact(repo, package, PACKAGE_CACHE));
    try!(archive.verify());
    let package = try!(archive.unpack());
    outputln!("Installed {}", package);
    let deps = package.deps.clone();
    acc.push(package);
    if let Some(ref pkgs) = deps {
        for pkg in pkgs {
            try!(install(repo, &pkg, acc))
        }
    }
    Ok(())
}
