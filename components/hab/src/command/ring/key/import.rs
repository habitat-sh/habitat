use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::crypto::RingKey};

use crate::error::Result;

pub fn start(ui: &mut UI, content: &str, cache: &Path) -> Result<()> {
    ui.begin("Importing ring key from standard input")?;
    let key = RingKey::write_file_from_str(content, cache)?;
    ui.end(format!("Imported ring key {}.", &key.name_with_rev()))?;
    Ok(())
}
