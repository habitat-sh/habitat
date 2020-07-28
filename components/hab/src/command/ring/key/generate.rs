use crate::common::ui::{UIWriter,
                        UI};
use habitat_core::crypto::{keys::KeyCache,
                           RingKey};
use std::path::Path;

use crate::error::Result;

pub fn start<P>(ui: &mut UI, ring: &str, cache: P) -> Result<()>
    where P: AsRef<Path>
{
    ui.begin(format!("Generating ring key for {}", ring))?;
    let key = RingKey::new(ring);
    let cache: KeyCache = cache.as_ref().into();
    cache.write_ring_key(&key)?;
    ui.end(format!("Generated ring key {}.", key.name_with_rev()))?;
    Ok(())
}
