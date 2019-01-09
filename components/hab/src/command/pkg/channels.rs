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

//! Find out what channels a package belongs to.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg channels acme/redis/2.0.7/2112010203120101
//! ```
//! This will return a list of all the channels that acme/redis/2.0.7/2112010203120101
//! is in.
//!
//! Notes:
//!    The package should already have been uploaded to Builder.
//!    If the specified package does not exist, this will fail.
//!

use crate::api_client::Client;
use crate::common::ui::{UIWriter, UI};
use crate::hcore::package::PackageIdent;

use crate::error::Result;
use crate::{PRODUCT, VERSION};

/// Return a list of channels that a package is in.
///
/// # Failures
///
/// * Fails if it cannot find the specified package in Builder.
pub fn start(ui: &mut UI, bldr_url: &str, ident: &PackageIdent, token: Option<&str>) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    ui.begin(format!("Retrieving channels for {}", ident))?;
    let channels = api_client.package_channels(ident, token)?;
    for channel in &channels {
        println!("{}", channel);
    }

    Ok(())
}
