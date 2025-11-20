use crate::{common::ui::{UI,
                         UIWriter},
            error::Result};
use habitat_core::{crypto::keys::{Key,
                                  KeyCache,
                                  PublicOriginSigningKey,
                                  SecretOriginSigningKey},
                   error::Error as CoreError};

pub fn start(ui: &mut UI, content: &str, key_cache: &KeyCache) -> Result<()> {
    ui.begin("Importing origin key from standard input")?;

    // Yeah, this is a little gross
    match content.parse::<PublicOriginSigningKey>() {
        Ok(key) => {
            key_cache.write_key(&key)?;
            ui.end(format!("Imported public origin key {}", &key.named_revision()))?;
            Ok(())
        }
        _ => {
            match content.parse::<SecretOriginSigningKey>() {
                Ok(key) => {
                    key_cache.write_key(&key)?;
                    ui.end(format!("Imported secret origin key {}", &key.named_revision()))?;
                    Ok(())
                }
                _ => {
                    // This is a LOT gross
                    Err(CoreError::CryptoError("Could not parse content as an public or secret \
                                                origin signing key!"
                                                                    .to_string()).into())
                }
            }
        }
    }
}
