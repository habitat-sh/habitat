use habitat_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   fs::CACHE_KEY_PATH};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct CacheKeyPath {
    /// Cache for creating and searching encryption keys. Default value is hab/cache/keys if root
    /// and .hab/cache/keys under the home directory otherwise.
    #[structopt(name = "CACHE_KEY_PATH",
                long = "cache-key-path",
                env = CACHE_KEY_PATH_ENV_VAR,
                required = true,
                // TODO (DM): This default value needs to be set dynamically based on user. We should set it
                // here instead of looking up the correct value later on. I dont understand why this value
                // has to be required.
                default_value = CACHE_KEY_PATH,
                hide_default_value = true)]
    cache_key_path: PathBuf,
}
