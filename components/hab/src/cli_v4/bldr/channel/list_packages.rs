use crate::{cli_v4::utils::{AuthToken,
                            BldrUrl},
            command::bldr::channel::list_packages::start,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_common::ui::UI;
use habitat_core::{ChannelIdent,
                   origin::Origin};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          about = "List packages promoted to a Builder channel",
          help_template = "{name} {version} {author-section} \
                           {about-section}{usage-heading}{usage}\n\n{all-args}\n")]
pub(crate) struct ListPackagesOpts {
    /// Origin that owns the channel
    #[arg(value_name = "ORIGIN")]
    origin: Origin,

    /// Channel to list
    #[arg(value_name = "CHANNEL")]
    channel: ChannelIdent,

    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Optional for public origins; use --auth for private origins
    #[command(flatten)]
    auth_token: AuthToken,

    /// Return at most this many packages (all packages by default)
    #[arg(short = 'l', long = "limit", value_name = "N")]
    limit: Option<usize>,

    /// Print a JSON array of fully qualified package identifiers
    #[arg(long = "json")]
    json: bool,
}

impl ListPackagesOpts {
    pub(crate) async fn do_list(&self, ui: &mut UI) -> HabResult<()> {
        start(ui,
              &self.bldr_url.to_string(),
              &self.origin,
              &self.channel,
              self.auth_token.try_from_cli_or_config().as_deref(),
              self.limit.unwrap_or(usize::MAX),
              self.json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_builder_options() {
        let opts = ListPackagesOpts::try_parse_from(["list-packages",
                                                     "core",
                                                     "stable",
                                                     "--url",
                                                     "https://builder.example.test",
                                                     "--auth",
                                                     "secret",
                                                     "--limit",
                                                     "2",
                                                     "--json"]).unwrap();
        assert_eq!(opts.origin.to_string(), "core");
        assert_eq!(opts.channel.to_string(), "stable");
        assert_eq!(opts.bldr_url.to_string(), "https://builder.example.test/");
        assert_eq!(opts.auth_token.try_from_cli_or_config().as_deref(),
                   Some("secret"));
        assert_eq!(opts.limit, Some(2));
        assert!(opts.json);
    }
}
