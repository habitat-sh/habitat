use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::{crypto::keys::{generate_signing_key_pair,
                                   KeyCache},
                    package::ident,
                    Error::InvalidOrigin}};

use crate::error::{Error,
                   Result};

pub fn start(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    if ident::is_valid_origin_name(origin) {
        ui.begin(format!("Generating origin key for {}", &origin))?;

        let cache = KeyCache::new(cache);
        let (public, secret) = generate_signing_key_pair(origin);
        cache.write_key(&public)?;
        cache.write_key(&secret)?;

        ui.end(format!("Generated origin key pair {}.", public.name_with_rev()))?;
        Ok(())
    } else {
        Err(Error::from(InvalidOrigin(origin.to_string())))
    }
}
