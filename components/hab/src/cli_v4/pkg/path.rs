// Implementation of `hab pkg path`

use clap_v4 as clap;

use clap::Parser;

use habitat_core::{fs::FS_ROOT_PATH,
                   package::PackageIdent};

use crate::{command::pkg::path,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgPathOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT")]
    pkg_ident: PackageIdent,
}

impl PkgPathOptions {
    pub(super) fn do_path(&self) -> HabResult<()> { path::start(&self.pkg_ident, &FS_ROOT_PATH) }
}
