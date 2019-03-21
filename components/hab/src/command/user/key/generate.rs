use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::crypto::BoxKeyPair};

use crate::error::Result;

pub fn start(ui: &mut UI, user: &str, cache: &Path) -> Result<()> {
    ui.begin(format!("Generating user key for {}", &user))?;
    let pair = BoxKeyPair::generate_pair_for_user(user)?;
    pair.to_pair_files(cache)?;
    ui.end(format!("Generated user key pair {}.", &pair.name_with_rev()))?;
    Ok(())
}
