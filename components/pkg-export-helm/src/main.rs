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

use clap;
use env_logger;
use habitat_common as common;
use habitat_core as hcore;
use habitat_pkg_export_docker as export_docker;
use habitat_pkg_export_kubernetes as export_k8s;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use failure;
#[macro_use]
extern crate failure_derive;

mod chart;
mod chartfile;
mod deps;
mod error;
mod maintainer;
mod values;

use std::{result,
          str::FromStr};

use clap::Arg;

use crate::{common::ui::{UIWriter,
                         UI},
            export_docker::Result,
            export_k8s::Cli,
            hcore::PROGRAM_NAME};
use url::Url;

use crate::chart::Chart;

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

fn export_for_cli_matches(ui: &mut UI, matches: &clap::ArgMatches<'_>) -> Result<()> {
    let chart = Chart::new_for_cli_matches(ui, matches)?;
    chart.generate()?;

    Ok(())
}

lazy_static! {
    pub static ref VERSION_HELP: String = format!(
        "Version of the chart to create (default: {{pkg_version}} if available, or {})",
        chartfile::DEFAULT_VERSION
    );
}

fn cli<'a, 'b>() -> clap::App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about =
        "Creates a Docker image and generates a Helm chart for the specified Habitat package.";

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
                .help("Name of the chart to create, if different from the package name"),
        )
        .arg(
            Arg::with_name("VERSION")
                .value_name("VERSION")
                .short("V")
                .long("version")
                .validator(valid_version)
                .help(&VERSION_HELP),
        )
        .arg(
            Arg::with_name("DESCRIPTION")
                .value_name("DESCRIPTION")
                .long("desc")
                .help("A single-sentence description"),
        )
        .arg(
            Arg::with_name("KEYWORD")
                .value_name("KEYWORD")
                .long("keyword")
                .short("k")
                .multiple(true)
                .help("A keyword for this project"),
        )
        .arg(
            Arg::with_name("HOME")
                .value_name("URL")
                .long("home")
                .validator(valid_url)
                .help("The URL of the project's home page"),
        )
        .arg(
            Arg::with_name("ICON")
                .value_name("URL")
                .long("icon")
                .validator(valid_url)
                .help("A URL of an SVG or PNG image to be used as an icon"),
        )
        .arg(
            Arg::with_name("SOURCE")
                .value_name("URL")
                .long("source")
                .short("s")
                .multiple(true)
                .validator(valid_url)
                .help("A URL to the source code for the project"),
        )
        .arg(
            Arg::with_name("MAINTAINER")
                .value_name("MAINTAINER_SPEC")
                .long("maint")
                .short("m")
                .multiple(true)
                .validator(valid_maintainer)
                .help("A maintainer of the project, in the form of NAME,[EMAIL[,URL]]"),
        )
        .arg(
            Arg::with_name("DEPRECATED")
                .long("depr")
                .help("Mark this chart as deprecated"),
        )
        .arg(
            Arg::with_name("OPERATOR_VERSION")
                .value_name("OPERATOR_VERSION")
                .long("operator-version")
                .validator(valid_version)
                .help("Version of the Habitat operator to set as dependency")
                .default_value(deps::DEFAULT_OPERATOR_VERSION),
        )
        .arg(
            Arg::with_name("OUTPUTDIR")
                .value_name("OUTPUTDIR")
                .short("o")
                .long("output-dir")
                .help(
                    "The directory to put the chart directory under (default: current working \
                     directory)",
                ),
        )
        .arg(Arg::with_name("DOWNLOAD_DEPS").long("download-deps").help(
            "Whether to download dependencies. The Kubernetes Habitat Operator is the only \
             dependency currently. (default: no)",
        ))
}

fn valid_version(val: String) -> result::Result<(), String> {
    let split: Vec<&str> = val.split('.').collect();
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

fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}

fn valid_maintainer(val: String) -> result::Result<(), String> {
    maintainer::Maintainer::from_str(&val)
        .map(|_| ())
        .map_err(|e| format!("{}", e))
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

    #[test]
    fn test_maintainer() {
        valid_maintainer("name".to_owned()).unwrap();
        // Currently Email address is not checked for validity. See FIXME in Maintainer::from_str().
        valid_maintainer("name,email".to_owned()).unwrap();
        valid_maintainer("name,email,http://blah".to_owned()).unwrap();

        assert!(valid_maintainer("name,email,blah".to_owned()).is_err());
        assert!(valid_maintainer("name,email,http://blah,x".to_owned()).is_err());
    }
}
