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

use std::io::{stdin, stdout, Write};

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
    debug!("Starting a job for {}", ident);

    let api_client = ApiClient::new(depot_url, PRODUCT, VERSION, None).map_err(
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
        print!(
            "If you choose to start a group for this package, all of the above packages will be built as well. Is this what you want? Y/N "
        );

        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect(
            "Did not enter a correct string",
        );
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        println!("");

        if s.to_lowercase() == "y" {
            let depot_client = DepotClient::new(depot_url, PRODUCT, VERSION, None)
                .map_err(Error::DepotClient)?;
            depot_client.schedule_job(ident, token).map_err(
                Error::DepotClient,
            )?;
        } else {
            println!("Aborted.");
        }
    } else {
        api_client.create_job(ident, token).map_err(
            Error::APIClient,
        )?;
        ui.status(Status::Creating, format!("job for {}", ident))?;
    }
    Ok(())
}
