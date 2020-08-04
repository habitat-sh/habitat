use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            error::Result};
use habitat_core::crypto::{artifact,
                           keys::sig_key_pair::SecretOriginSigningKey};
use std::path::Path;

pub fn start(ui: &mut UI, key: &SecretOriginSigningKey, src: &Path, dst: &Path) -> Result<()> {
    ui.begin(format!("Signing {}", src.display()))?;
    ui.status(Status::Signing,
              format!("{} with {} to create {}",
                      src.display(),
                      key.name_with_rev(),
                      dst.display()))?;
    artifact::sign(src, dst, key)?;
    ui.end(format!("Signed artifact {}.", dst.display()))?;
    Ok(())
}
