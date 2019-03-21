use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::crypto::SigKeyPair};

use crate::error::Result;

pub fn start(ui: &mut UI, content: &str, cache: &Path) -> Result<()> {
    ui.begin("Importing origin key from standard input")?;
    let (pair, pair_type) = SigKeyPair::write_file_from_str(content, cache)?;
    ui.end(format!("Imported {} origin key {}.",
                   &pair_type,
                   &pair.name_with_rev()))?;
    Ok(())
}
