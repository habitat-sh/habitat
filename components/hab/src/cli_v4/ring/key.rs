use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::ui::UI;

mod export;
mod generate;
mod import;

use export::RingKeyExportOpts;
use generate::RingKeyGenerateOpts;
use import::RingKeyImportOpts;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum RingKeyCommand {
    /// Outputs the latest ring key contents to stdout
    Export(RingKeyExportOpts),

    /// Reads a stdin stream containing ring key contents and writes the key to disk
    Import(RingKeyImportOpts),

    /// Generates a Habitat ring key
    Generate(RingKeyGenerateOpts),
}

impl RingKeyCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            RingKeyCommand::Export(opts) => opts.do_export().await,
            RingKeyCommand::Generate(opts) => opts.do_generate(ui).await,
            RingKeyCommand::Import(opts) => opts.do_import(ui).await,
        }
    }
}
