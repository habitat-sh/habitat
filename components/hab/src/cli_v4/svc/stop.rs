use clap_v4 as clap;

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;
use habitat_core::{os::process::ShutdownTimeout,
                   package::PackageIdent};

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult,
            gateway_util};

/// Stop a running Habitat service.
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct StopCommand {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    remote_sup: RemoteSup,

    /// The delay in seconds after sending the shutdown signal to wait before killing the service
    /// process
    ///
    /// The default value can be set in the packages plan file.
    #[arg(long = "shutdown-timeout")]
    pub shutdown_timeout: Option<ShutdownTimeout>,
}

impl StopCommand {
    pub(crate) async fn do_command(&self) -> HabResult<()> {
        let remote_sup = self.remote_sup.clone();
        let msg = habitat_sup_protocol::ctl::SvcStop { ident:              Some(self.pkg_ident
                                                                                    .clone()
                                                                                    .into()),
                                                       timeout_in_seconds: self.shutdown_timeout
                                                                               .map(Into::into), };
        gateway_util::send(remote_sup.inner(), msg).await
    }
}
