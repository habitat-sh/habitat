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

use std::str;
use std::path::Path;

use base64;
use hcore::crypto::BoxKeyPair;
use common::ui::UI;
use depot_client::{self, Client};
use common::command::package::install::{RETRIES, RETRY_WAIT};

use {PRODUCT, VERSION};
use error::{Error, Result};

use retry::retry;

pub const BUILDER_KEY_NAME: &'static str = "bldr";

pub fn start(ui: &mut UI, bldr_url: &str, content: &str, cache: &Path) -> Result<()> {
    let depot_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;
    ui.begin("Downloading builder public key")?;

    download_builder_key(ui, &depot_client, cache)?;
    let kp = BoxKeyPair::get_latest_pair_for(BUILDER_KEY_NAME, cache)?;
    let ciphertext = kp.encrypt(content.as_bytes(), None)?;
    let s = base64::encode(&ciphertext);

    ui.end("Encryption complete")?;
    println!("{}", s);

    Ok(())
}

fn download_builder_key(ui: &mut UI, depot_client: &Client, cache: &Path) -> Result<()> {
    let download_fn = || -> Result<()> {
        depot_client.fetch_builder_latest_key(cache, ui.progress())?;
        Ok(())
    };

    if retry(RETRIES, RETRY_WAIT, download_fn, |res| res.is_ok()).is_err() {
        return Err(Error::from(depot_client::Error::DownloadFailed(format!(
            "We tried {} \
                    times but \
                    could not \
                    download the \
                    builder public key. \
                    Giving up.",
            RETRIES
        ))));
    };
    Ok(())
}
