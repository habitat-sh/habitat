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

extern crate clap;
extern crate env_logger;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_pkg_export_docker as export_docker;
extern crate habitat_pkg_export_kubernetes as export_k8s;
extern crate handlebars;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

extern crate failure;

mod chart;
mod chartfile;
mod values;

use std::result;

use clap::Arg;

use common::ui::UI;
use export_docker::Result;
use export_k8s::Cli;
use hcore::PROGRAM_NAME;

use chart::Chart;

fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    let m = cli().get_matches();
    debug!("clap cli args: {:?}", m);

    if let Err(e) = export_for_cli_matches(&mut ui, &m) {
        let _ = ui.fatal(e);
        std::process::exit(1)
    }
}

fn export_for_cli_matches(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    let mut chart = Chart::new_for_cli_matches(ui, matches)?;
    chart.generate()?;

    Ok(())
}

fn cli<'a, 'b>() -> clap::App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates a Docker image and generates a Helm chart for the specified Habitat \
                 package. Habitat operator must be deployed within the Kubernetes cluster before \
                 the generated chart can be installed.";

    Cli::new(name, about)
        .add_docker_args()
        .add_runtime_args()
        .add_secret_names_args()
        .add_bind_args()
        .app
        .arg(
            Arg::with_name("CHART")
                .value_name("CHART")
                .long("chart")
                .short("h")
                .help(
                    "Name of the chart to create, if different from the package name",
                ),
        )
        .arg(
            Arg::with_name("VERSION")
                .value_name("VERSION")
                .long("version")
                .validator(valid_version)
                // Keep the default version here in sync with chartfile::DEFAULT_VERSION
                .help("Version of the chart to create (default: 0.0.1)"),
        )
        .arg(
            Arg::with_name("DESCRIPTION")
                .value_name("DESCRIPTION")
                .long("desc")
                .help("A single-sentence description"),
        )
}

fn valid_version(val: String) -> result::Result<(), String> {
    let split: Vec<&str> = val.split(".").collect();
    if split.len() != 3 {
        return Err(format!("Version '{}' is not valid", &val));
    }

    for s in split {
        for c in s.chars() {
            if !c.is_digit(10) {
                return Err(format!("Version '{}' is not valid", &val));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_version() {
        valid_version("0.0.1".to_owned()).unwrap();
        valid_version("0.1.1".to_owned()).unwrap();
        valid_version("1.1.1".to_owned()).unwrap();
        valid_version("1.1.10".to_owned()).unwrap();
        valid_version("1.10.1".to_owned()).unwrap();
        valid_version("10.1.1".to_owned()).unwrap();

        assert!(valid_version("1".to_owned()).is_err());
        assert!(valid_version("1.1".to_owned()).is_err());
        assert!(valid_version("X".to_owned()).is_err());
        assert!(valid_version("1.2.Z".to_owned()).is_err());
        assert!(valid_version("1.1.1.1".to_owned()).is_err());
        assert!(valid_version("٣.7.৬".to_owned()).is_err());
    }
}
