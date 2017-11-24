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

#[macro_use]
extern crate clap;
extern crate hab;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate handlebars;
extern crate rusoto_core;
extern crate rusoto_ecr;
extern crate rusoto_credential as aws_creds;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate tempdir;
extern crate base64;
extern crate url;

extern crate failure;
#[macro_use]
extern crate failure_derive;

mod build;
pub mod cli;
mod docker;
mod error;
mod fs;
pub mod rootfs;
mod util;

use common::ui::UI;
use aws_creds::StaticProvider;
use rusoto_core::Region;
use rusoto_core::request::*;
use rusoto_ecr::{Ecr, EcrClient, GetAuthorizationTokenRequest};

pub use cli::Cli;
pub use build::BuildSpec;
pub use docker::{DockerImage, DockerBuildRoot};
pub use error::{Error, Result};

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
    pub registry_type: &'a str,
}

impl<'a> Naming<'a> {
    /// Creates a `Naming` from cli arguments.
    pub fn new_from_cli_matches(m: &'a clap::ArgMatches) -> Self {
        let registry_type = m.value_of("REGISTRY_TYPE").unwrap_or("docker");
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

/// A credentials username and password pair.
///
/// This is a value struct which references username and password values.
#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(registry_type: &str, username: &str, password: &str) -> Result<Self> {
        match registry_type {
            "amazon" => {
                // The username and password should be valid IAM credentials
                let provider =
                    StaticProvider::new_minimal(username.to_string(), password.to_string());
                // TODO TED: Make the region configurable
                let client =
                    EcrClient::new(default_tls_client().unwrap(), provider, Region::UsWest2);
                let auth_token_req = GetAuthorizationTokenRequest { registry_ids: None };
                let token = match client.get_authorization_token(&auth_token_req) {
                    Ok(resp) => {
                        match resp.authorization_data {
                            Some(auth_data) => auth_data[0].clone().authorization_token.unwrap(),
                            None => return Err(Error::NoECRTokensReturned)?,
                        }
                    }
                    Err(e) => return Err(Error::TokenFetchFailed(e))?,
                };

                let creds: Vec<String> = match base64::decode(&token) {
                    Ok(decoded_token) => {
                        match String::from_utf8(decoded_token) {
                            Ok(dts) => dts.split(':').map(String::from).collect(),
                            Err(err) => return Err(Error::InvalidToken(err))?,
                        }
                    }
                    Err(err) => return Err(Error::Base64DecodeError(err))?,
                };

                Ok(Credentials {
                    username: creds[0].to_string(),
                    password: creds[1].to_string(),
                })
            }
            _ => {
                Ok(Credentials {
                    username: username.to_string(),
                    password: password.to_string(),
                })
            }
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
