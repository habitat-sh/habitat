use crate::{error::Result,
            key_type::KeyType};
use habitat_core::{crypto::keys::{KeyCache,
                                  KeyFile},
                   origin::Origin};
use std::{io,
          io::Write};

pub fn start(origin: &Origin, key_type: KeyType, key_cache: &KeyCache) -> Result<()> {
    match key_type {
        KeyType::Public => {
            let key = key_cache.latest_public_origin_signing_key(origin)?;
            let contents = key.to_key_string();
            io::stdout().write_all(contents.as_bytes())?;
        }
        KeyType::Secret => {
            let key = key_cache.latest_secret_origin_signing_key(origin)?;
            let contents = key.to_key_string();
            io::stdout().write_all(contents.as_bytes())?;
        }
    }
    Ok(())
}
