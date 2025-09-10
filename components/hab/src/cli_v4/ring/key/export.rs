use crate::{cli_v4::utils::CacheKeyPath,
            command::ring::key::export::start,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_core::crypto::keys::KeyCache;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct RingKeyExportOpts {
    /// Ring key name
    #[arg(value_name = "RING")]
    ring: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl RingKeyExportOpts {
    pub(crate) async fn do_export(&self) -> HabResult<()> {
        let key_path: PathBuf = (&self.cache_key_path).into();
        let key_cache = KeyCache::new(key_path);
        key_cache.setup()?;
        start(&self.ring, &key_cache)
    }
}
