//! Uploads a package to a [Depot](../depot).
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg upload /path/to/acme-redis-2.0.7-2112010203120101-x86_64-linux.hart \
//!     -u http://localhost:9632
//! ```
//!
//! Will upload a package to Builder.
//!
//! # Notes
//!
//! This should be extended to cover uploading specific packages, and finding them by ways more
//! complex than just latest version.

use crate::{api_client::{self,
                         BuildOnUpload,
                         BuilderAPIClient,
                         Client},
            common::{command::package::install::{RETRIES,
                                                 RETRY_WAIT},
                     ui::{Status,
                          UIWriter,
                          UI}},
            error::{Error,
                    Result},
            hcore::{crypto::{artifact::get_artifact_header,
                             keys::parse_name_with_rev},
                    package::{PackageArchive,
                              PackageIdent,
                              PackageTarget},
                    ChannelIdent},
            PRODUCT,
            VERSION};
use reqwest::StatusCode;
use retry::delay;
use std::path::{Path,
                PathBuf};

/// Upload a package from the cache to a Depot. The latest version/release of the package
/// will be uploaded if not specified.
///
/// # Failures
///
/// * Fails if it cannot find a package
/// * Fails if the package doesn't have a `.hart` file in the cache
/// * Fails if it cannot upload the file
#[allow(clippy::too_many_arguments)]
pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   additional_release_channel: &Option<ChannelIdent>,
                   token: &str,
                   archive_path: &Path,
                   force_upload: bool,
                   auto_build: BuildOnUpload,
                   key_path: &Path)
                   -> Result<()> {
    let mut archive = PackageArchive::new(PathBuf::from(archive_path))?;

    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    upload_public_key(ui, token, &api_client, &mut archive, &key_path).await?;

    let tdeps = archive.tdeps()?;
    let ident = archive.ident()?;
    let target = archive.target()?;

    match api_client.check_package((&ident, target), Some(token))
                    .await
    {
        Ok(_) if !force_upload => {
            ui.status(Status::Using, format!("existing {}", &ident))?;
            Ok(())
        }
        Err(api_client::Error::APIError(StatusCode::NOT_FOUND, _)) | Ok(_) => {
            for dep in tdeps.into_iter() {
                match api_client.check_package((&dep, target), Some(token)).await {
                    Ok(_) => ui.status(Status::Using, format!("existing {}", &dep))?,
                    Err(api_client::Error::APIError(StatusCode::NOT_FOUND, _)) => {
                        let candidate_path = match archive_path.parent() {
                            Some(p) => PathBuf::from(p),
                            None => unreachable!(),
                        };
                        match retry::retry_future!(delay::Fixed::from(RETRY_WAIT).take(RETRIES),
                                                   attempt_upload_dep(ui,
                                                                      &api_client,
                                                                      token,
                                                                      (&dep, target),
                                                                      additional_release_channel,
                                                                      &candidate_path,
                                                                      key_path)).await
                        {
                            Ok(_) => trace!("attempt_upload_dep succeeded"),
                            Err(_) => {
                                return Err(Error::from(api_client::Error::UploadFailed(format!(
                                    "We tried {} times but could not upload {}. Giving up.",
                                    RETRIES, &dep
                                ))));
                            }
                        }
                    }
                    Err(e) => return Err(Error::from(e)),
                }
            }

            match retry::retry_future!(delay::Fixed::from(RETRY_WAIT).take(RETRIES),
                                       upload_into_depot(ui,
                                                         &api_client,
                                                         token,
                                                         (&ident, target),
                                                         additional_release_channel,
                                                         force_upload,
                                                         auto_build,
                                                         &mut archive)).await
            {
                Ok(_) => trace!("upload_into_depot succeeded"),
                Err(_) => {
                    return Err(Error::from(api_client::Error::UploadFailed(format!(
                        "We tried {} times but could not upload {}. Giving up.",
                        RETRIES, &ident
                    ))));
                }
            }
            ui.end(format!("Upload of {} complete.", &ident))?;
            Ok(())
        }
        Err(e) => Err(Error::from(e)),
    }
}

/// Uploads a package to the depot. All packages are always
/// automatically put into the `unstable` channel, but if
/// `additional_release_channel` is provided, packages will be
/// promoted to that channel as well.
#[allow(clippy::too_many_arguments)]
async fn upload_into_depot(ui: &mut UI,
                           api_client: &BuilderAPIClient,
                           token: &str,
                           (ident, target): (&PackageIdent, PackageTarget),
                           additional_release_channel: &Option<ChannelIdent>,
                           force_upload: bool,
                           auto_build: BuildOnUpload,
                           mut archive: &mut PackageArchive)
                           -> Result<()> {
    ui.status(Status::Uploading, archive.path.display())?;
    let package_uploaded =
        match api_client.put_package(&mut archive, token, force_upload, auto_build, ui.progress())
                        .await
        {
            Ok(_) => true,
            Err(api_client::Error::APIError(StatusCode::CONFLICT, _)) => {
                println!("Package already exists on remote; skipping.");
                true
            }
            Err(api_client::Error::APIError(StatusCode::UNPROCESSABLE_ENTITY, _)) => {
                return Err(Error::PackageArchiveMalformed(format!("{}",
                                                                  archive.path
                                                                         .display())));
            }
            Err(api_client::Error::APIError(StatusCode::NOT_IMPLEMENTED, _)) => {
                println!("Package platform or architecture not supported by the targeted depot; \
                          skipping.");
                false
            }
            Err(api_client::Error::APIError(StatusCode::FAILED_DEPENDENCY, _)) => {
                ui.fatal("Package upload introduces a circular dependency - please check \
                          pkg_deps; skipping.")?;
                false
            }
            Err(e) => return Err(Error::from(e)),
        };
    ui.status(Status::Uploaded, ident)?;

    // Promote to additional_release_channel if specified
    if package_uploaded && additional_release_channel.is_some() {
        let channel = additional_release_channel.clone().unwrap();
        ui.begin(format!("Promoting {} to channel '{}'", ident, channel))?;

        if channel != ChannelIdent::stable() && channel != ChannelIdent::unstable() {
            match api_client.create_channel(&ident.origin, &channel, token)
                            .await
            {
                Ok(_) => (),
                Err(api_client::Error::APIError(StatusCode::CONFLICT, _)) => (),
                Err(e) => return Err(Error::from(e)),
            };
        }

        match api_client.promote_package((ident, target), &channel, token)
                        .await
        {
            Ok(_) => (),
            Err(e) => return Err(Error::from(e)),
        };
        ui.status(Status::Promoted, ident)?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn attempt_upload_dep(ui: &mut UI,
                            api_client: &BuilderAPIClient,
                            token: &str,
                            (ident, target): (&PackageIdent, PackageTarget),
                            additional_release_channel: &Option<ChannelIdent>,
                            archives_dir: &PathBuf,
                            key_path: &Path)
                            -> Result<()> {
    let candidate_path = archives_dir.join(ident.archive_name_with_target(target).unwrap());

    if candidate_path.is_file() {
        let mut archive = PackageArchive::new(candidate_path)?;
        upload_public_key(ui, &token, api_client, &mut archive, key_path).await?;
        upload_into_depot(ui,
                          api_client,
                          token,
                          (ident, target),
                          additional_release_channel,
                          false,
                          BuildOnUpload::Disable,
                          &mut archive).await
    } else {
        let archive_name = ident.archive_name_with_target(target).unwrap();

        ui.status(Status::Missing,
                  format!("artifact {}. It was not found in {}. Please make sure that all the \
                           dependent artifacts are present in the same directory as the \
                           original artifact that you are attemping to upload.",
                          archive_name,
                          archives_dir.display()))?;
        Err(Error::FileNotFound(archives_dir.to_string_lossy().into_owned()))
    }
}

async fn upload_public_key(ui: &mut UI,
                           token: &str,
                           api_client: &BuilderAPIClient,
                           archive: &mut PackageArchive,
                           key_path: &Path)
                           -> Result<()> {
    let hart_header = get_artifact_header(&archive.path)?;
    let public_keyfile_name = format!("{}.pub", &hart_header.key_name);
    let public_keyfile = key_path.join(&public_keyfile_name);

    let (name, rev) = parse_name_with_rev(&hart_header.key_name)?;

    match api_client.put_origin_key(&name, &rev, &public_keyfile, token, ui.progress())
                    .await
    {
        Ok(()) => {
            ui.begin(format!("Uploading public origin key {}", &public_keyfile_name))?;

            ui.status(Status::Uploaded,
                      format!("public origin key {}", &public_keyfile_name))?;
            Ok(())
        }
        Err(api_client::Error::APIError(StatusCode::CONFLICT, _)) => {
            ui.status(Status::Using,
                      format!("existing public origin key {}", &public_keyfile_name))?;
            Ok(())
        }
        Err(err) => Err(Error::from(err)),
    }
}
