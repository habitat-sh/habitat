// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::result::Result as StdResult;

use depot_client::{self, Client};
use depot_client::Error::APIError;
use hcore;
use hcore::fs::{am_i_root, cache_key_path};
use hcore::crypto::{artifact, SigKeyPair};
use hcore::crypto::keys::parse_name_with_rev;
use hcore::package::{Identifiable, PackageArchive, PackageIdent, Target, PackageInstall};
use hcore::package::metadata::PackageType;
use hyper::status::StatusCode;

use error::{Error, Result};
use ui::{Status, UI};

use retry::retry;

pub const RETRIES: u64 = 5;
pub const RETRY_WAIT: u64 = 3000;

/// Represents a locally-available `.hart` file for package
/// installation purposes only.
///
/// The struct itself must be public because it is used in
/// `InstallSource` enum. The members are intentionally private,
/// though; by design, the only way an instance of this struct can be
/// created is to call `parse::<InstallSource>` on a file path that
/// refers to a `.hart` file.
///
/// In other words, you are probably more interested in the
/// `InstallSource` enum; this struct is just an implementation
/// detail.
#[derive(Debug)]
pub struct LocalArchive {
    // In an ideal world, we would just implement `InstallSource` in
    // terms of a `PackageArchive` directly, since that can provide
    // both an ident and path.
    //
    // However, asking for the ident of a `PackageArchive` is
    // currently a mutating operation. As a result, that mutability
    // requirement leaked out to all consumers of `InstallSource` in a
    // way that was rather confusing.
    //
    // Instead, we simply bundle up both the path to the archive file
    // along with the `PackageIdent` we extract from it when we create
    // an instance of this struct (these data are the only things we
    // really need to install from a local archive). The members are
    // private to ensure that this module has full control over the
    // creation of instances of the struct, and can thus ensure that
    // the ident and path are mutually consistent and valid.
    ident: PackageIdent,
    path: PathBuf,
}

/// Encapsulate all possible sources we can install packages from.
#[derive(Debug)]
pub enum InstallSource {
    /// We can install from a package identifier
    Ident(PackageIdent),
    /// We can install from a locally-available `.hart` file
    Archive(LocalArchive),
}

impl FromStr for InstallSource {
    type Err = hcore::Error;

    /// Create an `InstallSource` from either a package identifier
    /// string (e.g. "core/hab"), or from the path to a local package.
    ///
    /// Returns an error if the string is neither a valid package
    /// identifier, or is not the path to an actual Habitat package.
    fn from_str(s: &str) -> StdResult<InstallSource, Self::Err> {
        let path = Path::new(s);
        if path.is_file() {
            // Is it really an archive? If it can produce an
            // identifer, we'll say "yes".
            let mut archive = PackageArchive::new(path);
            match archive.ident() {
                Ok(ident) => Ok(InstallSource::Archive(LocalArchive {
                    ident: ident,
                    path: path.to_path_buf(),
                })),
                Err(e) => Err(e),
            }
        } else {
            if let Some(extension) = path.extension() {
                if extension == "hart" {
                    return Err(hcore::Error::FileNotFound(s.to_string()));
                }
            }

            match s.parse::<PackageIdent>() {
                Ok(ident) => Ok(InstallSource::Ident(ident)),
                Err(e) => Err(e),
            }
        }
    }
}

impl From<PackageIdent> for InstallSource {
    /// Convenience function to generate an `InstallSource` from an
    /// existing `PackageIdent`.
    fn from(ident: PackageIdent) -> Self {
        InstallSource::Ident(ident)
    }
}

impl AsRef<PackageIdent> for InstallSource {
    fn as_ref(&self) -> &PackageIdent {
        match *self {
            InstallSource::Ident(ref ident) => ident,
            InstallSource::Archive(ref local_archive) => &local_archive.ident,
        }
    }
}

/// Install a Habitat package.
///
/// If an `InstallSource::Ident` is given, we retrieve the package
/// from the specified Builder `url`. Providing a fully-qualified
/// identifer will result in that exact package being installed
/// (regardless of `channel`). Providing a partially-qualified
/// identifier will result in the installation of latest appropriate
/// release from the given `channel`.
///
/// If an `InstallSource::Archive` is given, then this exact artifact will be
/// installed, instead of retrieving it from Builder.
///
/// In either case, however, any dependencies of will be retrieved
/// from Builder (if they're not already cached locally).
///
/// At the end of this function, the specified package and all its
/// dependencies will be installed on the system.

// TODO (CM): Consider passing in a configured depot client instead of
// product / version... That might make it easier to share with the
// `sup` crate
pub fn start<P1, P2>(
    ui: &mut UI,
    url: &str,
    channel: Option<&str>,
    install_source: &InstallSource,
    product: &str,
    version: &str,
    fs_root_path: P1,
    artifact_cache_path: P2,
    token: Option<&str>,
) -> Result<PackageInstall>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    if env::var_os("HAB_NON_ROOT").is_none() && !am_i_root() {
        ui.warn(
            "Installing a package requires root or administrator privileges. Please retry \
                   this command as a super user or use a privilege-granting facility such as \
                   sudo.",
        )?;
        ui.br()?;
        return Err(Error::RootRequired);
    }


    // TODO (CM): rename fs::cache_key_path so the naming is
    // consistent and flows better.
    let key_cache_path = cache_key_path(Some(fs_root_path.as_ref()));
    debug!("install key_cache_path: {}", key_cache_path.display());

    let task = InstallTask::new(
        url,
        product,
        version,
        fs_root_path.as_ref(),
        artifact_cache_path.as_ref(),
        &key_cache_path,
    )?;

    match *install_source {
        InstallSource::Ident(ref ident) => task.from_ident(ui, ident.clone(), channel, token),
        InstallSource::Archive(ref local_archive) => task.from_archive(ui, local_archive),
    }
}

struct InstallTask<'a> {
    depot_client: Client,
    fs_root_path: &'a Path,
    /// The path to the local artifact cache (e.g., /hab/cache/artifacts)
    artifact_cache_path: &'a Path,
    key_cache_path: &'a Path,
}

impl<'a> InstallTask<'a> {
    fn new(
        url: &str,
        product: &str,
        version: &str,
        fs_root_path: &'a Path,
        artifact_cache_path: &'a Path,
        key_cache_path: &'a Path,
    ) -> Result<Self> {
        Ok(InstallTask {
            depot_client: Client::new(url, product, version, Some(fs_root_path))?,
            fs_root_path: fs_root_path,
            artifact_cache_path: artifact_cache_path,
            key_cache_path: key_cache_path,
        })
    }

    /// Install a package from the Depot, based on a given identifier.
    ///
    /// If the identifier is fully-qualified, that specific package
    /// release will be installed (if it exists on Builder).
    ///
    /// However, if the identifier is _not_ fully-qualified, the
    /// latest version from the given channel will be installed
    /// instead.
    ///
    /// In either case, the identifier returned will be the
    /// fully-qualified identifier of package that was infstalled
    /// (which, as we have seen, may not be the same as the identifier
    /// that was passed in).
    fn from_ident(
        &self,
        ui: &mut UI,
        ident: PackageIdent,
        channel: Option<&str>,
        token: Option<&str>,
    ) -> Result<PackageInstall> {
        if channel.is_some() {
            ui.begin(format!(
                "Installing {} from channel '{}'",
                &ident,
                channel.unwrap()
            ))?;
        } else {
            ui.begin(format!("Installing {}", &ident))?;
        }


        // The "target_ident" will be the fully-qualified identifier
        // of the package we will ultimately install, once we
        // determine if we need to get a more recent version or not.
        let target_ident = if !ident.fully_qualified() {
            match self.fetch_latest_pkg_ident_for(&ident, channel) {
                Ok(latest_ident) => latest_ident,
                Err(Error::DepotClient(APIError(StatusCode::NotFound, _))) => {
                    if let Ok(recommendations) = self.get_channel_recommendations(&ident) {
                        if !recommendations.is_empty() {
                            ui.warn(
                                "The package does not have any versions in the specified channel.",
                            )?;
                            ui.warn(
                                "Did you intend to install one of the folowing instead?",
                            )?;
                            for r in recommendations {
                                ui.warn(format!("  {} in channel {}", r.1, r.0))?;
                            }
                        }
                    }

                    return Err(Error::PackageNotFound);
                }
                Err(e) => {
                    debug!("error fetching ident: {:?}", e);
                    return Err(e);
                }
            }
        } else {
            // This is just outputting some information in case the
            // fully-qualified identifier we were given isn't actually
            // in this channel. It shouldn't matter, though, because we've got
            // a fully-qualified identifier.
            if let Some(channel) = channel {
                let ch = channel.to_string();
                match self.depot_client.package_channels(&ident, token) {
                    Ok(channels) => {
                        if channels.iter().find(|ref c| ***c == ch).is_none() {
                            ui.warn(format!(
                                "Can not find {} in the {} channel but installing anyway since the package ident was fully qualified.", &ident, &ch
                            ))?;
                        }
                    }
                    Err(e) => {
                        debug!("Failed to get channel list: {:?}", e);
                        // TODO (CM): Do we really want to return an
                        // error here? Because we couldn't output a
                        // warning message?
                        return Err(Error::ChannelNotFound);
                    }
                };
            }

            ident
        };

        match self.installed_package(&target_ident) {
            Some(package_install) => {
                // The installed package was found on disk
                ui.status(Status::Using, &target_ident)?;
                ui.end(format!(
                    "Install of {} complete with {} new packages installed.",
                    &target_ident,
                    0
                ))?;
                Ok(package_install)
            }
            None => {
                // No installed package was found
                self.install_package(ui, &target_ident, token)
            }
        }
    }

    /// Get a list of suggested package identifiers from all
    /// channels. This is used to generate actionable user feedback
    /// when the desired package was not found in the specified
    /// channel.
    fn get_channel_recommendations(&self, ident: &PackageIdent) -> Result<Vec<(String, String)>> {
        let mut res = Vec::new();

        let channels = match self.depot_client.list_channels(ident.origin()) {
            Ok(channels) => channels,
            Err(e) => {
                debug!("Failed to get channel list: {:?}", e);
                return Err(Error::PackageNotFound);
            }
        };

        for channel in channels {
            match self.fetch_latest_pkg_ident_for(ident, Some(&channel)) {
                Ok(pkg) => res.push((channel, format!("{}", pkg))),
                Err(_) => (),
            };
        }

        Ok(res)
    }

    /// Given an archive on disk, ensure that it is properly installed
    /// and return the package's identifier.
    fn from_archive(&self, ui: &mut UI, local_archive: &LocalArchive) -> Result<PackageInstall> {
        let ref ident = local_archive.ident;
        match self.installed_package(ident) {
            Some(package_install) => {
                ui.status(Status::Using, ident)?;
                ui.end(format!(
                    "Install of {} complete with {} new packages installed.",
                    ident,
                    0
                ))?;
                Ok(package_install)
            }
            None => {
                self.store_artifact_in_cache(ident, &local_archive.path)?;
                self.install_package(ui, ident, None)
            }
        }
    }

    /// Given the identifier of an artifact, ensure that the artifact,
    /// as well as all its dependencies, have been cached and
    /// installed. Handles both standalone and composite package
    /// types.
    ///
    /// If the package is already present in the cache, it is not
    /// re-downloaded. Any dependencies of the package that are not
    /// installed will be re-cached (as needed) and installed.
    fn install_package(
        &self,
        ui: &mut UI,
        ident: &PackageIdent,
        token: Option<&str>,
    ) -> Result<PackageInstall> {
        // TODO (CM): rename artifact to archive
        let mut artifact = self.get_cached_artifact(ui, ident, token)?;

        match artifact.package_type()? {
            PackageType::Standalone => {
                // Ensure that all transitive dependencies, as well as the
                // original package itself, are cached locally.
                let dependencies = artifact.tdeps()?;
                let mut artifacts_to_install = Vec::with_capacity(dependencies.len() + 1);
                for dependency in dependencies.iter() {
                    if self.installed_package(dependency).is_some() {
                        ui.status(Status::Using, dependency)?;
                    } else {
                        artifacts_to_install.push(self.get_cached_artifact(ui, dependency, token)?);
                    }
                }
                // The package we're actually trying to install goes last; we
                // want to ensure that its dependencies get installed before
                // it does.
                artifacts_to_install.push(artifact);

                // Ensure all uninstalled artifacts get installed
                for artifact in artifacts_to_install.iter_mut() {
                    self.unpack_artifact(ui, artifact)?;
                }

                ui.end(format!(
                    "Install of {} complete with {} new packages installed.",
                    ident,
                    artifacts_to_install.len()
                ))?;
            }
            PackageType::Composite => {
                let services = artifact.resolved_services()?;
                for service in services {
                    // We don't track the transitive dependencies of
                    // all services at the composite level, because
                    // each service itself does that. Thus, we need to
                    // install them just like we would if we weren't
                    // in a composite.
                    //
                    // We don't really need a channel down here, as
                    // all these identifiers are fully-qualified.
                    self.from_ident(ui, service, None, token)?;
                }
                // All the services have been unpacked; let's do the
                // same with the composite package itself.
                self.unpack_artifact(ui, &mut artifact)?;
            }
        }

        // Return the thing we just installed
        PackageInstall::load(ident, Some(self.fs_root_path)).map_err(Error::from)
    }

    /// This ensures the identified package is in the local cache,
    /// verifies it, and returns a handle to the package's metadata.
    fn get_cached_artifact(
        &self,
        ui: &mut UI,
        ident: &PackageIdent,
        token: Option<&str>,
    ) -> Result<PackageArchive> {
        if self.is_artifact_cached(&ident)? {
            debug!(
                "Found {} in artifact cache, skipping remote download",
                ident
            );
        } else {
            if retry(
                RETRIES,
                RETRY_WAIT,
                || self.fetch_artifact(ui, ident, token),
                |res| res.is_ok(),
            ).is_err()
            {
                return Err(Error::from(depot_client::Error::DownloadFailed(format!(
                    "We tried {} times but could not download {}. Giving up.",
                    RETRIES,
                    ident
                ))));
            }
        }

        let mut artifact = PackageArchive::new(self.cached_artifact_path(ident)?);
        ui.status(Status::Verifying, artifact.ident()?)?;
        self.verify_artifact(ui, ident, &mut artifact)?;
        Ok(artifact)
    }

    /// Adapter function wrapping `PackageArchive::unpack`
    fn unpack_artifact(&self, ui: &mut UI, artifact: &mut PackageArchive) -> Result<()> {
        artifact.unpack(Some(self.fs_root_path))?;
        ui.status(Status::Installed, artifact.ident()?)?;
        Ok(())
    }

    /// Adapter function to retrieve an installed package given an
    /// identifier, if it exists.
    fn installed_package(&self, ident: &PackageIdent) -> Option<PackageInstall> {
        PackageInstall::load(ident, Some(self.fs_root_path)).ok()
    }

    // TODO (CM): This could return a plain bool IF we could ensure
    // above that the package identifier is FULLY QUALIFIED
    fn is_artifact_cached(&self, ident: &PackageIdent) -> Result<bool> {
        Ok(self.cached_artifact_path(ident)?.is_file())
    }

    /// Returns the path to the location this package would exist at in
    /// the local package cache. It does not mean that the package is
    /// actually *in* the package cache, though.
    fn cached_artifact_path(&self, ident: &PackageIdent) -> Result<PathBuf> {
        let name = fully_qualified_archive_name(ident)?;
        Ok(self.artifact_cache_path.join(name))
    }

    fn fetch_latest_pkg_ident_for(
        &self,
        ident: &PackageIdent,
        channel: Option<&str>,
    ) -> Result<PackageIdent> {
        Ok(self.depot_client.show_package(ident, channel)?.into())
    }

    /// Retrieve the identified package from the depot, ensuring that
    /// the artifact is cached locally.
    fn fetch_artifact(&self, ui: &mut UI, ident: &PackageIdent, token: Option<&str>) -> Result<()> {
        ui.status(Status::Downloading, ident)?;
        match self.depot_client.fetch_package(
            ident,
            token,
            self.artifact_cache_path,
            ui.progress(),
        ) {
            Ok(_) => Ok(()),
            Err(depot_client::Error::APIError(StatusCode::NotImplemented, _)) => {
                println!(
                    "Host platform or architecture not supported by the targted depot; \
                          skipping."
                );
                Ok(())
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    fn fetch_origin_key(&self, ui: &mut UI, name_with_rev: &str) -> Result<()> {
        ui.status(
            Status::Downloading,
            format!("{} public origin key", &name_with_rev),
        )?;
        let (name, rev) = parse_name_with_rev(&name_with_rev)?;
        self.depot_client.fetch_origin_key(
            &name,
            &rev,
            self.key_cache_path,
            ui.progress(),
        )?;
        ui.status(
            Status::Cached,
            format!("{} public origin key", &name_with_rev),
        )?;
        Ok(())
    }

    /// Copies the artifact to the local artifact cache directory
    // TODO (CM): Oh, we could just pass in the LocalArchive
    fn store_artifact_in_cache(&self, ident: &PackageIdent, artifact_path: &Path) -> Result<()> {
        let cache_path = self.cached_artifact_path(ident)?;
        fs::create_dir_all(self.artifact_cache_path)?;

        // Handle the pathological case where you're trying to install
        // an artifact file directly from the cache. Otherwise, you'd
        // end up with a 0-byte file in the cache and wouldn't be able
        // to subsequently verify it.
        if artifact_path != cache_path {
            fs::copy(artifact_path, cache_path)?;
        }
        Ok(())
    }

    fn verify_artifact(
        &self,
        ui: &mut UI,
        ident: &PackageIdent,
        artifact: &mut PackageArchive,
    ) -> Result<()> {
        let artifact_ident = artifact.ident()?;
        if ident != &artifact_ident {
            return Err(Error::ArtifactIdentMismatch((
                artifact.file_name(),
                artifact_ident.to_string(),
                ident.to_string(),
            )));
        }

        let artifact_target = artifact.target()?;
        artifact_target.validate()?;

        let nwr = artifact::artifact_signer(&artifact.path)?;
        if let Err(_) = SigKeyPair::get_public_key_path(&nwr, self.key_cache_path) {
            self.fetch_origin_key(ui, &nwr)?;
        }

        artifact.verify(&self.key_cache_path)?;
        debug!("Verified {} signed by {}", ident, &nwr);
        Ok(())
    }
}

/// Adapter function wrapping `PackageIdent::archive_name` that
/// returns an error if the identifier is not fully-qualified
/// (only fully-qualified identifiers can yield an archive name).
fn fully_qualified_archive_name(ident: &PackageIdent) -> Result<String> {
    ident.archive_name().ok_or(Error::HabitatCore(
        hcore::Error::FullyQualifiedPackageIdentRequired(
            ident.to_string(),
        ),
    ))
}
