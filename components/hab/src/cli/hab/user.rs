use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath};
use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat users
pub enum User {
    Key(Key),
}

/// Commands relating to Habitat user keys
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "key", no_version)]
pub enum Key {
    Generate(UserKeyGenerate),
}

/// Generates a Habitat user key
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "generate", no_version)]
pub struct UserKeyGenerate {
    /// Name of the user key
    #[structopt(name = "USER")]
    user:           String,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}
