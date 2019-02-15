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

use std::{fmt, result, str::FromStr};

use crate::error::Error;

#[derive(Clone, Debug)]
pub enum Topology {
    Standalone,
    Leader,
}

impl Topology {
    fn as_str(&self) -> &str {
        match *self {
            Topology::Leader => "leader",
            Topology::Standalone => "standalone",
        }
    }
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Topology {
    type Err = Error;

    fn from_str(topology: &str) -> result::Result<Self, Self::Err> {
        match topology {
            "leader" => Ok(Topology::Leader),
            "standalone" => Ok(Topology::Standalone),
            _ => Err(Error::InvalidTopology(String::from(topology))),
        }
    }
}

impl Default for Topology {
    fn default() -> Topology {
        Topology::Standalone
    }
}
