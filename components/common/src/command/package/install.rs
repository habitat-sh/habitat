// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

//! Installs a Habitat package from a [depot](../depot).
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg install core/redis
//! ```
//!
//! Will install `core/redis` package from a custom depot:
//!
//! ```bash
//! $ hab pkg install core/redis/3.0.1 redis -u http://depot.co:9633
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

use std::path::{Path, PathBuf};
use std::str::FromStr;

use ansi_term::Colour::{Blue, Green, Yellow};
use depot_client::Client;
use hcore::crypto::{artifact, SigKeyPair};
use hcore::crypto::keys::parse_name_with_rev;
use hcore::fs::cache_artifact_path;
use hcore::package::{Identifiable, PackageArchive, PackageIdent, PackageInstall};
use protocol::depotsrv;

use command::ProgressBar;
use error::Result;

pub fn start<P1: ?Sized, P2: ?Sized, P3: ?Sized>(url: &str,
                                                 ident_or_archive: &str,
                                                 fs_root_path: &P1,
                                                 cache_artifact_path: &P2,
                                                 cache_key_path: &P3)
                                                 -> Result<()>
    where P1: AsRef<Path>,
          P2: AsRef<Path>,
          P3: AsRef<Path>
{
    if Path::new(ident_or_archive).is_file() {
        try!(from_archive(url,
                          &ident_or_archive,
                          fs_root_path,
                          cache_artifact_path,
                          cache_key_path));
    } else {
        let ident = try!(PackageIdent::from_str(ident_or_archive));
        try!(from_url(url,
                      &ident,
                      fs_root_path,
                      cache_artifact_path,
                      cache_key_path));
    }
    Ok(())
}

/// Given a package name and a base url, downloads the package
/// to the cache artifact path. Returns the filename in the cache as a String
///
/// # Failures
///
/// * Fails if it cannot download the package from the upstream
pub fn from_url<P1: ?Sized, P2: ?Sized, P3: ?Sized>(url: &str,
                                                    ident: &PackageIdent,
                                                    fs_root_path: &P1,
                                                    cache_artifact_path: &P2,
                                                    cache_key_path: &P3)
                                                    -> Result<depotsrv::Package>
    where P1: AsRef<Path>,
          P2: AsRef<Path>,
          P3: AsRef<Path>
{
    println!("{}",
             Yellow.bold().paint(format!("» Installing {}", ident)));
    let depot_client = try!(Client::new(url, Some(fs_root_path.as_ref())));
    let pkg_data = try!(depot_client.show_package(ident.clone()));
    for dep in pkg_data.get_tdeps().into_iter() {
        let d: PackageIdent = (*dep).clone().into();
        try!(install_from_depot(url,
                                &d,
                                &d,
                                fs_root_path.as_ref(),
                                cache_artifact_path.as_ref(),
                                cache_key_path.as_ref()));
    }
    try!(install_from_depot(url,
                            &pkg_data.get_ident().clone().into(),
                            ident,
                            fs_root_path.as_ref(),
                            cache_artifact_path.as_ref(),
                            cache_key_path.as_ref()));
    println!("{}",
             Blue.paint(format!("★ Install of {} complete with {} packages installed.",
                                ident,
                                1 + &pkg_data.get_tdeps().len())));
    Ok(pkg_data)
}

pub fn from_archive<P1: ?Sized, P2: ?Sized, P3: ?Sized, P4: ?Sized>(url: &str,
                                                                    path: &P1,
                                                                    fs_root_path: &P2,
                                                                    cache_artifact_path: &P3,
                                                                    cache_key_path: &P4)
                                                                    -> Result<()>
    where P1: AsRef<Path>,
          P2: AsRef<Path>,
          P3: AsRef<Path>,
          P4: AsRef<Path>
{
    println!("{}",
             Yellow.bold().paint(format!("» Installing {}", path.as_ref().display())));
    let mut archive = PackageArchive::new(PathBuf::from(path.as_ref()));
    let ident = try!(archive.ident());
    let tdeps = try!(archive.tdeps());
    for dep in &tdeps {
        try!(install_from_depot(url,
                                &dep,
                                dep.as_ref(),
                                fs_root_path.as_ref(),
                                cache_artifact_path.as_ref(),
                                cache_key_path.as_ref()));
    }
    try!(install_from_archive(url,
                              archive,
                              &ident,
                              fs_root_path.as_ref(),
                              cache_key_path.as_ref()));
    println!("{}",
             Blue.paint(format!("★ Install of {} complete with {} packages installed.",
                                &ident,
                                1 + &tdeps.len())));
    Ok(())
}

fn install_from_depot(url: &str,
                      ident: &PackageIdent,
                      given_ident: &PackageIdent,
                      fs_root_path: &Path,
                      cache_artifact_path: &Path,
                      cache_key_path: &Path)
                      -> Result<()> {
    match PackageInstall::load(ident, Some(&fs_root_path)) {
        Ok(_) => {
            if given_ident.fully_qualified() {
                println!("{} {}", Green.paint("→ Using"), ident);
            } else {
                println!("{} {} which satisfies {}",
                         Green.paint("→ Using"),
                         ident.as_ref(),
                         given_ident);
            }
        }
        Err(_) => {
            println!("{} {}",
                     Green.bold().paint("↓ Downloading"),
                     ident.as_ref());
            let mut progress = ProgressBar::default();
            let depot_client = try!(Client::new(url, Some(fs_root_path)));
            let mut archive = try!(depot_client.fetch_package((*ident).clone(),
                                                               cache_artifact_path,
                                                               Some(&mut progress)));
            let ident = try!(archive.ident());
            try!(verify(url, &archive, &ident, fs_root_path, cache_key_path));
            try!(archive.unpack(Some(fs_root_path)));
            println!("{} {}", Green.bold().paint("✓ Installed"), ident.as_ref());
        }
    }
    Ok(())
}

fn install_from_archive(url: &str,
                        archive: PackageArchive,
                        ident: &PackageIdent,
                        fs_root_path: &Path,
                        cache_key_path: &Path)
                        -> Result<()> {
    match PackageInstall::load(ident.as_ref(), Some(&fs_root_path)) {
        Ok(_) => {
            println!("{} {}", Green.paint("→ Using"), ident);
        }
        Err(_) => {
            println!("{} {} from cache",
                     Green.bold().paint("← Extracting"),
                     ident);
            try!(verify(url, &archive, &ident, fs_root_path, cache_key_path));
            try!(archive.unpack(Some(fs_root_path)));
            println!("{} {}", Green.bold().paint("✓ Installed"), ident);
        }
    }
    Ok(())
}

/// get the signer for the artifact and see if we have the key locally.
/// If we don't, attempt to download it from the depot.
fn verify(url: &str,
          archive: &PackageArchive,
          ident: &PackageIdent,
          fs_root_path: &Path,
          cache_key_path: &Path)
          -> Result<()> {
    let nwr = try!(artifact::artifact_signer(&archive.path));
    if let Err(_) = SigKeyPair::get_public_key_path(&nwr, cache_key_path) {
        println!("{} {} public origin key",
                 Green.bold().paint("↓ Downloading"),
                 &nwr);
        let (name, rev) = try!(parse_name_with_rev(&nwr));
        let mut progress = ProgressBar::default();
        let depot_client = try!(Client::new(url, Some(fs_root_path)));
        try!(depot_client.fetch_origin_key(&name, &rev, cache_key_path, Some(&mut progress)));
        println!("{} {} public origin key",
                 Green.bold().paint("☑ Cached"),
                 &nwr);
    }

    try!(archive.verify(&cache_key_path));
    info!("Verified {} signed by {}", &ident, &nwr);
    Ok(())
}
