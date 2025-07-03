use clap_v4 as clap;
use clap::Subcommand;
use habitat_common::{ui::UI};
use crate::error::Result as HabResult;

mod apply;
mod show;

pub use apply::ConfigApplyOptions;
pub use show::ConfigShowOptions;

#[derive(Debug, Clone, Subcommand)]
pub enum ConfigCommand {
    Apply(ConfigApplyOptions),

    Show(ConfigShowOptions),
}

impl ConfigCommand {
    pub async fn do_command(
        &self,
        ui: &mut UI,
    ) -> HabResult<()> {
        match self {
            ConfigCommand::Apply(opts) => opts.do_apply(ui).await,
            ConfigCommand::Show(opts)  => opts.do_show().await,
        }
    }
}
