use clap_v4 as clap;
use clap::Parser;
use crate::{error::Result as HabResult, command::config::sub_svc_config};
use crate::cli_v4::utils::RemoteSup;
use habitat_core::package::PackageIdent;

#[derive(Debug, Clone, Parser)]
#[command(
    arg_required_else_help = true,
    rename_all = "kebab-case",
    help_template = "{name} {version} {author-section} {about-section}\n\
                     {usage-heading} {usage}\n\n{all-args}\n",
    about = "Show the current config of a running service"
)]
pub(crate) struct ConfigShowOptions {
    /// Remote Supervisor control address (overrides HAB_SUP_CTL_ADDR)
    #[command(flatten)]
    remote_sup: RemoteSup,

    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(long)]
    ident: PackageIdent,
}

impl ConfigShowOptions {
    pub(crate) async fn do_show(&self) -> HabResult<()> {
        sub_svc_config(
            self.ident.clone(),
            self.remote_sup.inner().cloned(),
        )
        .await
    }
}
