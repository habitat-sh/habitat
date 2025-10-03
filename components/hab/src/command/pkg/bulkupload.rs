//! Uploads packages from cache to a [Depot](../depot).
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg bulkupload /path/to/artifact_download_dir \
//!     -u http://localhost:9632
//! ```
//!
//! Will upload all packages in cache to Builder.

use crate::{api_client::{self,
                         Client},
            command,
            common::ui::{Glyph,
                         Status,
                         UIReader,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use glob::glob_with;
use habitat_core::{crypto::{keys::{Key,
                                   KeyCache,
                                   PublicOriginSigningKey},
                            PUBLIC_KEY_SUFFIX},
                   ChannelIdent};
use log::debug;
use reqwest::StatusCode;
use std::{collections::BTreeSet,
          convert::TryFrom,
          path::{Path,
                 PathBuf}};

/// Bulk Upload the packages from the cache to a Depot.
///
/// # Failures
///
/// * Fails if it cannot create a missing origin
/// * Fails if it cannot upload the artifact
#[allow(clippy::too_many_arguments)]
pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   additional_release_channel: &Option<ChannelIdent>,
                   token: &str,
                   artifact_path: &Path,
                   force_upload: bool,
                   auto_create_origins: bool,
                   key_cache: &KeyCache)
                   -> Result<()> {
    let artifact_paths = paths_with_extension(artifact_path, "hart");
    let pub_keys_paths = paths_with_extension(key_cache.as_ref(), PUBLIC_KEY_SUFFIX);

    ui.begin(format!("Preparing to upload artifacts to the '{}' channel on {}",
                     additional_release_channel.clone()
                                               .unwrap_or_else(ChannelIdent::unstable),
                     bldr_url))?;
    ui.status(Status::Using,
              format!("{} for artifacts and {} for signing keys",
                      &artifact_path.display(),
                      key_cache.as_ref().display()))?;
    ui.status(Status::Found,
              format!("{} artifact(s) for upload.", artifact_paths.len()))?;
    ui.status(Status::Discovering,
              String::from("origin names from local key cache"))?;

    let mut origins = BTreeSet::new();
    for pub_key_path in pub_keys_paths {
        // We discover origin names from the public signing keys as opposed to building the
        // list from the packages themselves. Previously, we looped through all the artifacts
        // using PackageArchive::new() but that proves too expensive an operation at any sort of
        // scale. Relevant: https://github.com/habitat-sh/habitat/issues/5153
        debug!("Parsing public signing key {}", pub_key_path.display());
        let public_key: PublicOriginSigningKey = TryFrom::try_from(pub_key_path)?;
        origins.insert(public_key.named_revision().name().to_string());
    }
    let mut origins_to_create: Vec<String> = Vec::new();
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    for origin in origins {
        match api_client.check_origin(&origin, token).await {
            Ok(()) => {
                ui.status(Status::Custom(Glyph::CheckMark,
                                         format!("Origin '{}' already exists", &origin)),
                          String::from(""))?;
            }
            Err(api_client::Error::APIError(StatusCode::NOT_FOUND, _)) => {
                ui.status(Status::Missing, format!("origin '{}'", &origin))?;
                origins_to_create.push(origin);
            }
            Err(err) => return Err(Error::from(err)),
        }
    }

    if !origins_to_create.is_empty() {
        if !auto_create_origins {
            ui.warn(String::from("Origins are required for uploading the artifacts. The \
                                  Builder account that creates the origin is the owner."))?;
            if !ask_create_origins(ui)? {
                return Ok(());
            };
        };
        for origin_to_create in origins_to_create {
            command::origin::create::start(ui, bldr_url, token, &origin_to_create).await?;
        }
    };

    for artifact_path in &artifact_paths {
        command::pkg::upload::start(ui,
                                    bldr_url,
                                    additional_release_channel,
                                    token,
                                    artifact_path,
                                    force_upload,
                                    key_cache).await?
    }

    Ok(())
}

fn paths_with_extension<P>(path: P, suffix: &str) -> Vec<PathBuf>
    where P: AsRef<Path>
{
    let options = glob::MatchOptions { case_sensitive:              true,
                                       require_literal_separator:   true,
                                       require_literal_leading_dot: true, };
    let pattern = format!("*.{}", suffix);
    glob_with(&path.as_ref().join(pattern).display().to_string(),
              options).expect("Failed to read glob pattern")
                      .filter_map(std::result::Result::ok)
                      .collect()
}

fn ask_create_origins(ui: &mut UI) -> Result<bool> {
    Ok(ui.prompt_yes_no("Create the above missing origins under your Builder account?",
                        Some(true))?)
}
