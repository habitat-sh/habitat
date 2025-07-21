use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::ui::UI;

mod apply;
mod show;

use apply::ConfigApplyOptions;
use show::ConfigShowOptions;

#[derive(Debug, Clone, Subcommand)]
#[command(author = "The Habitat Maintainers <humans@habitat.sh>",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum ConfigCommand {
    /// Apply a configuration to a running service
    Apply(ConfigApplyOptions),

    /// Show the current config of a running service
    Show(ConfigShowOptions),
}

impl ConfigCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            ConfigCommand::Apply(opts) => opts.do_apply(ui).await,
            ConfigCommand::Show(opts) => opts.do_show().await,
        }
    }
}
