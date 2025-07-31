// Implementation of `hab pkg export` command

use std::ffi::OsString;

use clap_v4 as clap;

use clap::{Args,
           Subcommand};

use habitat_common::ui::{UIWriter,
                         UI};

use crate::{command::pkg::export,
            error::Result as HabResult};

#[derive(Debug, Clone, Args)]
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
    Container(PkgExportCommandOptions),

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[command(hide = true)]
    Docker(PkgExportCommandOptions),

    /// Tar Exporter
    #[cfg(any(target_os = "linux", target_os = "windows"))]
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
            PkgExportCommand::Docker(opts) => {
                ui.warn("'hab pkg export docker' is now a deprecated alias for 'hab pkg export \
                         container'. Please update your automation and processes accordingly.")?;
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
