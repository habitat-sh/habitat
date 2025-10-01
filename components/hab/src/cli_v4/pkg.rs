// Implementation of `hab pkg` command

use clap_v4 as clap;

use clap::Subcommand;

use habitat_common::{ui::UI,
                     FeatureFlag};

use crate::error::Result as HabResult;

mod binds;
mod binlink;
mod build;
mod bulk_upload;

mod channels;
mod config;

mod delete;
mod demote;
mod dependencies;
mod download;

mod env;
mod exec;

#[cfg(any(all(target_os = "linux",
              any(target_arch = "x86_64", target_arch = "aarch64")),
          all(target_os = "windows", target_arch = "x86_64")))]
mod export;

mod hash;
mod header;

mod info;
mod install;

mod list;

mod uninstall;
mod upload;

mod path;
mod promote;
mod provides;

mod search;
mod sign;

mod verify;

#[derive(Clone, Debug, Subcommand)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(super) enum PkgCommand {
    /// Displays the binds for a service
    Binds(binds::PkgBindsOptions),

    /// Creates a binlink for a package binary in a common 'PATH' location
    Binlink(binlink::PkgBinlinkOptions),

    /// Builds a plan using Habitat Studio
    Build(build::PkgBuildOptions),

    /// Bulk uploads Habitat artifacts to builder depot from a local directory
    Bulkupload(bulk_upload::PkgBulkUploadOptions),

    /// Find out what channels a package belongs to
    Channels(channels::PkgChannelsOptions),

    /// Displays the default configuration options for a service
    Config(config::PkgConfigOptions),

    /// Removes a package from Builder
    Delete(delete::PkgDeleteOptions),

    /// Demote a package from a specified channel
    Demote(demote::PkgDemoteOptions),

    /// Returns Habitat Artifact dependencies, by default the direct dependencies
    /// of the package
    Dependencies(dependencies::PkgDependenciesOptions),

    /// Download Habitat artifacts (including dependencies and keys) from Builder
    Download(download::PkgDownloadOptions),

    /// Prints the runtime environment of a specific installed package
    Env(env::PkgEnvOptions),

    /// Execute a command using the 'PATH' context of an installed package
    Exec(exec::PkgExecOptions),

    #[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64")))]
    #[clap(subcommand)]
    /// Exports the package to the specified format
    Export(export::PkgExportCommand),

    /// Generates a blake2b hashsum from a target at any given filepath
    Hash(hash::PkgHashOptions),

    /// Returns the Habitat Artifact header
    Header(header::PkgHeaderOptions),

    /// Returns the Habitat Artifact information
    Info(info::PkgInfoOptions),

    /// Installs a Habitat package from Builder or locally from a Habitat Artifact
    Install(install::PkgInstallOptions),

    /// List all versions of installed packages
    List(list::PkgListOptions),

    /// Prints the path to a specific installed release of a package
    Path(path::PkgPathOptions),

    /// Promote a package to a specified channel
    Promote(promote::PkgPromoteOptions),

    /// Search installed Habitat packages for a given file
    Provides(provides::PkgProvidesOptions),

    /// Search for a package in Builder
    Search(search::PkgSearchOptions),

    /// Signs an archive with an origin key, generating a Habitat Artifact
    Sign(sign::PkgSignOptions),

    /// Safely uninstall a package and dependencies from a local filesystem
    Uninstall(uninstall::PkgUninstallOptions),

    /// Uploads a local Habitat Artifact to Builder
    Upload(upload::PkgUploadOptions),

    /// Verifies a Habitat Artifact with an origin key
    Verify(verify::PkgVerifyOptions),
}

impl PkgCommand {
    pub(crate) async fn do_command(&self,
                                   ui: &mut UI,
                                   feature_flags: FeatureFlag)
                                   -> HabResult<()> {
        match self {
            Self::Binds(opts) => opts.do_binds(),
            Self::Binlink(opts) => opts.do_binlink(ui),
            Self::Build(opts) => opts.do_build(ui, feature_flags).await,
            Self::Bulkupload(opts) => opts.do_bulkupload(ui).await,

            Self::Channels(opts) => opts.do_channels(ui).await,
            Self::Config(opts) => opts.do_config(),

            Self::Delete(opts) => opts.do_delete(ui).await,
            Self::Demote(opts) => opts.do_demote(ui).await,
            Self::Dependencies(opts) => opts.do_dependencies(),
            Self::Download(opts) => opts.do_download(ui).await,

            Self::Env(opts) => opts.do_env(),
            Self::Exec(opts) => opts.do_exec(),
            #[cfg(any(all(target_os = "linux",
                          any(target_arch = "x86_64", target_arch = "aarch64")),
                      all(target_os = "windows", target_arch = "x86_64")))]
            Self::Export(cmd) => cmd.do_export(ui).await,

            Self::Hash(opts) => opts.do_hash(),
            Self::Header(opts) => opts.do_header(ui),

            Self::Info(opts) => opts.do_info(ui),
            Self::Install(opts) => opts.do_install(ui, feature_flags).await,

            Self::List(opts) => opts.do_list(),

            Self::Path(opts) => opts.do_path(),
            Self::Promote(opts) => opts.do_promote(ui).await,
            Self::Provides(opts) => opts.do_provides(),

            Self::Search(opts) => opts.do_search().await,
            Self::Sign(opts) => opts.do_sign(ui),

            Self::Uninstall(opts) => opts.do_uninstall(ui).await,
            Self::Upload(opts) => opts.do_upload(ui).await,

            Self::Verify(opts) => opts.do_verify(ui),
        }
    }
}
