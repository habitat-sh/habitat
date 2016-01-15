// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Installs a gpg key from a [repo](../repo) or a local file.
//!
//! # Examples
//!
//! ```bash
//! $ bldr key chef-public -u http://localhost:9633
//! ```
//!
//! Will download the `chef-public` gpg key from the specified repo.
//!
//! ```bash
//! $ bldr key /tmp/chef-public.asc
//! ```
//!
//! Will install the key found in `/tmp/chef-public.asc`.

use std::fs;

use fs::KEY_CACHE;
use util::gpg;
use repo;
use config::Config;
use error::BldrResult;

/// Install a GPG key. If `config.url()` is empty, we assume the value
/// of `config.key()` is a path to the key. Otherwise, we download the
/// key from the repo at `config.url()`, drop it in `/opt/bldr/cache/keys`,
/// and then import it into GPG.
///
/// # Failures
///
/// * If the directory `/opt/bldr/cache/keys` cannot be created
/// * If the we fail to download the key from the repo
/// * If the GPG import process fails
pub fn install(config: &Config) -> BldrResult<()> {
    match *config.url() {
        Some(ref url) => {
            if url.is_empty() {
                try!(gpg::import(&config.key()));
            }
            try!(fs::create_dir_all(KEY_CACHE));
            let filename = try!(repo::client::fetch_key(url, &config.key(), KEY_CACHE));
            try!(gpg::import(&filename));
        }
        None => try!(gpg::import(&config.key())),
    }
    Ok(())
}
