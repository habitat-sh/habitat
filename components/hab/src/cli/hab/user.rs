#![allow(dead_code)]

use super::util::CacheKeyPath;

use clap::Parser;

#[derive(Parser)]
/// Commands relating to Habitat users
pub enum User {
    Key(Key),
}

/// Commands relating to Habitat user keys
#[derive(Parser)]
pub enum Key {
    Generate(UserKeyGenerate),
}

/// Generates a Habitat user key
#[derive(Parser)]
pub struct UserKeyGenerate {
    /// Name of the user key
    #[clap(name = "USER")]
    user: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}
