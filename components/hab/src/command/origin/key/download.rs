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

use std::path::Path;

use common;
use common::command::package::install::{RETRIES, RETRY_WAIT};
use common::ui::{Status, UIWriter, UI};
use depot_client::Client;
use hcore::crypto::SigKeyPair;

use error::{Error, Result};
use {PRODUCT, VERSION};

use retry::retry;

pub fn start(
    ui: &mut UI,
    depot: &str,
    origin: &str,
    revision: Option<&str>,
    secret: bool,
    encryption: bool,
    token: Option<&str>,
    cache: &Path,
) -> Result<()> {
    let depot_client = Client::new(depot, PRODUCT, VERSION, None)?;

    if secret {
        handle_secret(ui, &depot_client, origin, token, cache)
    } else if encryption {
        handle_encryption(ui, &depot_client, origin, token, cache)
    } else {
        handle_public(ui, &depot_client, origin, revision, cache)
    }
}

fn handle_public(
    ui: &mut UI,
    depot_client: &Client,
    origin: &str,
    revision: Option<&str>,
    cache: &Path,
) -> Result<()> {
    match revision {
        Some(revision) => {
            let nwr = format!("{}-{}", origin, revision);
            ui.begin(format!("Downloading public origin key {}", &nwr))?;
            match download_key(ui, depot_client, &nwr, origin, revision, cache) {
                Ok(()) => {
                    let msg = format!("Download of {} public origin key completed.", nwr);
                    ui.end(msg)?;
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        None => {
            ui.begin(format!("Downloading public origin keys for {}", origin))?;
            match depot_client.show_origin_keys(origin) {
                Ok(ref keys) if keys.len() == 0 => {
                    ui.end(format!("No public keys for {}.", origin))?;
                    Ok(())
                }
                Ok(keys) => {
                    for key in keys {
                        let nwr = format!("{}-{}", key.origin, key.revision);
                        download_key(ui, depot_client, &nwr, &key.origin, &key.revision, cache)?;
                    }
                    ui.end(format!(
                        "Download of {} public origin keys completed.",
                        &origin
                    ))?;
                    Ok(())
                }
                Err(e) => Err(Error::from(e)),
            }
        }
    }
}

fn handle_secret(
    ui: &mut UI,
    depot_client: &Client,
    origin: &str,
    token: Option<&str>,
    cache: &Path,
) -> Result<()> {
    if token.is_none() {
        ui.end(format!(
            "No auth token found. You must pass a token to download secret keys."
        ))?;
        return Ok(());
    }

    ui.begin(format!("Downloading secret origin keys for {}", origin))?;
    download_secret_key(ui, &depot_client, origin, token.unwrap(), cache)?; // unwrap is safe because we already checked it above
    ui.end(format!(
        "Download of {} public origin keys completed.",
        &origin
    ))?;
    Ok(())
}

fn handle_encryption(
    ui: &mut UI,
    depot_client: &Client,
    origin: &str,
    token: Option<&str>,
    cache: &Path,
) -> Result<()> {
    if token.is_none() {
        ui.end(format!(
            "No auth token found. You must pass a token to download secret keys."
        ))?;
        return Ok(());
    }

    ui.begin(format!(
        "Downloading public encryption origin key for {}",
        origin
    ))?;
    download_public_encryption_key(ui, &depot_client, origin, token.unwrap(), cache)?; // unwrap is safe because we already checked it above
    ui.end(format!(
        "Download of {} public encryption keys completed.",
        &origin
    ))?;
    Ok(())
}

pub fn download_public_encryption_key(
    ui: &mut UI,
    depot_client: &Client,
    name: &str,
    token: &str,
    cache: &Path,
) -> Result<()> {
    let download_fn = || -> Result<()> {
        ui.status(Status::Downloading, "latest public encryption key")?;
        let key_path =
            depot_client.fetch_origin_public_encryption_key(name, token, cache, ui.progress())?;
        ui.status(
            Status::Cached,
            key_path.file_name().unwrap().to_str().unwrap(), // lol
        )?;
        Ok(())
    };

    if retry(RETRIES, RETRY_WAIT, download_fn, |res| res.is_ok()).is_err() {
        return Err(Error::from(common::error::Error::DownloadFailed(format!(
            "We tried {} \
             times but \
             could not \
             download the latest \
             public encryption key. \
             Giving up.",
            RETRIES,
        ))));
    }

    Ok(())
}

fn download_secret_key(
    ui: &mut UI,
    depot_client: &Client,
    name: &str,
    token: &str,
    cache: &Path,
) -> Result<()> {
    let download_fn = || -> Result<()> {
        ui.status(Status::Downloading, "latest secret key")?;
        let key_path = depot_client.fetch_secret_origin_key(name, token, cache, ui.progress())?;
        ui.status(
            Status::Cached,
            key_path.file_name().unwrap().to_str().unwrap(), // lol
        )?;
        Ok(())
    };

    if retry(RETRIES, RETRY_WAIT, download_fn, |res| res.is_ok()).is_err() {
        return Err(Error::from(common::error::Error::DownloadFailed(format!(
            "We tried {} \
             times but \
             could not \
             download the latest \
             secret origin key. \
             Giving up.",
            RETRIES,
        ))));
    }

    Ok(())
}

fn download_key(
    ui: &mut UI,
    depot_client: &Client,
    nwr: &str,
    name: &str,
    rev: &str,
    cache: &Path,
) -> Result<()> {
    match SigKeyPair::get_public_key_path(&nwr, &cache) {
        Ok(_) => ui.status(Status::Using, &nwr)?,
        Err(_) => {
            let download_fn = || -> Result<()> {
                ui.status(Status::Downloading, &nwr)?;
                depot_client.fetch_origin_key(name, rev, cache, ui.progress())?;
                ui.status(Status::Cached, &nwr)?;
                Ok(())
            };

            if retry(RETRIES, RETRY_WAIT, download_fn, |res| res.is_ok()).is_err() {
                return Err(Error::from(common::error::Error::DownloadFailed(format!(
                    "We tried {} \
                     times but \
                     could not \
                     download {}/{} \
                     origin key. \
                     Giving up.",
                    RETRIES, &name, &rev
                ))));
            }
        }
    }
    Ok(())
}
