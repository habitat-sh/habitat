use clap_v4 as clap;

use clap::Parser;

use crate::{cli_v4::utils::origin_param_or_env,
            command::bldr::channel::list::start,
            error::Result as HabResult};

use habitat_common::ui::UI;
use habitat_core::origin::Origin;

#[derive(Debug, Clone, Parser)]
#[command(help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct ListOpts {
    /// Include sandbox channels for the origin
    #[arg(name = "SANDBOX", short = 's', long = "sandbox")]
    sandbox: bool,

    /// Only list channels whose names contain this string (case-sensitive)
    #[arg(long = "filter", value_name = "STRING", value_parser = non_blank)]
    filter: Option<String>,

    /// Specify an alternate Builder endpoint
    #[arg(short = 'u',
          long = "url",
          value_name = "BLDR_URL",
          env = "HAB_BLDR_URL",
          default_value = "https://bldr.habitat.sh")]
    url: String,

    /// Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN' or cli.toml
    #[arg(value_name = "ORIGIN", index=1, value_parser = clap::value_parser!(Origin))]
    origin: Option<Origin>,
}

fn non_blank(s: &str) -> Result<String, String> {
    if s.trim().is_empty() {
        Err("filter value must not be empty or whitespace-only".to_string())
    } else {
        Ok(s.to_string())
    }
}

impl ListOpts {
    pub(crate) async fn do_list(&self, ui: &mut UI) -> HabResult<()> {
        let origin = origin_param_or_env(&self.origin)?;
        start(ui, &self.url, &origin, self.sandbox, self.filter.as_deref()).await
    }
}
