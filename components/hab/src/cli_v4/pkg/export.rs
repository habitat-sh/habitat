// Implementation of `hab pkg export` command

use std::ffi::OsString;

use clap_v4 as clap;

use clap::{Args,
           Subcommand};

use habitat_common::ui::UI;

use crate::{command::pkg::export,
            error::Result as HabResult};

#[derive(Debug, Clone, Args)]
#[command(trailing_var_arg = true, allow_hyphen_values = true)]
pub(crate) struct PkgExportCommandOptions {
    /// Arguments to be passed to the command
    #[arg(name = "ARGS")]
    args: Vec<String>,
}

#[derive(Debug, Clone, Subcommand)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) enum PkgExportCommand {
    /// Container Exporter
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[command(disable_help_flag = true)]
    Container(PkgExportCommandOptions),

    /// Tar Exporter
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[command(disable_help_flag = true)]
    Tar(PkgExportCommandOptions),
}

impl PkgExportCommand {
    pub(super) async fn do_export(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            PkgExportCommand::Container(opts) => {
                export::container::start(ui,
                                         &opts.args
                                              .iter()
                                              .map(OsString::from)
                                              .collect::<Vec<_>>()).await
            }

            #[cfg(any(target_os = "linux", target_os = "windows"))]
            PkgExportCommand::Tar(opts) => {
                export::tar::start(ui,
                                   &opts.args
                                        .iter()
                                        .map(OsString::from)
                                        .collect::<Vec<_>>()).await
            }
        }
    }
}
