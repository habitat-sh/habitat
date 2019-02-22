// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
        let param = h
            .param(0)
            .and_then(|v| v.value().as_str())
            .and_then(|v| PackageIdent::from_str(v).ok())
            .ok_or_else(|| RenderError::new("Invalid package identifier for \"pkgPathFor\""))?;
        let deps =
            serde_json::from_value::<Vec<PackageIdent>>(rc.context().data()["pkg"]["deps"].clone())
                .unwrap();
        let target_pkg = deps
            .iter()
            .find(|ident| ident.satisfies(&param))
            .and_then(|i| {
                Some(
                    fs::pkg_install_path(&i, Some(&*fs::FS_ROOT_PATH))
                        .to_string_lossy()
                        .into_owned(),
                )
            })
            .unwrap_or_default();
        rc.writer.write_all(target_pkg.into_bytes().as_ref())?;
        Ok(())
    }
}

pub static PKG_PATH_FOR: PkgPathForHelper = PkgPathForHelper;
