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

//! Uploads a gpg key to a [repo](../repo).
//!
//! # Examples
//!
//! ```bash
//! $ bldr key-upload chef-public -u http://localhost:9633
//! ```
//!
//! Will upload the `chef-public` key from the local key cache to the repo url.
//!
//! ```bash
//! $ bldr key-upload /tmp/chef-public -u http://localhost:9633
//! ```
//!
//! Will upload the key at `/tmp/chef-public.asc` to the repo url.
//!

use error::{BldrResult, BldrError};
use config::Config;
use std::fs::File;
use std::path::Path;

use util::http;

/// Upload a key to a repository.
///
/// If the key starts with a `/`, we treat it as a path to a specific file; otherwise, it's a key
/// to grab from the cache in `/opt/bldr/cache/keys`. Either way, we read the file and upload it to
/// the repository.
///
/// # Failures
///
/// * If the file fails to exist, or if we can't read it
/// * If the http upload fails
pub fn key(config: &Config) -> BldrResult<()> {
    if let Some('/') = config.key().chars().nth(0) {
        println!("   {}: uploading {}.asc", config.key(), config.key());
        let mut file = try!(File::open(&format!("{}.asc", config.key())));
        let path = Path::new(config.key());
        let file_name = try!(path.file_name().ok_or(BldrError::NoFilePart));
        try!(http::upload(&format!("{}/keys/{}", config.url(), file_name.to_string_lossy()), &mut file));
    } else {
        println!("   {}: uploading {}.asc", config.key(), config.key());
        let mut file = try!(File::open(&format!("/opt/bldr/cache/keys/{}.asc", config.key())));
        try!(http::upload(&format!("{}/keys/{}", config.url(), config.key()), &mut file));
    }
    println!("   {}: complete", config.key());
    Ok(())
}

