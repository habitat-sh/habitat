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
    bldr_url: &str,
    ident: &PackageIdent,
    token: &str,
    group: bool,
) -> Result<()> {
    let api_client = ApiClient::new(bldr_url, PRODUCT, VERSION, None).map_err(
        Error::APIClient,
    )?;

    let depot_client = DepotClient::new(bldr_url, PRODUCT, VERSION, None).map_err(
        Error::DepotClient,
    )?;

    if group {
        let rdeps = api_client.fetch_rdeps(ident).map_err(Error::APIClient)?;
        if rdeps.len() > 0 {
            ui.warn("Found the following reverse dependencies:")?;

            for rdep in rdeps {
                ui.warn(format!("{}", rdep))?;
            }

            let question = "If you choose to start a group build for this package, \
            all of the above will be built as well. Is this what you want?";

            if !ui.prompt_yes_no(question, Some(true))? {
                ui.fatal("Aborted")?;
                return Ok(());
            }
        }
    }

    ui.status(
        Status::Creating,
        format!("build job for {}", ident),
    )?;

    let id = depot_client.schedule_job(ident, !group, token).map_err(
        Error::DepotClient,
    )?;

    ui.status(
        Status::Created,
        format!("build job. The id is {}", id),
    )?;

    Ok(())
}
