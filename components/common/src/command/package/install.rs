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

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use ansi_term::Colour::{Blue, Green, Yellow};
use depot_client::Client;
use hcore;
use hcore::crypto::{artifact, SigKeyPair};
use hcore::crypto::keys::parse_name_with_rev;
use hcore::package::{Identifiable, PackageArchive, PackageIdent, PackageInstall};

use command::ProgressBar;
use error::{Error, Result};

pub fn start<P1: ?Sized, P2: ?Sized, P3: ?Sized>(url: &str,
                                                 ident_or_archive: &str,
                                                 product: &str,
                                                 version: &str,
                                                 fs_root_path: &P1,
                                                 cache_artifact_path: &P2,
                                                 cache_key_path: &P3)
                                                 -> Result<PackageIdent>
    where P1: AsRef<Path>,
          P2: AsRef<Path>,
          P3: AsRef<Path>
{
    let task = try!(InstallTask::new(url,
                                     product,
                                     version,
                                     fs_root_path.as_ref(),
                                     cache_artifact_path.as_ref(),
                                     cache_key_path.as_ref()));

    if Path::new(ident_or_archive).is_file() {
        task.from_artifact(&Path::new(ident_or_archive))
    } else {
        task.from_ident(try!(PackageIdent::from_str(ident_or_archive)))
    }
}

struct InstallTask<'a> {
    depot_client: Client,
    fs_root_path: &'a Path,
    cache_artifact_path: &'a Path,
    cache_key_path: &'a Path,
}

impl<'a> InstallTask<'a> {
    pub fn new(url: &str,
               product: &str,
               version: &str,
               fs_root_path: &'a Path,
               cache_artifact_path: &'a Path,
               cache_key_path: &'a Path)
               -> Result<Self> {
        Ok(InstallTask {
            depot_client: try!(Client::new(url, product, version, Some(fs_root_path))),
            fs_root_path: fs_root_path,
            cache_artifact_path: cache_artifact_path,
            cache_key_path: cache_key_path,
        })
    }

    pub fn from_ident(&self, ident: PackageIdent) -> Result<PackageIdent> {
        println!("{}",
                 Yellow.bold().paint(format!("» Installing {}", &ident)));
        let mut ident = ident;
        if !ident.fully_qualified() {
            ident = try!(self.fetch_latest_pkg_ident_for(ident));
        }
        if try!(self.is_package_installed(&ident)) {
            println!("{} {}", Green.paint("→ Using"), &ident);
            println!("{}",
                     Blue.paint(format!("★ Install of {} complete with {} new packages \
                                         installed.",
                                        &ident,
                                        0)));
            return Ok(ident);
        }

        self.install_package(ident, None)
    }

    pub fn from_artifact(&self, artifact_path: &Path) -> Result<PackageIdent> {
        let ident = try!(PackageArchive::new(artifact_path).ident());
        if try!(self.is_package_installed(&ident)) {
            println!("{} {}", Green.paint("→ Using"), &ident);
            println!("{}",
                     Blue.paint(format!("★ Install of {} complete with {} new packages \
                                         installed.",
                                        &ident,
                                        0)));
            return Ok(ident);
        }
        try!(self.cache_artifact(&ident, artifact_path));
        let src_path = artifact_path.parent().unwrap();

        self.install_package(ident, Some(src_path))
    }

    fn install_package(&self,
                       ident: PackageIdent,
                       src_path: Option<&Path>)
                       -> Result<PackageIdent> {
        let mut artifact = try!(self.get_cached_artifact(ident.clone(), src_path));
        let mut artifacts: Vec<PackageArchive> = Vec::new();

        for ident in try!(artifact.tdeps()) {
            if try!(self.is_package_installed(&ident)) {
                println!("{} {}", Green.paint("→ Using"), ident);
            } else {
                artifacts.push(try!(self.get_cached_artifact(ident, src_path)));
            }
        }
        artifacts.push(artifact);

        let num_installed = artifacts.len();
        for mut artifact in artifacts {
            try!(self.extract_artifact(&mut artifact));
        }
        println!("{}",
                 Blue.paint(format!("★ Install of {} complete with {} new packages installed.",
                                    &ident,
                                    num_installed)));
        Ok(ident)
    }

    fn get_cached_artifact(&self,
                           ident: PackageIdent,
                           src_path: Option<&Path>)
                           -> Result<PackageArchive> {
        if try!(self.is_artifact_cached(&ident)) {
            debug!("Found {} in artifact cache, skipping remote download",
                   &ident);
        } else {
            try!(self.fetch_artifact(&ident, src_path));
        }

        let mut artifact = PackageArchive::new(try!(self.cached_artifact_path(&ident)));
        try!(self.verify_artifact(&ident, &mut artifact));
        Ok(artifact)
    }

    fn extract_artifact(&self, artifact: &mut PackageArchive) -> Result<()> {
        try!(artifact.unpack(Some(self.fs_root_path)));
        println!("{} {}",
                 Green.bold().paint("✓ Installed"),
                 try!(artifact.ident()));
        Ok(())
    }

    fn is_package_installed(&self, ident: &PackageIdent) -> Result<bool> {
        match PackageInstall::load(ident, Some(self.fs_root_path)) {
            Ok(_) => Ok(true),
            Err(hcore::Error::PackageNotFound(_)) => Ok(false),
            Err(e) => Err(Error::HabitatCore(e)),
        }
    }

    fn is_artifact_cached(&self, ident: &PackageIdent) -> Result<bool> {
        Ok(try!(self.cached_artifact_path(ident)).is_file())
    }

    fn cached_artifact_path(&self, ident: &PackageIdent) -> Result<PathBuf> {
        let name = match ident.archive_name() {
            Some(n) => n,
            None => {
                return Err(Error::HabitatCore(hcore::Error::InvalidPackageIdent(ident.to_string())))
            }
        };
        Ok(self.cache_artifact_path.join(name))
    }

    fn fetch_latest_pkg_ident_for(&self, fuzzy_ident: PackageIdent) -> Result<PackageIdent> {
        Ok(try!(self.depot_client.show_package(fuzzy_ident)).into())
    }

    fn fetch_artifact(&self, ident: &PackageIdent, src_path: Option<&Path>) -> Result<()> {
        if let Some(src_path) = src_path {
            let name = match ident.archive_name() {
                Some(n) => n,
                None => return Err(
                    Error::HabitatCore(hcore::Error::InvalidPackageIdent(ident.to_string()))),
            };
            let local_artifact = src_path.join(name);
            if local_artifact.is_file() {
                try!(self.cache_artifact(ident, &local_artifact));
                return Ok(());
            }
        }

        println!("{} {}", Green.bold().paint("↓ Downloading"), ident);
        let mut progress = ProgressBar::default();
        try!(self.depot_client
            .fetch_package(ident.clone(), self.cache_artifact_path, Some(&mut progress)));
        Ok(())
    }

    fn fetch_origin_key(&self, name_with_rev: &str) -> Result<()> {
        println!("{} {} public origin key",
                 Green.bold().paint("↓ Downloading"),
                 &name_with_rev);
        let (name, rev) = try!(parse_name_with_rev(&name_with_rev));
        let mut progress = ProgressBar::default();
        try!(self.depot_client
            .fetch_origin_key(&name, &rev, self.cache_key_path, Some(&mut progress)));
        println!("{} {} public origin key",
                 Green.bold().paint("☑ Cached"),
                 &name_with_rev);
        Ok(())
    }

    fn cache_artifact(&self, ident: &PackageIdent, artifact_path: &Path) -> Result<()> {
        let name = match ident.archive_name() {
            Some(n) => n,
            None => {
                return Err(Error::HabitatCore(hcore::Error::InvalidPackageIdent(ident.to_string())))
            }
        };
        try!(fs::copy(artifact_path, self.cache_artifact_path.join(name)));
        Ok(())
    }

    fn verify_artifact(&self, ident: &PackageIdent, artifact: &mut PackageArchive) -> Result<()> {
        let artifact_ident = try!(artifact.ident());
        if ident != &artifact_ident {
            return Err(Error::ArtifactIdentMismatch((artifact.file_name(),
                                                     artifact_ident.to_string(),
                                                     ident.to_string())));
        }

        let nwr = try!(artifact::artifact_signer(&artifact.path));
        if let Err(_) = SigKeyPair::get_public_key_path(&nwr, self.cache_key_path) {
            try!(self.fetch_origin_key(&nwr));
        }

        try!(artifact.verify(&self.cache_key_path));
        info!("Verified {} signed by {}", ident, &nwr);
        Ok(())
    }
}
