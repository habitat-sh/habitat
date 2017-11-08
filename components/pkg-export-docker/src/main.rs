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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate base64;
extern crate clap;
extern crate env_logger;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_pkg_export_docker as export_docker;
extern crate rusoto_core;
extern crate rusoto_credential as aws_creds;
extern crate rusoto_ecr;
extern crate chrono;
#[macro_use]
extern crate log;

use std::env;

use clap::{App, Arg};
use hcore::channel;
use common::ui::{Coloring, UI, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR};
use hcore::env as henv;
use hcore::PROGRAM_NAME;
use hcore::url as hurl;

use aws_creds::StaticProvider;
use rusoto_core::Region;
use rusoto_core::request::*;
use rusoto_ecr::{Ecr, EcrClient, GetAuthorizationTokenRequest};
use export_docker::{Cli, BuildSpec, Credentials, Error, Result, Naming};

fn main() {
    env_logger::init().unwrap();
    let mut ui = ui();
    if let Err(e) = start(&mut ui) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    let cli = cli();
    let m = cli.get_matches();
    debug!("clap cli args: {:?}", m);
    let default_channel = channel::default();
    let default_url = hurl::default_bldr_url();
    let spec = BuildSpec::new_from_cli_matches(&m, &default_channel, &default_url);
    let naming = Naming::new_from_cli_matches(&m);

    let docker_image = export_docker::export(ui, spec, &naming)?;
    docker_image.create_report(
        ui,
        env::current_dir()?.join("results"),
    )?;
    if m.is_present("PUSH_IMAGE") {
        match naming.registry_type {
            "amazon" => {
                // The username and password should be valid IAM credentials
                let provider = StaticProvider::new_minimal(
                    m.value_of("REGISTRY_USERNAME").unwrap().to_string(),
                    m.value_of("REGISTRY_PASSWORD").unwrap().to_string(),
                );
                // TODO TED: Make the region configurable
                let client =
                    EcrClient::new(default_tls_client().unwrap(), provider, Region::UsWest2);
                let auth_token_req = GetAuthorizationTokenRequest { registry_ids: None };
                let token = match client.get_authorization_token(&auth_token_req) {
                    Ok(resp) => {
                        match resp.authorization_data {
                            Some(auth_data) => auth_data[0].clone().authorization_token.unwrap(),
                            None => return Err(Error::NoECRTokensReturned),
                        }
                    }
                    Err(e) => return Err(Error::TokenFetchFailed(e)),
                };

                let creds: Vec<String> = match base64::decode(&token) {
                    Ok(decoded_token) => {
                        match String::from_utf8(decoded_token) {
                            Ok(dts) => dts.split(':').map(String::from).collect(),
                            Err(err) => return Err(Error::InvalidToken(err)),
                        }
                    }
                    Err(err) => return Err(Error::Base64DecodeError(err)),
                };

                let credentials = Credentials {
                    username: &creds[0],
                    password: &creds[1],
                };
                docker_image.push(ui, &credentials, naming.registry_url)?;
            }
            _ => {
                let credentials = Credentials {
                    username: m.value_of("REGISTRY_USERNAME").unwrap(),
                    password: m.value_of("REGISTRY_PASSWORD").unwrap(),
                };
                docker_image.push(ui, &credentials, naming.registry_url)?;
            }
        }
    }
    if m.is_present("RM_IMAGE") {
        docker_image.rm(ui)?;
    }

    Ok(())
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates (an optionally pushes) a Docker image from a set of Habitat packages";

    let app = Cli::new(name, about)
        .add_base_packages_args()
        .add_builder_args()
        .add_tagging_args()
        .add_publishing_args()
        .app;

    app.arg(
        Arg::with_name("PKG_IDENT_OR_ARTIFACT")
            .value_name("PKG_IDENT_OR_ARTIFACT")
            .required(true)
            .multiple(true)
            .help(
                "One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a \
                Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)",
            ),
    )
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
