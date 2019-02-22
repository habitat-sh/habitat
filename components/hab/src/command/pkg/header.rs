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

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::crypto::artifact};

use crate::error::Result;

pub fn start(ui: &mut UI, src: &Path) -> Result<()> {
    ui.begin(format!("Reading package header for {}", &src.display()))?;
    ui.para("")?;
    if let Ok(header) = artifact::get_artifact_header(src) {
        println!("Package        : {}", &src.display());
        println!("Format Version : {}", header.format_version);
        println!("Key Name       : {}", header.key_name);
        println!("Hash Type      : {}", header.hash_type);
        println!("Raw Signature  : {}", header.signature_raw);
    } else {
        ui.warn("Failed to read package header.")?;
    }
    Ok(())
}
