// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! Installs a Habitat package from a [depot](../depot).
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg install chef/redis
//! ```
//!
//! Will install `chef/redis` package from a custom depot:
//!
//! ```bash
//! $ hab pkg install chef/redis/3.0.1 redis -u http://depot.co:9633
//! ```
//!
//! This would install the `3.0.1` version of redis.
//!
//! # Internals
//!
//! * Download the artifact
//! * Verify it is un-altered
//! * Unpack it
//!

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use hcore::fs::CACHE_ARTIFACT_PATH;
use hcore::package::{PackageArchive, PackageIdent, PackageInstall};
use depot_core::data_object;
use depot_client;

use error::Result;

pub fn start(url: &str, ident_or_archive: &str) -> Result<()> {
    if Path::new(ident_or_archive).is_file() {
        try!(from_archive(url, &ident_or_archive));
    } else {
        let ident = try!(PackageIdent::from_str(ident_or_archive));
        try!(from_url(url, &ident));
    }
    Ok(())
}

/// Given a package name and a base url, downloads the package
/// to the `CACHE_ARTIFACT_PATH`. Returns the filename in the cache as a String
///
/// # Failures
///
/// * Fails if it cannot create the `CACHE_ARTIFACT_PATH`
/// * Fails if it cannot download the package from the upstream
pub fn from_url<P: AsRef<PackageIdent>>(url: &str, ident: &P) -> Result<data_object::Package> {
    println!("Installing {}", ident.as_ref());
    let pkg_data = try!(depot_client::show_package(url, ident.as_ref()));
    try!(fs::create_dir_all(CACHE_ARTIFACT_PATH));
    for dep in &pkg_data.tdeps {
        try!(install_from_depot(url, &dep, dep.as_ref()));
    }
    try!(install_from_depot(url, &pkg_data.ident, ident.as_ref()));
    Ok(pkg_data)
}

pub fn from_archive<P: AsRef<Path>>(url: &str, path: &P) -> Result<()> {
    println!("Installing from {}", path.as_ref().display());
    let mut archive = PackageArchive::new(PathBuf::from(path.as_ref()));
    let ident = try!(archive.ident());
    try!(fs::create_dir_all(CACHE_ARTIFACT_PATH));
    for dep in try!(archive.tdeps()) {
        try!(install_from_depot(url, &dep, dep.as_ref()));
    }
    try!(install_from_archive(archive, &ident));
    Ok(())
}

fn install_from_depot<P: AsRef<PackageIdent>>(url: &str,
                                              ident: &P,
                                              given_ident: &PackageIdent)
                                              -> Result<()> {
    match PackageInstall::load(ident.as_ref(), None) {
        Ok(_) => {
            if given_ident.fully_qualified() {
                println!("Package {} already installed", ident.as_ref());
            } else {
                println!("Package that satisfies {} ({}) already installed",
                         given_ident,
                         ident.as_ref());
            }
        }
        Err(_) => {
            let mut archive = try!(depot_client::fetch_package(url,
                                                               ident.as_ref(),
                                                               CACHE_ARTIFACT_PATH));
            let ident = try!(archive.ident());
            try!(archive.verify());
            try!(archive.unpack());
            println!("Installed {}", ident);
        }
    }
    Ok(())
}

fn install_from_archive(archive: PackageArchive, ident: &PackageIdent) -> Result<()> {
    match PackageInstall::load(ident.as_ref(), None) {
        Ok(_) => {
            println!("Package {} already installed", ident);
        }
        Err(_) => {
            try!(archive.unpack());
            println!("Installed {}", ident);
        }
    }
    Ok(())
}
