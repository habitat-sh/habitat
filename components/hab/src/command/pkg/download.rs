//! Downloads a Habitat package from a [depot](../depot).
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg download core/redis
//! ```
//!
//! Will download `core/redis` package and all its transitive dependencies from a custom depot:
//!
//! ```bash
//! $ hab pkg download -u http://depot.co:9633 -t x86-64_linux --download-directory download core/redis/3.0.1
//! ```
//!
//! This would download the `3.0.1` version of redis for linux and all
//! of its transitive dependencies, and the accompanying signing keys
//! to a directory 'download'
//!
//! The most common usage will have a file containing newline separated list of package
//! identifiers.
//!
//! # Internals
//!
//! * Resolve the list of partial artifact identifiers to fully qualified idents
//! * Gather the TDEPS of the list (done concurrently with the above step)
//! * Download the artifact
//! * Verify it is un-altered
//! * Fetch the signing keys

use std::{collections::{HashMap,
                        HashSet},
          fs::DirBuilder,
          path::{Path,
                 PathBuf},
          time::Duration};

use crate::{api_client::{self,
                         BuilderAPIClient,
                         Client,
                         Error::APIError,
                         Package},
            common::Error as CommonError,
            hcore::{crypto::{artifact,
                             keys::parse_name_with_rev,
                             SigKeyPair},
                    fs::cache_root_path,
                    package::{Identifiable,
                              PackageArchive,
                              PackageIdent,
                              PackageTarget},
                    ChannelIdent,
                    Error as CoreError}};

use habitat_common::ui::{Glyph,
                         Status,
                         UIWriter};

use reqwest::StatusCode;
use retry::delay;

use crate::error::{Error,
                   Result};

pub const RETRIES: usize = 5;
pub const RETRY_WAIT: Duration = Duration::from_millis(3000);

#[derive(Debug, Deserialize)]
pub struct PackageSetFile {
    pub format_version:  Option<u8>,
    pub file_descriptor: Option<String>,

    // Note: flatten cause *all* keys not explicitly listed as a struct member to be treated as a
    // target. If someone gives a bad key, the error message will be incorrect as to the cause.
    // This is a limitation of the target as keys choice for the TOML file format.
    #[serde(flatten)]
    pub targets: HashMap<PackageTarget, Vec<PackageSetValue>>,
}

#[derive(Debug, Deserialize)]
pub struct PackageSetValue {
    pub channel:  ChannelIdent,
    pub packages: Vec<String>,
}

#[derive(Debug)]
pub struct PackageSet {
    pub target:  PackageTarget,
    pub channel: ChannelIdent,
    pub idents:  Vec<PackageIdent>,
}

/// Download a Habitat package.
///
/// If an `PackageIdent` is given, we retrieve the package from the specified Builder
/// `url`. Providing a fully-qualified identifer will result in that exact package being downloaded
/// (regardless of `channel`). Providing a partially-qualified identifier will result in the
/// installation of latest appropriate release from the given `channel`.
///
/// Any dependencies of will be retrieved from Builder (if they're not already downloaded locally).
///
/// At the end of this function, the specified package and all its
/// dependencies will be downloaded on the system in the
/// <download_path>/artifacts directory. Any signing keys will also be
/// downloaded and put in the <download_path/keys> directory.

/// Also, in the future we may want to accept an alternate builder to 'filter' what we pull down by
/// That would greatly optimize the 'sync' to on prem builder case, as we could point to that
/// and only fetch what we don't already have.
#[allow(clippy::too_many_arguments)]
pub async fn start<U>(ui: &mut U,
                      url: &str,
                      product: &str,
                      version: &str,
                      package_sets: &[PackageSet], // clippy suggestion
                      download_path: Option<&PathBuf>,
                      token: Option<&str>,
                      verify: bool,
                      ignore_missing_seeds: bool)
                      -> Result<()>
    where U: UIWriter
{
    debug!(
           "Starting download with url: {}, product: {}, version: {}, 
         download_path: {:?}, token: {:?}, verify: {}, ignore_missing_seeds: {}, set_count: {}",
           url,
           product,
           version,
           download_path,
           token,
           verify,
           ignore_missing_seeds,
           package_sets.len()
    );

    let download_path_default = &cache_root_path::<PathBuf>(None); // Satisfy E0716
    let download_path_expanded = download_path.unwrap_or(download_path_default).as_ref();
    debug!("Using download_path {:?} expanded to {:?}",
           download_path, download_path_expanded);

    debug!("{} {}",
           package_sets.is_empty(),
           package_sets[0].idents.is_empty());

    if package_sets.is_empty() {
        ui.fatal("No package identifers provided. Specify identifiers on the command line, or \
                  via a input file")?;
        return Err(CommonError::MissingCLIInputError(String::from("No package identifiers \
                                                                   found")).into());
    }

    // We deliberately use None to specify the default path as this is used for cert paths, which
    // we don't want to override.
    let api_client = Client::new(url, product, version, None)?;

    let task = DownloadTask { package_sets,
                              url,
                              api_client,
                              token,
                              download_path: download_path_expanded,
                              verify,
                              ignore_missing_seeds };

    let download_count = task.execute(ui).await?;

    debug!("Expanded package count: {}", download_count);

    Ok(())
}

struct DownloadTask<'a> {
    package_sets:         &'a [PackageSet],
    url:                  &'a str,
    api_client:           BuilderAPIClient,
    token:                Option<&'a str>,
    download_path:        &'a Path,
    verify:               bool,
    ignore_missing_seeds: bool,
}

impl<'a> DownloadTask<'a> {
    async fn execute<T>(&self, ui: &mut T) -> Result<usize>
        where T: UIWriter
    {
        // This was written intentionally with an eye towards data parallelism
        // Any or all of these phases should naturally fit a fork-join model
        ui.begin(format!("Storing in download directory {:?} ", self.download_path))?;

        self.verify_and_prepare_download_directory(ui)?;

        // Phase 1: Expand to fully qualified deps and TDEPS
        let expanded_idents = self.expand_sources(ui).await?;

        // Phase 2: Download artifacts
        let downloaded_artifacts = self.download_artifacts(ui, &expanded_idents).await?;

        Ok(downloaded_artifacts.len())
    }

    // For each source, use the builder/depot to expand it to a fully qualifed form
    // The same call gives us the TDEPS, add those as well.
    async fn expand_sources<T>(&self, ui: &mut T) -> Result<HashSet<(PackageIdent, PackageTarget)>>
        where T: UIWriter
    {
        let mut expanded_packages = Vec::<(Package, PackageTarget)>::new();
        let mut expanded_idents = HashSet::<(PackageIdent, PackageTarget)>::new();

        // This loop should be easy to convert to a parallel map.
        for package_set in self.package_sets {
            ui.begin(format!("Resolving dependencies for {} package idents",
                             package_set.idents.len()))?;
            ui.begin(format!("Using channel {} from {}", package_set.channel, self.url))?;
            ui.begin(format!("Using target {}", package_set.target))?;

            for ident in &package_set.idents {
                if let Some(package) = self.determine_latest_from_ident(ui,
                                                                        &package_set.channel,
                                                                        package_set.target,
                                                                        &ident)
                                           .await?
                {
                    expanded_packages.push((package, package_set.target));
                }
            }
        }

        // Collect all the expanded deps into one structure
        // Done separately because it's not as easy to parallelize
        for (package, target) in expanded_packages {
            for ident in package.tdeps {
                expanded_idents.insert((ident.clone(), target));
            }
            expanded_idents.insert((package.ident.clone(), target));
        }

        ui.status(Status::Found,
                  format!("{} artifacts", expanded_idents.len()))?;

        Ok(expanded_idents)
    }

    async fn download_artifacts<T>(&self,
                                   ui: &mut T,
                                   expanded_idents: &HashSet<(PackageIdent, PackageTarget)>)
                                   -> Result<Vec<PackageArchive>>
        where T: UIWriter
    {
        let mut downloaded_artifacts = Vec::<PackageArchive>::new();

        ui.status(Status::Downloading,
                  format!("Downloading {} artifacts (and their signing keys)",
                          expanded_idents.len()))?;

        for (ident, target) in expanded_idents {
            let archive: PackageArchive =
                match self.get_downloaded_archive(ui, ident, *target).await {
                    Ok(v) => v,
                    Err(e) => {
                        // Is this the right status? Or should this be a debug message?
                        debug!("Error fetching archive {} for {}: {:?}", ident, *target, e);
                        ui.status(Status::Missing,
                                  format!("Error fetching archive {} for {}", ident, *target))?;
                        return Err(e);
                    }
                };

            downloaded_artifacts.push(archive);
        }

        Ok(downloaded_artifacts)
    }

    async fn determine_latest_from_ident<T>(&self,
                                            ui: &mut T,
                                            channel_default: &ChannelIdent,
                                            target: PackageTarget,
                                            ident: &PackageIdent)
                                            -> Result<Option<Package>>
        where T: UIWriter
    {
        // Unlike in the install command, we always hit the online depot; our purpose is to sync
        // with latest, and falling back to a local package would defeat that. Find the
        // latest package in the proper channel from Builder API,

        // We rely on the fact that all packages are always added to the unstable channel as well as
        // any other channel specified.
        let mut channel = &ChannelIdent::unstable();
        if !ident.fully_qualified() {
            channel = channel_default
        };

        ui.status(Status::Determining, format!("latest version of {}", ident))?;
        match self.fetch_latest_package_in_channel_for(ident, target, &channel, self.token)
                  .await
        {
            Ok(latest_package) => {
                ui.status(Status::Using, format!("{}", latest_package.ident))?;
                Ok(Some(latest_package))
            }
            Err(Error::APIClient(APIError(StatusCode::NOT_FOUND, _))) => {
                // In install we attempt to recommend a channel to look in. That's a bit of a
                // heavyweight process, and probably a bad idea in the context of
                // what's a normally a batch process. It might be OK to fall back to
                // the stable channel, but for now, error.
                ui.warn(format!("No packages matching ident {} for {} exist in the '{}' \
                                 channel. Check the package ident, target, channel and Builder \
                                 url ({}) for correctness.",
                                ident, target, &channel, self.url))?;
                if self.ignore_missing_seeds {
                    Ok(None)
                } else {
                    Err(CommonError::PackageNotFound(format!("{} for {} in channel {}",
                                                             ident, target, &channel)).into())
                }
            }
            Err(e) => {
                debug!("Error fetching ident {} for target {}: {:?}",
                       ident, target, e);
                ui.warn(format!("Error fetching ident {} for target {}", ident, target))?;
                Err(e)
            }
        }
    }

    // This function and its sibling get_cached_artifact in
    // install.rs deserve to be refactored to eke out commonality.
    /// This ensures the identified package is in the local download directory,
    /// verifies it, and returns a handle to the package's metadata.
    async fn get_downloaded_archive<T>(&self,
                                       ui: &mut T,
                                       ident: &PackageIdent,
                                       target: PackageTarget)
                                       -> Result<PackageArchive>
        where T: UIWriter
    {
        if self.downloaded_artifact_path(ident, target).is_file() {
            debug!("Found {} in download directory, skipping remote download",
                   ident);
            ui.status(Status::Custom(Glyph::Elipses, String::from("Using cached")),
                      format!("{}", ident))?;
        } else if let Err(err) = retry::retry_future!(delay::Fixed::from(RETRY_WAIT).take(RETRIES),
                                                      self.fetch_artifact(ui, ident, target)).await
        {
            return Err(CommonError::DownloadFailed(format!("We tried {} times but could not \
                                                            download {} for {}. Last error \
                                                            was: {}",
                                                           RETRIES, ident, target, err)).into());
        }

        // At this point the artifact is in the download directory...
        let mut artifact = PackageArchive::new(self.downloaded_artifact_path(ident, target))?;
        self.fetch_keys_and_verify_artifact(ui, ident, target, &mut artifact)
            .await?;
        Ok(artifact)
    }

    // This function and its sibling in install.rs deserve to be refactored to eke out commonality.
    /// Retrieve the identified package from the depot, ensuring that
    /// the artifact is downloaded.
    async fn fetch_artifact<T>(&self,
                               ui: &mut T,
                               ident: &PackageIdent,
                               target: PackageTarget)
                               -> Result<()>
        where T: UIWriter
    {
        ui.status(Status::Downloading, format!("{}", ident))?;
        match self.api_client
                  .fetch_package((ident, target),
                                 self.token,
                                 &self.path_for_artifact(),
                                 ui.progress())
                  .await
        {
            Ok(_) => Ok(()),
            Err(api_client::Error::APIError(StatusCode::NOT_IMPLEMENTED, _)) => {
                println!("Host platform or architecture not supported by the targeted depot; \
                          skipping.");
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn fetch_origin_key<T>(&self,
                                 ui: &mut T,
                                 name_with_rev: &str,
                                 token: Option<&str>)
                                 -> Result<()>
        where T: UIWriter
    {
        let (name, rev) = parse_name_with_rev(&name_with_rev)?;
        self.api_client
            .fetch_origin_key(&name, &rev, token, &self.path_for_keys(), ui.progress())
            .await?;
        Ok(())
    }

    async fn fetch_keys_and_verify_artifact<T>(&self,
                                               ui: &mut T,
                                               ident: &PackageIdent,
                                               target: PackageTarget,
                                               artifact: &mut PackageArchive)
                                               -> Result<()>
        where T: UIWriter
    {
        // We need to look at the artifact to know the signing keys to fetch
        // Once we have them, it's the natural time to verify.
        // Otherwise, it might make sense to take this fetch out of the verification code.
        let signer = artifact::artifact_signer(&artifact.path)?;
        if SigKeyPair::get_public_key_path(&signer, &self.path_for_keys()).is_err() {
            ui.status(Status::Downloading,
                      format!("public key for signer {:?}", signer))?;
            self.fetch_origin_key(ui, &signer, self.token).await?;
        }

        if self.verify {
            ui.status(Status::Verifying, artifact.ident()?)?;
            artifact.verify(&self.path_for_keys())?;
            debug!("Verified {} for {} signed by {}", ident, target, &signer);
        }
        Ok(())
    }

    // This function and its sibling in install.rs deserve to be refactored to eke out commonality.
    /// Returns the path to the location this package would exist at in
    /// the local package cache. It does not mean that the package is
    /// actually *in* the package download directory, though.
    fn downloaded_artifact_path(&self, ident: &PackageIdent, target: PackageTarget) -> PathBuf {
        self.path_for_artifact()
            .join(ident.archive_name_with_target(target).unwrap())
    }

    async fn fetch_latest_package_in_channel_for(&self,
                                                 ident: &PackageIdent,
                                                 target: PackageTarget,
                                                 channel: &ChannelIdent,
                                                 token: Option<&str>)
                                                 -> Result<Package> {
        self.api_client
            .show_package_metadata((&ident, target), channel, token)
            .await
            .map_err(Error::from)
    }

    /// The cache_*_path functions in fs don't let you override a path base with Some(base)
    /// So we have to build our own paths.
    fn path_for_keys(&self) -> PathBuf { self.download_path.join("keys") }

    fn path_for_artifact(&self) -> PathBuf { self.download_path.join("artifacts") }

    /// Sanity check the download directory tree. The errors from the api around permissions are
    /// opaque; this validates the directory in advance to help provide useful feedback.
    fn verify_and_prepare_download_directory<T>(&self, ui: &mut T) -> Result<()>
        where T: UIWriter
    {
        let system_paths = [self.download_path,
                            &self.path_for_keys(),
                            &self.path_for_artifact()];

        ui.status(Status::Verifying,
                  format!("the download directory \"{}\"",
                          self.download_path.display()))?;

        let mut builder = DirBuilder::new();
        builder.recursive(true);

        // Create directories if they don't exist
        for dir in &system_paths {
            builder.create(dir).map_err(|_| {
                                    mk_perm_error(format!("Can't create directory {:?} needed \
                                                           for download",
                                                          dir))
                                })?
        }

        // Check permissions of directories:
        for dir in &system_paths {
            let metadata = std::fs::metadata(dir)?;
            if !metadata.is_dir() {
                return Err(mk_perm_error(format!("{} isn't a directory, needed for \
                                                  download",
                                                 dir.display())));
            }
            if metadata.permissions().readonly() {
                return Err(mk_perm_error(format!("{} isn't writeable, needed for \
                                                  download",
                                                 dir.display())));
            }
        }
        Ok(())
    }
}

fn mk_perm_error(msg: String) -> Error { CoreError::PermissionFailed(msg).into() }
