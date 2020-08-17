use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::crypto::keys::{generate_user_encryption_key_pair,
                                 Key,
                                 KeyCache};
use std::path::Path;

pub fn start(ui: &mut UI, user: &str, key_cache: &KeyCache) -> Result<()> {
    ui.begin(format!("Generating user key for {}", user))?;
    let (public, secret) = generate_user_encryption_key_pair(user);
    key_cache.write_key(&public)?;
    key_cache.write_key(&secret)?;
    ui.end(format!("Generated user encryption key pair {}.",
                   &public.named_revision()))?;
    Ok(())
}
