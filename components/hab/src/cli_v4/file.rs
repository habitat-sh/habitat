use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::ui::UI;

mod upload;
use upload::FileUploadOptions;

#[derive(Debug, Clone, Subcommand)]
#[command(about = "Commands relating to Habitat files",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum FileCommand {
    /// Uploads a file to be shared between members of a Service Group
    Upload(FileUploadOptions),
}

impl FileCommand {
    pub async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            FileCommand::Upload(opts) => opts.do_upload(ui).await,
        }
    }
}
