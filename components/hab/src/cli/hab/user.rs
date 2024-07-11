#![allow(dead_code)]

use super::util::CacheKeyPath;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat users
pub enum User {
    Key(Key),
}

/// Commands relating to Habitat user keys
#[derive(StructOpt)]
#[structopt(name = "key", no_version)]
pub enum Key {
    Generate(UserKeyGenerate),
}

/// Generates a Habitat user key
#[derive(StructOpt)]
#[structopt(name = "generate", no_version)]
pub struct UserKeyGenerate {
    /// Name of the user key
    #[structopt(name = "USER")]
    user:           String,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}
