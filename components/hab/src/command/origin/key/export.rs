use crate::error::Result;
use habitat_core::crypto::keys::{KeyCache,
                                 KeyFile,
                                 PairType};
use std::{io,
          io::Write};

pub fn start(origin: &str, pair_type: PairType, key_cache: &KeyCache) -> Result<()> {
    match pair_type {
        PairType::Public => {
            let key = key_cache.latest_public_origin_signing_key(origin)?;
            let contents = key.to_key_string();
            io::stdout().write_all(contents.as_bytes())?;
        }
        PairType::Secret => {
            let key = key_cache.latest_secret_origin_signing_key(origin)?;
            let contents = key.to_key_string();
            io::stdout().write_all(contents.as_bytes())?;
        }
    }
    Ok(())
}
