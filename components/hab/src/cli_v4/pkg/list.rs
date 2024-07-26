// Implemenatation of `hab pkg list`

use clap_v4 as clap;

use clap::Parser;

use habitat_core::package::PackageIdent;

use crate::{command::pkg::{list,
                           list::ListingType},
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
#[group(required = true, multiple = false)]
pub(crate) struct PkgListOptions {
    /// List all installed packages
    #[arg(name = "ALL", short = 'a', long = "all")]
    all: bool,

    // TODO : Validations
    /// An origin to list
    #[arg(name = "ORIGIN", short = 'o', long = "origin")]
    origin: Option<String>,

    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT")]
    pkg_ident: Option<PackageIdent>,
}

impl PkgListOptions {
    pub(super) fn do_list(&self) -> HabResult<()> { list::start(&self.into()) }
}

impl From<&PkgListOptions> for ListingType {
    fn from(opts: &PkgListOptions) -> Self {
        if opts.all {
            ListingType::AllPackages
        } else if let Some(origin) = &opts.origin {
            ListingType::Origin(origin.clone())
        } else if let Some(ident) = &opts.pkg_ident {
            ListingType::Ident(ident.clone())
        } else {
            unreachable!();
        }
    }
}
