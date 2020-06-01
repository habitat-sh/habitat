use crate::{common::ui::{UIWriter,
                         UI},
            error::Result,
            hcore::package::PackageArchive};
use serde::Serialize;
use serde_json::{self,
                 Value as Json};
use std::path::Path;

fn convert_to_json<T>(src: &T) -> Json
    where T: Serialize
{
    serde_json::to_value(src).unwrap_or(Json::Null)
}

pub fn start(ui: &mut UI, src: &Path, to_json: bool) -> Result<()> {
    let ident = PackageArchive::new(src)?.ident()?;

    if to_json {
        println!("{}", convert_to_json(&ident));
    } else {
        ui.begin(format!("Reading PackageIdent from {}", &src.display()))?;
        ui.para("")?;

        println!("Package Path   : {}", &src.display());
        println!("Origin         : {}", &ident.origin);
        println!("Name           : {}", &ident.name);
        println!("Version        : {}", &ident.version.unwrap());
        println!("Release        : {}", &ident.release.unwrap());
    }
    Ok(())
}
