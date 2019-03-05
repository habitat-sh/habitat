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

use std::{fs::File,
          io,
          path::Path};

use crate::hcore::crypto::{keys::PairType,
                           SigKeyPair};

use crate::error::Result;

#[warn(clippy::needless_pass_by_value)] // Remove after making `PairType` Copy
pub fn start(origin: &str, pair_type: PairType, cache: &Path) -> Result<()> {
    let latest = SigKeyPair::get_latest_pair_for(origin, cache, Some(&pair_type))?;
    let path = match pair_type {
        PairType::Public => SigKeyPair::get_public_key_path(&latest.name_with_rev(), cache)?,
        PairType::Secret => SigKeyPair::get_secret_key_path(&latest.name_with_rev(), cache)?,
    };
    let mut file = File::open(&path)?;
    debug!("Streaming file contents of {} {} to standard out",
           &pair_type,
           &path.display());
    io::copy(&mut file, &mut io::stdout())?;
    Ok(())
}
