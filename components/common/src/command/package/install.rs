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

use std::{borrow::Cow,
          fmt,
          fs::{self,
               File},
          io::{self,
               BufRead,
               BufReader},
          path::{Path,
                 PathBuf},
          result::Result as StdResult,
          str::FromStr};

use crate::{api_client::{self,
                         Client,
                         Error::APIError},
            hcore::{self,
                    crypto::{artifact,
                             keys::parse_name_with_rev,
                             SigKeyPair},
                    fs::{cache_key_path,
                         pkg_install_path,
                         svc_hooks_path,
                         AtomicWriter},
                    package::{list::temp_package_directory,
                              Identifiable,
                              PackageArchive,
                              PackageIdent,
                              PackageInstall,
                              PackageTarget},
                    ChannelIdent}};
use glob;
use hyper::status::StatusCode;
use retry::retry;

use crate::{error::{Error,
                    Result},
            templating::{self,
                         hooks::{Hook,
                                 InstallHook},
                         package::Pkg},
            ui::{Status,
                 UIWriter}};

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
#[derive(Clone, Debug)]
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
    ident:  PackageIdent,
    target: PackageTarget,
    path:   PathBuf,
}

/// Encapsulate all possible sources we can install packages from.
#[derive(Clone, Debug)]
pub enum InstallSource {
    /// We can install from a package identifier
    Ident(PackageIdent, PackageTarget),
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
            let target = archive.target()?;
            match archive.ident() {
                Ok(ident) => {
                    Ok(InstallSource::Archive(LocalArchive { ident,
                                                             target,
                                                             path:
                                                                 path.to_path_buf() }))
                }
                Err(e) => Err(e),
            }
        } else {
            if let Some(extension) = path.extension() {
                if extension == "hart" {
                    return Err(hcore::Error::FileNotFound(s.to_string()));
                }
            }

            match s.parse::<PackageIdent>() {
                // TODO fn: I would have preferred to explicitly choose a `PackageTarget` here, but
                // we're limited to the input string in this trait implementation. For the moment
                // this will work when the appropriate and correct answer for the `PackageTarget`
                // is the currently active one, but will be insufficient if used in a situation
                // where the user needs to provide the target explicitly.
                //
                // To me, this implies that this trait impl isn't strictly true anymore--there
                // would otherwise have to be a canonical way to express an ident **and** target in
                // one string, such as `"x86_64-linux::core/redis"` (or similar). As there is
                // currently no such representation, I'd argue that this `FromStr` is no longer
                // reasonable. However, it's doing the job for now and we can proceed with caution.
                Ok(ident) => Ok(InstallSource::Ident(ident, PackageTarget::active_target())),
                Err(e) => Err(e),
            }
        }
    }
}

impl From<(PackageIdent, PackageTarget)> for InstallSource {
    /// Convenience function to generate an `InstallSource` from an
    /// existing `PackageIdent`.
    fn from((ident, target): (PackageIdent, PackageTarget)) -> Self {
        InstallSource::Ident(ident, target)
    }
}

impl Into<PackageIdent> for InstallSource {
    fn into(self) -> PackageIdent {
        match self {
            InstallSource::Ident(ident, _) => ident,
            InstallSource::Archive(local_archive) => local_archive.ident,
        }
    }
}

impl AsRef<PackageIdent> for InstallSource {
    fn as_ref(&self) -> &PackageIdent {
        match *self {
            InstallSource::Ident(ref ident, _) => ident,
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
    fn default() -> Self { InstallMode::Online }
}

/// Governs how install hooks behave when loading packages
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstallHookMode {
    /// Run the install hook and all install hooks of dependent packages
    /// that have not yet been run or have previously failed
    Run,
    /// Do not run any install hooks when loading a package
    Ignore,
}

impl Default for InstallHookMode {
    fn default() -> Self { InstallHookMode::Run }
}

/// When querying Builder, we may not find a package that satisfies
/// the desired package identifier, but we may have such a package
/// already installed locally. In most cases, it should be fine for us
/// to use the locally-installed package. However, it can cause issues
/// when building packages using HAB_BLDR_CHANNEL, due to how the
/// fallback logic in `hab-plan-build` is currently implemented.
///
/// This enum governs whether or not we should use a locally-installed
/// package to satisfy a dependency, or if we should ignore it, thus
/// giving the user the opportunity to try installing from another
/// channel.
///
/// Usage of this is currently hidden behind the IGNORE_LOCAL feature
/// flag, as there is still some question as to the best way to solve
/// this.
#[derive(Debug, Eq, PartialEq)]
pub enum LocalPackageUsage {
    /// Use locally-installed packages if they satisfy the desired
    /// identifier, but a package cannot be found in Builder.
    ///
    /// This *may* be a different package than Builder may have found
    /// in another channel.
    Prefer,
    /// Do not use locally-installed packages if a package cannot be
    /// found in Builder.
    Ignore,
}

impl Default for LocalPackageUsage {
    /// The default behavior is to use locally-installed packages if
    /// they can satisfy the desired identifier, and if no more
    /// suitable package could not be found in Builder.
    fn default() -> Self { LocalPackageUsage::Prefer }
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
        where I: Into<Cow<'a, PackageIdent>>
    {
        let ident = ident.into();
        if ident.as_ref().fully_qualified() {
            Ok(FullyQualifiedPackageIdent { ident })
        } else {
            Err(Error::HabitatCore(
                hcore::Error::FullyQualifiedPackageIdentRequired(ident.to_owned().to_string()),
            ))
        }
    }

    fn archive_name(&self) -> String {
        self.ident.as_ref().archive_name().unwrap_or_else(|_| {
                                              panic!("PackageIdent {} should be fully qualified",
                                                     self.ident.as_ref())
                                          })
    }
}

impl<'a> AsRef<PackageIdent> for FullyQualifiedPackageIdent<'a> {
    fn as_ref(&self) -> &PackageIdent { self.ident.as_ref() }
}

impl<'a> fmt::Display for FullyQualifiedPackageIdent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.ident.as_ref().fmt(f) }
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
#[allow(clippy::too_many_arguments)]
pub fn start<U>(ui: &mut U,
                url: &str,
                channel: &ChannelIdent,
                install_source: &InstallSource,
                product: &str,
                version: &str,
                fs_root_path: &Path,
                artifact_cache_path: &Path,
                token: Option<&str>,
                install_mode: &InstallMode,
                local_package_usage: &LocalPackageUsage,
                install_hook_mode: InstallHookMode)
                -> Result<PackageInstall>
    where U: UIWriter
{
    // TODO (CM): rename fs::cache_key_path so the naming is
    // consistent and flows better.
    let key_cache_path = &cache_key_path(Some(fs_root_path));
    debug!("install key_cache_path: {}", key_cache_path.display());

    let api_client = Client::new(url, product, version, Some(fs_root_path))?;
    let task = InstallTask { install_mode,
                             local_package_usage,
                             api_client,
                             channel,
                             fs_root_path,
                             artifact_cache_path,
                             key_cache_path,
                             install_hook_mode };

    match *install_source {
        InstallSource::Ident(ref ident, target) => {
            task.with_ident(ui, ident.clone(), target, token)
        }
        InstallSource::Archive(ref local_archive) => task.with_archive(ui, local_archive, token),
    }
}

pub fn check_install_hooks<T, P>(ui: &mut T,
                                 package: &PackageInstall,
                                 fs_root_path: P)
                                 -> Result<()>
    where T: UIWriter,
          P: AsRef<Path>
{
    for dependency in package.tdeps()? {
        run_install_hook_unless_already_successful(
            ui,
            &PackageInstall::load(&dependency, Some(fs_root_path.as_ref()))?,
        )?;
    }

    run_install_hook_unless_already_successful(ui, &package)
}

fn run_install_hook_unless_already_successful<T>(ui: &mut T, package: &PackageInstall) -> Result<()>
    where T: UIWriter
{
    match read_install_hook_status(package.installed_path.join(InstallHook::STATUS_FILE))? {
        Some(0) => Ok(()),
        _ => run_install_hook(ui, package),
    }
}

fn read_install_hook_status(path: PathBuf) -> Result<Option<i32>> {
    match File::open(&path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().next() {
                Some(Ok(line)) => {
                    match line.parse::<i32>() {
                        Ok(status) => Ok(Some(status)),
                        Err(_) => Err(Error::StatusFileCorrupt(path)),
                    }
                }
                _ => Err(Error::StatusFileCorrupt(path)),
            }
        }
        Err(_) => Ok(None),
    }
}

fn run_install_hook<T>(ui: &mut T, package: &PackageInstall) -> Result<()>
    where T: UIWriter
{
    if let Some(ref hook) = InstallHook::load(&package.ident.name,
                                              &svc_hooks_path(package.ident.name.clone()),
                                              &package.installed_path.join("hooks"))
    {
        ui.status(Status::Executing,
                  format!("install hook for '{}'", &package.ident(),))?;
        templating::compile_for_package_install(package)?;
        if !hook.run(&package.ident().name,
                     &Pkg::from_install(package)?,
                     None::<String>)
        {
            return Err(Error::InstallHookFailed(package.ident().clone()));
        }
    }
    Ok(())
}

struct InstallTask<'a> {
    install_mode: &'a InstallMode,
    local_package_usage: &'a LocalPackageUsage,
    api_client: Client,
    channel: &'a ChannelIdent,
    fs_root_path: &'a Path,
    /// The path to the local artifact cache (e.g., /hab/cache/artifacts)
    artifact_cache_path: &'a Path,
    key_cache_path: &'a Path,
    install_hook_mode: InstallHookMode,
}

impl<'a> InstallTask<'a> {
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
    fn with_ident<T>(&self,
                     ui: &mut T,
                     ident: PackageIdent,
                     target: PackageTarget,
                     token: Option<&str>)
                     -> Result<PackageInstall>
        where T: UIWriter
    {
        ui.begin(format!("Installing {}", &ident))?;
        let target_ident = self.determine_latest_from_ident(ui, ident, target, token)?;

        match self.installed_package(&target_ident) {
            Some(package_install) => {
                // The installed package was found on disk
                ui.status(Status::Using, &target_ident)?;
                if self.install_hook_mode != InstallHookMode::Ignore {
                    check_install_hooks(ui, &package_install, self.fs_root_path)?;
                }
                ui.end(format!("Install of {} complete with {} new packages installed.",
                               &target_ident, 0))?;
                Ok(package_install)
            }
            None => {
                // No installed package was found
                self.install_package(ui, &target_ident, target, token)
            }
        }
    }

    /// Given an archive on disk, ensure that it is properly installed
    /// and return the package's identifier.
    fn with_archive<T>(&self,
                       ui: &mut T,
                       local_archive: &LocalArchive,
                       token: Option<&str>)
                       -> Result<PackageInstall>
        where T: UIWriter
    {
        ui.begin(format!("Installing {}", local_archive.path.display()))?;
        let target_ident = FullyQualifiedPackageIdent::from(&local_archive.ident)?;
        match self.installed_package(&target_ident) {
            Some(package_install) => {
                // The installed package was found on disk
                ui.status(Status::Using, &target_ident)?;
                if self.install_hook_mode != InstallHookMode::Ignore {
                    check_install_hooks(ui, &package_install, self.fs_root_path)?;
                }
                ui.end(format!("Install of {} complete with {} new packages installed.",
                               &target_ident, 0))?;
                Ok(package_install)
            }
            None => {
                // No installed package was found
                self.store_artifact_in_cache(&target_ident, &local_archive.path)?;
                self.install_package(ui, &target_ident, local_archive.target, token)
            }
        }
    }

    fn determine_latest_from_ident<T>(&self,
                                      ui: &mut T,
                                      ident: PackageIdent,
                                      target: PackageTarget,
                                      token: Option<&str>)
                                      -> Result<FullyQualifiedPackageIdent<'_>>
        where T: UIWriter
    {
        if ident.fully_qualified() {
            // If we have a fully qualified package identifier, then our work is done--there can
            // only be *one* package that satisfies a fully qualified identifier.

            FullyQualifiedPackageIdent::from(ident)
        } else if self.is_offline() {
            // If we can't contact a Builder API, then we'll find the latest installed package or
            // cached artifact that satisfies the fuzzy package identifier.

            ui.status(Status::Determining,
                      format!("latest version of {} locally installed or cached (offline)",
                              &ident))?;
            match self.latest_installed_or_cached(&ident) {
                Ok(i) => Ok(i),
                Err(Error::PackageNotFound(_)) => {
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

            ui.status(Status::Determining,
                      format!("latest version of {} in the '{}' channel",
                              &ident, self.channel))?;
            let latest_remote = match self.fetch_latest_pkg_ident_for(&ident, target, token) {
                Ok(latest_ident) => Some(latest_ident),
                Err(Error::APIClient(APIError(StatusCode::NotFound, _))) => None,
                Err(e) => {
                    debug!("error fetching ident: {:?}", e);
                    return Err(e);
                }
            };

            match (latest_local, latest_remote) {
                (Ok(local), Some(remote)) => {
                    if local.as_ref() > remote.as_ref() {
                        // Return the latest identifier reported by
                        // the Builder API *unless* there is a newer
                        // version found installed locally.
                        ui.status(Status::Found,
                                  format!("newer installed version ({}) than remote version \
                                           ({})",
                                          &local,
                                          remote.as_ref()))?;
                        Ok(local)
                    } else {
                        Ok(remote)
                    }
                }
                (Ok(local), None) => {
                    if self.ignore_locally_installed_packages() {
                        // This is the behavior that is currently
                        // governed by the IGNORE_LOCAL feature-flag
                        self.recommend_channels(ui, &ident, target, token)?;
                        ui.warn(format!("Locally-installed package '{}' would satisfy '{}', \
                                         but we are ignoring that as directed",
                                        local.as_ref(),
                                        &ident,))?;
                        Err(Error::PackageNotFound("".to_string()))
                    } else {
                        ui.status(Status::Missing,
                                  format!("remote version of '{}' in the '{}' channel, but an \
                                           installed version was found locally ({})",
                                          &ident,
                                          self.channel,
                                          local.as_ref()))?;
                        FullyQualifiedPackageIdent::from(local.as_ref().clone())
                    }
                }
                (Err(_), Some(remote)) => Ok(remote),
                (Err(_), None) => {
                    self.recommend_channels(ui, &ident, target, token)?;
                    Err(Error::PackageNotFound("".to_string()))
                }
            }
        }
    }

    /// Given the identifier of an artifact, ensure that the artifact,
    /// as well as all its dependencies, have been cached and
    /// installed.
    ///
    /// If the package is already present in the cache, it is not
    /// re-downloaded. Any dependencies of the package that are not
    /// installed will be re-cached (as needed) and installed.
    fn install_package<T>(&self,
                          ui: &mut T,
                          ident: &FullyQualifiedPackageIdent<'_>,
                          target: PackageTarget,
                          token: Option<&str>)
                          -> Result<PackageInstall>
        where T: UIWriter
    {
        // TODO (CM): rename artifact to archive
        let mut artifact = self.get_cached_artifact(ui, ident, target, token)?;

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
                if self.install_hook_mode != InstallHookMode::Ignore {
                    run_install_hook_unless_already_successful(
                        ui,
                        &PackageInstall::load(dependency, Some(self.fs_root_path))?,
                    )?;
                }
            } else {
                artifacts_to_install.push(self.get_cached_artifact(
                    ui,
                    &FullyQualifiedPackageIdent::from(dependency)?,
                    target,
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
            if self.install_hook_mode != InstallHookMode::Ignore {
                run_install_hook(ui,
                                 &PackageInstall::load(&artifact.ident()?,
                                                       Some(self.fs_root_path))?)?;
            }
        }

        ui.end(format!("Install of {} complete with {} new packages installed.",
                       ident,
                       artifacts_to_install.len()))?;

        // Return the thing we just installed
        PackageInstall::load(ident.as_ref(), Some(self.fs_root_path)).map_err(Error::from)
    }

    /// This ensures the identified package is in the local cache,
    /// verifies it, and returns a handle to the package's metadata.
    fn get_cached_artifact<T>(&self,
                              ui: &mut T,
                              ident: &FullyQualifiedPackageIdent<'_>,
                              target: PackageTarget,
                              token: Option<&str>)
                              -> Result<PackageArchive>
        where T: UIWriter
    {
        if self.is_artifact_cached(&ident) {
            debug!("Found {} in artifact cache, skipping remote download",
                   ident);
        } else if self.is_offline() {
            return Err(Error::OfflineArtifactNotFound(ident.as_ref().clone()));
        } else if retry(RETRIES,
                        RETRY_WAIT,
                        || self.fetch_artifact(ui, ident, target, token),
                        |res| res.is_ok()).is_err()
        {
            return Err(Error::DownloadFailed(format!("We tried {} times but \
                                                      could not download {}. \
                                                      Giving up.",
                                                     RETRIES, ident)));
        }

        let mut artifact = PackageArchive::new(self.cached_artifact_path(ident));
        ui.status(Status::Verifying, artifact.ident()?)?;
        self.verify_artifact(ui, ident, &mut artifact)?;
        Ok(artifact)
    }

    /// Adapter function wrapping `PackageArchive::unpack`
    fn unpack_artifact<T>(&self, ui: &mut T, artifact: &mut PackageArchive) -> Result<()>
        where T: UIWriter
    {
        let ident = &artifact.ident()?;
        let real_install_path = &pkg_install_path(ident, Some(self.fs_root_path));

        // This match will always return Ok(Path) as the install path is at least 2 levels
        // below the fs_root_path
        match real_install_path.parent() {
            Some(real_install_base) => {
                let temp_dir = temp_package_directory(real_install_path)?;
                let temp_install_path = &pkg_install_path(ident, Some(temp_dir.path()));
                artifact.unpack(Some(temp_dir.path()))?;

                if let Err(e) = fs::rename(temp_install_path, real_install_path) {
                    // The rename might fail if the real_install_path
                    // was created while we were unpacking. If the
                    // package now exists, ignore the failure.
                    debug!("rename failed with {:?}, checking for installed package", e);
                    if PackageInstall::load(ident, Some(self.fs_root_path)).is_err() {
                        return Err(Error::from(e));
                    }
                }

                if cfg!(unix) {
                    fs::File::open(real_install_base).and_then(|f| f.sync_all())?;
                }

                ui.status(Status::Installed, ident)?;
                Ok(())
            }
            None => unreachable!("Install path doesn't have a parent"),
        }
    }

    /// Adapter function to retrieve an installed package given an
    /// identifier, if it exists.
    fn installed_package(&self, ident: &FullyQualifiedPackageIdent<'_>) -> Option<PackageInstall> {
        PackageInstall::load(ident.as_ref(), Some(self.fs_root_path)).ok()
    }

    /// Checks for the latest installed package or cached artifact that matches a given package
    /// identifier and returns a fully qualified package identifier if a match exists.
    fn latest_installed_or_cached(&self,
                                  ident: &PackageIdent)
                                  -> Result<FullyQualifiedPackageIdent<'_>> {
        let latest_installed = self.latest_installed_ident(&ident);
        let latest_cached = self.latest_cached_ident(&ident);
        debug!("latest installed: {:?}, latest_cached: {:?}",
               &latest_installed, &latest_cached,);
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
            (Err(_), Err(_)) => return Err(Error::PackageNotFound("".to_string())),
        };
        debug!("offline mode: winner: {:?}", &latest);

        Ok(latest)
    }

    fn latest_installed_ident(&self,
                              ident: &PackageIdent)
                              -> Result<FullyQualifiedPackageIdent<'_>> {
        match PackageInstall::load(ident, Some(self.fs_root_path)) {
            Ok(pi) => FullyQualifiedPackageIdent::from(pi.ident().clone()),
            Err(_) => Err(Error::PackageNotFound("".to_string())),
        }
    }

    fn latest_cached_ident(&self, ident: &PackageIdent) -> Result<FullyQualifiedPackageIdent<'_>> {
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
            ident.archive_name()?
        };
        let glob_path = self.artifact_cache_path.join(filename_glob);
        let glob_path = glob_path.to_string_lossy();
        debug!("looking for cached artifacts, glob={}", glob_path.as_ref());

        let mut latest: Vec<(PackageIdent, PackageArchive)> = Vec::with_capacity(1);
        for file in glob::glob(glob_path.as_ref()).expect("glob pattern should compile")
                                                  .filter_map(StdResult::ok)
        {
            let mut artifact = PackageArchive::new(&file);
            let artifact_ident = artifact.ident().ok();
            if artifact_ident.is_none() {
                continue;
            }
            let artifact_ident = artifact_ident.unwrap();
            if artifact_ident.origin == ident.origin && artifact_ident.name == ident.name {
                if latest.is_empty() {
                    latest.push((artifact_ident, artifact));
                } else if artifact_ident > latest[0].0 {
                    latest.pop();
                    latest.push((artifact_ident, artifact));
                }
            }
        }

        if latest.is_empty() {
            Err(Error::PackageNotFound("".to_string()))
        } else {
            Ok(FullyQualifiedPackageIdent::from(latest.pop()
                                                      .unwrap()
                                                      .1
                                                      .ident()?)?)
        }
    }

    fn is_artifact_cached(&self, ident: &FullyQualifiedPackageIdent<'_>) -> bool {
        self.cached_artifact_path(ident).is_file()
    }

    /// Returns the path to the location this package would exist at in
    /// the local package cache. It does not mean that the package is
    /// actually *in* the package cache, though.
    fn cached_artifact_path(&self, ident: &FullyQualifiedPackageIdent<'_>) -> PathBuf {
        self.artifact_cache_path.join(ident.archive_name())
    }

    fn fetch_latest_pkg_ident_for(&self,
                                  ident: &PackageIdent,
                                  target: PackageTarget,
                                  token: Option<&str>)
                                  -> Result<FullyQualifiedPackageIdent<'_>> {
        self.fetch_latest_pkg_ident_in_channel_for(ident, target, &self.channel, token)
    }

    fn fetch_latest_pkg_ident_in_channel_for(&self,
                                             ident: &PackageIdent,
                                             target: PackageTarget,
                                             channel: &ChannelIdent,
                                             token: Option<&str>)
                                             -> Result<FullyQualifiedPackageIdent<'_>> {
        let origin_package = self.api_client
                                 .show_package(ident, target, channel, token)?;
        FullyQualifiedPackageIdent::from(origin_package)
    }

    /// Retrieve the identified package from the depot, ensuring that
    /// the artifact is cached locally.
    fn fetch_artifact<T>(&self,
                         ui: &mut T,
                         ident: &FullyQualifiedPackageIdent<'_>,
                         target: PackageTarget,
                         token: Option<&str>)
                         -> Result<()>
        where T: UIWriter
    {
        ui.status(Status::Downloading, ident)?;
        match self.api_client.fetch_package(ident.as_ref(),
                                            target,
                                            token,
                                            self.artifact_cache_path,
                                            ui.progress())
        {
            Ok(_) => Ok(()),
            Err(api_client::Error::APIError(StatusCode::NotImplemented, _)) => {
                println!("Host platform or architecture not supported by the targeted depot; \
                          skipping.");
                Ok(())
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    fn fetch_origin_key<T>(&self, ui: &mut T, name_with_rev: &str) -> Result<()>
        where T: UIWriter
    {
        if self.is_offline() {
            return Err(Error::OfflineOriginKeyNotFound(name_with_rev.to_string()));
        } else {
            ui.status(Status::Downloading,
                      format!("{} public origin key", &name_with_rev))?;
            let (name, rev) = parse_name_with_rev(&name_with_rev)?;
            self.api_client
                .fetch_origin_key(&name, &rev, self.key_cache_path, ui.progress())?;
            ui.status(Status::Cached,
                      format!("{} public origin key", &name_with_rev))?;
            Ok(())
        }
    }

    /// Copies the artifact to the local artifact cache directory
    // TODO (CM): Oh, we could just pass in the LocalArchive
    fn store_artifact_in_cache(&self,
                               ident: &FullyQualifiedPackageIdent<'_>,
                               artifact_path: &Path)
                               -> Result<()> {
        // Canonicalize both paths to ensure that there aren't any symlinks when comparing them
        // later. These calls can fail under certain circumstances, so we warn, allow failure and
        // try to continue.
        let artifact_path = artifact_path.canonicalize()
                                         .unwrap_or_else(|_| artifact_path.to_path_buf());
        fs::create_dir_all(self.artifact_cache_path)?;
        let cache_path = self.artifact_cache_path
                             .canonicalize()
                             .unwrap_or_else(|_| self.artifact_cache_path.to_path_buf())
                             .join(ident.archive_name());

        // Handle the pathological case where you're trying to install
        // an artifact file directly from the cache. Otherwise, you'd
        // end up with a 0-byte file in the cache and wouldn't be able
        // to subsequently verify it.
        if artifact_path == cache_path {
            debug!("skipping artifact copy, it is being referenced directly from cache, \
                    artifact_path={}, cache_path={}",
                   artifact_path.display(),
                   cache_path.display());
        } else {
            debug!("copying artifact to cache, artifact_path={}, cached_path={}",
                   artifact_path.display(),
                   cache_path.display());
            let w = AtomicWriter::new(&cache_path)?;
            w.with_writer(|mut w| {
                 let mut f = File::open(artifact_path)?;
                 io::copy(&mut f, &mut w)
             })?;
        }
        Ok(())
    }

    fn verify_artifact<T>(&self,
                          ui: &mut T,
                          ident: &FullyQualifiedPackageIdent<'_>,
                          artifact: &mut PackageArchive)
                          -> Result<()>
        where T: UIWriter
    {
        let artifact_ident = artifact.ident()?;
        if ident.as_ref() != &artifact_ident {
            return Err(Error::ArtifactIdentMismatch((artifact.file_name(),
                                                     artifact_ident.to_string(),
                                                     ident.to_string())));
        }

        // TODO fn: this un-alterable target behavior that's piggybacking off `verify_artifact()`
        // is troubling and feels like it should at least be configuratble somewhere, allowing a
        // consumer of the install logic to deal with artifacts not meant for the currently active
        // system. Until we have better ideas, this implementation preserves past behavior.
        let artifact_target = artifact.target()?;
        let active_target = PackageTarget::active_target();
        if active_target != artifact_target {
            return Err(Error::HabitatCore(hcore::Error::WrongActivePackageTarget(
                active_target,
                artifact_target,
            )));
        }

        let nwr = artifact::artifact_signer(&artifact.path)?;
        if SigKeyPair::get_public_key_path(&nwr, self.key_cache_path).is_err() {
            self.fetch_origin_key(ui, &nwr)?;
        }

        artifact.verify(&self.key_cache_path)?;
        debug!("Verified {} signed by {}", ident, &nwr);
        Ok(())
    }

    fn is_offline(&self) -> bool { self.install_mode == &InstallMode::Offline }

    /// We may not want to use currently-installed packages if one
    /// can't be found in Builder in the given channel.
    ///
    /// Specifically, as long as our channel fallback-logic in
    /// `hab-plan-build` relies on attempting to install a package
    /// instead of asking Builder what it has, we should provide an
    /// escape hatch.
    ///
    /// This implementation isn't necessarily how this behavior should
    /// ultimately be implemented; it's feature-flagged for now while
    /// we figure that out.
    fn ignore_locally_installed_packages(&self) -> bool {
        self.local_package_usage == &LocalPackageUsage::Ignore
    }

    // TODO fn: I'm skeptical as to whether we want these warnings all the time. Perhaps it's
    // better to warn that nothing is found and redirect a user to run another standalone
    // `hab pkg ...` subcommand to get more information.
    fn recommend_channels<T>(&self,
                             ui: &mut T,
                             ident: &PackageIdent,
                             target: PackageTarget,
                             token: Option<&str>)
                             -> Result<()>
        where T: UIWriter
    {
        if let Ok(recommendations) = self.get_channel_recommendations(&ident, target, token) {
            if !recommendations.is_empty() {
                ui.warn(format!("No releases of {} exist in the '{}' channel",
                                &ident, self.channel))?;
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
    fn get_channel_recommendations(&self,
                                   ident: &PackageIdent,
                                   target: PackageTarget,
                                   token: Option<&str>)
                                   -> Result<Vec<(String, String)>> {
        let mut res = Vec::new();

        let channels = match self.api_client.list_channels(ident.origin(), false) {
            Ok(channels) => channels,
            Err(e) => {
                debug!("Failed to get channel list: {:?}", e);
                return Err(Error::PackageNotFound("".to_string()));
            }
        };

        for channel in channels.into_iter().map(ChannelIdent::from) {
            if let Ok(pkg) =
                self.fetch_latest_pkg_ident_in_channel_for(ident, target, &channel, token)
            {
                res.push((channel.to_string(), format!("{}", pkg)));
            }
        }

        Ok(res)
    }
}
