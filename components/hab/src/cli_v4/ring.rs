use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::ui::UI;

mod key;
use key::RingKeyCommand;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          about = "Commands relating to Habitat rings",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum RingCommand {
    /// Commands relating to Habitat ring keys
    #[command(subcommand)]
    Key(RingKeyCommand),
}

impl RingCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            RingCommand::Key(cmd) => cmd.do_command(ui).await,
        }
    }
}
