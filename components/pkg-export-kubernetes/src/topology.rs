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

use std::result;
use std::str::FromStr;
use std::fmt;

use error::Error;

#[derive(Debug, Clone)]
pub enum Topology {
    Standalone,
    Leader,
}

impl FromStr for Topology {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s {
            "standalone" => Ok(Topology::Standalone),
            "leader" => Ok(Topology::Leader),
            _ => Err(Error::InvalidTopology(s.to_string())),
        }
    }
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_lowercase();

        write!(f, "{}", s)
    }
}
