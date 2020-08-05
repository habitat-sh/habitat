use crate::error::Result;
use habitat_core::crypto::keys::{Key,
                                 KeyCache,
                                 PairType};
use std::{io,
          io::Write,
          path::Path};

pub fn start(origin: &str, pair_type: PairType, cache: &Path) -> Result<()> {
    let cache = KeyCache::new(cache);

    let key = match pair_type {
        PairType::Public => cache.latest_public_origin_signing_key(origin)?,
        PairType::Secret => cache.latest_secret_origin_signing_key(origin)?,
    };

    // debug!("Streaming file contents of {} {} to standard out",
    //        pair_type,
    //        path.display());

    let contents = key.to_key_string();
    io::stdout().write_all(contents.as_bytes())?;

    Ok(())
}
