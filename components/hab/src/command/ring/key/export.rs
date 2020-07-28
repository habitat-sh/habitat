use crate::error::Result;
use habitat_core::crypto::keys::KeyCache;
use std::{fs::File,
          io,
          path::Path};

pub fn start<P>(ring: &str, cache: P) -> Result<()>
    where P: AsRef<Path>
{
    let cache: KeyCache = cache.as_ref().into();
    let latest = cache.latest_ring_key_revision(ring)?;
    let path = cache.ring_key_cached_path(&latest)?;
    let mut file = File::open(&path)?;
    debug!("Streaming file contents of {} to standard out",
           &path.display());
    io::copy(&mut file, &mut io::stdout())?;
    Ok(())
}
