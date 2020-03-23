use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath};
use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat rings
pub enum Ring {
    Key(Key),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat ring keys
pub enum Key {
    /// Outputs the latest ring key contents to stdout
    Export {
        /// Ring key name
        #[structopt(name = "RING")]
        ring:           String,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Generates a Habitat ring key
    Generate {
        /// Ring key name
        #[structopt(name = "RING")]
        ring:           String,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Reads a stdin stream containing ring key contents and writes the key to disk
    Import {
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
}
