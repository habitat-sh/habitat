use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::{crypto::keys::{KeyCache,
                                  PublicOriginSigningKey,
                                  SecretOriginSigningKey},
                   error::Error as CoreError};
use std::path::Path;

pub fn start(ui: &mut UI, content: &str, cache: &Path) -> Result<()> {
    ui.begin("Importing origin key from standard input")?;
    let cache = KeyCache::new(cache);

    // Yeah, this is a little gross
    if let Ok(key) = content.parse::<PublicOriginSigningKey>() {
        cache.write_key(&key)?;
        ui.end(format!("Imported public origin key {}", &key.name_with_rev()))?;
        Ok(())
    } else if let Ok(key) = content.parse::<SecretOriginSigningKey>() {
        cache.write_key(&key)?;
        ui.end(format!("Imported secret origin key {}", &key.name_with_rev()))?;
        Ok(())
    } else {
        // This is a LOT gross
        Err(CoreError::CryptoError("Could not parse content as an \
                                    public or secret origin signing \
                                    key!"
                                         .to_string()))?
    }
}
