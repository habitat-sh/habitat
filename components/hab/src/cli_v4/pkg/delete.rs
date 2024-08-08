// Implementation of `hab pkg delete` command

use clap_v4 as clap;

use clap::Parser;

use habitat_common::{cli::PACKAGE_TARGET_ENVVAR,
                     ui::UI};

use habitat_core::package::{target,
                            PackageIdent,
                            PackageTarget};

use crate::{cli_v4::utils::{AuthToken,
                            BldrUrl},
            command::pkg::delete,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgDeleteOptions {
    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Fully Qualified package identifier for the package
    #[arg(name = "PKG_IDENT")]
    pkg_ident: PackageIdent,

    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[structopt(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,

    #[structopt(flatten)]
    auth_token: AuthToken,
}

impl PkgDeleteOptions {
    pub(super) async fn do_delete(&self, ui: &mut UI) -> HabResult<()> {
        let auth_token = self.auth_token.from_cli_or_config()?;

        let target = self.pkg_target.unwrap_or_else(|| {
                                        match PackageTarget::active_target() {
                                            #[cfg(feature = "supported_targets")]
                                            target::X86_64_DARWIN => target::X86_64_LINUX,
                                            t => t,
                                        }
                                    });

        delete::start(ui,
                      &self.bldr_url.to_string(),
                      (&self.pkg_ident, target),
                      &auth_token).await
    }
}
