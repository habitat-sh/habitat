use crate::error::Result;
use habitat_core::crypto::keys::{Key,
                                 KeyCache,
                                 KeyFile};
use std::{io::{self,
               Write},
          path::Path};

pub fn start<P>(ring: &str, cache: P) -> Result<()>
    where P: AsRef<Path>
{
    let cache = KeyCache::new(cache.as_ref());
    cache.setup()?;
    let key = cache.latest_ring_key_revision(ring)?;
    debug!("Streaming key contents of {} to standard output",
           key.named_revision());
    let contents = key.to_key_string();
    io::stdout().write_all(contents.as_bytes())?;
    Ok(())
}
