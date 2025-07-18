use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::ui::UI;

mod channel;
use channel::ChannelCommand;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          about = "Commands relating to Habitat Builder",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum BldrCommand {
    /// Commands relating to Habitat Builder channels
    #[command(subcommand)]
    Channel(ChannelCommand),
}

impl BldrCommand {
    pub async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            BldrCommand::Channel(cmd) => cmd.do_command(ui).await,
        }
    }
}
