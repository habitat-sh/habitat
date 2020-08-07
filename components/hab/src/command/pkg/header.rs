use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::crypto::artifact;
use std::path::Path;

pub fn start(ui: &mut UI, src: &Path) -> Result<()> {
    ui.begin(format!("Reading package header for {}", &src.display()))?;
    ui.para("")?;
    if let Ok(header) = artifact::get_artifact_header(src) {
        println!("Package        : {}", &src.display());
        println!("Format Version : {}", header.format());
        println!("Key Name       : {}", header.signer());
        println!("Hash Type      : {}", header.hash_type());
        println!("Raw Signature  : {}", header.signature_raw());
    } else {
        ui.warn("Failed to read package header.")?;
    }
    Ok(())
}
