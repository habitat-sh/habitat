#![allow(dead_code)]

use super::util::CacheKeyPath;

use clap::Parser;

#[derive(Parser)]
/// Commands relating to Habitat rings
pub enum Ring {
    Key(Key),
}

#[derive(Parser)]
/// Commands relating to Habitat ring keys
pub enum Key {
    Export(RingKeyExport),
    Generate(RingKeyGenerate),
    Import(RingKeyImport),
}

/// Outputs the latest ring key contents to stdout
#[derive(Parser)]
pub struct RingKeyExport {
    /// Ring key name
    #[clap(name = "RING")]
    ring: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Generates a Habitat ring key
#[derive(Parser)]
pub struct RingKeyGenerate {
    /// Ring key name
    #[clap(name = "RING")]
    ring: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Reads a stdin stream containing ring key contents and writes the key to disk
#[derive(Parser)]
pub struct RingKeyImport {
    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}
