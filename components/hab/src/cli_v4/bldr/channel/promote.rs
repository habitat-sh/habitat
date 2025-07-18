use crate::{cli_v4::utils::AuthToken,
            command::bldr::channel::promote::start,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_common::ui::UI;
use habitat_core::{origin::Origin,
                   ChannelIdent};
#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct PromoteOpts {
    /// The channel from which all packages will be selected for promotion
    #[arg(value_name = "SOURCE_CHANNEL", value_parser = clap::value_parser!(ChannelIdent))]
    source_channel: ChannelIdent,

    /// The channel to which packages will be promoted
    #[arg(value_name = "TARGET_CHANNEL", value_parser = clap::value_parser!(ChannelIdent))]
    target_channel: ChannelIdent,

    /// Authentication token for Builder [env: HAB_AUTH_TOKEN]
    #[command(flatten)]
    token: AuthToken,

    /// Specify an alternate Builder endpoint [env: HAB_BLDR_URL] [default: https://bldr.habitat.sh]
    #[arg(short = 'u',
          long = "url",
          value_name = "BLDR_URL",
          env = "HAB_BLDR_URL",
          default_value = "https://bldr.habitat.sh")]
    url: String,

    /// Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN' or cli.toml
    #[arg(short = 'o', long, value_name = "ORIGIN", env = "HAB_ORIGIN", value_parser = clap::value_parser!(Origin))]
    origin: Origin,
}

impl PromoteOpts {
    pub(crate) async fn do_promote(&self, ui: &mut UI) -> HabResult<()> {
        let token = self.token.from_cli_or_config()?;
        start(ui,
              &self.url,
              &token,
              &self.origin,
              &self.source_channel,
              &self.target_channel).await
    }
}
