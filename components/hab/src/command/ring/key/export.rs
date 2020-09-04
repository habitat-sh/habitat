use crate::error::Result;
use habitat_core::crypto::keys::{Key,
                                 KeyCache,
                                 KeyFile};
use std::io::{self,
              Write};

pub fn start(ring: &str, key_cache: &KeyCache) -> Result<()> {
    key_cache.setup()?;
    let key = key_cache.latest_ring_key_revision(ring)?;
    debug!("Streaming key contents of {} to standard output",
           key.named_revision());
    let contents = key.to_key_string();
    io::stdout().write_all(contents.as_bytes())?;
    Ok(())
}
