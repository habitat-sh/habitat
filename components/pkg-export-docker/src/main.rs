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
#[macro_use]
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
extern crate url;

use std::env;
use std::path::Path;
use std::result;
use std::str::FromStr;

use clap::App;
use common::ui::{Coloring, UI, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR};
use hcore::channel;
use hcore::env as henv;
use hcore::PROGRAM_NAME;
use hcore::package::PackageIdent;
use hcore::url as hurl;
use url::Url;

use aws_creds::StaticProvider;
use rusoto_core::Region;
use rusoto_core::request::*;
use rusoto_ecr::{Ecr, EcrClient, GetAuthorizationTokenRequest};
use export_docker::{BuildSpec, Credentials, Error, Result, Naming};

const DEFAULT_HAB_IDENT: &'static str = "core/hab";
const DEFAULT_LAUNCHER_IDENT: &'static str = "core/hab-launcher";
const DEFAULT_SUP_IDENT: &'static str = "core/hab-sup";

fn main() {
    env_logger::init().unwrap();
    let mut ui = ui();
    if let Err(e) = start(&mut ui) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    let m = cli().get_matches();
    debug!("clap cli args: {:?}", m);
    let default_channel = channel::default();
    let default_url = hurl::default_bldr_url();
    let registry_type = m.value_of("REGISTRY_TYPE").unwrap_or("docker");
    let registry_url = m.value_of("REGISTRY_URL");

    let spec = BuildSpec {
        hab: m.value_of("HAB_PKG").unwrap_or(DEFAULT_HAB_IDENT),
        hab_launcher: m.value_of("HAB_LAUNCHER_PKG").unwrap_or(
            DEFAULT_LAUNCHER_IDENT,
        ),
        hab_sup: m.value_of("HAB_SUP_PKG").unwrap_or(DEFAULT_SUP_IDENT),
        url: m.value_of("BLDR_URL").unwrap_or(&default_url),
        channel: m.value_of("CHANNEL").unwrap_or(&default_channel),
        base_pkgs_url: m.value_of("BASE_PKGS_BLDR_URL").unwrap_or(&default_url),
        base_pkgs_channel: m.value_of("BASE_PKGS_CHANNEL").unwrap_or(&default_channel),
        idents_or_archives: m.values_of("PKG_IDENT_OR_ARTIFACT").unwrap().collect(),
    };
    let naming = Naming {
        custom_image_name: m.value_of("IMAGE_NAME"),
        latest_tag: !m.is_present("NO_TAG_LATEST"),
        version_tag: !m.is_present("NO_TAG_VERSION"),
        version_release_tag: !m.is_present("NO_TAG_VERSION_RELEASE"),
        custom_tag: m.value_of("TAG_CUSTOM"),
        registry_url: registry_url,
        registry_type: registry_type,
    };

    let docker_image = export_docker::export(ui, spec, &naming)?;
    docker_image.create_report(
        ui,
        env::current_dir()?.join("results"),
    )?;
    if m.is_present("PUSH_IMAGE") {
        match registry_type {
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
                docker_image.push(ui, &credentials, registry_url)?;
            }
            _ => {
                let credentials = Credentials {
                    username: m.value_of("REGISTRY_USERNAME").unwrap(),
                    password: m.value_of("REGISTRY_PASSWORD").unwrap(),
                };
                docker_image.push(ui, &credentials, registry_url)?;
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
    clap_app!((name) =>
        (about: "Creates (an optionally pushes) a Docker image from a set of Habitat packages")
        (version: export_docker::VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n\n")
        (@arg IMAGE_NAME: --("image-name") -n +takes_value
            "Image name (default: \"{{pkg_origin}}/{{pkg_name}}\" supports: \
            {{pkg_origin}}, {{pkg_name}}, {{pkg_version}}, {{pkg_release}}, {{channel}})")
        (@arg PKG_IDENT_OR_ARTIFACT: +required +multiple
            "One or more Habitat package identifiers (ex: acme/redis) and/or filepaths \
            to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")

        // Builder
        (@arg BLDR_URL: --url -u +takes_value {valid_url}
            "Install packages from Builder at the specified URL \
            (default: https://bldr.habitat.sh)")
        (@arg CHANNEL: --channel -c +takes_value
            "Install packages from the specified release channel \
            (default: stable)")
        (@arg BASE_PKGS_BLDR_URL: --("base-pkgs-url") +takes_value {valid_url}
            "Install base packages from Builder at the specified URL \
            (default: https://bldr.habitat.sh)")
        (@arg BASE_PKGS_CHANNEL: --("base-pkgs-channel") +takes_value
            "Install base packages from the specified release channel \
            (default: stable)")

        // Base packages
        (@arg HAB_PKG: --("hab-pkg") +takes_value {valid_ident_or_hart}
            "Habitat CLI package identifier (ex: acme/redis) or filepath to a Habitat artifact \
            (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart) to install \
            (default: core/hab)")
        (@arg HAB_LAUNCHER_PKG: --("launcher-pkg") +takes_value {valid_ident_or_hart}
            "Launcher package identifier (ex: acme/redis) or filepath to a Habitat artifact \
            (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart) to install \
            (default: core/hab-launcher)")
        (@arg HAB_SUP_PKG: --("sup-pkg") +takes_value {valid_ident_or_hart}
            "Supervisor package identifier (ex: acme/redis) or filepath to a Habitat artifact \
            (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart) to install \
            (default: core/hab-sup)")

        // Tagging
        (@arg TAG_VERSION_RELEASE: --("tag-version-release")
            conflicts_with[NO_TAG_VERSION_RELEASE]
            "Tag image with :\"{{pkg_version}}-{{pkg_release}}\" (default: yes)")
        (@arg NO_TAG_VERSION_RELEASE: --("no-tag-version-release")
            conflicts_with[TAG_VERSION_RELEASE]
            "Do not tag image with :\"{{pkg_version}}-{{pkg_release}}\" (default: no)")
        (@arg TAG_VERSION: --("tag-version") conflicts_with[NO_TAG_VERSION]
            "Tag image with :\"{{pkg_version}}\" (default: yes)")
        (@arg NO_TAG_VERSION: --("no-tag-version") conflicts_with[TAG_VERSION]
            "Do not tag image with :\"{{pkg_version}}\" (default: no)")
        (@arg TAG_LATEST: --("tag-latest") conflicts_with[NO_TAG_LATEST]
            "Tag image with :\"latest\" (default: yes)")
        (@arg NO_TAG_LATEST: --("no-tag-latest") conflicts_with[TAG_LATEST]
            "Do not tag image with :\"latest\" (default: no)")
        (@arg TAG_CUSTOM: --("tag-custom") +takes_value
            "Tag image with additional custom tag (supports: \
            {{pkg_origin}}, {{pkg_name}}, {{pkg_version}}, {{pkg_release}}, {{channel}})")

        // Publishing
        (@arg PUSH_IMAGE: --("push-image")
            conflicts_with[NO_PUSH_IMAGE] requires[REGISTRY_USERNAME] requires[REGISTRY_PASSWORD]
            "Push image to remote registry (default: no)")
        (@arg NO_PUSH_IMAGE: --("no-push-image") conflicts_with[PUSH_IMAGE]
            "Do not push image to remote registry (default: yes)")
        (@arg REGISTRY_USERNAME: --("username") -U +takes_value requires[REGISTRY_PASSWORD]
            "Remote registry username, required for pushing image to remote registry")
        (@arg REGISTRY_PASSWORD: --("password") -P +takes_value requires[REGISTRY_USERNAME]
            "Remote registry password, required for pushing image to remote registry")
        (@arg REGISTRY_TYPE: --("registry-type") -R +takes_value
            "Remote registry type, Ex: Amazon, Docker, Google (default: docker)")
        (@arg REGISTRY_URL: --("registry-url") -G +takes_value
            "Remote registry url")
        // Cleanup
        (@arg RM_IMAGE: --("rm-image")
            "Remove local image from engine after build and/or push (default: no)")

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

fn valid_ident_or_hart(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else if val.ends_with(".hart") {
        Err(format!("Habitat artifact file: '{}' not found", &val))
    } else {
        match PackageIdent::from_str(&val) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}
