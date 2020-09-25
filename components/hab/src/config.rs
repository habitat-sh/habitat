use crate::{error::{Error as HabError,
                    Result as HabResult},
            CTL_SECRET_ENVVAR};
use habitat_common::cli_config::CliConfig;
use habitat_core::env as henv;
use habitat_sup_client::SrvClient;

/// Check if the HAB_CTL_SECRET env var. If not, check the CLI config to see if there is a ctl
/// secret set and return a copy of that value.
pub fn ctl_secret_key(config: &CliConfig) -> HabResult<String> {
    match henv::var(CTL_SECRET_ENVVAR) {
        Ok(v) => Ok(v),
        Err(_) => {
            match config.ctl_secret {
                Some(ref v) => Ok(v.to_string()),
                None => SrvClient::read_secret_key().map_err(HabError::from),
            }
        }
    }
}
