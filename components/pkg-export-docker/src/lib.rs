#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use crate::naming::Naming;
pub use crate::{build::BuildSpec,
                cli::cli,
                docker::{DockerBuildRoot,
                         DockerImage},
                error::{Error,
                        Result}};
use habitat_common::ui::{UIWriter,
                         UI};
use habitat_core::url::default_bldr_url;
use rusoto_core::{request::HttpClient,
                  Region};
use rusoto_credential::StaticProvider;
use rusoto_ecr::{Ecr,
                 EcrClient,
                 GetAuthorizationTokenRequest};
use std::{env,
          fmt,
          result,
          str::FromStr};

mod accounts;
mod build;
mod cli;
mod docker;
mod error;
mod graph;
mod naming;
mod os;
#[cfg(unix)]
mod rootfs;
mod util;

/// The version of this library and program when built.
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// The Habitat Package Identifier string for a Busybox package.
const BUSYBOX_IDENT: &str = "core/busybox-static";
/// The Habitat Package Identifier string for SSL certificate authorities (CA) certificates package.
const CACERTS_IDENT: &str = "core/cacerts";

#[derive(Clone, Copy, Debug)]
pub enum RegistryType {
    Amazon,
    Azure,
    Docker,
}

impl RegistryType {
    fn variants() -> &'static [&'static str] { &["amazon", "azure", "docker"] }
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = match *self {
            RegistryType::Amazon => "amazon",
            RegistryType::Azure => "azure",
            RegistryType::Docker => "docker",
        };
        write!(f, "{}", disp)
    }
}

impl Default for RegistryType {
    fn default() -> Self { RegistryType::Docker }
}

/// A credentials username and password pair.
///
/// This is a value struct which references username and password values.
#[derive(Debug)]
pub struct Credentials {
    pub token: String,
}

impl Credentials {
    pub async fn new(registry_type: RegistryType, username: &str, password: &str) -> Result<Self> {
        match registry_type {
            RegistryType::Amazon => {
                // The username and password should be valid IAM credentials
                let provider =
                    StaticProvider::new_minimal(username.to_string(), password.to_string());
                // TODO TED: Make the region configurable
                let client = EcrClient::new_with(HttpClient::new()?, provider, Region::UsWest2);
                let auth_token_req = GetAuthorizationTokenRequest { registry_ids: None };
                let token = client.get_authorization_token(auth_token_req)
                                  .await
                                  .map_err(Error::TokenFetchFailed)
                                  .and_then(|resp| {
                                      resp.authorization_data
                                          .ok_or(Error::NoECRTokensReturned)
                                          .and_then(|auth_data| {
                                              auth_data[0].clone()
                                                          .authorization_token
                                                          .ok_or(Error::NoECRTokensReturned)
                                          })
                                  })?;

                Ok(Credentials { token })
            }
            RegistryType::Docker | RegistryType::Azure => {
                Ok(Credentials { token: base64::encode(&format!("{}:{}",
                                                                username.to_string(),
                                                                password.to_string())), })
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
pub async fn export<'a>(ui: &'a mut UI,
                        build_spec: BuildSpec,
                        naming: &Naming,
                        memory: Option<&'a str>)
                        -> Result<DockerImage> {
    ui.begin(format!("Building a runnable Docker image with: {}",
                     build_spec.idents_or_archives.join(", ")))?;
    let build_root = DockerBuildRoot::from_build_root(build_spec.create(ui).await?, ui)?;
    let image = build_root.export(ui, naming, memory)?;
    build_root.destroy(ui)?;
    ui.end(format!("Docker image '{}' created with tags: {}",
                   image.name(),
                   image.tags().join(", ")))?;

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
pub async fn export_for_cli_matches(ui: &mut UI,
                                    matches: &clap::ArgMatches<'_>)
                                    -> Result<Option<DockerImage>> {
    os::ensure_proper_docker_platform()?;

    let default_url = default_bldr_url();
    let spec = BuildSpec::new_from_cli_matches(&matches, &default_url)?;
    let naming = Naming::new_from_cli_matches(&matches);

    let docker_image = export(ui, spec, &naming, matches.value_of("MEMORY_LIMIT")).await?;
    docker_image.create_report(ui, env::current_dir()?.join("results"))?;

    if matches.is_present("PUSH_IMAGE") {
        let credentials = Credentials::new(naming.registry_type,
                                           matches.value_of("REGISTRY_USERNAME")
                                                  .expect("Username not specified"),
                                           matches.value_of("REGISTRY_PASSWORD")
                                                  .expect("Password not specified")).await?;
        docker_image.push(ui, &credentials, naming.registry_url.as_deref())?;
    }
    if matches.is_present("RM_IMAGE") {
        docker_image.rm(ui)?;

        Ok(None)
    } else {
        Ok(Some(docker_image))
    }
}
