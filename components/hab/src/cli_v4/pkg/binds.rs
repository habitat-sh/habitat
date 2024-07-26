// Implemenatation of `hab pkg binds`

use clap_v4 as clap;

use clap::Parser;

use habitat_core::{fs::FS_ROOT_PATH,
                   package::PackageIdent};

use habitat_common::command::package::binds;

use crate::error::Result as HabResult;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgBindsOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT")]
    pkg_ident: PackageIdent,
}

impl PkgBindsOptions {
    pub(super) fn do_binds(&self) -> HabResult<()> {
        binds::start(&self.pkg_ident, &*FS_ROOT_PATH).map_err(Into::into)
    }
}
