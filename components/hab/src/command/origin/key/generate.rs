use crate::{common::ui::{UIWriter,
                         UI},
            error::{Error,
                    Result}};
use habitat_core::{crypto::keys::{Key,
                                  KeyCache},
                   package::ident,
                   Error::InvalidOrigin};

pub fn start(ui: &mut UI, origin: &str, key_cache: &KeyCache) -> Result<()> {
    if ident::is_valid_origin_name(origin) {
        ui.begin(format!("Generating origin key for {}", &origin))?;
        let (public, _secret) = key_cache.new_signing_pair(origin)?;
        ui.end(format!("Generated origin key pair {}.", public.named_revision()))?;
        Ok(())
    } else {
        Err(Error::from(InvalidOrigin(origin.to_string())))
    }
}
