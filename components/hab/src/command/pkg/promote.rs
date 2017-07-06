// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

//! Promote a package to a specified channel.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg promote acme/redis/2.0.7/2112010203120101 stable
//! ```
//!//! This will promote the acme package specified to the stable channel.
//!
//! Notes:
//!    The package should already have been uploaded to the Depot.
//!    If the specified channel does not exist, it will be created.
//!


use common::ui::{Status, UI};
use depot_client::{self, Client};
use hcore::package::PackageIdent;
use hyper::status::StatusCode;

use {PRODUCT, VERSION};
use error::{Error, Result};


/// Promote a package to the specified channel.
///
/// # Failures
///
/// * Fails if it cannot find the specified package in the Depot
pub fn start(
    ui: &mut UI,
    url: &str,
    ident: &PackageIdent,
    channel: &str,
    token: &str,
) -> Result<()> {

    let depot_client = try!(Client::new(url, PRODUCT, VERSION, None));

    try!(ui.begin(
        format!("Promoting {} to channel '{}'", ident, channel),
    ));

    if channel != "stable" && channel != "unstable" {
        match depot_client.create_channel(&ident.origin, channel, token) {
            Ok(_) => (),
            Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => (),
            Err(e) => {
                println!("Failed to create '{}' channel: {:?}", channel, e);
                return Err(Error::from(e));
            }
        };
    }

    match depot_client.promote_package(ident, channel, token) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to promote '{}': {:?}", ident, e);
            return Err(Error::from(e));
        }
    }

    try!(ui.status(Status::Promoted, ident));

    Ok(())
}
