use clap_v4 as clap;

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;
use habitat_core::package::PackageIdent;

use crate::cli_v4::utils::RemoteSup;

/// Query the status of a Habitat Service
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct StatusCommand {
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    remote_sup: RemoteSup,
}
