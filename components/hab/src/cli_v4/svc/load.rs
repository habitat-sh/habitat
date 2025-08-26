use clap_v4 as clap;

use std::convert::TryFrom;

use clap::{builder::BoolishValueParser,
           value_parser,
           Parser};

use serde::{Deserialize,
            Serialize};

use hab_common_derive::GenConfig;

use crate::{cli_v4::utils::{shared_load_cli_to_ctl,
                            PkgIdent,
                            RemoteSup,
                            SharedLoad},
            error::{Error as HabError,
                    Result as HabResult},
            gateway_util};

const DEFAULT_SVC_CONFIG_FILE: &str = "/hab/sup/default/config/svc.toml";

/// Load a service to be started and supervised by Habitat from a package identifier If an installed
/// package doesn't satisfy the given package identifier, a suitable package will be installed from
/// Builder.
#[derive(GenConfig)]
#[derive(Clone, Debug, Parser, Serialize, Deserialize)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct LoadCommand {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = value_parser!(PkgIdent),  required_unless_present_any(["generate_config", "config_files"]))]
    pkg_ident: Option<PkgIdent>,

    /// Load or reload an already loaded service. If the service was previously loaded and
    /// running this operation will also restart the service
    #[arg(short = 'f', long = "force")]
    #[serde(skip)]
    force: bool,

    #[arg(long = "generate-config", hide = true)]
    #[serde(skip)]
    generate_config: bool,

    #[arg(long = "config-files",
          env = "HAB_FEAT_SERVICE_CONFIG_FILES",
          value_parser = BoolishValueParser::new(),
          hide = true)]
    #[serde(skip)]
    config_files: bool,

    #[command(flatten)]
    #[serde(flatten)]
    remote_sup: RemoteSup,

    #[command(flatten)]
    #[serde(flatten)]
    shared_load: SharedLoad,
}

impl TryFrom<LoadCommand> for habitat_sup_protocol::ctl::SvcLoad {
    type Error = HabError;

    fn try_from(cmd: LoadCommand) -> HabResult<Self> {
        shared_load_cli_to_ctl(cmd.pkg_ident.unwrap().pkg_ident(),
                               cmd.shared_load,
                               cmd.force)
    }
}

impl LoadCommand {
    pub(super) async fn do_command(&self) -> HabResult<()> {
        if self.generate_config {
            println!("{}", self.gen_config());
            Ok(())
        } else {
            let load_command = if self.config_files {
                match Self::from_default_config_file() {
                    Ok(mut config_load_command) => {
                        if config_load_command.pkg_ident.is_none() || self.pkg_ident.is_some() {
                            config_load_command.pkg_ident = self.pkg_ident.clone()
                        }

                        if self.remote_sup.inner().is_some() {
                            config_load_command.remote_sup = self.remote_sup.clone();
                        }

                        if config_load_command.shared_load == SharedLoad::default() {
                            config_load_command.shared_load = self.shared_load.clone();
                        }
                        config_load_command
                    }
                    Err(e) => {
                        match e {
                            HabError::TomlDeserializeError(_) => {
                                return Err(e);
                            }
                            _ => self.clone(),
                        }
                    }
                }
            } else {
                self.clone()
            };

            if load_command.pkg_ident.is_none() {
                return Err(HabError::ArgumentError("<PKG_IDENT> for the service is \
                                                    required."
                                                              .to_string()));
            }
            let remote_sup = self.remote_sup.clone();
            let msg = habitat_sup_protocol::ctl::SvcLoad::try_from(load_command)?;
            gateway_util::send(remote_sup.inner(), msg).await
        }
    }

    fn from_default_config_file() -> HabResult<Self> {
        let default_config_file_exists = std::fs::exists(DEFAULT_SVC_CONFIG_FILE)?;
        if !default_config_file_exists {
            return Err(HabError::FileNotFound(format!("Default Config file '{}' \
                                                       does not exist.",
                                                      DEFAULT_SVC_CONFIG_FILE)));
        }
        let toml_string = std::fs::read_to_string(DEFAULT_SVC_CONFIG_FILE)?;
        let config_load: Self = toml::from_str(&toml_string)?;
        Ok(config_load)
    }
}
