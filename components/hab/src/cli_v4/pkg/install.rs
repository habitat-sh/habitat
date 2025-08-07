// Implementation of `hab pkg install` command

use clap_v4 as clap;

use std::path::PathBuf;

use clap::{builder::NonEmptyStringValueParser,
           parser::ValueSource,
           ArgAction,
           CommandFactory,
           Parser};

use habitat_core::{env::Config,
                   fs::{cache_artifact_path,
                        FS_ROOT_PATH},
                   package::PackageIdent,
                   ChannelIdent};

use habitat_common::{cli::{BINLINK_DIR_ENVVAR,
                           DEFAULT_BINLINK_DIR},
                     command::package::install::{self,
                                                 InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     ui::UI,
                     FeatureFlag,
                     FEATURE_FLAGS};

use crate::{command::pkg::binlink,
            error::Result as HabResult,
            PRODUCT,
            VERSION};

use crate::cli_v4::utils::{AuthToken,
                           BldrUrl};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          rename_all = "screaming_snake",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgInstallOptions {
    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Install from the specified release channel
    #[arg(short = 'c',
                long = "channel",
                env = habitat_core::ChannelIdent::ENVVAR)]
    channel: Option<ChannelIdent>,

    /// One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a Habitat
    /// Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[arg(required = true)]
    pkg_ident_or_artifact: Vec<InstallSource>,

    /// Binlink all binaries from installed package(s) into BINLINK_DIR
    #[arg(short = 'b', long = "binlink")]
    binlink: bool,

    /// Binlink all binaries from installed package(s) into BINLINK_DIR
    #[arg(long = "binlink-dir",
                default_value = DEFAULT_BINLINK_DIR,
                env = BINLINK_DIR_ENVVAR, value_parser = NonEmptyStringValueParser::new())]
    binlink_dir: String,

    /// Overwrite existing binlinks
    #[arg(short = 'f', long = "force", action = ArgAction::SetTrue)]
    force: bool,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Do not run any install hooks
    #[arg(long = "ignore-install-hook", action = ArgAction::SetTrue)]
    ignore_install_hook: bool,

    /// Install packages in offline mode
    #[arg(long = "offline",
        action = ArgAction::SetTrue,
        hide = !FEATURE_FLAGS.contains(FeatureFlag::OFFLINE_INSTALL))]
    offline: bool,

    /// Do not use locally-installed packages when a corresponding package cannot be installed
    /// from Builder
    #[arg(long = "ignore-local",
                action = ArgAction::SetTrue,
                )]
    ignore_local: bool,
}

impl PkgInstallOptions {
    pub(crate) async fn do_install(&self,
                                   ui: &mut UI,
                                   feature_flags: FeatureFlag)
                                   -> HabResult<()> {
        use habitat_core::package::Identifiable;

        let pkg_install_args: Vec<_> = std::env::args_os().skip(2).collect();

        let auth_token = self.auth_token.try_from_cli_or_config();

        let install_mode = if feature_flags.contains(FeatureFlag::OFFLINE_INSTALL) && self.offline {
            InstallMode::Offline
        } else {
            InstallMode::default()
        };

        let local_package_usage = if self.ignore_local {
            LocalPackageUsage::Ignore
        } else {
            LocalPackageUsage::default()
        };

        let install_hook_mode = if self.ignore_install_hook {
            InstallHookMode::Ignore
        } else {
            InstallHookMode::default()
        };

        let matches = Self::command().get_matches_from(pkg_install_args);
        let do_binlink = match matches.value_source("binlink_dir") {
            Some(ValueSource::CommandLine) => true,
            _ => self.binlink,
        };

        for install_source in &self.pkg_ident_or_artifact {
            let ident: &PackageIdent = install_source.as_ref();
            let channel = if let Some(ref channel) = self.channel {
                channel.clone()
            } else if ident.origin() == "core" {
                ChannelIdent::base()
            } else {
                ChannelIdent::stable()
            };

            let pkg_install = install::start(ui,
                                             &self.bldr_url.to_string(),
                                             &channel,
                                             install_source,
                                             PRODUCT,
                                             VERSION,
                                             &FS_ROOT_PATH,
                                             &cache_artifact_path(Some(FS_ROOT_PATH.as_path())),
                                             auth_token.as_deref(),
                                             &install_mode,
                                             &local_package_usage,
                                             install_hook_mode).await?;

            if do_binlink {
                let binlink_dir = PathBuf::from(&self.binlink_dir);
                binlink::binlink_all_in_pkg(ui,
                                            pkg_install.ident(),
                                            &binlink_dir,
                                            &FS_ROOT_PATH,
                                            self.force)?;
            }
        }

        Ok(())
    }
}
