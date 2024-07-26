use clap_v4 as clap;

use clap::{Parser,
           Subcommand};

use habitat_core::{fs::FS_ROOT_PATH,
                   package::PackageIdent};

use habitat_common::command::package::binds;

use crate::error::Result as HabResult;

#[derive(Clone, Debug, Subcommand)]
pub(crate) enum PkgCommand {
    /// Displays the binds for a service
    Binds(PkgBindOptions),

    /// Creates a binlink for a package binary in a common 'PATH' location
    BinLink(PkgBinlinkOptions),

    /// Builds a plan using Habitat Studio
    Build(PkgBuildOptions),

    /// Bulk uploads Habitat artifacts from to a depo from a local directory
    Bulkupload(PkgBulkUploadOptions),

    /// Find out what channels a package belongs to
    Channels(PkgChannelsOptions),

    /// Displays the default configuration options for a service
    Config(PkgConfigOptions),

    /// Removes a package from Builder
    Delete(PkgDeleteOptions),

    /// Demote a package from a specified channel
    Demote(PkgDemoteOptions),

    /// Returns Habitat Artifact dependencies, by default the direct dependencies
    Dependencies(PkgDependenciesOptions),

    /// Download Habitat artifacts (including dependencies and keys) from Builder
    Download(PkgDownloadOptions),

    /// Prints the runtime environment of a specific installed package
    Env(PkgEnvOptions),

    /// Execute a command using the 'PATH' context of an installed package
    Exec(PkgExecOptions),

    /// Exports the package to the specified format
    Export(PkgExportOptions),

    /// Generates a blake2b hashsum from a target at any given filepath
    Hash(PkgHashOptions),

    /// Returns the Habitat Artifact header
    Header(PkgHeaderOptions),

    /// Returns the Habitat Artifact information
    Info(PkgInfoOptions),

    /// Installs a Habitat package from Builder or locally from a Habitat Artifact
    Install(PkgInstallOptions),

    /// List all versions of installed packages
    List(PkgListOptions),

    /// Prints the path to a specific installed release of a package
    Path(PkgPathOptions),

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
    Verify(PkgVerifyOptions),
}

impl PkgCommand {
    pub(crate) fn do_command(&self) -> HabResult<()> {
        match self {
            Self::Binds(bind_options) => {
                binds::start(&bind_options.pkg_ident, &*FS_ROOT_PATH).map_err(Into::into)
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgBindOptions {
    #[arg(name = "PKG_IDENT")]
    pkg_ident: PackageIdent,
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgBinlinkOptions {}

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgPathOptions {}

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgDeleteOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgListOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgDemoteOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgPromoteOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgConfigOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgChannelsOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgUploadOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgVerifyOptions;

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
pub(crate) struct PkgBuildOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgInfoOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgHeaderOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgDependenciesOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgDownloadOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgExecOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgEnvOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgHashOptions;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgExportOptions;
