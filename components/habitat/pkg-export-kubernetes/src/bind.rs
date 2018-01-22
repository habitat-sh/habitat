// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

use clap::ArgMatches;
use std::str::FromStr;

use error::Error;

#[derive(Debug, Clone)]
pub struct Bind {
    pub name: String,
    pub service: String,
    pub group: String,
}

impl FromStr for Bind {
    type Err = Error;

    fn from_str(bind_spec: &str) -> Result<Self, Error> {
        let split: Vec<&str> = bind_spec.split(":").collect();
        if split.len() != 3 || split[0] == "" || split[1] == "" || split[2] == "" {
            return Err(Error::InvalidBindSpec(bind_spec.to_owned()));
        }

        Ok(Bind {
            name: split[0].to_owned(),
            service: split[1].to_owned(),
            group: split[2].to_owned(),
        })
    }
}

pub fn parse_bind_args(matches: &ArgMatches) -> Result<Vec<Bind>, Error> {
    let mut binds = Vec::new();

    if let Some(bind_args) = matches.values_of("BIND") {
        for arg in bind_args {
            let b = arg.parse::<Bind>()?;

            binds.push(b);
        }
    };

    Ok(binds)
}
