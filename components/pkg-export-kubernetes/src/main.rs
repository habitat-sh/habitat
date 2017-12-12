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

extern crate clap;
extern crate env_logger;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_pkg_export_docker as export_docker;
extern crate handlebars;
extern crate rand;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

extern crate failure;
#[macro_use]
extern crate failure_derive;

mod topology;
mod error;
mod manifest;
mod cli;

use hcore::PROGRAM_NAME;
use common::ui::UI;

use export_docker::Result;

use manifest::Manifest;

// Synced with the version of the Habitat operator.
pub const VERSION: &'static str = "0.1.0";

fn main() {
    env_logger::init().unwrap();
    let mut ui = UI::default_with_env();
    let m = cli().get_matches();
    debug!("clap cli args: {:?}", m);

    if let Err(e) = start(&mut ui, &m) {
        let _ = ui.fatal(e);
        std::process::exit(1)
    }
}

fn start(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    if !matches.is_present("NO_DOCKER_IMAGE") {
        export_docker::export_for_cli_matches(ui, &matches)?;
    }
    let mut manifest = Manifest::new_from_cli_matches(ui, &matches)?;
    manifest.generate()
}

fn cli<'a, 'b>() -> clap::App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates a Docker image and Kubernetes manifest for a Habitat package. Habitat \
                 operator must be deployed within the Kubernetes cluster before the generated \
                 manifest can be applied to this cluster.";

    cli::Cli::new(name, about).add_all_args().app
}
