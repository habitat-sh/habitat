use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath};
use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat users
pub enum User {
    /// Commands relating to Habitat user keys
    Key(Key),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat users
pub enum Key {
    /// Generates a Habitat user key
    Generate {
        /// Name of the user key
        #[structopt(name = "USER")]
        user:           String,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
}
