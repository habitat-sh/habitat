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

use std::fmt;
use std::result;
use std::str::FromStr;
use error::Error;
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::uname;
#[cfg(not(windows))]
pub mod linux;
#[cfg(not(windows))]
pub use self::linux::uname;

#[derive(Debug)]
pub struct Uname {
    pub sys_name: String,
    pub node_name: String,
    pub release: String,
    pub version: String,
    pub machine: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Architecture {
    X86_64,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let architecture_string = format!("{:?}", self);
        write!(f, "{}", architecture_string.to_lowercase())
    }
}

#[derive(Debug, Hash, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Platform {
    Linux,
    Windows,
    Darwin,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let platform_string = format!("{:?}", self);
        write!(f, "{}", platform_string.to_lowercase())
    }
}

impl FromStr for Architecture {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let architecture = value.trim().to_lowercase();
        match architecture.as_ref() {
            "x86_64" => Ok(Architecture::X86_64),
            _ => return Err(Error::InvalidArchitecture(value.to_string())),
        }
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let platform = value.trim().to_lowercase();
        match platform.as_ref() {
            "linux" => Ok(Platform::Linux),
            "windows" => Ok(Platform::Windows),
            "darwin" => Ok(Platform::Darwin),
            _ => return Err(Error::InvalidPlatform(value.to_string())),
        }
    }
}
