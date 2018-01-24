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

use {PRODUCT, VERSION};
use error::{Error, Result};

pub fn start(ui: &mut UI, bldr_url: &str, token: &str, origin: &str) -> Result<()> {
    let depot_client = DepotClient::new(bldr_url, PRODUCT, VERSION, None).map_err(
        Error::DepotClient,
    )?;

    ui.status(
        Status::Determining,
        format!("secrets for {}.", origin),
    )?;

    match depot_client.list_origin_secrets(origin, token) {
        Ok(secrets) => {
            println!("{}", secrets.join("\n"));
            Ok(())
        }
        Err(e) => Err(Error::DepotClient(e)),
    }
}
