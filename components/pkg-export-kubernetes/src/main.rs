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

use clap::{App, Arg};
use std::env;
use std::result;

use hcore::channel;
use hcore::PROGRAM_NAME;
use hcore::url as hurl;
use common::ui::UI;

use export_docker::{Cli, Credentials, BuildSpec, Naming, PkgIdentArgOptions, Result};

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

fn cli<'a, 'b>() -> App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates a Docker image and Kubernetes manifest for a Habitat package. Habitat \
                 operator must be deployed within the Kubernetes cluster before the generated \
                 manifest can be applied to this cluster.";

    let app = Cli::new(name, about)
        .add_base_packages_args()
        .add_builder_args()
        .add_tagging_args()
        .add_publishing_args()
        .add_pkg_ident_arg(PkgIdentArgOptions { multiple: false })
        .app;

    app.arg(
        Arg::with_name("OUTPUT")
            .value_name("OUTPUT")
            .long("output")
            .short("o")
            .help(
                "Name of manifest file to create. Pass '-' for stdout (default: -)",
            ),
    ).arg(
            Arg::with_name("COUNT")
                .value_name("COUNT")
                .long("count")
                .validator(valid_natural_number)
                .help("Count is the number of desired instances"),
        )
        .arg(
            Arg::with_name("TOPOLOGY")
                .value_name("TOPOLOGY")
                .long("topology")
                .short("t")
                .possible_values(&["standalone", "leader"])
                .help(
                    "A topology describes the intended relationship between peers \
                    within a Habitat service group. Specify either standalone or leader \
                    topology (default: standalone)",
                ),
        )
        .arg(
            Arg::with_name("GROUP")
                .value_name("GROUP")
                .long("service-group")
                .short("g")
                .help(
                    "group is a logical grouping of services with the same package and \
                    topology type connected together in a ring (default: default)",
                ),
        )
        .arg(
            Arg::with_name("CONFIG_SECRET_NAME")
                .value_name("CONFIG_SECRET_NAME")
                .long("config-secret-name")
                .short("n")
                .help(
                    "name of the Kubernetes Secret containing the config file - \
                    user.toml - that the user has previously created. Habitat will \
                    use it for initial configuration of the service",
                ),
        )
        .arg(
            Arg::with_name("RING_SECRET_NAME")
                .value_name("RING_SECRET_NAME")
                .long("ring-secret-name")
                .short("r")
                .help(
                    "name of the Kubernetes Secret that contains the ring key, which \
                    encrypts the communication between Habitat supervisors",
                ),
        )
        .arg(
            Arg::with_name("BIND")
                .value_name("BIND")
                .long("bind")
                .short("b")
                .multiple(true)
                .number_of_values(1)
                .help(
                    "Bind to another service to form a producer/consumer relationship, \
                    specified as name:service:group",
                ),
        )
        .arg(
            Arg::with_name("NO_DOCKER_IMAGE")
                .long("no-docker-image")
                .short("d")
                .help(
                    "Disable creation of the Docker image and only create a Kubernetes manifest",
                ),
        )
}

fn valid_natural_number(val: String) -> result::Result<(), String> {
    match val.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("{} is not a natural number", val)),
    }
}
