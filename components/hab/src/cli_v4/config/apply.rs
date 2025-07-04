use clap_v4 as clap;
use clap::Parser;
use std::path::PathBuf;
use habitat_common::ui::UI;
use crate::{error::Result as HabResult, command::config::sub_svc_set};
use crate::cli_v4::utils::RemoteSup;

#[derive(Debug, Clone, Parser)]
#[command(
    arg_required_else_help = true,
    rename_all = "kebab-case",
    help_template = "{name} {version} {author-section} {about-section}\n\
                     {usage-heading} {usage}\n\n{all-args}\n",
    about = "Apply a configuration to a running service"
)]
pub struct ConfigApplyOptions {
    /// Supervisor control address (overrides HAB_SUP_CTL_ADDR)
    #[command(flatten)]
    pub remote_sup: RemoteSup,

    /// Service group identifier, e.g. `core/redis.default`
    #[arg(long)]
    pub group: String,

    /// Path to the config file ("-" for stdin)
    #[arg(long)]
    pub file: PathBuf,

    /// Configuration version number to set
    #[arg(long, default_value_t = 0)]
    pub version: u64,

    /// Encrypt the payload for this username
    #[arg(long)]
    pub user: Option<String>,
}

impl ConfigApplyOptions {
    pub async fn do_apply(&self, ui: &mut UI) -> HabResult<()> {
        let service_group = self.group.parse()
            .expect("Invalid service group identifier");

        sub_svc_set(
            ui,
            service_group,
            &self.file,
            self.version,
            self.user.clone(),
            self.remote_sup.inner().cloned(),
        )
        .await
    }
}
