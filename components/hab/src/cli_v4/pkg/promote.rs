// Implementation of `hab pkg promote` command

use clap_v4 as clap;

use clap::Parser;

use habitat_common::{cli::PACKAGE_TARGET_ENVVAR,
                     ui::UI};

use habitat_core::{package::{target,
                             PackageIdent,
                             PackageTarget},
                   ChannelIdent};

use crate::{cli_v4::utils::{AuthToken,
                            BldrUrl},
            command::pkg::promote,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgPromoteOptions {
    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Fully Qualified package identifier for the package
    #[arg(name = "PKG_IDENT")]
    pkg_ident: PackageIdent,

    /// Demote from the specified release channel
    #[arg(name = "CHANNEL")]
    channel: ChannelIdent,

    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[arg(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,

    #[command(flatten)]
    auth_token: AuthToken,
}

impl PkgPromoteOptions {
    pub(super) async fn do_promote(&self, ui: &mut UI) -> HabResult<()> {
        let auth_token = self.auth_token.from_cli_or_config()?;

        let target = self.pkg_target.unwrap_or_else(|| {
                                        match PackageTarget::active_target() {
                                            #[cfg(feature = "supported_targets")]
                                            target::X86_64_DARWIN => target::X86_64_LINUX,
                                            t => t,
                                        }
                                    });

        promote::start(ui,
                       &self.bldr_url.to_string(),
                       (&self.pkg_ident, target),
                       &self.channel,
                       auth_token.as_str()).await
    }
}
