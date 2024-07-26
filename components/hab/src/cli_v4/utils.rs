// Utilities that are used by v4 macros
//
// Note we are duplicating this functionality because trivially using
// `cfg_attr(feature = "v4"),...]` is not easy to make work with existing code. Eventually this
// will be the only `util` left (hope so)

use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;
use lazy_static::lazy_static;
use url::Url;

use habitat_common::cli_config::CliConfig;

use habitat_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   env as hcore_env,
                   fs::CACHE_KEY_PATH,
                   url::{BLDR_URL_ENVVAR,
                         DEFAULT_BLDR_URL}};

use crate::error::{Error as HabError,
                   Result as HabResult};

lazy_static! {
    pub(crate) static ref CACHE_KEY_PATH_DEFAULT: String =
        CACHE_KEY_PATH.to_string_lossy().to_string();
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct CacheKeyPath {
    /// Cache for creating and searching for encryption keys
    #[arg(long = "cache-key-path",
                env = CACHE_KEY_PATH_ENV_VAR,
                default_value = &*CACHE_KEY_PATH_DEFAULT)]
    pub(crate) cache_key_path: PathBuf,
}

impl From<PathBuf> for CacheKeyPath {
    fn from(cache_key_path: PathBuf) -> Self { Self { cache_key_path } }
}

impl From<&CacheKeyPath> for PathBuf {
    fn from(cache_key_path: &CacheKeyPath) -> PathBuf { cache_key_path.cache_key_path.clone() }
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct BldrUrl {
    // TODO:agadgil: Use the Url Validator
    /// Specify an alternate Builder endpoint.
    #[arg(name = "BLDR_URL", short = 'u', long = "url")]
    bldr_url: Option<Url>,
}

impl BldrUrl {
    //
    pub(crate) fn to_string(&self) -> String {
        if let Some(url) = &self.bldr_url {
            url.to_string()
        } else {
            match hcore_env::var(BLDR_URL_ENVVAR) {
                Ok(v) => v,
                Err(_) => {
                    // Okay to unwrap it never returns Err!!
                    match CliConfig::load().unwrap().bldr_url {
                        Some(v) => v,
                        None => DEFAULT_BLDR_URL.to_string(),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct AuthToken {
    // TODO: Add Validator for this?
    /// Authentication token for Builder.
    #[arg(name = "AUTH_TOKEN",
          short = 'z',
          long = "auth",
          env = "HAB_AUTH_TOKEN")]
    auth_token: Option<String>,
}

impl AuthToken {
    // This function returns a result. Use this when `auth_token` is required. Either as a command
    // line option or env or from config.
    pub(crate) fn from_cli_or_config(&self) -> HabResult<String> {
        if self.auth_token.is_some() {
            Ok(self.auth_token.clone().expect("TOKEN EXPECTED"))
        } else {
            CliConfig::load()?.auth_token
                              .ok_or_else(|| HabError::ArgumentError("No auth token specified".into()))
        }
    }

    // This function returns an `Option`, so if there is any "error" reading from config or env is
    // not set simply returns a None.
    pub(crate) fn try_from_cli_or_config(&self) -> Option<String> {
        if self.auth_token.is_some() {
            self.auth_token.clone()
        } else {
            match CliConfig::load() {
                Ok(result) => result.auth_token,
                Err(_) => None,
            }
        }
    }
}
