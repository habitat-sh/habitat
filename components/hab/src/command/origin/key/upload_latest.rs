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

use common::ui::{Status, UI};
use depot_client::{self, Client};
use error::{Error, Result};
use hcore::crypto::keys::parse_name_with_rev;
use hcore::crypto::{PUBLIC_SIG_KEY_VERSION, SECRET_SIG_KEY_VERSION, SigKeyPair};
use hyper::status::StatusCode;

use super::get_name_with_rev;
use {PRODUCT, VERSION};

pub fn start(
    ui: &mut UI,
    depot: &str,
    token: &str,
    origin: &str,
    with_secret: bool,
    cache: &Path,
) -> Result<()> {
    let depot_client = try!(Client::new(depot, PRODUCT, VERSION, None));
    try!(ui.begin(
        format!("Uploading latest public origin key {}", &origin),
    ));
    let latest = try!(SigKeyPair::get_latest_pair_for(origin, cache));
    let public_keyfile = try!(SigKeyPair::get_public_key_path(
        &latest.name_with_rev(),
        cache,
    ));
    let name_with_rev = try!(get_name_with_rev(&public_keyfile, PUBLIC_SIG_KEY_VERSION));
    let (name, rev) = try!(parse_name_with_rev(&name_with_rev));
    try!(ui.status(Status::Uploading, public_keyfile.display()));

    match depot_client.put_origin_key(&name, &rev, &public_keyfile, token, ui.progress()) {
        Ok(()) => try!(ui.status(Status::Uploaded, &name_with_rev)),
        Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => {
            try!(ui.status(
                Status::Using,
                format!(
                    "public key revision {} which already \
                                   exists in the depot",
                    &name_with_rev
                ),
            ));
        }
        Err(err) => return Err(Error::from(err)),
    }
    try!(ui.end(format!(
        "Upload of public origin key {} complete.",
        &name_with_rev
    )));

    if with_secret {
        let secret_keyfile = try!(SigKeyPair::get_secret_key_path(
            &latest.name_with_rev(),
            cache,
        ));

        // we already have this value, but get_name_with_rev will also
        // check the SECRET_SIG_KEY_VERSION
        let name_with_rev = try!(get_name_with_rev(&secret_keyfile, SECRET_SIG_KEY_VERSION));
        try!(ui.status(Status::Uploading, secret_keyfile.display()));
        match depot_client.put_origin_secret_key(
            &name,
            &rev,
            &secret_keyfile,
            token,
            ui.progress(),
        ) {
            Ok(()) => {
                try!(ui.status(Status::Uploaded, &name_with_rev));
                try!(ui.end(format!("Upload of secret origin key {} complete.",
                                    &name_with_rev)));
            }
            Err(e) => {
                return Err(Error::DepotClient(e));
            }
        }
    }
    Ok(())
}
