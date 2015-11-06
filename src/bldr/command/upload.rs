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

//! Uploads a package to a [repository](../repo).
//!
//! # Examples
//!
//! ```bash
//! $ bldr upload redis -u http://localhost:9633
//! ```
//!
//! Will upload a package to the repository.
//!
//! # Notes
//!
//! This should be extended to cover uploading specific packages, and finding them by ways more
//! complex than just latest version.
//!
use error::BldrResult;
use config::Config;
use std::fs::File;

use pkg::Package;
use util::http;

/// Upload a package to a repository.
///
/// Find the latest package, then read it from the cache, and upload to the repository.
///
/// # Failures
///
/// * Fails if it cannot find a package
/// * Fails if the package doesn't have a `.bldr` file in the cache
/// * Fails if it cannot upload the file
pub fn package(config: &Config) -> BldrResult<()> {
    let package = try!(Package::latest(config.package(), None));
    println!("   {}: uploading {}", config.package(), package.cache_file().to_string_lossy());
    let mut file = try!(File::open(package.cache_file()));
    try!(http::upload(&format!("{}/pkgs/{}/{}/{}/{}", config.url(), package.derivation, package.name, package.version, package.release), &mut file));
    println!("   {}: complete", config.package());
    Ok(())
}
