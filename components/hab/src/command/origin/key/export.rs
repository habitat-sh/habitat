use crate::error::Result;
use habitat_core::crypto::keys::{Key,
                                 KeyCache,
                                 PairType};
use std::{io,
          io::Write,
          path::Path};

pub fn start(origin: &str, pair_type: PairType, cache: &Path) -> Result<()> {
    let cache = KeyCache::new(cache);

    match pair_type {
        PairType::Public => {
            let key = cache.latest_public_origin_signing_key(origin)?;
            let contents = key.to_key_string();
            io::stdout().write_all(contents.as_bytes())?;
        }
        PairType::Secret => {
            let key = cache.latest_secret_origin_signing_key(origin)?;
            let contents = key.to_key_string();
            io::stdout().write_all(contents.as_bytes())?;
        }
    }

    // debug!("Streaming file contents of {} {} to standard out",
    //        pair_type,
    //        path.display());

    Ok(())
}
