use crate::{common::ui::{UI,
                         UIWriter},
            error::Result};
use habitat_core::crypto::keys::{Key,
                                 KeyCache};

pub fn start(ui: &mut UI, user: &str, key_cache: &KeyCache) -> Result<()> {
    ui.begin(format!("Generating user key for {}", user))?;
    let (public, _secret) = key_cache.new_user_encryption_pair(user)?;
    ui.end(format!("Generated user encryption key pair {}.",
                   public.named_revision()))?;
    Ok(())
}
