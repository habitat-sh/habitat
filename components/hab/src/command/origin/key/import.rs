use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::{crypto::keys::{Key,
                                  KeyCache,
                                  PublicOriginSigningKey,
                                  SecretOriginSigningKey},
                   error::Error as CoreError};

pub fn start(ui: &mut UI, content: &str, key_cache: &KeyCache) -> Result<()> {
    ui.begin("Importing origin key from standard input")?;

    // Yeah, this is a little gross
    if let Ok(key) = content.parse::<PublicOriginSigningKey>() {
        key_cache.write_key(&key)?;
        ui.end(format!("Imported public origin key {}", &key.named_revision()))?;
        Ok(())
    } else if let Ok(key) = content.parse::<SecretOriginSigningKey>() {
        key_cache.write_key(&key)?;
        ui.end(format!("Imported secret origin key {}", &key.named_revision()))?;
        Ok(())
    } else {
        // This is a LOT gross
        Err(CoreError::CryptoError("Could not parse content as an \
                                    public or secret origin signing \
                                    key!"
                                         .to_string()))?
    }
}
