// Implementation of `hab pkg header` command
use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;

use habitat_core::crypto;

use habitat_common::{cli::clap_validators::FileExistsValueParser,
                     ui::UI};

use crate::{command::pkg::header,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgHeaderOptions {
    /// Filepath to the Habitat Package file
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: PathBuf,
}

impl PkgHeaderOptions {
    pub(super) fn do_header(&self, ui: &mut UI) -> HabResult<()> {
        crypto::init()?;

        header::start(ui, &self.source)
    }
}
