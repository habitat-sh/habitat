// Implementation of `hab pkg header` command
use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;

use habitat_core::crypto;

use habitat_common::ui::UI;

use crate::{command::pkg::header,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgHeaderOptions {
    /// Filepath to the Habitat Package file
    #[arg(name = "SOURCE")]
    source: PathBuf, /* TODO: Convert it to more semantic `PathBuf`, when we get rid of
                      * `clap-v2` functionality, revisit `command::pkg::hash` */
}

impl PkgHeaderOptions {
    pub(super) fn do_header(&self, ui: &mut UI) -> HabResult<()> {
        crypto::init()?;

        header::start(ui, &self.source)
    }
}
