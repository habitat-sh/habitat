// Implementation of `hab pkg exec` command

use clap_v4 as clap;

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;

use habitat_core::package::PackageIdent;

use crate::{cli_v4::utils::CommandAndArgs,
            command::pkg::exec,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgExecOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    cmd: CommandAndArgs,
}

impl PkgExecOptions {
    pub(super) fn do_exec(&self) -> HabResult<()> {
        // Required to convert to OsStr
        // TODO: This should be internal implementation detail later on and move to actual command
        // implementation when `v2` is removed
        exec::start(&self.pkg_ident, &self.cmd.cmd, &self.cmd.args)
    }
}
