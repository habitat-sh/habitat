// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

use failure::SyncFailure;
use handlebars::Handlebars;
use clap;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use common::ui::{UI, Status};
use export_docker::Result;

// Keep the default operator version in main::cli() in sync with this one
pub const DEFAULT_OPERATOR_VERSION: &'static str = "0.4.0";
pub const OPERATOR_REPO_URL: &'static str = "https://kinvolk.github.io/habitat-operator\
                                             /helm/charts/stable/";

// Helm requirements.yaml template
const DEPSFILE: &'static str = include_str!("../defaults/HelmDeps.hbs");

pub struct Deps {
    operator_version: String,
    update: bool,
}

impl Deps {
    pub fn new_for_cli_matches(matches: &clap::ArgMatches) -> Self {
        Deps {
            operator_version: matches
                .value_of("OPERATOR_VERSION")
                .unwrap_or(DEFAULT_OPERATOR_VERSION)
                .to_owned(),
            update: matches.is_present("DOWNLOAD_DEPS"),
        }
    }

    pub fn generate(&mut self, write: &mut Write) -> Result<()> {
        let out = self.into_string()?;
        write.write(out.as_bytes())?;

        Ok(())
    }

    pub fn download<P: AsRef<Path>>(&self, dir: P, ui: &mut UI) -> Result<()> {
        if !self.update {
            return Ok(());
        }
        ui.status(Status::Downloading, "dependencies")?;

        // Ignore the results cause it's not a problem if this execution fails.
        Command::new("helm")
            .arg("repo")
            .arg("add")
            .arg("habitat-operator")
            .arg(OPERATOR_REPO_URL)
            .spawn()?
            .wait()
            .ok();

        Command::new("helm")
            .arg("dep")
            .arg("up")
            .arg(dir.as_ref().as_os_str())
            .spawn()?
            .wait()
            .map(|_| ())
            .map_err(From::from)
    }

    // TODO: Implement TryInto trait instead when it's in stable std crate
    fn into_string(&self) -> Result<String> {
        let json = json!({
            "operator_version": self.operator_version,
            "operator_repo_url": OPERATOR_REPO_URL,
        });

        let r = Handlebars::new().template_render(DEPSFILE, &json).map_err(
            SyncFailure::new,
        )?;
        let s = r.lines().filter(|l| *l != "").collect::<Vec<_>>().join(
            "\n",
        ) + "\n";

        Ok(s)
    }
}
