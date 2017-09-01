// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

// This module holds code that's common to dealing with the integrations for builder-api and
// builder-depot

use std::path::Path;

use base64;

use hab_core::crypto::BoxKeyPair;
use error::{Error, Result};
use keys;

pub fn encrypt<A>(key_dir: A, content: &str) -> Result<String>
where
    A: AsRef<Path>,
{
    let display_path = key_dir.as_ref().display();

    let kp = match BoxKeyPair::get_latest_pair_for(keys::BUILDER_KEY_NAME, &key_dir.as_ref()) {
        Ok(p) => p,
        Err(_) => {
            let e = format!("Can't find bldr key pair at {}", &display_path);
            error!("Can't find bldr key pair at {}", &display_path);
            return Err(Error::EncryptError(e));
        }
    };

    let ciphertext = match kp.encrypt(content.as_bytes(), None) {
        Ok(s) => s,
        Err(err) => {
            let e = format!("Unable to encrypt with bldr key pair, err={:?}", &err);
            error!("Unable to encrypt with bldr key pair, err={:?}", err);
            return Err(Error::EncryptError(e));
        }
    };

    Ok(base64::encode(&ciphertext))
}

pub fn decrypt<A>(key_dir: A, b64text: &str) -> Result<String>
where
    A: AsRef<Path>,
{
    let ciphertext = base64::decode(b64text).map_err(Error::Base64Error)?;
    let plaintext = match BoxKeyPair::decrypt(&ciphertext, &key_dir.as_ref()) {
        Ok(bytes) => String::from_utf8(bytes).map_err(Error::FromUtf8Error)?,
        Err(err) => {
            let e = format!("Unable to decrypt with bldr key pair, err={:?}", &err);
            error!("Unable to decrypt with bldr key pair, err={:?}", err);
            return Err(Error::DecryptError(e));
        }
    };

    Ok(plaintext)
}
