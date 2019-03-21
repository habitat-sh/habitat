use std::path::Path;

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::crypto::artifact};

use crate::error::Result;

pub fn start(ui: &mut UI, src: &Path, cache: &Path) -> Result<()> {
    ui.begin(format!("Verifying artifact {}", &src.display()))?;
    let (name_with_rev, hash) = artifact::verify(src, cache)?;
    ui.status(Status::Verified,
              format!("checksum {} signed with {}", &hash, &name_with_rev))?;
    ui.end(format!("Verified artifact {}.", &src.display()))?;
    Ok(())
}
