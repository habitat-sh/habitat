use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::{crypto::SigKeyPair,
                    package::ident,
                    Error::InvalidOrigin}};

use crate::error::{Error,
                   Result};

pub fn start(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    if ident::is_valid_origin_name(origin) {
        ui.begin(format!("Generating origin key for {}", &origin))?;
        let pair = SigKeyPair::generate_pair_for_origin(origin)?;
        pair.to_pair_files(cache)?;
        ui.end(format!("Generated origin key pair {}.", &pair.name_with_rev()))?;
        Ok(())
    } else {
        Err(Error::from(InvalidOrigin(origin.to_string())))
    }
}
