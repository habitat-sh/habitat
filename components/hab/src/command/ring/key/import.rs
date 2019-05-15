use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::crypto::SymKey};

use crate::error::Result;

pub fn start(ui: &mut UI, content: &str, cache: &Path) -> Result<()> {
    ui.begin("Importing ring key from standard input")?;
    let (pair, pair_type) = SymKey::write_file_from_str(content, cache)?;
    ui.end(format!("Imported {} ring key {}.",
                   &pair_type,
                   &pair.name_with_rev()))?;
    Ok(())
}
