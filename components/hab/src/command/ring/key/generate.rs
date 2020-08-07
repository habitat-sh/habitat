use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::crypto::keys::{KeyCache,
                                 RingKey};
use std::path::Path;

pub fn start<P>(ui: &mut UI, ring: &str, cache: P) -> Result<()>
    where P: AsRef<Path>
{
    ui.begin(format!("Generating ring key for {}", ring))?;
    let key = RingKey::new(ring);
    let cache = KeyCache::new(cache.as_ref());
    cache.setup()?;
    cache.write_key(&key)?;
    ui.end(format!("Generated ring key {}.", key.name_with_rev()))?;
    Ok(())
}
