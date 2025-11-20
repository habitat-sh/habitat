use crate::{common::ui::{UI,
                         UIWriter},
            error::{Error,
                    Result},
            hcore::package::PackageArchiveInfo};
use habitat_core::util::text_render::PortableText;
use std::path::Path;

pub fn start(ui: &mut UI, src: &Path, to_json: bool) -> Result<()> {
    let info = PackageArchiveInfo::from_path(src)?;

    if to_json {
        match info.as_json() {
            Ok(content) => {
                println!("{}", content);
                return Ok(());
            }
            Err(e) => {
                ui.fatal(format!("Failed to deserialize into json! {:?}.", e))?;
                return Err(Error::from(e));
            }
        }
    } else {
        ui.begin(format!("Reading PackageIdent from {}", &src.display()))?;
        ui.para("")?;

        println!("Package Path   : {}", &src.display());
        println!("Origin         : {}", info.origin);
        println!("Name           : {}", info.name);
        println!("Version        : {}", info.version);
        println!("Release        : {}", info.release);
    }
    Ok(())
}
