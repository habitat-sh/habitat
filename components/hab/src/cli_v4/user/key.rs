use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::ui::UI;

mod generate;
use generate::UserKeyGenerateOptions;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          about = "Commands relating to Habitat user keys",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum UserKeyCommand {
    /// Generates a Habitat user key
    Generate(UserKeyGenerateOptions),
}

impl UserKeyCommand {
    pub(crate) async fn do_key(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            UserKeyCommand::Generate(opts) => opts.do_generate(ui).await,
        }
    }
}
