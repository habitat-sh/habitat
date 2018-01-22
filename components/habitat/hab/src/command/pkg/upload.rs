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
//!

// Standard Library
use std::path::{Path, PathBuf};

// External Libraries
use hyper::status::StatusCode;
use retry::retry;

// Local Dependencies
use common::command::package::install::{RETRIES, RETRY_WAIT};
use common::ui::{Status, UI};
use depot_client::{self, Client};
use error::{Error, Result};
use hcore::channel::{STABLE_CHANNEL, UNSTABLE_CHANNEL};
use hcore::crypto::artifact::get_artifact_header;
use hcore::crypto::keys::parse_name_with_rev;
use hcore::package::{PackageArchive, PackageIdent, PackageTarget};
use {PRODUCT, VERSION};

/// Upload a package from the cache to a Depot. The latest version/release of the package
/// will be uploaded if not specified.
///
/// # Failures
///
/// * Fails if it cannot find a package
/// * Fails if the package doesn't have a `.hart` file in the cache
/// * Fails if it cannot upload the file
pub fn start<T, U>(
    ui: &mut UI,
    url: &str,
    additional_release_channel: Option<&str>,
    token: &str,
    archive_path: T,
    key_path: U,
) -> Result<()>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    let mut archive = PackageArchive::new(PathBuf::from(archive_path.as_ref()));
    let hart_header = get_artifact_header(&archive_path.as_ref())?;
    let public_keyfile_name = format!("{}.pub", &hart_header.key_name);
    let public_keyfile = key_path.as_ref().join(&public_keyfile_name);

    ui.status(
        Status::Signed,
        format!("artifact with {}", &public_keyfile_name),
    )?;

    let (name, rev) = parse_name_with_rev(&hart_header.key_name)?;
    let depot_client = Client::new(url, PRODUCT, VERSION, None)?;

    ui.begin(format!(
        "Uploading public origin key {}",
        &public_keyfile_name
    ))?;

    match depot_client.put_origin_key(&name, &rev, &public_keyfile, token, ui.progress()) {
        Ok(()) => {
            ui.status(Status::Uploaded,
                      format!("public origin key {}", &public_keyfile_name))?;
        }
        Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => {
            ui.status(
                Status::Using,
                format!(
                    "existing public origin key {}",
                    &public_keyfile_name
                ),
            )?;
        }
        Err(err) => return Err(Error::from(err)),
    };

    ui.begin(
        format!("Uploading {}", archive_path.as_ref().display()),
    )?;

    let tdeps = archive.tdeps()?;
    let ident = archive.ident()?;
    let target = archive.target()?;
    match depot_client.show_package(&ident, None, Some(token)) {
        Ok(_) => {
            ui.status(Status::Using, format!("existing {}", &ident))?;
            Ok(())
        }
        Err(depot_client::Error::APIError(StatusCode::NotFound, _)) => {
            for dep in tdeps.into_iter() {
                match depot_client.show_package(&dep, None, Some(token)) {
                    Ok(_) => ui.status(Status::Using, format!("existing {}", &dep))?,
                    Err(depot_client::Error::APIError(StatusCode::NotFound, _)) => {
                        let candidate_path = match archive_path.as_ref().parent() {
                            Some(p) => PathBuf::from(p),
                            None => unreachable!(),
                        };
                        if retry(
                            RETRIES,
                            RETRY_WAIT,
                            || {
                                attempt_upload_dep(
                                    ui,
                                    &depot_client,
                                    token,
                                    &dep,
                                    Some(&target),
                                    additional_release_channel,
                                    &candidate_path,
                                )
                            },
                            |res| res.is_ok(),
                        ).is_err()
                        {
                            return Err(Error::from(depot_client::Error::UploadFailed(format!(
                                "We tried \
                                                                                      {} times \
                                                                                      but could \
                                                                                      not upload \
                                                                                      {}. Giving \
                                                                                      up.",
                                RETRIES,
                                &dep
                            ))));
                        }
                    }
                    Err(e) => return Err(Error::from(e)),
                }
            }

            if retry(
                RETRIES,
                RETRY_WAIT,
                || {
                    upload_into_depot(
                        ui,
                        &depot_client,
                        token,
                        &ident,
                        additional_release_channel,
                        &mut archive,
                    )
                },
                |res| res.is_ok(),
            ).is_err()
            {
                return Err(Error::from(depot_client::Error::UploadFailed(format!(
                    "We tried {} times but could not upload {}. Giving up.",
                    RETRIES,
                    &ident
                ))));
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
fn upload_into_depot(
    ui: &mut UI,
    depot_client: &Client,
    token: &str,
    ident: &PackageIdent,
    additional_release_channel: Option<&str>,
    mut archive: &mut PackageArchive,
) -> Result<()> {
    ui.status(Status::Uploading, archive.path.display())?;
    let package_uploaded = match depot_client.put_package(&mut archive, token, ui.progress()) {
        Ok(_) => true,
        Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => {
            println!("Package already exists on remote; skipping.");
            true
        }
        Err(depot_client::Error::APIError(StatusCode::UnprocessableEntity, _)) => {
            return Err(Error::PackageArchiveMalformed(
                format!("{}", archive.path.display()),
            ));
        }
        Err(depot_client::Error::APIError(StatusCode::NotImplemented, _)) => {
            println!(
                "Package platform or architecture not supported by the targeted \
                    depot; skipping."
            );
            false
        }
        Err(depot_client::Error::APIError(StatusCode::FailedDependency, _)) => {
            ui.fatal(
                "Package upload introduces a circular dependency - please check \
                    pkg_deps; skipping.",
            )?;
            false
        }
        Err(e) => return Err(Error::from(e)),
    };
    ui.status(Status::Uploaded, ident)?;

    // Promote to additional_release_channel if specified
    if package_uploaded && additional_release_channel.is_some() {
        let channel_str = additional_release_channel.unwrap();
        ui.begin(format!(
            "Promoting {} to channel '{}'",
            ident,
            channel_str
        ))?;

        if channel_str != STABLE_CHANNEL && channel_str != UNSTABLE_CHANNEL {
            match depot_client.create_channel(&ident.origin, channel_str, token) {
                Ok(_) => (),
                Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => (),
                Err(e) => return Err(Error::from(e)),
            };
        }

        match depot_client.promote_package(ident, channel_str, token) {
            Ok(_) => (),
            Err(e) => return Err(Error::from(e)),
        };
        ui.status(Status::Promoted, ident)?;
    }

    Ok(())
}

fn attempt_upload_dep(
    ui: &mut UI,
    depot_client: &Client,
    token: &str,
    ident: &PackageIdent,
    target: Option<&PackageTarget>,
    additional_release_channel: Option<&str>,
    archives_dir: &PathBuf,
) -> Result<()> {
    let candidate_path = match target {
        Some(t) => archives_dir.join(ident.archive_name_with_target(t).unwrap()),
        None => archives_dir.join(ident.archive_name().unwrap()),
    };
    if candidate_path.is_file() {
        let mut archive = PackageArchive::new(candidate_path);
        upload_into_depot(
            ui,
            &depot_client,
            token,
            &ident,
            additional_release_channel,
            &mut archive,
        )
    } else {
        ui.status(
            Status::Missing,
            format!(
                "artifact for {} was not found in {}",
                ident.archive_name().unwrap(),
                archives_dir.display()
            ),
        )?;
        Err(Error::FileNotFound(
            archives_dir.to_string_lossy().into_owned(),
        ))
    }
}
