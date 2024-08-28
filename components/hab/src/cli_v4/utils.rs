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
                         DEFAULT_BLDR_URL},
                   AUTH_TOKEN_ENVVAR};

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
    #[arg(name = "AUTH_TOKEN", short = 'z', long = "auth")]
    auth_token: Option<String>,
}

impl AuthToken {
    // This function returns a result. Use this when `auth_token` is required. Either as a command
    // line option or env or from config.
    pub(crate) fn from_cli_or_config(&self) -> HabResult<String> {
        if let Some(auth_token) = &self.auth_token {
            Ok(auth_token.clone())
        } else {
            match hcore_env::var(AUTH_TOKEN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    CliConfig::load()?.auth_token.ok_or_else(|| {
                                                     HabError::ArgumentError("No auth token \
                                                                              specified"
                                                                                        .into())
                                                 })
                }
            }
        }
    }

    // This function returns an `Option`, so if there is any "error" reading from config or env is
    // not set simply returns a None.
    pub(crate) fn try_from_cli_or_config(&self) -> Option<String> {
        match self.from_cli_or_config() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    mod auth_token {

        use crate::cli_v4::utils::AuthToken;

        use clap_v4 as clap;

        use clap::Parser;

        habitat_core::locked_env_var!(HAB_AUTH_TOKEN, locked_auth_token);

        #[derive(Debug, Clone, Parser)]
        struct TestAuthToken {
            #[command(flatten)]
            a: AuthToken,
        }

        #[test]
        fn required_env_no_cli_success() {
            let env_var = locked_auth_token();
            env_var.set("env-auth-token");

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.from_cli_or_config();
            assert!(auth_token.is_ok(), "{:#?}", auth_token.err().unwrap());
        }

        #[test]
        fn required_no_env_cli_success() {
            let env_var = locked_auth_token();
            env_var.unset();

            let args = ["test-auth-token", "--auth", "foo-bar"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());
        }

        #[test]
        fn required_no_env_no_cli_error() {
            let env_var = locked_auth_token();
            env_var.unset();

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.from_cli_or_config();
            assert!(auth_token.is_err(), "{:#?}", auth_token.ok().unwrap());
        }

        #[test]
        fn required_empty_env_no_cli_error() {
            let env_var = locked_auth_token();
            env_var.set("");

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.from_cli_or_config();
            assert!(auth_token.is_err(), "{:#?}", auth_token.ok().unwrap());
        }
        #[test]
        fn optional_empty_env_no_cli_none() {
            let env_var = locked_auth_token();
            env_var.set("");

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.try_from_cli_or_config();
            assert!(auth_token.is_none(), "{:#?}", auth_token.unwrap());
        }

        #[test]
        fn tok_optional_from_env_no_cli_some() {
            let env_var = locked_auth_token();
            env_var.set("env-auth-token");

            let args = ["test-auth-token"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.try_from_cli_or_config();
            assert_eq!(Some("env-auth-token".to_string()),
                       auth_token,
                       "{:#?}",
                       auth_token);
        }

        #[test]
        fn optional_no_env_from_cli_some() {
            let env_var = locked_auth_token();
            env_var.set("env-auth-token");

            let args = ["test-auth-token", "--auth", "foo-bar"];
            let result = TestAuthToken::try_parse_from(args);
            assert!(result.is_ok(), "{:?}", result.err().unwrap());

            let test_auth_token = result.unwrap();
            let auth_token = test_auth_token.a.try_from_cli_or_config();
            assert_eq!(Some("foo-bar".to_string()), auth_token, "{:#?}", auth_token);
        }
    }

    mod bldr_url {

        use crate::cli_v4::utils::{BldrUrl,
                                   DEFAULT_BLDR_URL};

        use clap_v4 as clap;

        use clap::Parser;

        habitat_core::locked_env_var!(HAB_BLDR_URL, locked_bldr_url);

        #[derive(Debug, Clone, Parser)]
        struct TestBldrUrl {
            #[command(flatten)]
            u: BldrUrl,
        }

        #[test]
        fn no_env_no_cli_default() {
            let env_var = locked_bldr_url();
            env_var.unset();

            let args = ["test-bldr-url"];
            let result = TestBldrUrl::try_parse_from(args);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let test_bldr_url = result.unwrap();
            let bldr_url = test_bldr_url.u.to_string();
            assert_eq!(bldr_url.as_str(), DEFAULT_BLDR_URL, "{:#?}", bldr_url);
        }

        #[test]
        fn empty_env_no_cli_default() {
            let env_var = locked_bldr_url();
            env_var.set("");

            let args = ["test-bldr-url"];
            let result = TestBldrUrl::try_parse_from(args);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let test_bldr_url = result.unwrap();
            let bldr_url = test_bldr_url.u.to_string();
            assert_eq!(bldr_url.as_str(), DEFAULT_BLDR_URL, "{:#?}", bldr_url);
        }

        #[test]
        fn env_cli_passed_value() {
            let test_bldr_url_val = "https://test.bldr.habitat.sh/";
            let cli_bldr_url_val = "https://cli.bldr.habitat.sh/";
            let env_var = locked_bldr_url();
            env_var.set(test_bldr_url_val);

            let args = ["test-bldr-url", "--url", cli_bldr_url_val];
            let result = TestBldrUrl::try_parse_from(args);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let test_bldr_url = result.unwrap();
            let bldr_url = test_bldr_url.u.to_string();
            assert_eq!(bldr_url.as_str(), cli_bldr_url_val, "{:#?}", bldr_url);
        }

        #[test]
        fn env_no_cli_env_value() {
            let test_bldr_url_val = "https://test.bldr.habitat.sh/";
            let env_var = locked_bldr_url();
            env_var.set(test_bldr_url_val);

            let args = ["test-bldr-url"];
            let result = TestBldrUrl::try_parse_from(args);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let test_bldr_url = result.unwrap();
            let bldr_url = test_bldr_url.u.to_string();
            assert_eq!(bldr_url.as_str(), test_bldr_url_val, "{:#?}", bldr_url);
        }
    }
}
