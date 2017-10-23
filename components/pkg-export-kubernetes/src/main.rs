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
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

use clap::App;
use handlebars::Handlebars;
use std::result;
use std::str::FromStr;

use hcore::PROGRAM_NAME;
use hcore::env as henv;
use hcore::package::PackageIdent;
use common::ui::{Coloring, UI, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR};

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
    // clap_app!() ensures that we do have the mandatory args so unwrap() is fine here
    let pkg_ident_str = m.value_of("PKG_IDENT").unwrap();
    let pkg_ident = match PackageIdent::from_str(pkg_ident_str) {
        Ok(pi) => pi,
        Err(e) => return Err(format!("{}", e)),
    };
    let image = m.value_of("IMAGE").unwrap_or(pkg_ident_str);

    let json = json!({
        "metadata_name": pkg_ident.name,
        "image": image,
        "count": count,
    });

    match Handlebars::new().template_render(MANIFESTFILE, &json) {
        Ok(manifest) => {
            print!("{}", manifest);

            Ok(())
        },

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
        (@arg COUNT: --("count") +takes_value {valid_natural_number}
            "Count is the number of desired instances")
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
