use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::{crypto::keys::{box_key_pair::generate_service_encryption_key_pair,
                                  Key,
                                  KeyCache},
                   service::ServiceGroup};
use std::path::Path;

pub fn start(ui: &mut UI, org: &str, service_group: &ServiceGroup, cache: &Path) -> Result<()> {
    ui.begin(format!("Generating service key for {} in {}", &service_group, org))?;
    let cache = KeyCache::new(cache);
    let (public, secret) = generate_service_encryption_key_pair(org, &service_group.to_string());
    cache.write_key(&public)?;
    cache.write_key(&secret)?;
    ui.end(format!("Generated service key pair {}.", &public.named_revision()))?;
    Ok(())
}
