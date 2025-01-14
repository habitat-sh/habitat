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
    /// A path to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: String,
}

impl PkgHeaderOptions {
    pub(super) fn do_header(&self, ui: &mut UI) -> HabResult<()> {
        crypto::init()?;

        header::start(ui, &PathBuf::from(&self.source))
    }
}
