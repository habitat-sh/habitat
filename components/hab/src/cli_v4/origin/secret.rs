// Implemenatation of `hab origin secret`

use clap_v4 as clap;

use crate::{cli_v4::utils::{origin_param_or_env,
                            valid_origin,
                            AuthToken,
                            BldrUrl,
                            CacheKeyPath},
            command::origin::secret,
            error::{Error,
                    Result as HabResult}};
use clap::Parser;

use habitat_common::ui::UI;

use std::path::PathBuf;

use habitat_core::{crypto::keys::KeyCache,
                   origin::Origin};

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) enum OriginSecretCommand {
    /// Delete a secret for your origin
    Delete {
        /// The name of the variable key to be injected into the studio
        #[arg(value_name = "KEY_NAME")]
        key_name: String,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,

        /// The origin for which the secret will be deleted. Default is from 'HAB_ORIGIN' or
        /// cli.toml
        #[arg(name = "ORIGIN",short = 'o', long = "origin", value_parser = valid_origin)]
        origin: Option<String>,
    },

    /// List all secrets for your origin
    List {
        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,

        /// The origin for which secrets will be listed. Default is from 'HAB_ORIGIN' or cli.toml
        #[arg(name = "ORIGIN",short = 'o', long = "origin", value_parser = valid_origin)]
        origin: Option<String>,
    },

    /// Create and upload a secret for your origin
    Upload {
        /// The name of the variable key to be injected into the studio. Ex: KEY="some_value"
        #[arg(name = "KEY_NAME")]
        key_name: String,

        /// The contents of the variable to be injected into the studio
        #[arg(name = "SECRET")]
        secret: String,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,

        /// The origin for which the secret will be uploaded. Default is from 'HAB_ORIGIN' or
        /// cli.toml
        #[arg(name = "ORIGIN",short = 'o', long = "origin", value_parser = valid_origin)]
        origin: Option<String>,

        #[command(flatten)]
        cache_key_path: CacheKeyPath,
    },
}

impl OriginSecretCommand {
    pub(super) async fn execute(&self, ui: &mut UI) -> HabResult<()> {
        fn get_args(bldr_url: &BldrUrl,
                    auth_token: &AuthToken,
                    origin_opt: &Option<String>)
                    -> Result<(String, String, Origin), Error> {
            // URL → String
            let url = bldr_url.resolve()?.to_string();
            // Token → String
            let token = auth_token.resolve()?;
            // Origin → Origin newtype
            let origin = origin_param_or_env(origin_opt)?;
            Ok((url, token, origin))
        }

        match self {
            OriginSecretCommand::Delete { key_name,
                                          bldr_url,
                                          auth_token,
                                          origin, } => {
                let (url, token, origin) = get_args(bldr_url, auth_token, origin)?;
                secret::delete::start(ui, &url, &token, &origin, key_name).await
            }

            OriginSecretCommand::List { bldr_url,
                                        auth_token,
                                        origin, } => {
                let (url, token, origin) = get_args(bldr_url, auth_token, origin)?;
                secret::list::start(ui, &url, &token, &origin).await
            }

            OriginSecretCommand::Upload { key_name,
                                          secret,
                                          bldr_url,
                                          auth_token,
                                          origin,
                                          cache_key_path, } => {
                let (url, token, origin) = get_args(bldr_url, auth_token, origin)?;

                // Build and initialize the KeyCache
                let cache_dir: PathBuf = cache_key_path.cache_key_path.clone();
                let key_cache = KeyCache::new(cache_dir);
                key_cache.setup().map_err(Error::from)?;

                secret::upload::start(ui, &url, &token, &origin, key_name, secret, &key_cache).await
            }
        }
    }
}
