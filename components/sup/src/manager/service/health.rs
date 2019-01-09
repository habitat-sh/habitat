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

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum HealthCheck {
    Ok,
    Warning,
    Critical,
    Unknown,
}

impl Default for HealthCheck {
    fn default() -> HealthCheck {
        HealthCheck::Unknown
    }
}

impl From<i8> for HealthCheck {
    fn from(value: i8) -> HealthCheck {
        match value {
            0 => HealthCheck::Ok,
            1 => HealthCheck::Warning,
            2 => HealthCheck::Critical,
            3 => HealthCheck::Unknown,
            _ => HealthCheck::Unknown,
        }
    }
}

impl fmt::Display for HealthCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            HealthCheck::Ok => "OK",
            HealthCheck::Warning => "WARNING",
            HealthCheck::Critical => "CRITICAL",
            HealthCheck::Unknown => "UNKNOWN",
        };
        write!(f, "{}", msg)
    }
}
