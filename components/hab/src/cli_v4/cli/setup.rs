use crate::{cli_v4::utils::CacheKeyPath,
            command::cli::setup::start,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_common::ui::UI;
use habitat_core::{crypto::keys::KeyCache,
                   fs::cache_key_path};
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n",
          about = "Sets up the CLI with reasonable defaults")]
pub(crate) struct CliSetupOptions {
    /// Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default:
    /// /hab/cache/keys]
    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl CliSetupOptions {
    pub(crate) async fn do_setup(&self, ui: &mut UI) -> HabResult<()> {
        let key_path: PathBuf = (&self.cache_key_path).into();

        let key_cache = KeyCache::new(cache_key_path(key_path));
        key_cache.setup()?;

        start(ui, &key_cache)?;
        Ok(())
    }
}
