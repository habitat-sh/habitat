
use clap;
use habitat_common as common;
use habitat_core as hcore;
use habitat_pkg_export_docker as export_docker;

#[macro_use]
extern crate serde_json;

use failure;
#[macro_use]
extern crate failure_derive;

use std::{fs::File,
          io::{self,
               prelude::*}};

use crate::common::ui::{Glyph,
                        Status,
                        UIWriter,
                        UI};

pub mod cli;
pub mod env;
pub mod error;
pub mod hb;
pub mod manifest;
pub mod manifestjson;
pub mod storage;
pub mod topology;

use crate::export_docker::Result;

pub use crate::{cli::Cli,
                error::Error,
                hb::QuoteHelper,
                manifest::Manifest,
                manifestjson::ManifestJson,
                storage::PersistentStorage};

// Synced with the version of the Habitat operator.
pub const VERSION: &str = "0.1.0";

/// Convenient do-it-all function. You give it the CLI arguments from the user and it generates the
/// Kubernetes manifest. If user passed an `--output` argument with a value that is not "`-`", the
/// manifest is written to the provided file; otherwise, it is written to the standard output.
pub fn export_for_cli_matches(ui: &mut UI, matches: &clap::ArgMatches<'_>) -> Result<()> {
    let image = if !matches.is_present("NO_DOCKER_IMAGE") {
        export_docker::export_for_cli_matches(ui, &matches)?
    } else {
        ui.status(Status::Custom(Glyph::FingerPoint, String::from("Skipping")),
                  "Docker image generation")?;
        None
    };
    let mut manifest = Manifest::new_from_cli_matches(ui, &matches, image)?;

    let mut write: Box<dyn Write> = match matches.value_of("OUTPUT") {
        Some(o) if o != "-" => {
            ui.status(Status::Creating, format!("Kubernetes manifest file {}", o))?;
            let file = Box::new(File::create(o)?);
            ui.status(Status::Created, format!("Kubernetes manifest file {}", o))?;

            file
        }
        _ => {
            let stdout = Box::new(io::stdout());
            ui.status(Status::Custom(Glyph::RightArrow, String::from("Writing")),
                      "Kubernetes manifest to stdout")?;

            stdout
        }
    };
    manifest.generate(&mut write)
}
