// Implementation of `hab pkg info` command
use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use habitat_core::crypto;

use habitat_common::{cli::clap_validators::FileExistsValueParser,
                     ui::UI};

use crate::{command::pkg::info,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgInfoOptions {
    /// Output will be rendered in json. (Includes extended metadata)
    #[arg(name = "TO_JSON",
          short = 'j',
          long = "json",
          action = ArgAction::SetTrue)]
    json: bool,

    /// A path to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: PathBuf,
}

impl PkgInfoOptions {
    pub(super) fn do_info(&self, ui: &mut UI) -> HabResult<()> {
        crypto::init()?;

        info::start(ui, &self.source, self.json)
    }
}
