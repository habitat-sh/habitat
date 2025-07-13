use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::ui::UI;

mod key;
use key::UserKeyCommand;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          about = "Commands relating to Habitat users",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum UserCommand {
    /// Commands relating to Habitat user keys
    #[command(subcommand)]
    Key(UserKeyCommand),
}

impl UserCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            UserCommand::Key(cmd) => cmd.do_key(ui).await,
        }
    }
}
