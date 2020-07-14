use crate::env;
use std::borrow::Borrow;
use url::Url;

/// Default Builder URL environment variable
pub const BLDR_URL_ENVVAR: &str = "HAB_BLDR_URL";
/// Default Builder URL
pub const DEFAULT_BLDR_URL: &str = "https://bldr.habitat.sh";
/// Legacy environment variable for defining a default Builder endpoint
const LEGACY_BLDR_URL_ENVVAR: &str = "HAB_DEPOT_URL";

// Returns a Builder URL value if set in the environment. Does *not*
// return any default value if the value was not found in the environment!
pub fn bldr_url_from_env() -> Option<String> {
    env::var(BLDR_URL_ENVVAR).or_else(|_| env::var(LEGACY_BLDR_URL_ENVVAR))
                             .ok()
}

pub fn default_bldr_url() -> String {
    bldr_url_from_env().unwrap_or_else(|| DEFAULT_BLDR_URL.to_string())
}

/// Resolve a Builder URL. Taken from a provided value, the environment, or
/// (failing those) a default value.
pub fn bldr_url<U: Borrow<Url>>(bldr_url: Option<U>) -> String {
    bldr_url.map(|u| u.borrow().to_string())
            .unwrap_or_else(default_bldr_url)
}
