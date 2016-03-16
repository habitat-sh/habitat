// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Installs a bldr package from a [depot](../depot).
//!
//! # Examples
//!
//! ```bash
//! $ bldr install chef/redis
//! ```
//!
//! Will install `chef/redis` package from the package depot at `http://bldr.co:9633`.
//!
//! ```bash
//! $ bldr install chef/redis/3.0.1 redis -u http://bldr.co:9633
//! ```
//!
//! Will install the `3.0.1` version of redis.
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
use package::{Package, PackageIdent};
use depot::{self, data_object};

static LOGKEY: &'static str = "CI";

/// Given a package name and a base url, downloads the package
/// to `/opt/bldr/cache/pkgs`. Returns the filename in the cache as a String
///
/// # Failures
///
/// * Fails if it cannot create `/opt/bldr/cache/pkgs`
/// * Fails if it cannot download the package from the upstream
pub fn from_url<P: AsRef<PackageIdent>>(url: &str, ident: &P) -> BldrResult<data_object::Package> {
    let package = try!(depot::client::show_package(url, ident.as_ref()));
    try!(fs::create_dir_all(PACKAGE_CACHE));
    for dep in &package.tdeps {
        try!(install(url, &dep));
    }
    try!(install(url, &package.ident));
    Ok(package)
}

fn install<P: AsRef<PackageIdent>>(url: &str, package: &P) -> BldrResult<()> {
    match Package::load(package.as_ref(), None) {
        Ok(_) => {
            outputln!("Package that satisfies {} already installed",
                      package.as_ref());
        }
        Err(_) => {
            let mut archive = try!(depot::client::fetch_package(url,
                                                                package.as_ref(),
                                                                PACKAGE_CACHE));
            let package = try!(archive.ident());
            try!(archive.unpack());
            outputln!("Installed {}", package);
        }
    }
    Ok(())
}
