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

pub mod download;
pub mod export;
pub mod generate;
pub mod import;
pub mod upload;
pub mod upload_latest;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    error::{Error, Result},
    hcore,
};

// shared between origin::key::upload and origin::key::upload_latest
fn get_name_with_rev(keyfile: &Path, expected_vsn: &str) -> Result<String> {
    let f = File::open(&keyfile)?;
    let f = BufReader::new(f);
    let mut lines = f.lines();
    match lines.next() {
        Some(val) => {
            let val = val?;
            if val != expected_vsn {
                let msg = format!("Unsupported version: {}", &val);
                return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
            }
        }
        None => {
            let msg = "Corrupt key file, can't read file version".to_string();
            return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
        }
    }
    let name_with_rev = match lines.next() {
        Some(val) => val?,
        None => {
            let msg = "Corrupt key file, can't read name with rev".to_string();
            return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
        }
    };
    Ok(name_with_rev)
}
