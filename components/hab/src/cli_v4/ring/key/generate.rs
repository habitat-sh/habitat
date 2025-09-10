use crate::{cli_v4::utils::CacheKeyPath,
            command::ring::key::generate::start,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_common::ui::UI;
use habitat_core::crypto::{init,
                           keys::KeyCache};
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct RingKeyGenerateOpts {
    /// Ring key name
    #[arg(value_name = "RING")]
    ring: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl RingKeyGenerateOpts {
    pub(crate) async fn do_generate(&self, ui: &mut UI) -> HabResult<()> {
        let key_path: PathBuf = (&self.cache_key_path).into();
        let key_cache = KeyCache::new(key_path);
        key_cache.setup()?;
        init()?;
        start(ui, &self.ring, &key_cache)
    }
}
