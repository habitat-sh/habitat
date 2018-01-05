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

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use rand::{self, Rng};
use serde_json;
use time::Duration;

use error::{Error, Result};
use integrations::{encrypt, decrypt};

pub const BUILDER_KEY_NAME: &'static str = "bldr";

pub const BUILDER_ACCOUNT_NAME: &'static str = "__bldr__";
pub const BUILDER_ACCOUNT_EMAIL: &'static str = "bldr@habitat.sh";
pub const BUILDER_ACCOUNT_ID: u64 = 0;
pub const BUILDER_EXTERN_ID: u32 = 0;

const BUILDER_TOKEN_PREFIX: &'static str = "BLDR";
const BUILDER_TOKEN_EXPIRATION_SECONDS: i64 = 7200; // 2 hours

#[derive(Serialize, Deserialize, Debug)]
struct BuilderTokenPayload {
    pub expires: String,
    pub salt: u64,
}

pub fn is_bldr_token(token: &str) -> bool {
    token.starts_with(BUILDER_TOKEN_PREFIX)
}

pub fn generate_bldr_token(key_dir: &PathBuf) -> Result<String> {
    let mut rng = rand::thread_rng();
    let expires = Utc::now() + Duration::seconds(BUILDER_TOKEN_EXPIRATION_SECONDS);

    let payload = BuilderTokenPayload {
        expires: expires.to_rfc3339(),
        salt: rng.gen::<u64>(),
    };

    let plaintext = serde_json::to_string(&payload).unwrap();

    let ciphertext = encrypt(key_dir, &plaintext)?;

    Ok(format!("{}{}", BUILDER_TOKEN_PREFIX, ciphertext))
}

pub fn validate_bldr_token(key_dir: &PathBuf, token: &str) -> Result<()> {
    assert!(is_bldr_token(token));
    let encoded = token.to_owned().split_off(BUILDER_TOKEN_PREFIX.len());

    let plaintext = decrypt(key_dir, &encoded)?;

    let payload: BuilderTokenPayload = match serde_json::from_str(&plaintext) {
        Ok(p) => p,
        Err(e) => {
            warn!("Unable to deserialize builder token, err={:?}", e);
            return Err(Error::TokenExpired);
        }
    };

    let dt = DateTime::parse_from_rfc3339(&payload.expires).map_err(
        Error::ChronoError,
    )?;

    let now: DateTime<Utc> = Utc::now();
    let then: DateTime<Utc> = dt.with_timezone(&Utc);

    if then < now {
        Err(Error::TokenExpired)
    } else {
        Ok(())
    }
}
