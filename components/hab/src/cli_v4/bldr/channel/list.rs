use clap_v4 as clap;

use clap::Parser;

use crate::{cli_v4::utils::origin_param_or_env,
            command::bldr::channel::list::start,
            error::Result as HabResult};

use habitat_common::ui::UI;
use habitat_core::origin::Origin;

#[derive(Debug, Clone, Parser)]
#[command(override_usage = "hab bldr channel list [FLAGS] [OPTIONS] <ORIGIN>",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct ListOpts {
    /// Include sandbox channels for the origin
    #[arg(short = 's', long = "sandbox")]
    sandbox: bool,

    /// Specify an alternate Builder endpoint [env: HAB_BLDR_URL] [default: https://bldr.habitat.sh]
    #[arg(short = 'u',
          long = "auth",
          value_name = "BLDR_URL",
          env = "HAB_BLDR_URL",
          default_value = "https://bldr.habitat.sh")]
    url: String,

    /// Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN' or cli.toml
    #[arg(short = 'o', long, value_name = "ORIGIN", value_parser = clap::value_parser!(Origin))]
    origin: Option<Origin>,
}

impl ListOpts {
    pub(crate) async fn do_list(&self, ui: &mut UI) -> HabResult<()> {
        let origin = origin_param_or_env(&self.origin)?;
        start(ui, &self.url, &origin, self.sandbox).await
    }
}
