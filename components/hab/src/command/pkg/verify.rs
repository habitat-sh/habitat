use crate::{common::ui::{Status,
                         UI,
                         UIWriter},
            error::Result};
use habitat_core::crypto::{artifact,
                           keys::KeyCache};
use std::path::Path;

pub fn start(ui: &mut UI, src: &Path, key_cache: &KeyCache) -> Result<()> {
    ui.begin(format!("Verifying artifact {}", &src.display()))?;
    let (name_with_rev, hash) = artifact::verify(src, key_cache)?;
    ui.status(Status::Verified,
              format!("checksum {} signed with {}", &hash, &name_with_rev))?;
    ui.end(format!("Verified artifact {}.", &src.display()))?;
    Ok(())
}
