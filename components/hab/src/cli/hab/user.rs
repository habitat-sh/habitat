use super::util::CacheKeyPath;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat users
pub enum User {
    /// Commands relating to Habitat user keys
    Key(Key),
}

#[derive(StructOpt)]
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
