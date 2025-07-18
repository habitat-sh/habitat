use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::ui::UI;

mod create;
mod demote;
mod destroy;
mod list;
mod promote;

use create::CreateOpts;
use demote::DemoteOpts;
use destroy::DestroyOpts;
use list::ListOpts;
use promote::PromoteOpts;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          about = "Commands relating to Habitat Builder channels",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum ChannelCommand {
    /// Creates a new channel
    Create(CreateOpts),

    /// Destroys a channel
    Destroy(DestroyOpts),

    /// Lists origin channels
    List(ListOpts),

    /// Atomically promotes all packages in channel
    Promote(PromoteOpts),

    /// Atomically demotes selected packages in a target channel
    Demote(DemoteOpts),
}

impl ChannelCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            ChannelCommand::Create(opts) => opts.do_create(ui).await,
            ChannelCommand::Destroy(opts) => opts.do_destroy(ui).await,
            ChannelCommand::List(opts) => opts.do_list(ui).await,
            ChannelCommand::Promote(opts) => opts.do_promote(ui).await,
            ChannelCommand::Demote(opts) => opts.do_demote(ui).await,
        }
    }
}
