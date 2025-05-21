use clap_v4 as clap;

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;
use habitat_core::package::PackageIdent;

use crate::cli_v4::utils::{RemoteSup,
                           SharedLoad};

/// Load a service to be started and supervised by Habitat from a package identifier.
///
/// If an installed package doesn't satisfy the given package identifier, a suitable package will
/// be installed from Builder.
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct LoadCommand {
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    /// Load or reload an already loaded service. If the service was previously loaded and
    /// running this operation will also restart the service
    #[arg(short = 'f', long = "force")]
    force: bool,

    #[command(flatten)]
    remote_sup: RemoteSup,

    #[command(flatten)]
    shared_load: SharedLoad,
}
