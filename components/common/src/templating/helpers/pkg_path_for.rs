use std::str::FromStr;

use crate::hcore::{fs,
                   package::{Identifiable,
                             PackageIdent}};
use handlebars::{Handlebars,
                 Helper,
                 HelperDef,
                 RenderContext,
                 RenderError};
use serde_json;

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct PkgPathForHelper;

impl HelperDef for PkgPathForHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let param =
            h.param(0)
             .and_then(|v| v.value().as_str())
             .and_then(|v| PackageIdent::from_str(v).ok())
             .ok_or_else(|| RenderError::new("Invalid package identifier for \"pkgPathFor\""))?;
        let deps =
            serde_json::from_value::<Vec<PackageIdent>>(rc.context().data()["pkg"]["deps"].clone())
                .unwrap();
        let target_pkg =
            deps.iter()
                .find(|ident| ident.satisfies(&param))
                .and_then(|i| {
                    Some(fs::pkg_install_path(&i, Some(&*fs::FS_ROOT_PATH)).to_string_lossy()
                                                                           .into_owned())
                })
                .unwrap_or_default();
        rc.writer.write_all(target_pkg.into_bytes().as_ref())?;
        Ok(())
    }
}

pub static PKG_PATH_FOR: PkgPathForHelper = PkgPathForHelper;
