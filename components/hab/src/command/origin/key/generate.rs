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
use hcore::crypto::SigKeyPair;

use error::{Error, Result};
use regex::Regex;

pub fn start(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    if origin.len() > 255 {
        return Err(Error::InvalidOrigin);
    }
    let valid_origin = Regex::new(r"^[a-z0-9][\w\-_]*$").expect("Unable to compile regex");

    match valid_origin.is_match(origin) {
        false => Err(Error::InvalidOrigin),
        true => {
            ui.begin(format!("Generating origin key for {}", &origin))?;
            let pair = SigKeyPair::generate_pair_for_origin(origin, cache)?;
            ui.end(format!(
                "Generated origin key pair {}.",
                &pair.name_with_rev()
            ))?;
            Ok(())
        }
    }
}
