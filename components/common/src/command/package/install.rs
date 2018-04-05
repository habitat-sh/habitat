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

use std::borrow::Cow;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::result::Result as StdResult;

use depot_client::{self, Client};
use depot_client::Error::APIError;
use glob;
use hcore;
use hcore::fs::cache_key_path;
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

#[derive(Debug, Eq, PartialEq)]
pub enum InstallMode {
    Online,
    Offline,
}

impl Default for InstallMode {
    fn default() -> Self {
        InstallMode::Online
    }
}

/// Represents a release channel on a Builder Depot.
// TODO fn: this type could be further developed and generalized outside this module
struct Channel<'a>(&'a str);

impl<'a> Channel<'a> {
    fn new(name: &'a str) -> Self {
        Channel(name)
    }
}

impl<'a> Default for Channel<'a> {
    fn default() -> Self {
        Channel("stable")
    }
}

impl<'a> fmt::Display for Channel<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a fully-qualified Package Identifier, meaning that the normally optional version and
/// release package coordinates are guaranteed to be set. This fully-qualified-ness is checked on
/// construction and as the underlying representation is immutable, this state does not change.
#[derive(Debug)]
struct FullyQualifiedPackageIdent<'a> {
    // The ident is a struct field rather than a "newtype" struct to ensure its value cannot be
    // directly accessed
    ident: Cow<'a, PackageIdent>,
}

impl<'a> FullyQualifiedPackageIdent<'a> {
    // TODO fn: I would much rather have implemented `TryFrom` for this, but we need to wait until
    // the API has stabilzed and is released in Rust stable. Here's hoping!
    // Ref: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
    fn from<I>(ident: I) -> Result<Self>
    where
        I: Into<Cow<'a, PackageIdent>>,
    {
        let ident = ident.into();
        if ident.as_ref().fully_qualified() {
            Ok(FullyQualifiedPackageIdent { ident })
        } else {
            Err(Error::HabitatCore(
                hcore::Error::FullyQualifiedPackageIdentRequired(
                    ident.to_owned().to_string(),
                ),
            ))
        }
    }

    fn archive_name(&self) -> String {
        self.ident.as_ref().archive_name().expect(&format!(
            "PackageIdent {} should be fully qualified",
            self.ident.as_ref()
        ))
    }
}

impl<'a> AsRef<PackageIdent> for FullyQualifiedPackageIdent<'a> {
    fn as_ref(&self) -> &PackageIdent {
        self.ident.as_ref()
    }
}

impl<'a> fmt::Display for FullyQualifiedPackageIdent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ident.as_ref().fmt(f)
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
    install_mode: &InstallMode,
) -> Result<PackageInstall>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    // TODO (CM): rename fs::cache_key_path so the naming is
    // consistent and flows better.
    let key_cache_path = cache_key_path(Some(fs_root_path.as_ref()));
    debug!("install key_cache_path: {}", key_cache_path.display());

    let channel = match channel {
        Some(name) => Channel::new(name),
        None => Channel::default(),
    };

    let task = InstallTask::new(
        install_mode,
        url,
        channel,
        product,
        version,
        fs_root_path.as_ref(),
        artifact_cache_path.as_ref(),
        &key_cache_path,
    )?;

    match *install_source {
        InstallSource::Ident(ref ident) => task.from_ident(ui, ident.clone(), token),
        InstallSource::Archive(ref local_archive) => task.from_archive(ui, local_archive, token),
    }
}

struct InstallTask<'a> {
    install_mode: &'a InstallMode,
    depot_client: Client,
    channel: Channel<'a>,
    fs_root_path: &'a Path,
    /// The path to the local artifact cache (e.g., /hab/cache/artifacts)
    artifact_cache_path: &'a Path,
    key_cache_path: &'a Path,
}

impl<'a> InstallTask<'a> {
    fn new(
        install_mode: &'a InstallMode,
        url: &str,
        channel: Channel<'a>,
        product: &str,
        version: &str,
        fs_root_path: &'a Path,
        artifact_cache_path: &'a Path,
        key_cache_path: &'a Path,
    ) -> Result<Self> {
        Ok(InstallTask {
            install_mode: install_mode,
            depot_client: Client::new(url, product, version, Some(fs_root_path))?,
            channel: channel,
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
    /// instead, assuming a newer version is not found locally.
    ///
    /// In either case, the identifier returned will be the
    /// fully-qualified identifier of package that was infstalled
    /// (which, as we have seen, may not be the same as the identifier
    /// that was passed in).
    fn from_ident(
        &self,
        ui: &mut UI,
        ident: PackageIdent,
        token: Option<&str>,
    ) -> Result<PackageInstall> {
        ui.begin(format!("Installing {}", &ident))?;
        let target_ident = self.determine_latest_from_ident(ui, ident, token)?;

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

    /// Given an archive on disk, ensure that it is properly installed
    /// and return the package's identifier.
    fn from_archive(
        &self,
        ui: &mut UI,
        local_archive: &LocalArchive,
        token: Option<&str>,
    ) -> Result<PackageInstall> {
        ui.begin(
            format!("Installing {}", local_archive.path.display()),
        )?;
        let target_ident = FullyQualifiedPackageIdent::from(&local_archive.ident)?;

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
                self.store_artifact_in_cache(
                    &target_ident,
                    &local_archive.path,
                )?;
                self.install_package(ui, &target_ident, token)
            }
        }
    }

    fn determine_latest_from_ident(
        &self,
        ui: &mut UI,
        ident: PackageIdent,
        token: Option<&str>,
    ) -> Result<FullyQualifiedPackageIdent> {
        if ident.fully_qualified() {
            // If we have a fully qualified package identifier, then our work is done--there can
            // only be *one* package that satisfies a fully qualified identifier.

            FullyQualifiedPackageIdent::from(ident)
        } else if self.is_offline() {
            // If we can't contact a Builder API, then we'll find the latest installed package or
            // cached artifact that satisfies the fuzzy package identifier.

            ui.status(
                Status::Determining,
                format!(
                    "latest version of {} locally installed or cached (offline)",
                    &ident
                ),
            )?;
            match self.latest_installed_or_cached(&ident) {
                Ok(i) => Ok(i),
                Err(Error::PackageNotFound) => {
                    return Err(Error::OfflinePackageNotFound(ident.clone()));
                }
                Err(e) => return Err(e),
            }
        } else {
            // Otherwise, we're online and we have a fuzzy package identifier. Now we can find the
            // latest identifier from any installed packages and from a Builder API.

            // Find latest *installed* package, if any are found. We're using the fact that a
            // package is installed as a signal that it can satisfy a "latest" answer. Checking for
            // any cached artifacts is too aggressive in this case: if you really want that cached
            // version to win--install it first, then it will be counted.
            let latest_local = self.latest_installed_ident(&ident);

            ui.status(
                Status::Determining,
                format!(
                    "latest version of {} in the '{}' channel",
                    &ident,
                    self.channel
                ),
            )?;
            let latest_remote = match self.fetch_latest_pkg_ident_for(&ident, token) {
                Ok(latest_ident) => latest_ident,
                Err(Error::DepotClient(APIError(StatusCode::NotFound, _))) => {
                    match latest_local {
                        Ok(ref local) => {
                            ui.status(
                                Status::Missing,
                                format!(
                                    "remote version of {} in the '{}' channel, but a \
                                    newer installed version was found locally ({})",
                                    &ident,
                                    self.channel,
                                    local.as_ref()
                                ),
                            )?;
                            FullyQualifiedPackageIdent::from(local.as_ref().clone())?
                        }
                        Err(_) => {
                            self.recommend_channels(ui, &ident, token)?;
                            return Err(Error::PackageNotFound);
                        }
                    }
                }
                Err(e) => {
                    debug!("error fetching ident: {:?}", e);
                    return Err(e);
                }
            };

            // Return the latest identifier reported by the Builder API *unless* there is a newer
            // version found installed locally.
            match latest_local {
                Ok(local) => {
                    if local.as_ref() > latest_remote.as_ref() {
                        ui.status(
                            Status::Found,
                            format!(
                                "newer installed version ({}) than remote version ({})",
                                &local,
                                latest_remote.as_ref()
                            ),
                        )?;
                        Ok(local)
                    } else {
                        Ok(latest_remote)
                    }
                }
                Err(_) => Ok(latest_remote),
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
        ident: &FullyQualifiedPackageIdent,
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
                // TODO fn: I'd prefer this list to be a `Vec<FullyQualifiedPackageIdent>` but that
                // requires a conversion that could fail (i.e. returns a `Result<...>`). Should be
                // possible though.
                for dependency in dependencies.iter() {
                    if self.installed_package(&FullyQualifiedPackageIdent::from(dependency)?)
                        .is_some()
                    {
                        ui.status(Status::Using, dependency)?;
                    } else {
                        artifacts_to_install.push(self.get_cached_artifact(
                            ui,
                            &FullyQualifiedPackageIdent::from(dependency)?,
                            token,
                        )?);
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
                    self.from_ident(ui, service, token)?;
                }
                // All the services have been unpacked; let's do the
                // same with the composite package itself.
                self.unpack_artifact(ui, &mut artifact)?;
            }
        }

        // Return the thing we just installed
        PackageInstall::load(ident.as_ref(), Some(self.fs_root_path)).map_err(Error::from)
    }

    /// This ensures the identified package is in the local cache,
    /// verifies it, and returns a handle to the package's metadata.
    fn get_cached_artifact(
        &self,
        ui: &mut UI,
        ident: &FullyQualifiedPackageIdent,
        token: Option<&str>,
    ) -> Result<PackageArchive> {
        if self.is_artifact_cached(ident) {
            debug!(
                "Found {} in artifact cache, skipping remote download",
                ident
            );
        } else if self.is_offline() {
            return Err(Error::OfflineArtifactNotFound(ident.as_ref().clone()));
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

        let mut artifact = PackageArchive::new(self.cached_artifact_path(ident));
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
    fn installed_package(&self, ident: &FullyQualifiedPackageIdent) -> Option<PackageInstall> {
        PackageInstall::load(ident.as_ref(), Some(self.fs_root_path)).ok()
    }

    /// Checks for the latest installed package or cached artifact that matches a given package
    /// identifier and returns a fully qualified package identifier if a match exists.
    fn latest_installed_or_cached(
        &self,
        ident: &PackageIdent,
    ) -> Result<FullyQualifiedPackageIdent> {
        let latest_installed = self.latest_installed_ident(&ident);
        let latest_cached = self.latest_cached_ident(&ident);
        debug!(
                "latest installed: {:?}, latest_cached: {:?}",
                &latest_installed,
                &latest_cached,
            );
        let latest = match (latest_installed, latest_cached) {
            (Ok(pkg_install), Err(_)) => pkg_install,
            (Err(_), Ok(pkg_artifact)) => pkg_artifact,
            (Ok(pkg_install), Ok(pkg_artifact)) => {
                if pkg_install.as_ref() > pkg_artifact.as_ref() {
                    pkg_install
                } else {
                    pkg_artifact
                }
            }
            (Err(_), Err(_)) => return Err(Error::PackageNotFound),
        };
        debug!("offline mode: winner: {:?}", &latest);

        Ok(latest)
    }

    fn latest_installed_ident(&self, ident: &PackageIdent) -> Result<FullyQualifiedPackageIdent> {
        match PackageInstall::load(ident, Some(self.fs_root_path)) {
            Ok(pi) => FullyQualifiedPackageIdent::from(pi.ident().clone()),
            Err(_) => Err(Error::PackageNotFound),
        }
    }

    fn latest_cached_ident(&self, ident: &PackageIdent) -> Result<FullyQualifiedPackageIdent> {
        let filename_glob = {
            let mut ident = ident.clone();
            if ident.version.is_none() {
                ident.version = Some(String::from("?*"));
            }
            if ident.release.is_none() {
                // NOTE fn: setting the field value of `release` to a string that isn't a set sized
                // string of numeric characters might lead to issues later. Feels mildly like
                // danger territory, but works today!
                ident.release = Some(String::from("?*"));
            }
            match ident.archive_name() {
                Some(name) => name,
                None => return Err(Error::PackageNotFound),
            }
        };
        let glob_path = self.artifact_cache_path.join(filename_glob);
        let glob_path = glob_path.to_string_lossy();
        debug!("looking for cached artifacts, glob={}", glob_path.as_ref());

        let mut latest: Vec<(PackageIdent, PackageArchive)> = Vec::with_capacity(1);
        for file in glob::glob(glob_path.as_ref())
            .expect("glob pattern should compile")
            .filter_map(StdResult::ok)
        {
            let mut artifact = PackageArchive::new(&file);
            let artifact_ident = artifact.ident().ok();
            if let None = artifact_ident {
                continue;
            }
            let artifact_ident = artifact_ident.unwrap();
            if artifact_ident.origin == ident.origin && artifact_ident.name == ident.name {
                if latest.is_empty() {
                    latest.push((artifact_ident, artifact));
                } else {
                    if artifact_ident > latest[0].0 {
                        let _ = latest.pop();
                        latest.push((artifact_ident, artifact));
                    }
                }
            }
        }

        if latest.is_empty() {
            Err(Error::PackageNotFound)
        } else {
            Ok(FullyQualifiedPackageIdent::from(
                latest.pop().unwrap().1.ident()?,
            )?)
        }
    }

    fn is_artifact_cached(&self, ident: &FullyQualifiedPackageIdent) -> bool {
        self.cached_artifact_path(ident).is_file()
    }

    /// Returns the path to the location this package would exist at in
    /// the local package cache. It does not mean that the package is
    /// actually *in* the package cache, though.
    fn cached_artifact_path(&self, ident: &FullyQualifiedPackageIdent) -> PathBuf {
        self.artifact_cache_path.join(ident.archive_name())
    }

    fn fetch_latest_pkg_ident_for(
        &self,
        ident: &PackageIdent,
        token: Option<&str>,
    ) -> Result<FullyQualifiedPackageIdent> {
        self.fetch_latest_pkg_ident_in_channel_for(ident, &self.channel, token)
    }

    fn fetch_latest_pkg_ident_in_channel_for(
        &self,
        ident: &PackageIdent,
        channel: &Channel,
        token: Option<&str>,
    ) -> Result<FullyQualifiedPackageIdent> {
        let origin_package: PackageIdent = self.depot_client
            .show_package(ident, Some(channel.0), token)?
            .into();
        FullyQualifiedPackageIdent::from(origin_package)
    }

    /// Retrieve the identified package from the depot, ensuring that
    /// the artifact is cached locally.
    fn fetch_artifact(
        &self,
        ui: &mut UI,
        ident: &FullyQualifiedPackageIdent,
        token: Option<&str>,
    ) -> Result<()> {
        ui.status(Status::Downloading, ident)?;
        match self.depot_client.fetch_package(
            ident.as_ref(),
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
        if self.is_offline() {
            return Err(Error::OfflineOriginKeyNotFound(name_with_rev.to_string()));
        } else {
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
    }

    /// Copies the artifact to the local artifact cache directory
    // TODO (CM): Oh, we could just pass in the LocalArchive
    fn store_artifact_in_cache(
        &self,
        ident: &FullyQualifiedPackageIdent,
        artifact_path: &Path,
    ) -> Result<()> {
        // Canonicalize both paths to ensure that there aren't any symlinks when comparing them
        // later.
        let artifact_path = artifact_path.canonicalize()?;
        let cache_path = self.cached_artifact_path(ident).canonicalize()?;
        fs::create_dir_all(self.artifact_cache_path)?;

        // Handle the pathological case where you're trying to install
        // an artifact file directly from the cache. Otherwise, you'd
        // end up with a 0-byte file in the cache and wouldn't be able
        // to subsequently verify it.
        if artifact_path == cache_path {
            debug!(
                "skipping artifact copy, it is being referenced directly from cache, \
                artifact_path={}, cache_path={}",
                artifact_path.display(),
                cache_path.display()
            );
        } else {
            debug!(
                "copying artifact to cache, artifact_path={}, cached_path={}",
                artifact_path.display(),
                cache_path.display()
            );
            fs::copy(artifact_path, cache_path)?;
        }
        Ok(())
    }

    fn verify_artifact(
        &self,
        ui: &mut UI,
        ident: &FullyQualifiedPackageIdent,
        artifact: &mut PackageArchive,
    ) -> Result<()> {
        let artifact_ident = artifact.ident()?;
        if ident.as_ref() != &artifact_ident {
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

    fn is_offline(&self) -> bool {
        self.install_mode == &InstallMode::Offline
    }

    // TODO fn: I'm skeptical as to whether we want these warnings all the time. Perhaps it's
    // better to warn that nothing is found and redirect a user to run another standalone
    // `hab pkg ...` subcommand to get more information.
    fn recommend_channels(
        &self,
        ui: &mut UI,
        ident: &PackageIdent,
        token: Option<&str>,
    ) -> Result<()> {
        if let Ok(recommendations) = self.get_channel_recommendations(&ident, token) {
            if !recommendations.is_empty() {
                ui.warn(format!(
                    "No releases of {} exist in the '{}' channel",
                    &ident,
                    self.channel
                ))?;
                ui.warn("The following releases were found:")?;
                for r in recommendations {
                    ui.warn(format!("  {} in the '{}' channel", r.1, r.0))?;
                }
            }
        }

        Ok(())
    }

    /// Get a list of suggested package identifiers from all
    /// channels. This is used to generate actionable user feedback
    /// when the desired package was not found in the specified
    /// channel.
    fn get_channel_recommendations(
        &self,
        ident: &PackageIdent,
        token: Option<&str>,
    ) -> Result<Vec<(String, String)>> {
        let mut res = Vec::new();

        let channels = match self.depot_client.list_channels(ident.origin(), false) {
            Ok(channels) => channels,
            Err(e) => {
                debug!("Failed to get channel list: {:?}", e);
                return Err(Error::PackageNotFound);
            }
        };

        for channel in channels.iter().map(|c| Channel::new(c)) {
            match self.fetch_latest_pkg_ident_in_channel_for(ident, &channel, token) {
                Ok(pkg) => res.push((channel.to_string(), format!("{}", pkg))),
                Err(_) => (),
            };
        }

        Ok(res)
    }
}
