// Implementation of `hab pkg export` command

use clap_v4 as clap;

use clap::{Args,
           Subcommand};

use habitat_common::ui::UI;

use crate::{command::pkg::export,
            error::Result as HabResult};

#[derive(Debug, Clone, Args)]
pub(crate) struct PkgExportCommandOptions {
    /// Arguments to be passed to the command
    #[arg(name = "ARGS")]
    args: Vec<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub(crate) enum PkgExportCommand {
    /// Cloud Foundry Exporter
    #[cfg(target_os = "linux")]
    Cf(PkgExportCommandOptions),

    /// Container Exporter
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Container(PkgExportCommandOptions),

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[command(hide = true)]
    Docker(PkgExportCommandOptions),

    /// Mesos Exporter
    #[cfg(target_os = "linux")]
    Mesos(PkgExportCommandOptions),

    /// Tar Exporter
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    Tar(PkgExportCommandOptions),
}

impl PkgExportCommand {
    pub(super) async fn do_export(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            #[cfg(target_os = "linux")]
            PkgExportCommand::Cf(opts) => export::cf::start(ui, &opts.args).await,
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            PkgExportCommand::Container(opts) => export::container::start(ui, &opts.args).await,
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            PkgExportCommand::Docker(opts) => {
                ui.warn("'hab pkg export docker' is now a deprecated alias for 'hab pkg export \
                         container'. Please update your automation and processes accordingly.")?;
                export::container::start(ui, &opts.args).await
            }
            #[cfg(target_os = "linux")]
            PkgExportCommand::Mesos(opts) => export::mesos::start(ui, &opts.args).await,
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            PkgExportCommand::Tar(opts) => export::tar::start(ui, &opts.args).await,
        }
    }
}
