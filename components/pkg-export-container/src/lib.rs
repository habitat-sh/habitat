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
                container::{BuildContext,
                            ContainerImage},
                engine::Engine,
                error::{Error,
                        Result}};
use habitat_common::ui::{Status,
                         UIWriter,
                         UI};
use rusoto_core::{request::HttpClient,
                  Region};
use rusoto_credential::StaticProvider;
use rusoto_ecr::{Ecr,
                 EcrClient,
                 GetAuthorizationTokenRequest};
use std::{convert::TryFrom,
          env,
          fmt,
          path::Path,
          result,
          str::FromStr};
mod accounts;
mod build;
mod cli;
mod container;
mod engine;
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

/// Creates a build specification and naming policy from CLI
/// arguments, and then creates a container image from them.
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
                                    -> Result<Option<ContainerImage>> {
    os::ensure_proper_docker_platform()?;

    let spec = BuildSpec::try_from(matches)?;
    let naming = Naming::from(matches);
    let engine: Box<dyn Engine> = TryFrom::try_from(matches)?;
    let memory = matches.value_of("MEMORY_LIMIT");

    ui.begin(format!("Building a container image with: {}",
                     spec.idents_or_archives.join(", ")))?;

    let build_context = BuildContext::from_build_root(spec.create(ui).await?, ui)?;
    let container_image = build_context.export(ui, &naming, memory, engine.as_ref())?;

    build_context.destroy(ui)?;
    ui.end(format!("Container image '{}' created with tags: {}",
                   container_image.name(),
                   container_image.tags().join(", ")))?;

    container_image.create_report(ui, env::current_dir()?.join("results"))?;

    if matches.is_present("PUSH_IMAGE") {
        let credentials = Credentials::new(naming.registry_type,
                                           matches.value_of("REGISTRY_USERNAME")
                                                  .expect("Username not specified"),
                                           matches.value_of("REGISTRY_PASSWORD")
                                                  .expect("Password not specified")).await?;
        push_image(ui,
                   engine.as_ref(),
                   &container_image,
                   &credentials,
                   naming.registry_url.as_deref())?;
    }
    if matches.is_present("RM_IMAGE") {
        remove_image(ui, engine.as_ref(), &container_image)?;
        Ok(None)
    } else {
        Ok(Some(container_image))
    }
}

fn remove_image(ui: &mut UI, engine: &dyn Engine, image: &ContainerImage) -> Result<()> {
    ui.begin(format!("Cleaning up local Docker image '{}' with all tags",
                     image.name()))?;

    for identifier in image.expanded_identifiers() {
        ui.status(Status::Deleting, format!("local image '{}'", identifier))?;
        engine.remove_image(&identifier)?;
    }

    ui.end(format!("Local Docker image '{}' with tags: {} cleaned up",
                   image.name(),
                   image.tags().join(", "),))?;
    Ok(())
}

fn push_image(ui: &mut UI,
              engine: &dyn Engine,
              image: &ContainerImage,
              credentials: &Credentials,
              registry_url: Option<&str>)
              -> Result<()> {
    ui.begin(format!("Pushing Docker image '{}' with all tags to remote registry",
                     image.name()))?;

    // TODO (CM): UGH
    // This is just until we can sort out a better place for the
    // config file. The Engine will probably handle it.
    let workdir = image.workdir();

    create_docker_config_file(credentials, registry_url, workdir)?;

    for image_tag in image.expanded_identifiers() {
        ui.status(Status::Uploading,
                  format!("image '{}' to remote registry", image_tag))?;
        engine.push_image(&image_tag, workdir)?;
        ui.status(Status::Uploaded, format!("image '{}'", image_tag))?;
    }

    ui.end(format!("Docker image '{}' published with tags: {}",
                   image.name(),
                   image.tags().join(", "),))?;
    Ok(())
}

fn create_docker_config_file(credentials: &Credentials,
                             registry_url: Option<&str>,
                             workdir: &Path)
                             -> Result<()> {
    std::fs::create_dir_all(workdir)?; // why wouldn't this already exist?
    let config = workdir.join("config.json");

    let registry = match registry_url {
        Some(url) => url,
        None => "https://index.docker.io/v1/",
    };

    debug!("Using registry: {:?}", registry);
    let json = json!({
        "auths": {
            registry: {
                "auth": credentials.token
            }
        }
    });

    util::write_file(&config, &serde_json::to_string(&json).unwrap())?;
    Ok(())
}
