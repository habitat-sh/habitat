// Implementation of `hab pkg provides` command

use clap_v4 as clap;

use clap::{ArgAction,
           Parser};

use habitat_core::fs::FS_ROOT_PATH;

use crate::{command::pkg::provides,
            error::Result as HabResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgProvidesOptions {
    /// File name to find
    #[arg(name = "FILE")]
    file: String,

    /// Show fully qualified package names (ex: core/busybox-static/1.24.2/20160708162350)
    #[arg(name = "FULL_RELEASES", short = 'r', action = ArgAction::SetTrue)]
    full_releases: bool,

    /// Show full path to file
    #[arg(name = "FULL_PATHS", short = 'p', action = ArgAction::SetTrue)]
    full_paths: bool,
}

impl PkgProvidesOptions {
    pub(super) fn do_provides(&self) -> HabResult<()> {
        provides::start(&self.file,
                        &FS_ROOT_PATH,
                        self.full_releases,
                        self.full_paths)
    }
}
