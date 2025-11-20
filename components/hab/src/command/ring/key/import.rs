use crate::{common::ui::{UI,
                         UIWriter},
            error::Result};
use habitat_core::crypto::keys::{Key,
                                 KeyCache,
                                 RingKey};

pub fn start(ui: &mut UI, content: &str, key_cache: &KeyCache) -> Result<()> {
    ui.begin("Importing ring key from standard input")?;
    let key: RingKey = content.parse()?;
    key_cache.write_key(&key)?;
    ui.end(format!("Imported ring key {}.", &key.named_revision()))?;
    Ok(())
}
