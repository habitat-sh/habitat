use crate::{common::ui::{UIWriter,
                         UI},
            error::Result,
            hcore::package::PackageArchiveInfo};
use serde::Serialize;
use serde_json::{self,
                 Value as Json};
use std::path::Path;

fn convert_to_json<T>(src: &T) -> Result<Json>
    where T: Serialize
{
    serde_json::to_value(src).map_err(|e| habitat_core::Error::RenderContextSerialization(e).into())
}

pub fn start(ui: &mut UI, src: &Path, to_json: bool) -> Result<()> {
    let info = PackageArchiveInfo::new(src)?;

    if to_json {
        match convert_to_json(&info) {
            Ok(content) => {
                println!("{}", content);
                return Ok(());
            }
            Err(e) => {
                ui.fatal(format!("Failed to deserialize into json! {:?}.", e))?;
                return Err(e);
            }
        }
    } else {
        ui.begin(format!("Reading PackageIdent from {}", &src.display()))?;
        ui.para("")?;

        println!("Package Path   : {}", &src.display());
        println!("Origin         : {}", info.origin);
        println!("Name           : {}", info.name);
        println!("Version        : {}",
                 info.version.unwrap_or_else(|| "".to_string()));
        println!("Release        : {}",
                 info.release.unwrap_or_else(|| "".to_string()));
    }
    Ok(())
}
