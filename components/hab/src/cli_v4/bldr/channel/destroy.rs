use crate::{cli_v4::utils::{AuthToken,
                            origin_param_or_env},
            command::bldr::channel::destroy::start,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_common::ui::UI;
use habitat_core::{ChannelIdent,
                   origin::Origin};
#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct DestroyOpts {
    /// The channel name
    #[arg(value_name = "CHANNEL", value_parser = clap::value_parser!(ChannelIdent))]
    channel: ChannelIdent,

    /// Specify an alternate Builder endpoint
    #[arg(short = 'u',
          long,
          value_name = "BLDR_URL",
          env = "HAB_BLDR_URL",
          default_value = "https://bldr.habitat.sh")]
    url: String,

    /// Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN' or cli.toml
    #[arg(short = 'o', long, value_name = "ORIGIN", value_parser = clap::value_parser!(Origin))]
    origin: Option<Origin>,

    /// Authentication token for Builder
    #[command(flatten)]
    token: AuthToken,
}

impl DestroyOpts {
    pub(crate) async fn do_destroy(&self, ui: &mut UI) -> HabResult<()> {
        let origin = origin_param_or_env(&self.origin)?;
        let token = self.token.from_cli_or_config()?;
        start(ui, &self.url, &token, &origin, &self.channel).await
    }
}
