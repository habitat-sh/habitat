// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use glob::glob;
use std::path::Path;
use common::ui::UI;

use error::Result;

pub fn start(ui: &mut UI, cache: &Path) -> Result<()> {
    ui.heading("Local origin keys:")?;
    let path = cache.join("*.pub");

    match path.to_str() {
        Some(pattern) => {
            glob(&pattern)?
                .filter_map(|key| key.ok())
                .filter(|key| key.is_file())
                // It is safe to use `unwrap()` here because the only case in which `file_name()`
                // returns `None` is if the path is a directory, but we filter out directories in the
                // line above.
                .map(|key| key.file_name().unwrap().to_os_string())
                // We need this conversion because `OsString` does not implement `Display`.
                .filter_map(|key| key.into_string().ok())
                .for_each(|key| println!("» {}", key))
        }
        None => debug!("Failed to parse path: {:?}", path),
    }

    Ok(())
}
