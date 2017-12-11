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

use std::env;

use hcore::channel;
use hcore::PROGRAM_NAME;
use hcore::url as hurl;
use common::ui::UI;

use export_docker::{Credentials, BuildSpec, Naming, Result};

use manifest::Manifest;

// Synced with the version of the Habitat operator.
pub const VERSION: &'static str = "0.1.0";

fn main() {
    env_logger::init().unwrap();
    let mut ui = UI::default_with_env();
    if let Err(e) = start(&mut ui) {
        let _ = ui.fatal(e);
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    let m = cli().get_matches();
    debug!("clap cli args: {:?}", m);

    if !m.is_present("NO_DOCKER_IMAGE") {
        gen_docker_img(ui, &m)?;
    }
    let mut manifest = Manifest::new_from_cli_matches(ui, &m)?;
    manifest.generate()
}

fn gen_docker_img(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    let default_channel = channel::default();
    let default_url = hurl::default_bldr_url();
    let spec = BuildSpec::new_from_cli_matches(&matches, &default_channel, &default_url);
    let naming = Naming::new_from_cli_matches(&matches);

    let docker_image = export_docker::export(ui, spec, &naming)?;
    docker_image.create_report(
        ui,
        env::current_dir()?.join("results"),
    )?;

    if matches.is_present("PUSH_IMAGE") {
        let credentials = Credentials::new(
            naming.registry_type,
            matches.value_of("REGISTRY_USERNAME").unwrap(),
            matches.value_of("REGISTRY_PASSWORD").unwrap(),
        )?;
        docker_image.push(ui, &credentials, naming.registry_url)?;
    }
    if matches.is_present("RM_IMAGE") {
        docker_image.rm(ui)?;
    }

    Ok(())
}

fn cli<'a, 'b>() -> clap::App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates a Docker image and Kubernetes manifest for a Habitat package. Habitat \
                 operator must be deployed within the Kubernetes cluster before the generated \
                 manifest can be applied to this cluster.";

    cli::Cli::new(name, about).add_all_args().app
}
