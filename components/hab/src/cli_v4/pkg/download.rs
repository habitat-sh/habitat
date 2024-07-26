// Implementation of `hab pkg download` command

use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use habitat_common::{cli::{file_into_idents,
                           is_toml_file,
                           PACKAGE_TARGET_ENVVAR},
                     ui::UI,
                     Error as HabitatCommonError};

use habitat_core::{package::{target,
                             PackageIdent,
                             PackageTarget},
                   ChannelIdent};

use crate::{cli_v4::utils::{AuthToken,
                            BldrUrl},
            command::pkg::{download,
                           download::{PackageSet,
                                      PackageSetFile}},
            error::Result as HabResult,
            PRODUCT,
            VERSION};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgDownloadOptions {
    #[structopt(flatten)]
    auth_token: AuthToken,

    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Download from the specified release channel. Overridden if channel is specified in toml
    /// file
    #[arg(name = "CHANNEL",
          short = 'c',
          long = "channel",
          default_value = "stable")]
    channel: ChannelIdent,

    /// The path to store downloaded artifacts
    #[arg(name = "DOWNLOAD_DIRECTORY", long = "download-directory")]
    download_directory: Option<PathBuf>,

    // TODO: Add validations
    /// File with newline separated package identifiers, or TOML file (ending with .toml extension)
    #[arg(name = "PKG_IDENT_FILE", long = "file", num_args = 1..=10)]
    pkg_ident_file: Vec<String>,

    /// One or more Package Identifiers to download (eg. core/redis)
    #[arg(name = "PKG_IDENT", num_args = 1.., last = true)]
    pkg_ident: Vec<PackageIdent>,

    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[arg(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR, short = 't', long = "target")]
    pkg_target: Option<PackageTarget>,

    /// Verify package integrity after download (Warning: this can be slow)
    #[arg(name = "VERIFY", long = "verify", action = ArgAction::SetTrue)]
    verify: bool,

    /// Ignore packages specified that are not present on the target Builder
    #[arg(name = "IGNORE_MISSING_SEEDS", long = "ignore-missing-seeds", action = ArgAction::SetTrue)]
    ignore_missing_seed: bool,
}

impl PkgDownloadOptions {
    pub(super) async fn do_download(&self, ui: &mut UI) -> HabResult<()> {
        let auth_token = self.auth_token.try_from_cli_or_config();

        let target = self.pkg_target.unwrap_or_else(|| {
                                        match PackageTarget::active_target() {
                                            #[cfg(feature = "supported_targets")]
                                            target::X86_64_DARWIN => target::X86_64_LINUX,
                                            t => t,
                                        }
                                    });

        let mut package_sets = vec![];

        if !self.pkg_ident.is_empty() {
            package_sets.push(PackageSet { target,
                                           channel: self.channel.clone(),
                                           idents: self.pkg_ident.clone() });
        }
        let mut package_sets_from_file = self.idents_from_file_matches(target)?;
        package_sets.append(&mut package_sets_from_file);
        package_sets.retain(|set| !set.idents.is_empty());

        download::start(ui,
                        &self.bldr_url.to_string(),
                        PRODUCT,
                        VERSION,
                        &package_sets,
                        self.download_directory.as_ref(),
                        auth_token.as_ref().map(|x| x.as_str()),
                        self.verify,
                        self.ignore_missing_seed).await
    }

    fn idents_from_file_matches(&self, target: PackageTarget) -> HabResult<Vec<PackageSet>> {
        let mut sources: Vec<PackageSet> = Vec::new();

        if !self.pkg_ident_file.is_empty() {
            for f in &self.pkg_ident_file {
                if is_toml_file(f) {
                    let file_data = std::fs::read_to_string(f)?;
                    let toml_data: PackageSetFile =
                        toml::from_str(&file_data).map_err(HabitatCommonError::TomlParser)?;
                    sources.append(&mut toml_data.to_package_sets()?);
                } else {
                    let idents_from_file = file_into_idents(f)?;
                    let package_set = PackageSet { idents: idents_from_file,
                                                   channel: self.channel.clone(),
                                                   target };
                    sources.push(package_set)
                }
            }
        }
        Ok(sources)
    }
}
