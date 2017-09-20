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
    debug!("Starting a job for {}", ident);
    let api_client = ApiClient::new(bldr_url, PRODUCT, VERSION, None).map_err(
        Error::APIClient,
    )?;
    if group {
        let rdeps = api_client.fetch_rdeps(ident).map_err(Error::APIClient)?;
        println!("The following are the reverse dependencies for {}:", ident);
        println!("");

        for rdep in rdeps {
            println!("{}", rdep);
        }

        println!("");
        let question = "If you choose to start a group for this package, \
            all of the above packages will be built as well. \
            Is this what you want?";

        let doit = ui.prompt_yes_no(question, Some(true))?;
        println!("");

        if doit {
            ui.status(
                Status::Creating,
                format!("schedule group for {}. Please wait...", ident),
            )?;

            let depot_client = DepotClient::new(bldr_url, PRODUCT, VERSION, None).map_err(
                Error::DepotClient,
            )?;
            let id = depot_client.schedule_job(ident, token).map_err(
                Error::DepotClient,
            )?;

            ui.status(
                Status::Created,
                format!(
                    "schedule group for {}. The group ID is {}.",
                    ident,
                    id
                ),
            )?;
        } else {
            println!("Aborted.");
        }
    } else {
        let id = api_client.create_job(ident, token).map_err(
            Error::APIClient,
        )?;
        ui.status(
            Status::Created,
            format!("job for {}. The job ID is {}.", ident, id),
        )?;
    }
    Ok(())
}
