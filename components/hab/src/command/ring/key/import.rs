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
use crate::hcore::crypto::SymKey;

use crate::error::Result;

pub fn start(ui: &mut UI, content: &str, cache: &Path) -> Result<()> {
    ui.begin("Importing ring key from standard input")?;
    let (pair, pair_type) = SymKey::write_file_from_str(content, cache)?;
    ui.end(format!(
        "Imported {} ring key {}.",
        &pair_type,
        &pair.name_with_rev()
    ))?;
    Ok(())
}
