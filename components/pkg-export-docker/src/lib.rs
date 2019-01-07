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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate base64;
#[macro_use]
extern crate clap;
extern crate hab;
extern crate habitat_common as common;
extern crate habitat_core as hcore;
extern crate handlebars;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rusoto_core;
extern crate rusoto_credential as aws_creds;
extern crate rusoto_ecr;
#[macro_use]
extern crate serde_json;
extern crate tempfile;
extern crate url;

extern crate failure;
#[macro_use]
extern crate failure_derive;

mod accounts;
mod build;
#[cfg(unix)]
mod chmod;
pub mod cli;
mod docker;
mod error;
#[cfg(unix)]
pub mod rootfs;
mod util;

use std::env;
use std::fmt;
use std::result;
use std::str::FromStr;

use crate::common::ui::{UIWriter, UI};
use crate::hcore::url as hurl;
use crate::hcore::{channel, PROGRAM_NAME};

use crate::aws_creds::StaticProvider;
use clap::App;
use rusoto_core::request::*;
use rusoto_core::Region;
use rusoto_ecr::{Ecr, EcrClient, GetAuthorizationTokenRequest};

pub use crate::build::BuildSpec;
pub use crate::cli::{Cli, PkgIdentArgOptions};
pub use crate::docker::{DockerBuildRoot, DockerImage};
pub use crate::error::{Error, Result};

/// The version of this library and program when built.
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// The Habitat Package Identifier string for a Busybox package.
const BUSYBOX_IDENT: &'static str = "core/busybox-static";
/// The Habitat Package Identifier string for SSL certificate authorities (CA) certificates package.
const CACERTS_IDENT: &'static str = "core/cacerts";

/// An image naming policy.
///
/// This is a value struct which captures the naming and tagging intentions for an image.
#[derive(Debug)]
pub struct Naming<'a> {
    /// An optional custom image name which would override a computed default value.
    pub custom_image_name: Option<&'a str>,
    /// Whether or not to tag the image with a latest value.
    pub latest_tag: bool,
    /// Whether or not to tag the image with a value containing a version from a Package
    /// Identifier.
    pub version_tag: bool,
    /// Whether or not to tag the image with a value containing a version and release from a
    /// Package Identifier.
    pub version_release_tag: bool,
    /// An optional custom tag value for the image.
    pub custom_tag: Option<&'a str>,
    /// A URL to a custom Docker registry to publish to. This will be used as part of every tag
    /// before pushing.
    pub registry_url: Option<&'a str>,
    /// The type of registry we're publishing to. Ex: Amazon, Docker, Google, Azure.
    pub registry_type: RegistryType,
}

impl<'a> Naming<'a> {
    /// Creates a `Naming` from cli arguments.
    pub fn new_from_cli_matches(m: &'a clap::ArgMatches) -> Self {
        let registry_type =
            value_t!(m.value_of("REGISTRY_TYPE"), RegistryType).unwrap_or(RegistryType::Docker);
        let registry_url = m.value_of("REGISTRY_URL");

        Naming {
            custom_image_name: m.value_of("IMAGE_NAME"),
            latest_tag: !m.is_present("NO_TAG_LATEST"),
            version_tag: !m.is_present("NO_TAG_VERSION"),
            version_release_tag: !m.is_present("NO_TAG_VERSION_RELEASE"),
            custom_tag: m.value_of("TAG_CUSTOM"),
            registry_url: registry_url,
            registry_type: registry_type,
        }
    }
}

#[derive(Debug)]
pub enum RegistryType {
    Amazon,
    Azure,
    Docker,
}

impl RegistryType {
    fn variants() -> &'static [&'static str] {
        &["amazon", "azure", "docker"]
    }
}

impl FromStr for RegistryType {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value {
            "amazon" => Ok(RegistryType::Amazon),
            "azure" => Ok(RegistryType::Azure),
            "docker" => Ok(RegistryType::Docker),
            _ => Err(Error::InvalidRegistryType(String::from(value))),
        }
    }
}

impl fmt::Display for RegistryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = match *self {
            RegistryType::Amazon => "amazon",
            RegistryType::Azure => "azure",
            RegistryType::Docker => "docker",
        };
        write!(f, "{}", disp)
    }
}

/// A credentials username and password pair.
///
/// This is a value struct which references username and password values.
#[derive(Debug)]
pub struct Credentials {
    pub token: String,
}

impl Credentials {
    pub fn new(registry_type: RegistryType, username: &str, password: &str) -> Result<Self> {
        match registry_type {
            RegistryType::Amazon => {
                // The username and password should be valid IAM credentials
                let provider =
                    StaticProvider::new_minimal(username.to_string(), password.to_string());
                // TODO TED: Make the region configurable
                let client = EcrClient::new_with(HttpClient::new()?, provider, Region::UsWest2);
                let auth_token_req = GetAuthorizationTokenRequest { registry_ids: None };
                let token = client
                    .get_authorization_token(auth_token_req)
                    .sync()
                    .map_err(|e| Error::TokenFetchFailed(e))
                    .and_then(|resp| {
                        resp.authorization_data
                            .ok_or(Error::NoECRTokensReturned)
                            .and_then(|auth_data| {
                                auth_data[0]
                                    .clone()
                                    .authorization_token
                                    .ok_or(Error::NoECRTokensReturned)
                            })
                    })?;

                Ok(Credentials { token: token })
            }
            RegistryType::Docker | RegistryType::Azure => Ok(Credentials {
                token: base64::encode(&format!(
                    "{}:{}",
                    username.to_string(),
                    password.to_string()
                )),
            }),
        }
    }
}

/// Exports a Docker image to a Docker engine from a build specification and naming policy.
///
/// # Errors
///
/// * If a generic and temporary build root directory cannot be created containing a root
/// file system
/// * If additional Docker-related files cannot be created in the root file system
/// * If building the Docker image fails
/// * If destroying the temporary build root directory fails
pub fn export(ui: &mut UI, build_spec: BuildSpec, naming: &Naming) -> Result<DockerImage> {
    ui.begin(format!(
        "Building a runnable Docker image with: {}",
        build_spec.idents_or_archives.join(", ")
    ))?;
    let build_root = DockerBuildRoot::from_build_root(build_spec.create(ui)?, ui)?;
    let image = build_root.export(ui, naming)?;
    build_root.destroy(ui)?;
    ui.end(format!(
        "Docker image '{}' created with tags: {}",
        image.name(),
        image.tags().join(", ")
    ))?;

    Ok(image)
}

/// Creates a build specification and naming policy from Cli arguments, and then exports a Docker
/// image to a Docker engine from them.
///
/// # Errors
///
/// * The actual import fails.
/// * Current directory does not exist.
/// * There are insufficient permissions to access the current directory.
/// * Pushing the image to remote registry fails.
/// * Parsing of credentials fails.
/// * The image (tags) cannot be removed.
pub fn export_for_cli_matches(
    ui: &mut UI,
    matches: &clap::ArgMatches,
) -> Result<Option<DockerImage>> {
    let default_channel = channel::default();
    let default_url = hurl::default_bldr_url();
    let spec = BuildSpec::new_from_cli_matches(&matches, &default_channel, &default_url);
    let naming = Naming::new_from_cli_matches(&matches);

    let docker_image = export(ui, spec, &naming)?;
    docker_image.create_report(ui, env::current_dir()?.join("results"))?;

    if matches.is_present("PUSH_IMAGE") {
        let credentials = Credentials::new(
            naming.registry_type,
            matches
                .value_of("REGISTRY_USERNAME")
                .expect("Username not specified"),
            matches
                .value_of("REGISTRY_PASSWORD")
                .expect("Password not specified"),
        )?;
        docker_image.push(ui, &credentials, naming.registry_url)?;
    }
    if matches.is_present("RM_IMAGE") {
        docker_image.rm(ui)?;

        Ok(None)
    } else {
        Ok(Some(docker_image))
    }
}

/// Create the Clap CLI for the Docker exporter
pub fn cli<'a, 'b>() -> App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates (and optionally pushes) a Docker image from a set of Habitat packages";

    Cli::new(name, about)
        .add_base_packages_args()
        .add_builder_args()
        .add_tagging_args()
        .add_publishing_args()
        .add_pkg_ident_arg(PkgIdentArgOptions { multiple: true })
        .app
}
