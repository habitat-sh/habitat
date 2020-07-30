use crate::error::Result;
use habitat_core::crypto::keys::{KeyCache,
                                 ToKeyString};
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
           key.name_with_rev());
    let contents =
        key.to_key_string()
           .expect("If a ring key is found in the cache, it will necessarily have the contents \
                    of the key present print out");
    io::stdout().write_all(contents.as_bytes())?;
    Ok(())
}
