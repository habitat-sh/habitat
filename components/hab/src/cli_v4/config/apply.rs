use clap_v4 as clap;
use clap::Parser;
use std::path::PathBuf;
use habitat_common::{cli::clap_validators::FileExistsOrStdinValueParser,
                     ui::UI};
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
pub(crate) struct ConfigApplyOptions {
    /// Supervisor control address (overrides HAB_SUP_CTL_ADDR)
    #[command(flatten)]
    remote_sup: RemoteSup,

    /// Service group identifier, e.g. `core/redis.default`
    #[arg(value_name = "SERVICE_GROUP")]
    service_group: String,
    
    /// Configuration version number to set
    #[arg(value_name = "VERSION_NUMBER", value_parser = clap::value_parser!(u64))]
    config_version: u64,
    
    /// Path to the config file ("-" for stdin)
    #[arg(value_parser = FileExistsOrStdinValueParser, value_name = "FILE")]
    file: PathBuf,

    /// Encrypt the payload for this username
    #[arg(long)]
    user: Option<String>,
}

impl ConfigApplyOptions {
    pub(crate) async fn do_apply(&self, ui: &mut UI) -> HabResult<()> {
        let service_group = self.service_group.parse()
            .expect("Invalid service group identifier");

        sub_svc_set(
            ui,
            service_group,
            &self.file,
            self.config_version,
            self.user.clone(),
            self.remote_sup.inner().cloned(),
        )
        .await
    }
}
