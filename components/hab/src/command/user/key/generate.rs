use crate::common::ui::{UIWriter,
                        UI};
use habitat_core::crypto::keys::{box_key_pair::generate_user_encryption_key_pair,
                                 Key,
                                 KeyCache};
use std::path::Path;

use crate::error::Result;

pub fn start(ui: &mut UI, user: &str, cache: &Path) -> Result<()> {
    ui.begin(format!("Generating user key for {}", user))?;
    let cache = KeyCache::new(cache);
    let (public, secret) = generate_user_encryption_key_pair(user);
    cache.write_key(&public)?;
    cache.write_key(&secret)?;
    ui.end(format!("Generated user encryption key pair {}.",
                   &public.named_revision()))?;
    Ok(())
}
