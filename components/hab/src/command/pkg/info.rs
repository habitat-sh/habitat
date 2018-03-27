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

use std::path::Path;
use common::ui::UI;
use std::io::{self, Write};
use hcore::package::PackageArchive;
use error::Result;
use serde::Serialize;
use serde_json::{self, Value as Json};

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
        ui.begin(
            format!("Reading PackageIdent from {}", &src.display()),
        )?;
        ui.para("")?;

        io::stdout().write(
            format!("Package Path   : {}\n", &src.display())
                .as_bytes(),
        )?;
        io::stdout().write(
            format!("Origin         : {}\n", &ident.origin)
                .as_bytes(),
        )?;
        io::stdout().write(
            format!("Name           : {}\n", &ident.name)
                .as_bytes(),
        )?;
        io::stdout().write(
            format!(
                "Version        : {}\n",
                &ident.version.unwrap()
            ).as_bytes(),
        )?;
        io::stdout().write(
            format!(
                "Release        : {}\n",
                &ident.release.unwrap()
            ).as_bytes(),
        )?;
    }
    Ok(())
}
