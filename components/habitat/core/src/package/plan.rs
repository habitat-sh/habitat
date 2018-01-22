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

use std::io::BufRead;

use error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Plan {
    pub name: String,
    pub origin: String,
    pub version: Option<String>,
}

impl Plan {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut name: Option<String> = None;
        let mut origin: Option<String> = None;
        let mut version: Option<String> = None;
        for line in bytes.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.splitn(2, "=").collect();
                match parts[0] {
                    "pkg_name" => name = Some(parts[1].to_string()),
                    "pkg_origin" => origin = Some(parts[1].to_string()),
                    "pkg_version" => version = Some(parts[1].to_string()),
                    _ => (),
                }
            }
        }

        if name.is_none() || origin.is_none() {
            return Err(Error::PlanMalformed);
        }

        Ok(Plan {
            name: name.unwrap(),
            origin: origin.unwrap(),
            version: version,
        })
    }
}
