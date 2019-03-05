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

use super::super::key::download::download_public_encryption_key;
use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI}};

use crate::{error::{Error,
                    Result},
            hcore::crypto::BoxKeyPair,
            PRODUCT,
            VERSION};
use std::path::Path;

pub fn start(ui: &mut UI,
             bldr_url: &str,
             token: &str,
             origin: &str,
             key: &str,
             secret: &str,
             cache: &Path)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    let encryption_key = match BoxKeyPair::get_latest_pair_for(origin, cache) {
        Ok(key) => key,
        Err(_) => {
            debug!("Didn't find public encryption key in cache path");
            download_public_encryption_key(ui, &api_client, origin, token, cache)?;
            BoxKeyPair::get_latest_pair_for(origin, cache)?
        }
    };

    ui.status(Status::Encrypting, format!("value for key {}.", key))?;
    let encrypted_secret_string = encryption_key.encrypt(secret.as_bytes(), None)?;
    ui.status(Status::Encrypted, format!("{}=[REDACTED].", key))?;

    ui.status(Status::Uploading, format!("secret for key {}.", key))?;

    api_client.create_origin_secret(origin, token, key, &encrypted_secret_string)
              .map_err(Error::APIClient)?;

    ui.status(Status::Uploaded, format!("secret for {}.", key))?;

    Ok(())
}
