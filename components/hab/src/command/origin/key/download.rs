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

use common::ui::{Status, UI};
use depot_client::{self, Client};
use hcore::crypto::SigKeyPair;
use common::command::package::install::{RETRIES, RETRY_WAIT};

use {PRODUCT, VERSION};
use error::{Error, Result};

use retry::retry;

pub fn start(
    ui: &mut UI,
    depot: &str,
    origin: &str,
    revision: Option<&str>,
    cache: &Path,
) -> Result<()> {
    let depot_client = try!(Client::new(depot, PRODUCT, VERSION, None));
    match revision {
        Some(revision) => {
            let nwr = format!("{}-{}", origin, revision);
            try!(ui.begin(format!("Downloading public origin key {}", &nwr)));
            match download_key(ui, &depot_client, &nwr, origin, revision, cache) {
                Ok(()) => {
                    let msg = format!("Download of {} public origin key completed.", nwr);
                    try!(ui.end(msg));
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        None => {
            try!(ui.begin(
                format!("Downloading public origin keys for {}", origin),
            ));
            match depot_client.show_origin_keys(origin) {
                Ok(ref keys) if keys.len() == 0 => {
                    try!(ui.end(format!("No public keys for {}.", origin)));
                    Ok(())
                }
                Ok(keys) => {
                    for key in keys {
                        let nwr = format!("{}-{}", key.get_origin(), key.get_revision());
                        try!(download_key(
                            ui,
                            &depot_client,
                            &nwr,
                            key.get_origin(),
                            key.get_revision(),
                            cache,
                        ));
                    }
                    try!(ui.end(format!(
                        "Download of {} public origin keys completed.",
                        &origin
                    )));
                    Ok(())
                }
                Err(e) => Err(Error::from(e)),
            }
        }
    }
}

fn download_key(
    ui: &mut UI,
    depot_client: &Client,
    nwr: &str,
    name: &str,
    rev: &str,
    cache: &Path,
) -> Result<()> {
    match SigKeyPair::get_public_key_path(nwr, &cache) {
        Ok(_) => try!(ui.status(Status::Using, &nwr)),
        Err(_) => {
            let download_fn = || -> Result<()> {
                try!(ui.status(Status::Downloading, &nwr));
                try!(depot_client.fetch_origin_key(
                    name,
                    rev,
                    cache,
                    ui.progress(),
                ));
                try!(ui.status(Status::Cached, &nwr));
                Ok(())
            };

            if retry(RETRIES, RETRY_WAIT, download_fn, |res| res.is_ok()).is_err() {
                return Err(Error::from(depot_client::Error::DownloadFailed(format!(
                    "We tried {} \
                                                                                    times but \
                                                                                    could not \
                                                                                    download {}/{} \
                                                                                    origin key. \
                                                                                    Giving up.",
                    RETRIES,
                    &name,
                    &rev
                ))));
            }
        }
    }
    Ok(())
}
