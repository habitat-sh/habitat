use clap_v4 as clap;

use std::convert::Into;

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;
use habitat_core::{os::process::ShutdownTimeout,
                   package::PackageIdent};

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult,
            gateway_util};

/// Unload a service loaded by the Habitat Supervisor. If the service is running, it will be stopped
/// first.
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct UnloadCommand {
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

impl UnloadCommand {
    pub(super) async fn do_command(self) -> HabResult<()> {
        let msg = habitat_sup_protocol::ctl::SvcUnload { ident:              Some(self.pkg_ident
                                                                                      .into()),
                                                         timeout_in_seconds: self.shutdown_timeout
                                                                                 .map(Into::into), };
        gateway_util::send(self.remote_sup.inner(), msg).await
    }
}
