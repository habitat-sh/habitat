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

use error::{Error, Result};
use os::system::{Architecture, Platform};
use util::{deserialize_using_from_str, serialize_using_to_string};

pub trait Target: fmt::Display + Into<PackageTarget> {
    fn validate(&self) -> Result<()>;
}

/// Describes the platform (operating system/kernel)
/// and architecture (x86_64, i386, etc..) that a package is built for
#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct PackageTarget {
    #[serde(deserialize_with = "deserialize_using_from_str",
            serialize_with = "serialize_using_to_string")]
    pub platform: Platform,
    #[serde(deserialize_with = "deserialize_using_from_str",
            serialize_with = "serialize_using_to_string")]
    pub architecture: Architecture,
}

impl PackageTarget {
    /// Creates a new package target
    ///
    /// Errors:
    /// * InvalidPlatform
    /// * InvalidArchitecture
    pub fn new(platform: Platform, architecture: Architecture) -> Self {
        PackageTarget {
            platform: platform,
            architecture: architecture,
        }
    }

    pub fn current_platform() -> Platform {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "linux") {
            Platform::Linux
        } else if cfg!(target_os = "macos") {
            Platform::Darwin
        } else {
            unreachable!("binary built for an unknown platform")
        }
    }

    pub fn current_architecture() -> Architecture {
        if cfg!(target_arch = "x86_64") {
            Architecture::X86_64
        } else {
            unreachable!("binary built for an unknown architecture")
        }
    }
}

impl Target for PackageTarget {
    fn validate(&self) -> Result<()> {
        let default = PackageTarget::default();
        if self.architecture == default.architecture && self.platform == default.platform {
            Ok(())
        } else {
            Err(Error::TargetMatchError(
                format!("Package target ({}-{}) does not match system \
                                                 target ({}-{}).",
                                                self.architecture,
                                                self.platform,
                                                default.architecture,
                                                default.platform,),
            ))
        }
    }
}

impl Default for PackageTarget {
    fn default() -> PackageTarget {
        PackageTarget {
            platform: Self::current_platform(),
            architecture: Self::current_architecture(),
        }
    }
}

impl fmt::Display for PackageTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.architecture, self.platform)
    }
}

impl FromStr for PackageTarget {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let items: Vec<&str> = value.split("-").collect();
        let (architecture, platform) = match items.len() {
            2 => {
                (
                    try!(Architecture::from_str(items[0].into())),
                    try!(Platform::from_str(items[1]).into()),
                )
            }
            _ => return Err(Error::InvalidPackageTarget(value.to_string())),
        };
        Ok(PackageTarget::new(platform, architecture))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use os::system::{Architecture, Platform};

    #[test]
    fn package_target_matches_current_operating_system() {
        let target = PackageTarget::default();
        if cfg!(target_os = "windows") {
            assert_eq!(target.platform, Platform::Windows);
        } else if cfg!(target_os = "linux") {
            assert_eq!(target.platform, Platform::Linux);
        } else if cfg!(target_os = "macos") {
            assert_eq!(target.platform, Platform::Darwin);
        } else {
            unreachable!(
                "Platform not defined for target_os! Fix this by adding a conditional \
                          compilation to PackageTarget::current_platform()"
            );
        }
        assert_eq!(target.architecture, Architecture::X86_64);
    }

    #[test]
    fn package_target_from_string() {
        let target = PackageTarget::from_str("x86_64-windows").unwrap();
        assert_eq!(target.platform, Platform::Windows);
        assert_eq!(target.architecture, Architecture::X86_64);
    }

    #[test]
    #[should_panic]
    fn package_target_with_reversed_target_string() {
        PackageTarget::from_str("linux-x86_64").unwrap();
    }

    #[test]
    #[should_panic]
    fn package_target_with_invalid_platform() {
        PackageTarget::from_str("x86_64-intermezzos").unwrap();
    }

    #[test]
    #[should_panic]
    fn package_target_with_invalid_architecture() {
        let _ = PackageTarget::from_str("i986-linux").unwrap();
    }

    #[test]
    fn package_target_validate_matching_platform_and_architecture() {
        current_platform_target().validate().unwrap();
    }

    #[test]
    #[should_panic]
    fn package_target_does_not_validate_different_platform() {
        unsupported_platform_target().validate().unwrap();
    }

    fn current_platform_target() -> PackageTarget {
        if cfg!(target_os = "windows") {
            return PackageTarget::from_str("x86_64-windows").unwrap();
        } else if cfg!(target_os = "linux") {
            PackageTarget::from_str("x86_64-linux").unwrap()
        } else if cfg!(target_os = "macos") {
            PackageTarget::from_str("x86_64-darwin").unwrap()
        } else {
            unreachable!(
                "Test case not defined for target_os! Fix this by adding a conditional \
                          compilation to tests::current_platform_target()"
            );
        }
    }

    fn unsupported_platform_target() -> PackageTarget {
        if cfg!(target_os = "windows") {
            PackageTarget::from_str("x86_64-linux").unwrap()
        } else if cfg!(target_os = "linux") {
            PackageTarget::from_str("x86_64-windows").unwrap()
        } else if cfg!(target_os = "macos") {
            PackageTarget::from_str("x86_64-windows").unwrap()
        } else {
            unreachable!(
                "Test case not defined for target_os! Fix this by adding a conditional \
                          compilation to tests::unsupported_platform_target()"
            );
        }
    }
}
