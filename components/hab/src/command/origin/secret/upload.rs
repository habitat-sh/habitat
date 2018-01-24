// Copyright (c) 2016-2018 Chef Software Inc. and/or applicable contributors
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

use depot_client::Client as DepotClient;
use common::ui::{Status, UI, UIWriter};
use super::super::key::download::download_public_encryption_key;

use hcore::crypto::BoxKeyPair;
use {PRODUCT, VERSION};
use error::{Error, Result};
use std::path::Path;

pub fn start(
    ui: &mut UI,
    bldr_url: &str,
    token: &str,
    origin: &str,
    key: &str,
    secret: &str,
    cache: &Path,
) -> Result<()> {
    let depot_client = DepotClient::new(bldr_url, PRODUCT, VERSION, None).map_err(
        Error::DepotClient,
    )?;

    let encryption_key = match BoxKeyPair::get_latest_pair_for(origin, cache) {
        Ok(key) => key,
        Err(_) => {
            debug!("Didn't find public encryption key in cache path");
            download_public_encryption_key(ui, &depot_client, origin, token, cache)?;
            BoxKeyPair::get_latest_pair_for(origin, cache)?
        }
    };

    ui.status(
        Status::Encrypting,
        format!("value for key {}.", key),
    )?;
    let encrypted_secret_bytes = encryption_key.encrypt(secret.as_bytes(), None)?;
    let encrypted_secret_string = match String::from_utf8(encrypted_secret_bytes) {
        Ok(string_from_bytes) => string_from_bytes,
        Err(_) => {
            return Err(Error::ArgumentError(
                "Failed to convert encrypted bytes to string",
            ))
        }
    };
    ui.status(Status::Encrypted, format!("{}=[REDACTED].", key))?;

    ui.status(
        Status::Uploading,
        format!("secret for key {}.", key),
    )?;

    depot_client
        .create_origin_secret(origin, token, key, &encrypted_secret_string)
        .map_err(Error::DepotClient)?;

    ui.status(Status::Uploaded, format!("secret for {}.", key))?;

    Ok(())
}
