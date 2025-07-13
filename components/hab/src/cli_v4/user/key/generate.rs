use crate::{cli_v4::utils::CacheKeyPath,
            error::Result as HabResult};
use clap::Parser;
use clap_v4 as clap;
use habitat_common::ui::UI;

use habitat_core::{crypto::keys::KeyCache,
                   fs::cache_key_path};
use std::path::PathBuf;

use crate::command::user::key::generate::start;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n",
          about = "Generates a Habitat user key")]
pub(crate) struct UserKeyGenerateOptions {
    /// Name of the user key
    #[arg(value_name = "USER")]
    user: String,

    /// Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default:
    /// /hab/cache/keys]
    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl UserKeyGenerateOptions {
    pub(crate) async fn do_generate(&self, ui: &mut UI) -> HabResult<()> {
        let key_path: PathBuf = (&self.cache_key_path).into();

        let key_cache = KeyCache::new(cache_key_path(key_path));
        key_cache.setup()?;

        start(ui, &self.user, &key_cache)?;
        Ok(())
    }
}
