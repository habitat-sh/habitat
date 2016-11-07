// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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
use os::system::{uname, Uname};

use error::{Error, Result};

pub trait Target: fmt::Display + Into<PackageTarget> {
    fn platform(&self) -> &str;
    fn architecture(&self) -> &str;
    fn valid_for(&self, uname: Uname) -> Result<()>;
}

/// Describes the platform (operating system/kernel)
/// and architecture (x86_64, i386, etc..) that a package is built for
#[derive(RustcEncodable, RustcDecodable, Debug, Clone, Hash)]
pub struct PackageTarget {
    pub platform: String,
    pub architecture: String,
}

impl PackageTarget {
    /// Creates a new package target
    ///
    /// Errors:
    /// * InvalidPackageTargetPlatform
    /// * InvalidPackageTargetArchitecture
    pub fn new<T: Into<String>>(platform: T, architecture: T) -> Self {
        let valid_platform = PackageTarget::supported_platform(platform).unwrap();
        let valid_arch = PackageTarget::supported_architecture(architecture).unwrap();
        PackageTarget {
            platform: valid_platform,
            architecture: valid_arch,
        }
    }

    fn supported_platform<T: Into<String>>(value: T) -> Result<String> {
        let platform = value.into();
        let valid_platforms = vec!["linux", "windows",];
        if valid_platforms.iter().any(|&x| x == platform) {
            Ok(platform)
        } else {
            Err(Error::InvalidPackageTargetPlatform(platform))
        }
    }

    fn supported_architecture<T: Into<String>>(value: T) -> Result<String> {
        let architecture = value.into();
        let valid_architectures = vec!["x86_64"];
        if valid_architectures.iter().any(|&x| x == architecture) {
            Ok(architecture)
        } else {
            Err(Error::InvalidPackageTargetArchitecture(architecture))
        }
    }
}

impl Target for PackageTarget {
    fn platform(&self) -> &str {
        &self.platform
    }
    fn architecture(&self) -> &str {
        &self.architecture
    }
    fn valid_for(&self, uname: Uname) -> Result<()> {
        let architecture = uname.machine.trim().to_lowercase();
        let platform = uname.sys_name.trim().to_lowercase();
        if self.architecture == architecture && self.platform == platform {
            Ok(())
        } else {
            Err(Error::TargetMatchError(format!("Package target ({}-{}) does not match system \
                                                 target ({}-{}).",
                                                self.architecture,
                                                self.platform,
                                                architecture,
                                                platform,)))
        }
    }
}

impl Default for PackageTarget {
    fn default() -> PackageTarget {
        let current_system = uname().unwrap();
        PackageTarget::new(current_system.sys_name.trim().to_lowercase(),
                           current_system.machine.trim().to_lowercase())
    }
}

impl fmt::Display for PackageTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.architecture, self.platform)
    }
}

impl AsRef<PackageTarget> for PackageTarget {
    fn as_ref(&self) -> &PackageTarget {
        self
    }
}

impl FromStr for PackageTarget {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let items: Vec<&str> = value.split("-").collect();
        let (architecture, platform) = match items.len() {
            2 => (items[0], items[1]),
            _ => return Err(Error::InvalidPackageTarget(value.to_string())),
        };
        Ok(PackageTarget::new(platform, architecture))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use os::system::Uname;

    #[test]
    fn package_target_matches_current_operating_system() {
        let target = PackageTarget::default();
        if cfg!(target_os = "windows") {
            assert_eq!(target.platform, "windows");
        } else {
            assert_eq!(target.platform, "linux");
        }
        assert_eq!(target.architecture, "x86_64");
    }

    #[test]
    fn package_target_from_string() {
        let target = PackageTarget::from_str("x86_64-windows").unwrap();
        assert_eq!(target.platform, "windows");
        assert_eq!(target.architecture, "x86_64");
    }

    #[test]
    #[should_panic]
    fn package_target_with_reversed_target_string() {
        let _ = PackageTarget::from_str("linux-x86_64").unwrap();
    }

    #[test]
    #[should_panic]
    fn package_target_with_invalid_platform() {
        let _ = PackageTarget::from_str("x86_64-intermezzos").unwrap();
    }

    #[test]
    #[should_panic]
    fn package_target_with_invalid_architecture() {
        let _ = PackageTarget::from_str("i986-linux").unwrap();
    }

    #[test]
    fn package_target_valid_for_matching_platform_and_architecture() {
        let target = PackageTarget::from_str("x86_64-linux").unwrap();
        let _ = target.valid_for(Uname {
                sys_name: "linux".to_string(),
                node_name: "test_node".to_string(),
                release: "4.2.0-25-generic".to_string(),
                version: "#30-Ubuntu SMP Mon Jan 18 12:31:50 UTC 201".to_string(),
                machine: "x86_64".to_string(),
            })
            .unwrap();

    }

    #[test]
    #[should_panic]
    fn package_target_invalid_for_different_platform() {
        let target = PackageTarget::from_str("x86_64-windows").unwrap();
        let _ = target.valid_for(Uname {
                sys_name: "linux".to_string(),
                node_name: "test_node".to_string(),
                release: "4.2.0-25-generic".to_string(),
                version: "#30-Ubuntu SMP Mon Jan 18 12:31:50 UTC 201".to_string(),
                machine: "x86_64".to_string(),
            })
            .unwrap();
    }
}
