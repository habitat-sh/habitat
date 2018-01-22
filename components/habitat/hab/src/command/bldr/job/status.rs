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

use depot_client;
use common::ui::{Status, UI};

use {PRODUCT, VERSION};
use error::{Error, Result};

pub fn start(
    ui: &mut UI,
    bldr_url: &str,
    group_id: Option<&str>,
    origin: Option<&str>,
) -> Result<()> {
    let depot_client = depot_client::Client::new(bldr_url, PRODUCT, VERSION, None)
        .map_err(Error::DepotClient)?;

    if origin.is_some() {
        do_origin_status(ui, &depot_client, origin.unwrap())?;
    } else {
        do_job_group_status(ui, &depot_client, group_id.unwrap())?;
    }

    Ok(())
}

fn do_job_group_status(
    ui: &mut UI,
    depot_client: &depot_client::Client,
    group_id: &str,
) -> Result<()> {
    let gid = match group_id.parse::<i64>() {
        Ok(g) => g,
        Err(e) => {
            ui.fatal(format!("Failed to parse group id: {}", e))?;
            return Err(Error::ParseIntError(e));
        }
    };

    ui.status(
        Status::Determining,
        format!("status of Job Group {}", group_id),
    )?;

    match depot_client.get_schedule(gid) {
        Ok(status) => {
            println!("");
            println!("{}", status.to_string());
            Ok(())
        }
        Err(e) => Err(Error::ScheduleStatus(e)),
    }
}

fn do_origin_status(ui: &mut UI, depot_client: &depot_client::Client, origin: &str) -> Result<()> {
    ui.status(
        Status::Determining,
        format!("status of all job groups in {} origin", origin),
    )?;

    match depot_client.get_origin_schedule(origin) {
        Ok(status) => {
            println!("{}", status);
            Ok(())
        }
        Err(e) => Err(Error::ScheduleStatus(e)),
    }
}
