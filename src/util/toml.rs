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

use std::str::{self, FromStr};

use toml;

/// Create a `toml::Table` from a byte array.
///
/// `None` will be returned if the byte array contains invalid UTF-8 or does not represent a
/// `toml::Table`.
pub fn table_from_bytes(bytes: &[u8]) -> Option<toml::Table> {
    str::from_utf8(bytes)
        .ok()
        .and_then(|v| toml::Value::from_str(v).ok())
        .and_then(|v| match v {
            toml::Value::Table(s) => Some(s),
            _ => None,
        })
}
