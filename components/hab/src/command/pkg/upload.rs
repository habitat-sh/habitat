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
//! Will upload a package to the Depot.
//!
//! # Notes
//!
//! This should be extended to cover uploading specific packages, and finding them by ways more
//! complex than just latest version.
//!

use std::path::{Path, PathBuf};

use common::ui::{Status, UI};
use common::command::package::install::{RETRIES, RETRY_WAIT};
use depot_client::{self, Client};
use hcore::crypto::artifact::get_artifact_header;
use hcore::crypto::keys::parse_name_with_rev;
use hcore::package::{PackageArchive, PackageIdent};
use hyper::status::StatusCode;

use {PRODUCT, VERSION};
use error::{Error, Result};

use retry::retry;

/// Upload a package from the cache to a Depot. The latest version/release of the package
/// will be uploaded if not specified.
///
/// # Failures
///
/// * Fails if it cannot find a package
/// * Fails if the package doesn't have a `.hart` file in the cache
/// * Fails if it cannot upload the file
pub fn start<P: AsRef<Path>>(
    ui: &mut UI,
    url: &str,
    channel: Option<&str>,
    token: &str,
    archive_path: &P,
    key_path: &P,
) -> Result<()> {
    let mut archive = PackageArchive::new(PathBuf::from(archive_path.as_ref()));

    let hart_header = try!(get_artifact_header(&archive_path.as_ref()));

    let key_buf = key_path.as_ref().to_path_buf();
    let public_keyfile_name = format!("{}.pub", &hart_header.key_name);
    let public_keyfile = key_buf.join(&public_keyfile_name);

    try!(ui.status(
        Status::Signed,
        format!("artifact with {}", &public_keyfile_name),
    ));

    let (name, rev) = try!(parse_name_with_rev(&hart_header.key_name));
    let depot_client = try!(Client::new(url, PRODUCT, VERSION, None));

    try!(ui.begin(format!(
        "Uploading public origin key {}",
        &public_keyfile_name
    )));

    match depot_client.put_origin_key(&name, &rev, &public_keyfile, token, ui.progress()) {
        Ok(()) => {
            try!(ui.status(Status::Uploaded,
                           format!("public origin key {}", &public_keyfile_name)));
        }
        Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => {
            try!(ui.status(
                Status::Using,
                format!(
                    "existing public origin key {}",
                    &public_keyfile_name
                ),
            ));
        }
        Err(err) => return Err(Error::from(err)),
    };

    try!(ui.begin(
        format!("Uploading {}", archive_path.as_ref().display()),
    ));
    let tdeps = try!(archive.tdeps());
    let ident = try!(archive.ident());
    match depot_client.show_package(&ident, None) {
        Ok(_) => {
            try!(ui.status(Status::Using, format!("existing {}", &ident)));
            Ok(())
        }
        Err(depot_client::Error::APIError(StatusCode::NotFound, _)) => {
            for dep in tdeps.into_iter() {
                match depot_client.show_package(&dep, None) {
                    Ok(_) => try!(ui.status(Status::Using, format!("existing {}", &dep))),
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
                                    channel,
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

            if retry(RETRIES,
                     RETRY_WAIT,
                     || {
                         upload_into_depot(ui, &depot_client, token, &ident, channel, &mut archive)
                     },
                     |res| res.is_ok())
                       .is_err() {
                return Err(Error::from(depot_client::Error::UploadFailed(format!("We tried \
                                                                                  {} times \
                                                                                  but could \
                                                                                  not upload \
                                                                                  {}. Giving \
                                                                                  up.",
                                                                                 RETRIES,
                                                                                 &ident))));
            }
            try!(ui.end(format!("Upload of {} complete.", &ident)));
            Ok(())
        }
        Err(e) => Err(Error::from(e)),
    }
}

fn upload_into_depot(
    ui: &mut UI,
    depot_client: &Client,
    token: &str,
    ident: &PackageIdent,
    channel: Option<&str>,
    mut archive: &mut PackageArchive,
) -> Result<()> {
    try!(ui.status(Status::Uploading, archive.path.display()));
    let try_promote = match depot_client.put_package(&mut archive, token, ui.progress()) {
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
            try!(ui.fatal(
                "Package upload introduces a circular dependency - please check \
                    pkg_deps; skipping.",
            ));
            false
        }
        Err(e) => return Err(Error::from(e)),
    };
    try!(ui.status(Status::Uploaded, ident));

    // Promote to channel if specified
    if try_promote && channel.is_some() {
        let channel_str = channel.unwrap();
        try!(ui.begin(format!(
            "Promoting {} to channel '{}'",
            ident,
            channel_str
        )));

        if channel_str != "stable" && channel_str != "unstable" {
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
        try!(ui.status(Status::Promoted, ident));
    }

    Ok(())
}

fn attempt_upload_dep(
    ui: &mut UI,
    depot_client: &Client,
    token: &str,
    ident: &PackageIdent,
    channel: Option<&str>,
    archives_dir: &PathBuf,
) -> Result<()> {
    let candidate_path = archives_dir.join(ident.archive_name().unwrap());
    if candidate_path.is_file() {
        let mut archive = PackageArchive::new(candidate_path);
        upload_into_depot(ui, &depot_client, token, &ident, channel, &mut archive)
    } else {
        try!(ui.status(
            Status::Missing,
            format!(
                "artifact for {} was not found in {}",
                ident.archive_name().unwrap(),
                archives_dir.display()
            ),
        ));
        Err(Error::FileNotFound(
            archives_dir.to_string_lossy().into_owned(),
        ))
    }
}
