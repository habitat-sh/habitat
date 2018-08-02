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

use hcore::package::PackageIdent;
use hyper::status::StatusCode;
use std::str::FromStr;

use api_client;
use common::ui::{Status, UIReader, UIWriter, UI};
use depot_client::{self, SchedulerResponse};

use error::{Error, Result};
use {PRODUCT, VERSION};

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
    group_status: &SchedulerResponse,
    origin: Option<&str>,
    interactive: bool,
) -> Result<Vec<String>> {
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

    idents = idents.iter().map(|s| format!("{}\n", s)).collect();

    Ok(ui
        .edit(&idents)?
        .split("\n")
        .filter(|s| is_ident(s))
        .map(|s: &str| s.to_string())
        .collect())
}

fn get_group_status(bldr_url: &str, group_id: u64) -> Result<SchedulerResponse> {
    let depot_client =
        depot_client::Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::DepotClient)?;

    let group_status = depot_client
        .get_schedule(group_id as i64, true)
        .map_err(|e| Error::ScheduleStatus(e))?;

    Ok(group_status)
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
    promote: bool,
) -> Result<()> {
    let api_client =
        api_client::Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;
    let (promoted_demoted, promoting_demoting, to_from, changing_status, changed_status) =
        if promote {
            (
                "promoted",
                "Promoting",
                "to",
                Status::Promoting,
                Status::Promoted,
            )
        } else {
            (
                "demoted",
                "Demoting",
                "from",
                Status::Demoting,
                Status::Demoted,
            )
        };

    let gid = match group_id.parse::<u64>() {
        Ok(g) => g,
        Err(e) => {
            ui.fatal(format!("Failed to parse group id: {}", e))?;
            return Err(Error::ParseIntError(e));
        }
    };

    let group_status = get_group_status(bldr_url, gid)?;
    let idents = get_ident_list(ui, &group_status, origin, interactive)?;

    if idents.len() == 0 {
        ui.warn("No matching packages found")?;
        return Ok(());
    }

    if verbose {
        println!("Packages being {}:", promoted_demoted);
        for ident in idents.iter() {
            println!("  {}", ident)
        }
    }

    let question = format!(
        "{} {} package(s) to channel '{}'. Continue?",
        promoting_demoting,
        idents.len(),
        channel
    );

    if !ui.prompt_yes_no(&question, Some(true))? {
        ui.fatal("Aborted")?;
        return Ok(());
    }

    ui.status(
        changing_status,
        format!("job group {} {} channel '{}'", group_id, to_from, channel),
    )?;

    match api_client.job_group_promote_or_demote(gid, &idents, channel, token, promote) {
        Ok(_) => {
            ui.status(
                changed_status,
                format!("job group {} {} channel '{}'", group_id, to_from, channel),
            )?;
        }
        Err(api_client::Error::APIError(StatusCode::UnprocessableEntity, _)) => {
            return Err(Error::JobGroupPromoteOrDemoteUnprocessable(promote));
        }
        Err(e) => {
            return Err(Error::JobGroupPromoteOrDemote(e, promote));
        }
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;
    use std::io::{self, Cursor, Write};
    use std::sync::{Arc, RwLock};

    use super::get_ident_list;
    use common::ui::{Coloring, UI};
    use depot_client::{Project, SchedulerResponse};

    fn sample_project_list() -> Vec<Project> {
        let project1 = Project {
            name: "Project1".to_string(),
            ident: "core/project1/1.0.0/20180101000000".to_string(),
            state: "Success".to_string(),
            job_id: "12345678".to_string(),
        };
        let project2 = Project {
            name: "Project2".to_string(),
            ident: "core/project2/1.0.0/20180101000000".to_string(),
            state: "Success".to_string(),
            job_id: "12345678".to_string(),
        };

        vec![project1, project2]
    }

    fn ui() -> (UI, OutputBuffer, OutputBuffer) {
        let stdout_buf = OutputBuffer::new();
        let stderr_buf = OutputBuffer::new();

        let ui = UI::with_streams(
            Box::new(io::empty()),
            || Box::new(stdout_buf.clone()),
            || Box::new(stderr_buf.clone()),
            Coloring::Never,
            false,
        );

        (ui, stdout_buf, stderr_buf)
    }

    #[derive(Clone)]
    pub struct OutputBuffer {
        pub cursor: Arc<RwLock<Cursor<Vec<u8>>>>,
    }

    impl OutputBuffer {
        fn new() -> Self {
            OutputBuffer {
                cursor: Arc::new(RwLock::new(Cursor::new(Vec::new()))),
            }
        }
    }

    impl Write for OutputBuffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.cursor
                .write()
                .expect("Cursor lock is poisoned")
                .write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.cursor
                .write()
                .expect("Cursor lock is poisoned")
                .flush()
        }
    }

    #[test]
    fn test_get_ident_list() {
        let (mut ui, _stdout, _stderr) = ui();
        let group_status = SchedulerResponse {
            id: "12345678".to_string(),
            state: "Finished".to_string(),
            projects: sample_project_list(),
            created_at: "Properly formated timestamp".to_string(),
            project_name: "Test Project".to_string(),
        };

        let ident_list = get_ident_list(&mut ui, &group_status, Some("core"), false)
            .expect("Error fetching ident list");

        assert_eq!(
            ident_list,
            [
                "core/project1/1.0.0/20180101000000",
                "core/project2/1.0.0/20180101000000",
            ]
        )
    }

    #[test]
    fn test_get_ident_list_interactive() {
        let (mut ui, _stdout, _stderr) = ui();
        let group_status = SchedulerResponse {
            id: "12345678".to_string(),
            state: "Finished".to_string(),
            projects: sample_project_list(),
            created_at: "Properly formated timestamp".to_string(),
            project_name: "Test Project".to_string(),
        };
        env::set_var("EDITOR", "cat");

        let ident_list = get_ident_list(&mut ui, &group_status, Some("core"), true)
            .expect("Error fetching ident list");

        assert_eq!(
            ident_list,
            [
                "core/project1/1.0.0/20180101000000",
                "core/project2/1.0.0/20180101000000",
            ]
        )
    }
}
