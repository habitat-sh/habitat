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

use common::command::package::install::{RETRIES, RETRY_WAIT};
use common::ui::{Status, UIWriter, UI};
use depot_client::{self, Client};
use hcore::crypto::keys::parse_name_with_rev;
use hcore::crypto::{PUBLIC_SIG_KEY_VERSION, SECRET_SIG_KEY_VERSION};
use hyper::status::StatusCode;
use retry::retry;

use super::get_name_with_rev;
use error::{Error, Result};
use {PRODUCT, VERSION};

pub fn start(
    ui: &mut UI,
    depot: &str,
    token: &str,
    public_keyfile: &Path,
    secret_keyfile: Option<&Path>,
) -> Result<()> {
    let depot_client = Client::new(depot, PRODUCT, VERSION, None)?;
    ui.begin(format!(
        "Uploading public origin key {}",
        public_keyfile.display()
    ))?;

    let name_with_rev = get_name_with_rev(&public_keyfile, PUBLIC_SIG_KEY_VERSION)?;
    let (name, rev) = parse_name_with_rev(&name_with_rev)?;

    {
        let upload_fn = || -> Result<()> {
            ui.status(Status::Uploading, public_keyfile.display())?;
            match depot_client.put_origin_key(&name, &rev, public_keyfile, token, ui.progress()) {
                Ok(()) => ui.status(Status::Uploaded, &name_with_rev)?,
                Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => {
                    ui.status(
                        Status::Using,
                        format!(
                            "public key revision {} which already exists in the \
                             depot",
                            &name_with_rev
                        ),
                    )?;
                }
                Err(err) => return Err(Error::from(err)),
            }
            Ok(())
        };

        if retry(RETRIES, RETRY_WAIT, upload_fn, |res| res.is_ok()).is_err() {
            return Err(Error::from(depot_client::Error::UploadFailed(format!(
                "We tried \
                 {} times \
                 but could \
                 not upload \
                 {}/{} public \
                 origin key. \
                 Giving up.\
                 ",
                RETRIES, &name, &rev
            ))));
        }
    }

    ui.end(format!(
        "Upload of public origin key {} complete.",
        &name_with_rev
    ))?;

    if let Some(secret_keyfile) = secret_keyfile {
        let name_with_rev = get_name_with_rev(&secret_keyfile, SECRET_SIG_KEY_VERSION)?;
        let (name, rev) = parse_name_with_rev(&name_with_rev)?;

        let upload_fn = || -> Result<()> {
            ui.status(Status::Uploading, secret_keyfile.display())?;
            match depot_client.put_origin_secret_key(
                &name,
                &rev,
                secret_keyfile,
                token,
                ui.progress(),
            ) {
                Ok(()) => {
                    ui.status(Status::Uploaded, &name_with_rev)?;
                    ui.end(format!(
                        "Upload of secret origin key {} complete.",
                        &name_with_rev
                    ))?;
                    Ok(())
                }
                Err(e) => {
                    return Err(Error::DepotClient(e));
                }
            }
        };

        if retry(RETRIES, RETRY_WAIT, upload_fn, |res| res.is_ok()).is_err() {
            return Err(Error::from(depot_client::Error::UploadFailed(format!(
                "We tried \
                 {} times \
                 but could \
                 not upload \
                 {}/{} secret \
                 origin key. \
                 Giving up.\
                 ",
                RETRIES, &name, &rev
            ))));
        }
    }
    Ok(())
}
