use crate::{common::ui::{Status,
                         UI,
                         UIWriter},
            error::Result};
use habitat_core::crypto::{artifact,
                           keys::{Key,
                                  SecretOriginSigningKey}};
use std::path::Path;

pub fn start(ui: &mut UI, key: &SecretOriginSigningKey, src: &Path, dst: &Path) -> Result<()> {
    ui.begin(format!("Signing {}", src.display()))?;
    ui.status(Status::Signing,
              format!("{} with {} to create {}",
                      src.display(),
                      key.named_revision(),
                      dst.display()))?;
    artifact::sign(src, dst, key)?;
    ui.end(format!("Signed artifact {}.", dst.display()))?;
    Ok(())
}
