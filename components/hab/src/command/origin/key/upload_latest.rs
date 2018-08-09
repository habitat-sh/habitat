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

use std::path::Path;

use api_client::{self, Client};
use common::ui::{Status, UIWriter, UI};
use error::{Error, Result};
use hcore::crypto::keys::parse_name_with_rev;
use hcore::crypto::{SigKeyPair, PUBLIC_SIG_KEY_VERSION, SECRET_SIG_KEY_VERSION};
use hyper::status::StatusCode;

use super::get_name_with_rev;
use {PRODUCT, VERSION};

pub fn start(
    ui: &mut UI,
    bldr_url: &str,
    token: &str,
    origin: &str,
    with_secret: bool,
    cache: &Path,
) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;
    ui.begin(format!("Uploading latest public origin key {}", &origin))?;
    let latest = SigKeyPair::get_latest_pair_for(origin, cache, None)?;
    let public_keyfile = SigKeyPair::get_public_key_path(&latest.name_with_rev(), cache)?;
    let name_with_rev = get_name_with_rev(&public_keyfile, PUBLIC_SIG_KEY_VERSION)?;
    let (name, rev) = parse_name_with_rev(&name_with_rev)?;
    ui.status(Status::Uploading, public_keyfile.display())?;

    match api_client.put_origin_key(&name, &rev, &public_keyfile, token, ui.progress()) {
        Ok(()) => ui.status(Status::Uploaded, &name_with_rev)?,
        Err(api_client::Error::APIError(StatusCode::Conflict, _)) => {
            ui.status(
                Status::Using,
                format!(
                    "public key revision {} which already \
                     exists in the depot",
                    &name_with_rev
                ),
            )?;
        }
        Err(err) => return Err(Error::from(err)),
    }
    ui.end(format!(
        "Upload of public origin key {} complete.",
        &name_with_rev
    ))?;

    if with_secret {
        let secret_keyfile = SigKeyPair::get_secret_key_path(&latest.name_with_rev(), cache)?;

        // we already have this value, but get_name_with_rev will also
        // check the SECRET_SIG_KEY_VERSION
        let name_with_rev = get_name_with_rev(&secret_keyfile, SECRET_SIG_KEY_VERSION)?;
        ui.status(Status::Uploading, secret_keyfile.display())?;
        match api_client.put_origin_secret_key(&name, &rev, &secret_keyfile, token, ui.progress())
        {
            Ok(()) => {
                ui.status(Status::Uploaded, &name_with_rev)?;
                ui.end(format!(
                    "Upload of secret origin key {} complete.",
                    &name_with_rev
                ))?;
            }
            Err(e) => {
                return Err(Error::APIClient(e));
            }
        }
    }
    Ok(())
}
