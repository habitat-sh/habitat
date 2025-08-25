use crate::{build::BuildSpec,
            cli::cli,
            container::{BuildContext,
                        ContainerImage},
            engine::Engine,
            error::Error,
            naming::Naming};

use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_credential_types::{provider::SharedCredentialsProvider,
                           Credentials as AwsCredentials};
use aws_sdk_ecr as ecr;
use habitat_common::ui::{Status,
                         UIWriter,
                         UI};
use log::debug;
use serde_json::json;
use std::{convert::TryFrom,
          env,
          fmt,
          path::Path,
          result,
          str::FromStr};

#[cfg(not(windows))]
use crate::engine::fail_if_buildah_and_multilayer;

#[cfg(unix)]
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
const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// The Habitat Package Identifier string for a Busybox package.
const BUSYBOX_IDENT: &str = "core/busybox-static";

/// The Habitat Package Identifier string for SSL certificate authorities (CA) certificates package.
const CACERTS_IDENT: &str = "core/cacerts";

const DEFAULT_AWS_REGION: &str = "us-west-2";

// Default values of parameters
#[cfg(unix)]
const DEFAULT_BASE_IMAGE: &str = "scratch";
#[cfg(windows)]
const DEFAULT_BASE_IMAGE: &str = "mcr.microsoft.com/windows/servercore";

#[cfg(unix)]
const DEFAULT_USER_AND_GROUP_ID: u32 = 42;

#[cfg(unix)]
const DEFAULT_HAB_UID: u32 = 84;

#[cfg(unix)]
const DEFAULT_HAB_GID: u32 = 84;

#[derive(Clone, Copy, Debug, Default, clap::ValueEnum)]
pub(crate) enum RegistryType {
    Amazon,
    Azure,
    #[default]
    Docker,
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

/// A credentials username and password pair.
///
/// This is a value struct which references username and password values.
#[derive(Debug)]
pub(crate) struct Credentials {
    token: String,
}

impl Credentials {
    pub(crate) async fn new(registry_type: RegistryType,
                            username: &str,
                            password: &str)
                            -> Result<Self> {
        match registry_type {
            RegistryType::Amazon => {
                // The username and password should be valid IAM credentials
                let provider =
                    SharedCredentialsProvider::new(AwsCredentials::new(username.to_string(),
                                                                       password.to_string(),
                                                                       None,
                                                                       None,
                                                                       "static"));
                // TODO TED: Make the region configurable
                let loader = aws_config::defaults(BehaviorVersion::latest())
                    .region(ecr::config::Region::new(DEFAULT_AWS_REGION.to_string()))
                    .credentials_provider(provider.clone());
                let cfg = loader.load().await;
                let client = ecr::Client::new(&cfg);

                let token = client.get_authorization_token()
                                  .send()
                                  .await
                                  .map_err(|e| Error::TokenFetchFailed(Box::new(e)))
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
                Ok(Credentials { token: habitat_core::base64::encode(format!("{}:{}",
                                                                             username, password)), })
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
async fn export_for_cli_matches(ui: &mut UI,
                                matches: &clap::ArgMatches)
                                -> Result<Option<ContainerImage>> {
    os::ensure_proper_docker_platform()?;

    #[cfg(not(windows))]
    fail_if_buildah_and_multilayer(matches)?;

    let spec = BuildSpec::try_from(matches)?;
    let naming = Naming::from(matches);
    let engine: Box<dyn Engine> = TryFrom::try_from(matches)?;
    let memory = matches.get_one::<String>("MEMORY_LIMIT");

    ui.begin(format!("Building a container image with: {}",
                     spec.idents_or_archives.join(", ")))?;

    let build_context = BuildContext::from_build_root(spec.create(ui).await?, ui)?;
    let container_image =
        build_context.export(ui, &naming, memory.map(String::as_str), engine.as_ref())?;

    build_context.destroy(ui)?;
    ui.end(format!("Container image '{}' created with tags: {}",
                   container_image.name(),
                   container_image.tags().join(", ")))?;

    container_image.create_report(ui, env::current_dir()?.join("results"))?;

    if matches.get_flag("PUSH_IMAGE") {
        let credentials = Credentials::new(naming.registry_type,
                                           matches.get_one::<String>("REGISTRY_USERNAME")
                                                  .expect("Username not specified"),
                                           matches.get_one::<String>("REGISTRY_PASSWORD")
                                                  .expect("Password not specified")).await?;
        push_image(ui,
                   engine.as_ref(),
                   &container_image,
                   &credentials,
                   naming.registry_url.as_deref())?;
    }
    if matches.get_flag("RM_IMAGE") {
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
        engine.remove_image(identifier)?;
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
        engine.push_image(image_tag, workdir)?;
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

    let registry = registry_url.unwrap_or("https://index.docker.io/v1/");

    debug!("Using registry: {:?}", registry);
    let json = json!({
        "auths": {
            registry: {
                "auth": credentials.token
            }
        }
    });

    util::write_file(config, &serde_json::to_string(&json).unwrap())?;
    Ok(())
}

/// cli_driver: Public API for the package
pub async fn cli_driver(ui: &mut UI) -> Result<()> {
    let cli = cli();
    let m = cli.get_matches();
    debug!("clap cli args: {:?}", m);
    export_for_cli_matches(ui, &m).await.map(|_| ())
}
