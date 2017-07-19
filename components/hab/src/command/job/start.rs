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

use api_client::Client as ApiClient;
use depot_client::Client as DepotClient;
use common::ui::{Status, UI};
use hcore::package::PackageIdent;

use {PRODUCT, VERSION};
use error::{Error, Result};

pub fn start(
    ui: &mut UI,
    depot_url: &str,
    ident: &PackageIdent,
    token: &str,
    group: bool,
) -> Result<()> {
    debug!("Starting a job for {}", &ident);

    if group {
        let depot_client = DepotClient::new(depot_url, PRODUCT, VERSION, None)
            .map_err(Error::DepotClient)?;
        depot_client.schedule_job(ident, token).map_err(
            Error::DepotClient,
        )?;
    } else {
        let api_client = ApiClient::new(depot_url, PRODUCT, VERSION, None).map_err(
            Error::APIClient,
        )?;
        api_client.create_job(ident, token).map_err(
            Error::APIClient,
        )?;
    }
    ui.status(Status::Creating, format!("job for {}", ident))?;
    Ok(())
}
