// Implementation of `hab pkg download` command

use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use habitat_common::{cli::{clap_validators::{HabPkgIdentValueParser,
                                             TomlOrPkgIdentFileValueParser},
                           file_into_idents,
                           is_toml_file,
                           PACKAGE_TARGET_ENVVAR},
                     ui::UI,
                     Error as HabitatCommonError};

use habitat_core::{env::Config,
                   package::{target,
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
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgDownloadOptions {
    #[command(flatten)]
    auth_token: AuthToken,

    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Download from the specified release channel. Overridden if channel is specified in toml
    /// file
    #[arg(name = "CHANNEL",
          short = 'c',
          long = "channel",
          env = habitat_core::ChannelIdent::ENVVAR)]
    channel: Option<ChannelIdent>,

    /// The path to store downloaded artifacts
    #[arg(name = "DOWNLOAD_DIRECTORY", long = "download-directory")]
    download_directory: Option<PathBuf>,

    /// File with newline separated package identifiers, or TOML file (ending with .toml extension)
    #[arg(name = "PKG_IDENT_FILE", long = "file", num_args = 1..=10, value_parser = TomlOrPkgIdentFileValueParser)]
    pkg_ident_file: Vec<String>,

    /// One or more Package Identifiers to download (eg. core/redis)
    #[arg(name = "PKG_IDENT", num_args = 1.., value_parser = HabPkgIdentValueParser::simple())]
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
        use habitat_core::package::Identifiable;

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
            let (core_idents, non_core_idents): (Vec<_>, Vec<_>) =
                self.pkg_ident
                    .clone()
                    .into_iter()
                    .partition(|ident| ident.origin() == "core");

            if let Some(ref channel) = self.channel {
                package_sets.push(PackageSet { target,
                                               channel: channel.clone(),
                                               idents: self.pkg_ident.clone() });
            } else {
                package_sets.push(PackageSet { target,
                                               channel: ChannelIdent::base(),
                                               idents: core_idents });
                package_sets.push(PackageSet { target,
                                               channel: ChannelIdent::stable(),
                                               idents: non_core_idents });
            }
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
        use habitat_core::package::Identifiable;

        if !self.pkg_ident_file.is_empty() {
            for f in &self.pkg_ident_file {
                if is_toml_file(f) {
                    let file_data = std::fs::read_to_string(f)?;
                    let toml_data: PackageSetFile =
                        toml::from_str(&file_data).map_err(HabitatCommonError::TomlParser)?;
                    sources.append(&mut toml_data.to_package_sets()?);
                } else {
                    let idents_from_file = file_into_idents(f)?;
                    match self.channel {
                        Some(ref channel) => {
                            sources.push(PackageSet { idents: idents_from_file,
                                                      channel: channel.clone(),
                                                      target })
                        }
                        None => {
                            let (core_idents, non_core_idents): (Vec<_>, Vec<_>) =
                                idents_from_file.into_iter()
                                                .partition(|ident| ident.origin() == "core");
                            let core_package_set = PackageSet { idents: core_idents,
                                                                channel:
                                                                    ChannelIdent::from("base"),
                                                                target };
                            sources.push(core_package_set);
                            let non_core_package_set = PackageSet { idents: non_core_idents,
                                                                    channel:
                                                                        ChannelIdent::from("stable"),
                                                                    target };
                            sources.push(non_core_package_set);
                        }
                    }
                }
            }
        }
        Ok(sources)
    }
}

#[cfg(test)]
mod tests {
    use super::{PackageTarget,
                Parser,
                PkgDownloadOptions};
    use std::{collections::HashMap,
              path::Path};

    #[test]
    fn test_package_sets_from_file_e2e_tests_toml() {
        let mut toml_files_map = HashMap::<String, bool>::new();
        toml_files_map.insert("bad_header.toml".to_string(), false);
        toml_files_map.insert("bad_ident.toml".to_string(), false);
        toml_files_map.insert("bad_target.toml".to_string(), false);
        toml_files_map.insert("no_header.toml".to_string(), false);
        toml_files_map.insert("no_target.toml".to_string(), true);
        toml_files_map.insert("happy_path.toml".to_string(), true);

        let tomls_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let tomls_dir = Path::new(&tomls_dir).join("../../test/end-to-end/fixtures/pkg_download/");
        assert!(tomls_dir.is_dir());

        let no_header_toml_string = "no_header.toml".to_string();
        let _ = toml_files_map.get(&no_header_toml_string);
        for toml in tomls_dir.read_dir().unwrap() {
            if let Ok(toml) = toml {
                let key = toml.file_name().into_string().unwrap();
                let path = toml.path().into_os_string().into_string();
                eprintln!("{}: {:#?}", key, path);
                if let Ok(path) = path {
                    let args = ["download", "--file", &path];
                    let result = PkgDownloadOptions::try_parse_from(args);
                    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

                    let pkg_download = result.unwrap();
                    let result =
                        pkg_download.idents_from_file_matches(PackageTarget::active_target());
                    let should_be_ok = toml_files_map.get(&key).unwrap();
                    assert_eq!(result.is_ok(),
                               *should_be_ok,
                               "{}: {:#?}",
                               key,
                               result.err().unwrap());
                }
            }
        }
    }
}
