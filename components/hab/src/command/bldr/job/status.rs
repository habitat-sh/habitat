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

use std::io::Write;

use crate::{api_client,
            common::ui::{Status,
                         UIWriter,
                         UI}};
use tabwriter::TabWriter;

use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};

pub fn start(
    ui: &mut UI,
    bldr_url: &str,
    group_id: Option<&str>,
    origin: Option<&str>,
    limit: usize,
    show_jobs: bool,
) -> Result<()> {
    let api_client =
        api_client::Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    if origin.is_some() {
        do_origin_status(ui, &api_client, origin.unwrap(), limit)?;
    } else {
        do_job_group_status(ui, &api_client, group_id.unwrap(), show_jobs)?;
    }

    Ok(())
}

fn do_job_group_status(
    ui: &mut UI,
    api_client: &api_client::Client,
    group_id: &str,
    show_jobs: bool,
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
        format!("status of job group {}", group_id),
    )?;

    match api_client.get_schedule(gid, show_jobs) {
        Ok(sr) => {
            let mut tw = TabWriter::new(vec![]);
            writeln!(&mut tw, "CREATED AT\tGROUP ID\tSTATUS\tIDENT\tTARGET").unwrap();
            writeln!(
                &mut tw,
                "{}\t{}\t{}\t{}\t{}",
                sr.created_at, sr.id, sr.state, sr.project_name, sr.target
            )
            .unwrap();
            tw.flush().unwrap();
            let mut written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
            println!("\n{}", written);

            if show_jobs && !sr.projects.is_empty() {
                tw = TabWriter::new(vec![]);
                writeln!(&mut tw, "NAME\tSTATUS\tJOB ID\tIDENT\tTARGET").unwrap();
                for p in sr.projects {
                    // Don't show ident if the build did not succeed
                    // TODO: This will be fixed at the API level eventually
                    let ident = if p.state == "Success" { &p.ident } else { "" };
                    writeln!(
                        &mut tw,
                        "{}\t{}\t{}\t{}\t{}",
                        p.name, p.state, p.job_id, ident, p.target
                    )
                    .unwrap();
                }
                written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
                println!("{}", written);
            }
            Ok(())
        }
        Err(e) => Err(Error::ScheduleStatus(e)),
    }
}

fn do_origin_status(
    ui: &mut UI,
    api_client: &api_client::Client,
    origin: &str,
    limit: usize,
) -> Result<()> {
    ui.status(
        Status::Determining,
        format!("status of job groups in {} origin", origin),
    )?;

    match api_client.get_origin_schedule(origin, limit) {
        Ok(sr) => {
            let mut tw = TabWriter::new(vec![]);
            writeln!(&mut tw, "CREATED AT\tGROUP ID\tSTATUS\tIDENT\tTARGET").unwrap();
            for s in sr.iter() {
                writeln!(
                    &mut tw,
                    "{}\t{}\t{}\t{}\t{}",
                    s.created_at, s.id, s.state, s.project_name, s.target,
                )
                .unwrap();
            }

            tw.flush().unwrap();
            let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
            println!("\n{}", written);
            Ok(())
        }
        Err(e) => Err(Error::ScheduleStatus(e)),
    }
}
