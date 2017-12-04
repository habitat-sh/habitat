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

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate handlebars;
extern crate rand;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

use clap::App;
use handlebars::Handlebars;
use std::result;
use std::str::FromStr;
use std::io::prelude::*;
use std::io;
use std::fs::File;

use hcore::PROGRAM_NAME;
use hcore::env as henv;
use hcore::package::PackageIdent;
use common::ui::{Coloring, UI, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR};
use rand::Rng;

// Synced with the version of the Habitat operator.
pub const VERSION: &'static str = "0.1.0";

// Kubernetes manifest template
const MANIFESTFILE: &'static str = include_str!("../defaults/KubernetesManifest.hbs");

fn main() {
    env_logger::init().unwrap();
    let mut ui = ui();
    if let Err(e) = start(&mut ui) {
        let _ = ui.fatal(e);
        std::process::exit(1)
    }
}

fn ui() -> UI {
    let isatty = if henv::var(NONINTERACTIVE_ENVVAR)
        .map(|val| val == "true")
        .unwrap_or(false)
    {
        Some(false)
    } else {
        None
    };
    let coloring = if henv::var(NOCOLORING_ENVVAR)
        .map(|val| val == "true")
        .unwrap_or(false)
    {
        Coloring::Never
    } else {
        Coloring::Auto
    };
    UI::default_with(coloring, isatty)
}

fn start(_ui: &mut UI) -> result::Result<(), String> {
    let m = cli().get_matches();
    debug!("clap cli args: {:?}", m);
    let count = m.value_of("COUNT").unwrap_or("1");
    let topology = m.value_of("TOPOLOGY").unwrap_or("standalone");
    let group = m.value_of("GROUP");
    let config_secret_name = m.value_of("CONFIG_SECRET_NAME");
    let ring_secret_name = m.value_of("RING_SECRET_NAME");
    // clap_app!() ensures that we do have the mandatory args so unwrap() is fine here
    let pkg_ident_str = m.value_of("PKG_IDENT").unwrap();
    let pkg_ident = match PackageIdent::from_str(pkg_ident_str) {
        Ok(pi) => pi,
        Err(e) => return Err(format!("{}", e)),
    };
    // To allow multiple instances of Habitat application in Kubernetes,
    // random suffix in metadata_name is needed.
    let metadata_name = format!(
        "{}-{}",
        pkg_ident.name,
        rand::thread_rng()
            .gen_ascii_chars()
            .take(5)
            .collect::<String>()
    );
    let image = m.value_of("IMAGE").unwrap_or(pkg_ident_str);

    let json = json!({
        "metadata_name": metadata_name,
        "habitat_name": pkg_ident.name,
        "image": image,
        "count": count,
        "service_topology": topology,
        "service_group": group,
        "config_secret_name": config_secret_name,
        "ring_secret_name": ring_secret_name,
    });

    let mut write: Box<Write> = match m.value_of("OUTPUT") {
        Some(o) if o != "-" => {
            match File::create(o) {
                Ok(f) => Box::new(f),
                Err(e) => return Err(format!("{}", e)),
            }
        }
        _ => Box::new(io::stdout()),
    };

    match Handlebars::new().template_render(MANIFESTFILE, &json) {
        Ok(r) => {
            let out = r.lines().filter(|l| *l != "").collect::<Vec<_>>().join(
                "\n",
            ) + "\n";

            match write.write(out.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{}", e)),
            }
        }

        Err(e) => Err(format!("{}", e)),
    }
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    clap_app!((name) =>
        (about: "Creates a Kubernetes manifest for a Habitat package. Habitat \
                 operator must be deployed within the Kubernetes cluster to \
                 intercept the created objects.")
        (version: VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n\n")
        (@arg IMAGE: --("image") +takes_value
            "Image of the Habitat service exported as a Docker image")
        (@arg OUTPUT: -o --("output") +takes_value
            "Name of manifest file to create. Pass '-' for stdout (default: -)")
        (@arg COUNT: --("count") +takes_value {valid_natural_number}
            "Count is the number of desired instances")
        (@arg TOPOLOGY: -t --("service-topology") +takes_value
            "A topology describes the intended relationship between peers \
             within a service group. Specify either standalone or leader \
             topology (default: standalone)")
        (@arg GROUP: -g --("service-group") +takes_value
            "group is a logical grouping of services with the same package and \
             topology type connected together in a ring (default: default)")
        (@arg CONFIG_SECRET_NAME: -n --("config-secret-name") +takes_value
            "name of the Kubernetes Secret containing the config file - \
             user.toml - that the user has previously created. Habitat will \
             use it for initial configuration of the service")
        (@arg RING_SECRET_NAME: -r --("ring-secret-name") +takes_value
             "name of the Kubernetes Secret that contains the ring key, which \
              encrypts the communication between Habitat supervisors")
        (@arg PKG_IDENT: +required
            "Habitat package identifier (ex: acme/redis)")
    )
}

fn valid_natural_number(val: String) -> result::Result<(), String> {
    match val.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("{} is not a natural number", val)),
    }
}
