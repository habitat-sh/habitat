use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::{crypto::keys::{Key,
                                  KeyCache},
                   service::ServiceGroup};

pub fn start(ui: &mut UI,
             org: &str,
             service_group: &ServiceGroup,
             key_cache: &KeyCache)
             -> Result<()> {
    ui.begin(format!("Generating service key for {} in {}", &service_group, org))?;
    let (public, _secret) = key_cache.new_service_encryption_pair(org, &service_group.to_string())?;
    ui.end(format!("Generated service key pair {}.", &public.named_revision()))?;
    Ok(())
}
