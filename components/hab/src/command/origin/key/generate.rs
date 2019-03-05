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
            hcore::{crypto::SigKeyPair,
                    package::ident,
                    Error::InvalidOrigin}};

use crate::error::{Error,
                   Result};

pub fn start(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    if ident::is_valid_origin_name(origin) {
        ui.begin(format!("Generating origin key for {}", &origin))?;
        let pair = SigKeyPair::generate_pair_for_origin(origin)?;
        pair.to_pair_files(cache)?;
        ui.end(format!("Generated origin key pair {}.", &pair.name_with_rev()))?;
        Ok(())
    } else {
        Err(Error::from(InvalidOrigin(origin.to_string())))
    }
}
