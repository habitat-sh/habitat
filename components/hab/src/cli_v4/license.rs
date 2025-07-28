use clap_v4 as clap;

use crate::{error::Result as HabResult,
            license};
use clap::Subcommand;
use habitat_common::ui::UI;

#[derive(Clone, Debug, Subcommand)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          about = "Commands relating to Habitat license agreements",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(super) enum LicenseCommand {
    /// Accept the Chef Binary Distribution Agreement without prompting
    Accept,
}

impl LicenseCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            Self::Accept => {
                license::accept_license(ui)?;
                Ok(())
            }
        }
    }
}
