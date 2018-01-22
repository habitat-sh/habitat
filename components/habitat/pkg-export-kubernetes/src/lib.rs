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
extern crate habitat_common as common;
extern crate habitat_core as hcore;
extern crate habitat_pkg_export_docker as export_docker;
extern crate handlebars;
extern crate rand;
#[macro_use]
extern crate serde_json;

extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use common::ui::UI;

pub mod topology;
pub mod error;
pub mod manifest;
pub mod manifestjson;
pub mod bind;
pub mod cli;

use export_docker::Result;

pub use cli::Cli;
pub use bind::Bind;
pub use error::Error;
pub use manifest::Manifest;
pub use manifestjson::ManifestJson;
pub use topology::Topology;

// Synced with the version of the Habitat operator.
pub const VERSION: &'static str = "0.1.0";

pub fn export_for_cli_matches(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    if !matches.is_present("NO_DOCKER_IMAGE") {
        export_docker::export_for_cli_matches(ui, &matches)?;
    }
    let mut manifest = Manifest::new_from_cli_matches(ui, &matches)?;

    let mut write: Box<Write> = match matches.value_of("OUTPUT") {
        Some(o) if o != "-" => Box::new(File::create(o)?),
        _ => Box::new(io::stdout()),
    };
    manifest.generate(&mut write)
}
