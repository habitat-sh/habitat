use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::{FeatureFlag,
                     ui::UI};
mod completers;
mod setup;

use completers::CliCompletersOptions;
use setup::CliSetupOptions;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          about = "Commands relating to Habitat runtime config",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum CliCommand {
    /// Sets up the CLI with reasonable defaults
    Setup(CliSetupOptions),

    /// Creates command-line completers for your shell
    Completers(CliCompletersOptions),
}

impl CliCommand {
    pub(crate) async fn do_command(&self,
                                   ui: &mut UI,
                                   feature_flags: FeatureFlag)
                                   -> HabResult<()> {
        match self {
            CliCommand::Setup(opts) => opts.do_setup(ui).await,
            CliCommand::Completers(opts) => opts.do_completers(feature_flags),
        }
    }
}
