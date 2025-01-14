// Implementation of `hab pkg demote` command

use clap_v4 as clap;

use clap::Parser;

use habitat_common::{cli::{clap_validators::HabPkgIdentValueParser,
                           PACKAGE_TARGET_ENVVAR},
                     ui::UI};

use habitat_core::{package::{target,
                             PackageIdent,
                             PackageTarget},
                   ChannelIdent};

use crate::{cli_v4::utils::{AuthToken,
                            BldrUrl},
            command::pkg::demote,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgDemoteOptions {
    #[command(flatten)]
    bldr_url: BldrUrl,

    /// A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::full())]
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

impl PkgDemoteOptions {
    pub(super) async fn do_demote(&self, ui: &mut UI) -> HabResult<()> {
        let auth_token = self.auth_token.from_cli_or_config()?;

        let target = self.pkg_target.unwrap_or_else(|| {
                                        match PackageTarget::active_target() {
                                            #[cfg(feature = "supported_targets")]
                                            target::X86_64_DARWIN => target::X86_64_LINUX,
                                            t => t,
                                        }
                                    });

        demote::start(ui,
                      &self.bldr_url.to_string(),
                      (&self.pkg_ident, target),
                      &self.channel,
                      auth_token.as_str()).await
    }
}
