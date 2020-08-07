use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::crypto::keys::{KeyCache,
                                 RingKey};
use std::path::Path;

pub fn start(ui: &mut UI, content: &str, cache: &Path) -> Result<()> {
    ui.begin("Importing ring key from standard input")?;
    let cache = KeyCache::new(cache);
    cache.setup()?;
    let key: RingKey = content.parse()?;
    cache.write_key(&key)?;
    ui.end(format!("Imported ring key {}.", &key.name_with_rev()))?;
    Ok(())
}
