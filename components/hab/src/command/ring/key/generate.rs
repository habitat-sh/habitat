use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::crypto::keys::{Key,
                                 KeyCache,
                                 RingKey};

pub fn start(ui: &mut UI, ring: &str, key_cache: &KeyCache) -> Result<()> {
    ui.begin(format!("Generating ring key for {}", ring))?;
    let key = RingKey::new(ring);
    key_cache.setup()?;
    key_cache.write_key(&key)?;
    ui.end(format!("Generated ring key {}.", key.named_revision()))?;
    Ok(())
}
