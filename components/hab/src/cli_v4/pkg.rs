use clap_v4 as clap;

use clap::{Parser,
           Subcommand};

use habitat_common::ui::UI;

use crate::error::Result as HabResult;

mod binds;
mod binlink;
mod build;

mod config;

mod env;

mod hash;
mod header;

mod info;

mod path;

mod verify;

#[derive(Clone, Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub(super) enum PkgCommand {
    /// Displays the binds for a service
    Binds(binds::PkgBindsOptions),

    /// Creates a binlink for a package binary in a common 'PATH' location
    Binlink(binlink::PkgBinlinkOptions),

    /// Builds a plan using Habitat Studio
    Build(build::PkgBuildOptions),

    /// Bulk uploads Habitat artifacts from to a depo from a local directory
    Bulkupload(PkgBulkUploadOptions),

    /// Find out what channels a package belongs to
    Channels(PkgChannelsOptions),

    /// Displays the default configuration options for a service
    Config(config::PkgConfigOptions),

    /// Removes a package from Builder
    Delete(PkgDeleteOptions),

    /// Demote a package from a specified channel
    Demote(PkgDemoteOptions),

    /// Returns Habitat Artifact dependencies, by default the direct dependencies
    /// of the package
    Dependencies(PkgDependenciesOptions),

    /// Download Habitat artifacts (including dependencies and keys) from Builder
    Download(PkgDownloadOptions),

    /// Prints the runtime environment of a specific installed package
    Env(env::PkgEnvOptions),

    /// Execute a command using the 'PATH' context of an installed package
    Exec(PkgExecOptions),

    /// Exports the package to the specified format
    Export(PkgExportOptions),

    /// Generates a blake2b hashsum from a target at any given filepath
    Hash(hash::PkgHashOptions),

    /// Returns the Habitat Artifact header
    Header(header::PkgHeaderOptions),

    /// Returns the Habitat Artifact information
    Info(info::PkgInfoOptions),

    /// Installs a Habitat package from Builder or locally from a Habitat Artifact
    Install(PkgInstallOptions),

    /// List all versions of installed packages
    List(PkgListOptions),

    /// Prints the path to a specific installed release of a package
    Path(path::PkgPathOptions),

    /// Promote a package to a specified channel
    Promote(PkgPromoteOptions),

    /// Search installed Habitat packages for a given file
    Provides(PkgProvidesOptions),

    /// Search for a package in Builder
    Search(PkgSearchOptions),

    /// Signs an archive with an origin key, generating a Habitat Artifact
    Sign(PkgSignOptions),

    /// Safely uninstall a package and dependencies from a local filesystem
    Uninstall(PkgUninstallOptions),

    /// Uploads a local Habitat Artifact to Builder
    Upload(PkgUploadOptions),

    /// Verifies a Habitat Architect with an origin key
    Verify(verify::PkgVerifyOptions),
}

impl PkgCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            Self::Binds(opts) => opts.do_binds(),
            Self::Binlink(opts) => opts.do_binlink(ui),
            Self::Path(opts) => opts.do_path(),
            Self::Build(opts) => opts.do_build(ui).await,
            Self::Config(opts) => opts.do_config(),
            Self::Env(opts) => opts.do_env(),
            Self::Hash(opts) => opts.do_hash(),
            Self::Header(opts) => opts.do_header(ui),
            Self::Info(opts) => opts.do_info(ui),
            Self::Verify(opts) => opts.do_verify(ui),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgDeleteOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgListOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgDemoteOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgPromoteOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgChannelsOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgUploadOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgUninstallOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgInstallOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgProvidesOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgSearchOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgSignOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgBulkUploadOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgDependenciesOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgDownloadOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgExecOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgExportOptions;
