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
                         BuildOnUpload,
                         Client},
            command,
            common::ui::{Glyph,
                         Status,
                         UIReader,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            hcore::{package::PackageArchive,
                    ChannelIdent},
            PRODUCT,
            VERSION};
use glob::glob_with;
use reqwest::StatusCode;
use std::{collections::BTreeSet,
          path::{Path,
                 PathBuf}};

/// Bulk Upload the packages from the cache to a Depot.
///
/// # Failures
///
/// * Fails if it cannot create a missing origin
/// * Fails if it cannot upload the artifact
#[allow(clippy::too_many_arguments)]
pub fn start(ui: &mut UI,
             bldr_url: &str,
             additional_release_channel: &Option<ChannelIdent>,
             token: &str,
             artifact_path: &Path,
             force_upload: bool,
             auto_build: BuildOnUpload,
             auto_create_origins: bool,
             key_path: &Path)
             -> Result<()> {
    const OPTIONS: glob::MatchOptions = glob::MatchOptions { case_sensitive:              true,
                                                             require_literal_separator:   true,
                                                             require_literal_leading_dot: true, };
    let artifact_paths =
        vec_from_glob_with(&artifact_path.join("*.hart").display().to_string(), OPTIONS);

    ui.begin(format!("Preparing to upload artifacts to the '{}' channel on {}",
                     additional_release_channel.clone()
                                               .unwrap_or_else(ChannelIdent::unstable),
                     bldr_url))?;
    ui.status(Status::Using,
              format!("{} for artifacts and {} for signing keys",
                      &artifact_path.display(),
                      key_path.display()))?;
    ui.status(Status::Found,
              format!("{} artifact(s) for upload.", artifact_paths.len()))?;
    ui.status(Status::Discovering,
              String::from("origin names from local artifact cache"))?;

    let mut origins = BTreeSet::new();
    for artifact_path in &artifact_paths {
        let ident = PackageArchive::new(&artifact_path).ident()?;
        origins.insert(ident.origin);
    }
    let mut origins_to_create: Vec<String> = Vec::new();
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    for origin in origins {
        match api_client.check_origin(&origin, token) {
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
            command::origin::create::start(ui, &bldr_url, &token, &origin_to_create)?;
        }
    };

    for artifact_path in &artifact_paths {
        command::pkg::upload::start(ui,
                                    &bldr_url,
                                    &additional_release_channel,
                                    &token,
                                    &artifact_path,
                                    force_upload,
                                    auto_build,
                                    &key_path)?
    }

    Ok(())
}

fn vec_from_glob_with(pattern: &str, options: glob::MatchOptions) -> Vec<PathBuf> {
    glob_with(pattern, options).unwrap()
                               .map(std::result::Result::unwrap)
                               .collect()
}

fn ask_create_origins(ui: &mut UI) -> Result<bool> {
    Ok(ui.prompt_yes_no("Create the above missing origins under your Builder account?",
                        Some(true))?)
}
