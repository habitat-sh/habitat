// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use crate::common::ui::{UIWriter, UI};
use crate::error::Result;
use crate::hcore::package::PackageArchive;
use serde::Serialize;
use serde_json::{self, Value as Json};
use std::path::Path;

fn convert_to_json<T>(src: &T) -> Json
where
    T: Serialize,
{
    serde_json::to_value(src).unwrap_or(Json::Null)
}

pub fn start(ui: &mut UI, src: &Path, to_json: bool) -> Result<()> {
    let ident = PackageArchive::new(src).ident()?;

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
