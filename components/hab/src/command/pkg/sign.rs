use std::path::Path;

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::crypto::{artifact,
                            SigKeyPair}};

use crate::error::Result;

pub fn start(ui: &mut UI, origin: &SigKeyPair, src: &Path, dst: &Path) -> Result<()> {
    ui.begin(format!("Signing {}", src.display()))?;
    ui.status(Status::Signing,
              format!("{} with {} to create {}",
                      src.display(),
                      &origin.name_with_rev(),
                      dst.display()))?;
    artifact::sign(src, dst, origin)?;
    ui.end(format!("Signed artifact {}.", dst.display()))?;
    Ok(())
}
