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
use serde_json;
use std::result;
use std::str::FromStr;
use std::string::ToString;
use url::Url;

use crate::export_docker::Result;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Maintainer {
    name: String,
    email: Option<String>,
    url: Option<String>,
}

impl Maintainer {
    pub fn from_args(matches: &ArgMatches<'_>) -> Result<Vec<Self>> {
        let mut maintainers = Vec::new();

        if let Some(args) = matches.values_of("MAINTAINER") {
            for arg in args {
                let m = arg.parse::<Self>()?;

                maintainers.push(m);
            }
        };

        Ok(maintainers)
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "email": self.email,
            "url": self.url,
        })
    }
}

impl FromStr for Maintainer {
    type Err = Error;

    /// Creates a `Maintainer` struct from a string representation, which must be of the format
    /// `NAME[,EMAIL[,URL]]`.
    ///
    /// # Errors
    ///
    /// * `maintainer_str` is not of the format `NAME[,EMAIL[,URL]`
    /// * An invalid URL is specified
    fn from_str(maintainer_str: &str) -> result::Result<Self, Self::Err> {
        let values: Vec<&str> = maintainer_str.split(',').collect();
        if values.len() < 1 || values.len() > 3 {
            return Err(Error::InvalidMaintainer(maintainer_str.to_owned()));
        }

        let name = values[0].to_string();
        // FIXME: Check validity of email address
        let email = values.get(1).map(|&s| s.to_owned());
        let url = values.get(2).map(|&s| s.to_owned());
        if let Some(ref u) = url {
            Url::parse(&u).map_err(|e| Error::InvalidUrl(u.to_owned(), format!("{}", e)))?;
        };

        Ok(Maintainer { name, email, url })
    }
}
