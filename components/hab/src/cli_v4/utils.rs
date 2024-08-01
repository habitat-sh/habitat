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
                   fs::CACHE_KEY_PATH};

use crate::error::{Error as HabError,
                   Result as HabResult};

lazy_static! {
    pub static ref CACHE_KEY_PATH_DEFAULT: String = CACHE_KEY_PATH.to_string_lossy().to_string();
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
    #[arg(name = "BLDR_URL",
          short = 'u',
          long = "url",
          env = "HAB_BLDR_URL",
          default_value = "https://bldr.habitat.sh")]
    bldr_url: Url,
}

impl BldrUrl {
    pub(crate) fn as_str(&self) -> &str { self.bldr_url.as_str() }
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
    pub(crate) fn from_cli_or_config(&self) -> HabResult<String> {
        if self.auth_token.is_some() {
            Ok(self.auth_token.clone().expect("TOKEN EXPECTED"))
        } else {
            CliConfig::load()?.auth_token
                              .ok_or_else(|| HabError::ArgumentError("No auth token specified".into()))
        }
    }
}
