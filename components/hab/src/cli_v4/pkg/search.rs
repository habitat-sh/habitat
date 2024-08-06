// Implementation of `hab pkg search` command

use clap_v4 as clap;

use clap::Parser;

use crate::{cli_v4::utils::{AuthToken,
                            BldrUrl},
            command::pkg::search,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgSearchOptions {
    /// Search term
    #[structopt(name = "SEARCH_TERM")]
    search_term: String,

    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Limit how many packages to retrieve
    #[arg(name = "LIMIT", short = 'l', long = "limit", default_value_t = 50)]
    limit: usize,
}

impl PkgSearchOptions {
    pub(crate) async fn do_search(&self) -> HabResult<()> {
        let auth_token = self.auth_token.try_from_cli_or_config();

        search::start(&self.search_term,
                      &self.bldr_url.to_string(),
                      self.limit,
                      auth_token.as_deref()).await
    }
}
