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
#![cfg_attr(feature="clippy", allow(needless_pass_by_value))]

use std::io;
use std::fs::File;
use std::path::Path;

use hcore::crypto::SigKeyPair;
use hcore::crypto::keys::PairType;

use error::Result;

pub fn start(origin: &str, pair_type: PairType, cache: &Path) -> Result<()> {
    let latest = try!(SigKeyPair::get_latest_pair_for(origin, cache));
    let path = match pair_type {
        PairType::Public => {
            try!(SigKeyPair::get_public_key_path(
                &latest.name_with_rev(),
                cache,
            ))
        }
        PairType::Secret => {
            try!(SigKeyPair::get_secret_key_path(
                &latest.name_with_rev(),
                cache,
            ))
        }
    };
    let mut file = try!(File::open(&path));
    debug!(
        "Streaming file contents of {} {} to standard out",
        &pair_type,
        &path.display()
    );
    try!(io::copy(&mut file, &mut io::stdout()));
    Ok(())
}
