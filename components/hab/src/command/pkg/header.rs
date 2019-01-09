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

use crate::common::ui::{UIWriter, UI};
use crate::hcore::crypto::artifact;
use std::io::{self, Write};

use crate::error::Result;

pub fn start(ui: &mut UI, src: &Path) -> Result<()> {
    ui.begin(format!("Reading package header for {}", &src.display()))?;
    ui.para("")?;
    if let Ok(header) = artifact::get_artifact_header(src) {
        io::stdout().write(format!("Package        : {}\n", &src.display()).as_bytes())?;
        io::stdout().write(format!("Format Version : {}\n", header.format_version).as_bytes())?;
        io::stdout().write(format!("Key Name       : {}\n", header.key_name).as_bytes())?;
        io::stdout().write(format!("Hash Type      : {}\n", header.hash_type).as_bytes())?;
        io::stdout().write(format!("Raw Signature  : {}\n", header.signature_raw).as_bytes())?;
    } else {
        ui.warn("Failed to read package header.")?;
    }
    Ok(())
}
