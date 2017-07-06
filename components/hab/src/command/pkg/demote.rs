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

//! Demote a package from a specified channel.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg demote acme/redis/2.0.7/2112010203120101 stable
//! ```
//! This will demote the acme package specified from the stable channel, removing it.
//!
//! Notes:
//!    The package should already have been uploaded to the Depot.
//!    If the specified channel does not exist, this will fail.
//!


use common::ui::{Status, UI};
use depot_client::Client;
use hcore::package::PackageIdent;

use {PRODUCT, VERSION};
use error::{Error, Result};


/// Demote a package from the specified channel.
///
/// # Failures
///
/// * Fails if it cannot find the specified package in the Depot.
/// * Fails if the channel specified does not exist.
/// * Fails if "unstable" is the channel specified.
pub fn start(
    ui: &mut UI,
    url: &str,
    ident: &PackageIdent,
    channel: &str,
    token: &str,
) -> Result<()> {

    let depot_client = try!(Client::new(url, PRODUCT, VERSION, None));

    try!(ui.begin(format!("Demoting {} from {}", ident, channel)));

    if channel == "unstable" {
        return Err(Error::CannotRemoveFromChannel(
            (format!("{}", ident), "unstable".to_string()),
        ));
    }

    match depot_client.demote_package(ident, channel, token) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to demote '{}': {:?}", ident, e);
            return Err(Error::from(e));
        }
    }

    try!(ui.status(Status::Demoted, ident));

    Ok(())
}
