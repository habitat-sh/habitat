use crate::{common::ui::{UI,
                         UIWriter},
            error::Result};
use habitat_core::crypto::artifact;
use std::path::Path;

pub fn start(ui: &mut UI, src: &Path) -> Result<()> {
    ui.begin(format!("Reading package header for {}", &src.display()))?;
    ui.para("")?;
    match artifact::get_artifact_header(src) {
        Ok(header) => {
            println!("Package        : {}", &src.display());
            println!("Format Version : {}", header.format());
            println!("Key Name       : {}", header.signer());
            println!("Hash Type      : {}", header.hash_type());
            // This should just be "Signature", but this is a public
            // interface right now.
            println!("Raw Signature  : {}", header.encoded_signature());
        }
        _ => {
            ui.warn("Failed to read package header.")?;
        }
    }
    Ok(())
}
