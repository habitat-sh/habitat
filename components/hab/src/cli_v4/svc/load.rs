use clap_v4 as clap;

use std::convert::TryFrom;

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;
use habitat_core::package::PackageIdent;

use crate::{cli_v4::utils::{shared_load_cli_to_ctl,
                            RemoteSup,
                            SharedLoad},
            error::{Error as HabError,
                    Result as HabResult}};

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

impl TryFrom<LoadCommand> for habitat_sup_protocol::ctl::SvcLoad {
    type Error = HabError;

    fn try_from(cmd: LoadCommand) -> HabResult<Self> {
        shared_load_cli_to_ctl(cmd.pkg_ident, cmd.shared_load, cmd.force)
    }
}
