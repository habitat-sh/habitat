use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::crypto::RingKey};

use crate::error::Result;

pub fn start<P>(ui: &mut UI, ring: &str, cache: P) -> Result<()>
    where P: AsRef<Path>
{
    ui.begin(format!("Generating ring key for {}", ring))?;
    let key = RingKey::new(ring);
    key.write_to_cache(cache)?;
    ui.end(format!("Generated ring key {}.", key.name_with_rev()))?;
    Ok(())
}
