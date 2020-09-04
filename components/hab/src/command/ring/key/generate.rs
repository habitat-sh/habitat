use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::crypto::keys::{Key,
                                 KeyCache};

pub fn start(ui: &mut UI, ring: &str, key_cache: &KeyCache) -> Result<()> {
    ui.begin(format!("Generating ring key for {}", ring))?;
    let key = key_cache.new_ring_key(ring)?;
    ui.end(format!("Generated ring key {}.", key.named_revision()))?;
    Ok(())
}
