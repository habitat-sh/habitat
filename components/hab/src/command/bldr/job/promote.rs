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

use hyper::status::StatusCode;
use hcore::package::PackageIdent;
use std::str::FromStr;

use api_client;
use depot_client;
use common::ui::{Status, UI};

use {PRODUCT, VERSION};
use error::{Error, Result};

fn is_ident(s: &str) -> bool {
    PackageIdent::from_str(s).is_ok()
}

fn in_origin(ident: &str, origin: Option<&str>) -> bool {
    if origin.is_some() {
        let pi = PackageIdent::from_str(ident).unwrap(); // unwrap Ok
        origin.unwrap() == pi.origin
    } else {
        true
    }
}

pub fn get_ident_list(
    ui: &mut UI,
    bldr_url: &str,
    group_id: u64,
    origin: Option<&str>,
    interactive: bool,
) -> Result<Vec<String>> {
    let depot_client = depot_client::Client::new(bldr_url, PRODUCT, VERSION, None)
        .map_err(Error::DepotClient)?;

    let group_status = depot_client.get_schedule(group_id as i64).map_err(|e| {
        Error::ScheduleStatus(e)
    })?;

    let mut idents: Vec<String> = group_status
        .projects
        .iter()
        .cloned()
        .filter(|p| p.state == "Success" && in_origin(&p.ident, origin))
        .map(|p| p.ident)
        .collect();

    if idents.len() == 0 || !interactive {
        return Ok(idents);
    }

    let prelude = "# This is the list of package identifiers that will be processed.\n\
                   # You may edit this file and remove any packages that you do\n\
                   # not want to apply to the specified channel.\n"
        .to_string();

    idents.insert(0, prelude);

    Ok(
        ui.edit(&idents)?
            .split("\n")
            .filter(|s| is_ident(s))
            .map(|s: &str| s.to_string())
            .collect(),
    )
}

pub fn start(
    ui: &mut UI,
    bldr_url: &str,
    group_id: &str,
    channel: &str,
    origin: Option<&str>,
    interactive: bool,
    verbose: bool,
    token: &str,
) -> Result<()> {
    let api_client = api_client::Client::new(bldr_url, PRODUCT, VERSION, None)
        .map_err(Error::APIClient)?;

    let gid = match group_id.parse::<u64>() {
        Ok(g) => g,
        Err(e) => {
            ui.fatal(format!("Failed to parse group id: {}", e))?;
            return Err(Error::ParseIntError(e));
        }
    };

    let idents = get_ident_list(ui, bldr_url, gid, origin, interactive)?;

    if idents.len() == 0 {
        ui.warn("No matching packages found for promotion")?;
        return Ok(());
    }

    if verbose {
        println!("Packages being promoted:");
        for ident in idents.iter() {
            println!("  {}", ident)
        }
    }

    let question = format!(
        "Promoting {} package(s) to channel '{}'. Continue?",
        idents.len(),
        channel
    );

    if !ui.prompt_yes_no(&question, Some(true))? {
        ui.fatal("Aborted")?;
        return Ok(());
    }

    ui.status(
        Status::Promoting,
        format!("job group {} to channel '{}'", group_id, channel),
    )?;

    match api_client.job_group_promote(gid, &idents, channel, token) {
        Ok(_) => {
            ui.status(
                Status::Promoted,
                format!("job group {} to channel '{}'", group_id, channel),
            )?;
        }
        Err(api_client::Error::APIError(StatusCode::UnprocessableEntity, _)) => {
            return Err(Error::JobGroupPromoteUnprocessable);
        }
        Err(e) => {
            return Err(Error::JobGroupPromote(e));
        }
    };

    Ok(())
}
