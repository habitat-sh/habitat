use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::crypto::artifact};

use crate::error::Result;

pub fn start(ui: &mut UI, src: &Path) -> Result<()> {
    ui.begin(format!("Reading package header for {}", &src.display()))?;
    ui.para("")?;
    if let Ok(header) = artifact::get_artifact_header(src) {
        println!("Package        : {}", &src.display());
        println!("Format Version : {}", header.format_version);
        println!("Key Name       : {}", header.key_name);
        println!("Hash Type      : {}", header.hash_type);
        println!("Raw Signature  : {}", header.signature_raw);
    } else {
        ui.warn("Failed to read package header.")?;
    }
    Ok(())
}
