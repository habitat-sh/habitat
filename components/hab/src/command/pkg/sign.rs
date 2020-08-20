use std::path::{Path,
                PathBuf};

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::{crypto::{artifact,
                             SigKeyPair},
                    package::{Identifiable,
                              PackageArchive}}};

use crate::error::Result;

pub fn start(ui: &mut UI, origin: &SigKeyPair, src: &Path, dst: &Path) -> Result<()> {
    // Detect if what we're about to sign is in fact a PackageArchive and
    // if so, we need to do some basic PackageIdent field validation.
    if let Ok(mut archive) = PackageArchive::new(PathBuf::from(dst)) {
        let ident = archive.ident()?;
        ui.status(Status::Verifying,
                  format!("{} has valid PackageIdent fields", ident))?;
        if !ident.valid() {
            ident.validate_fields()?;
        }
    }
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
