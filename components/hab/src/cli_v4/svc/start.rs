use clap_v4 as clap;

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;
use habitat_core::package::PackageIdent;

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult,
            gateway_util};

/// Start a loaded, but stopped, Habitat service.
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct StartCommand {
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    remote_sup: RemoteSup,
}

impl StartCommand {
    pub(crate) async fn do_command(&self) -> HabResult<()> {
        let remote_sup = self.remote_sup.clone();
        let msg =
            habitat_sup_protocol::ctl::SvcStart { ident: Some(self.pkg_ident.clone().into()), };
        gateway_util::send(remote_sup.inner(), msg).await
    }
}
