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

use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::result;
use std::str::FromStr;

use regex::Regex;

use error::{Error, Result};

pub trait Identifiable: fmt::Display + Into<PackageIdent> {
    fn origin(&self) -> &str;
    fn name(&self) -> &str;
    fn version(&self) -> Option<&str>;
    fn release(&self) -> Option<&str>;

    fn fully_qualified(&self) -> bool {
        self.version().is_some() && self.release().is_some()
    }

    fn satisfies<I: Identifiable>(&self, other: &I) -> bool {
        if self.origin() != other.origin() || self.name() != other.name() {
            return false;
        }
        if self.version().is_some() {
            if other.version().is_none() {
                return true;
            }
            if *self.version().unwrap() != *other.version().unwrap() {
                return false;
            }
        }
        if self.release().is_some() {
            if other.release().is_none() {
                return true;
            }
            if *self.release().unwrap() != *other.release().unwrap() {
                return false;
            }
        }
        true
    }
}

#[derive(RustcEncodable, RustcDecodable, Eq, PartialEq, Debug, Clone, Hash)]
pub struct PackageIdent {
    pub origin: String,
    pub name: String,
    pub version: Option<String>,
    pub release: Option<String>,
}

impl PackageIdent {
    /// Creates a new package identifier
    pub fn new<T: Into<String>>(origin: T,
                                name: T,
                                version: Option<T>,
                                release: Option<T>)
                                -> Self {
        PackageIdent {
            origin: origin.into(),
            name: name.into(),
            version: version.map(|v| v.into()),
            release: release.map(|v| v.into()),
        }
    }

    pub fn archive_name(&self) -> Option<String> {
        if self.fully_qualified() {
            Some(format!("{}-{}-{}-{}-x86_64-linux.hart",
                         self.origin,
                         self.name,
                         self.version.as_ref().unwrap(),
                         self.release.as_ref().unwrap()))
        } else {
            None
        }
    }
}

impl Identifiable for PackageIdent {
    fn origin(&self) -> &str {
        &self.origin
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> Option<&str> {
        self.version.as_ref().map(|f| f.as_str())
    }

    fn release(&self) -> Option<&str> {
        self.release.as_ref().map(|f| f.as_str())
    }
}

impl Default for PackageIdent {
    fn default() -> PackageIdent {
        PackageIdent::new("", "", None, None)
    }
}

impl fmt::Display for PackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.version.is_some() && self.release.is_some() {
            write!(f,
                   "{}/{}/{}/{}",
                   self.origin,
                   self.name,
                   self.version.as_ref().unwrap(),
                   self.release.as_ref().unwrap())
        } else if self.version.is_some() {
            write!(f,
                   "{}/{}/{}",
                   self.origin,
                   self.name,
                   self.version.as_ref().unwrap())
        } else {
            write!(f, "{}/{}", self.origin, self.name)
        }
    }
}

impl AsRef<PackageIdent> for PackageIdent {
    fn as_ref(&self) -> &PackageIdent {
        self
    }
}

impl FromStr for PackageIdent {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let items: Vec<&str> = value.split("/").collect();
        let (origin, name, ver, rel) = match items.len() {
            2 => (items[0], items[1], None, None),
            3 => (items[0], items[1], Some(items[2]), None),
            4 => (items[0], items[1], Some(items[2]), Some(items[3])),
            _ => return Err(Error::InvalidPackageIdent(value.to_string())),
        };
        Ok(PackageIdent::new(origin, name, ver, rel))
    }
}

impl PartialOrd for PackageIdent {
    /// Packages can be compared according to the following:
    ///
    /// * origin is ignored in the comparison - my redis and
    ///   your redis compare the same.
    /// * If the names are not equal, they cannot be compared.
    /// * If the versions are greater/lesser, return that as
    ///   the ordering.
    /// * If the versions are equal, return the greater/lesser
    ///   for the release.
    fn partial_cmp(&self, other: &PackageIdent) -> Option<Ordering> {
        if self.name != other.name {
            return None;
        }
        if self.version.is_none() && other.version.is_none() {
            return None;
        }
        if self.version.is_none() && other.version.is_some() {
            return Some(Ordering::Less);
        }
        if self.version.is_some() && other.version.is_none() {
            return Some(Ordering::Greater);
        }
        if self.release.is_none() && other.release.is_none() {
            return None;
        }
        if self.release.is_none() && other.release.is_some() {
            return Some(Ordering::Less);
        }
        if self.release.is_some() && other.release.is_none() {
            return Some(Ordering::Greater);
        }
        match version_sort(self.version.as_ref().unwrap(),
                           other.version.as_ref().unwrap()) {
            ord @ Ok(Ordering::Greater) |
            ord @ Ok(Ordering::Less) => ord.ok(),
            Ok(Ordering::Equal) => Some(self.release.cmp(&other.release)),
            Err(e) => {
                error!("This was a very bad version number: {:?}", e);
                return None;
            }
        }
    }
}

impl Ord for PackageIdent {
    /// Packages can be compared according to the following:
    ///
    /// * origin is ignored in the comparison - my redis and
    ///   your redis compare the same.
    /// * If the names are not equal, they cannot be compared.
    /// * If the versions are greater/lesser, return that as
    ///   the ordering.
    /// * If the versions are equal, return the greater/lesser
    ///   for the release.
    fn cmp(&self, other: &PackageIdent) -> Ordering {
        if self.name != other.name {
            return self.name.cmp(&other.name);
        }
        match version_sort(self.version.as_ref().unwrap(),
                           other.version.as_ref().unwrap()) {
            ord @ Ok(Ordering::Greater) |
            ord @ Ok(Ordering::Less) => ord.unwrap(),
            Ok(Ordering::Equal) => self.release.cmp(&other.release),
            Err(_) => Ordering::Less,
        }
    }
}

/// Sorts two packages according to their version.
///
/// We are a bit more strict than your average package management solution on versioning.
/// What we support is the "some number of digits or dots" (the version number),
/// followed by an optional "-" and any alphanumeric string (the extension). When determining sort
/// order, we:
///
/// * Separate the version numbers from the extensions
/// * Split the version numbers into an array of digits on any '.' characters. Digits are convered
///   into <u64>.
/// * Compare the version numbers by iterating over them. If 'a' is greater or lesser than 'b', we
///   return that as the result. If it is equal, we move to the next digit and repeat. If one of
///   the version numbers is exhausted before the other, it gains 0's for the missing slot.
/// * If the version numbers are equal, but either A or B has an extension (but not both) than the
///   version without the extension is greater. (1.0.0 is greater than 1.0.0-alpha6)
/// * If both have an extension, it is compared lexicographically, with the result as the final
///   ordering.
///
/// Returns a Error if we fail to match for any reason.
pub fn version_sort(a_version: &str, b_version: &str) -> Result<Ordering> {
    let (a_parts, a_extension) = try!(split_version(a_version));
    let (b_parts, b_extension) = try!(split_version(b_version));
    let mut a_iter = a_parts.iter();
    let mut b_iter = b_parts.iter();
    loop {
        let mut a_exhausted = false;
        let mut b_exhausted = false;
        let a_num = match a_iter.next() {
            Some(i) => try!(i.parse::<u64>()),
            None => {
                a_exhausted = true;
                0u64
            }
        };
        let b_num = match b_iter.next() {
            Some(i) => try!(i.parse::<u64>()),
            None => {
                b_exhausted = true;
                0u64
            }
        };
        if a_exhausted && b_exhausted {
            break;
        }
        match a_num.cmp(&b_num) {
            Ordering::Greater => {
                return Ok(Ordering::Greater);
            }
            Ordering::Equal => {
                continue;
            }
            Ordering::Less => {
                return Ok(Ordering::Less);
            }
        }
    }

    // If you have equal digits, and one has an extension, it is
    // the plain digits who win.
    // 1.0.0-alpha1 vs 1.0.0
    if a_extension.is_some() && b_extension.is_none() {
        return Ok(Ordering::Less);
    } else if a_extension.is_none() && b_extension.is_some() {
        return Ok(Ordering::Greater);
    } else if a_extension.is_none() && b_extension.is_none() {
        return Ok(Ordering::Equal);
    } else {
        let a = match a_extension {
            Some(a) => a,
            None => String::new(),
        };
        let b = match b_extension {
            Some(b) => b,
            None => String::new(),
        };
        return Ok(a.cmp(&b));
    }
}

fn split_version(version: &str) -> Result<(Vec<&str>, Option<String>)> {
    let re = try!(Regex::new(r"([\d\.]+)(-.+)?"));
    let caps = match re.captures(version) {
        Some(caps) => caps,
        None => return Err(Error::InvalidPackageIdent(version.to_string())),
    };
    let version_number = caps.at(1).unwrap();
    let extension = match caps.at(2) {
        Some(e) => {
            let mut estr: String = e.to_string();
            estr.remove(0);
            Some(estr)
        }
        None => None,
    };
    let version_parts: Vec<&str> = version_number.split('.').collect();
    Ok((version_parts, extension))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::split_version;
    use std::cmp::Ordering;
    use std::cmp::PartialOrd;

    #[test]
    fn package_ident_partial_eq() {
        let a = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        assert_eq!(a, b);
    }

    #[test]
    fn package_ident_partial_ord() {
        let a = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("1.0.1".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("Ordering should be greater"),
        }
    }

    #[test]
    fn package_ident_partial_ord_bad_name() {
        let a = PackageIdent::new("awesome".to_string(),
                                  "snoopy".to_string(),
                                  Some("1.0.1".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("awesome".to_string(),
                                  "banana".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        match a.partial_cmp(&b) {
            Some(_) => panic!("We tried to return an order"),
            None => assert!(true),
        }
    }

    #[test]
    fn package_ident_partial_ord_different_origin() {
        let a = PackageIdent::new("adam".to_string(),
                                  "blueberry".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("banana".to_string(),
                                  "blueberry".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Equal),
            None => panic!("We failed to return an order"),
        }
    }

    #[test]
    fn package_ident_partial_ord_release() {
        let a = PackageIdent::new("adam".to_string(),
                                  "brown".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131556".to_string()));
        let b = PackageIdent::new("boulder".to_string(),
                                  "brown".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("We failed to return an order"),
        }
    }

    #[test]
    fn split_version_returns_both_parts() {
        let svr = split_version("1.2.3-beta16");
        match svr {
            Ok((version_parts, Some(extension))) => {
                assert_eq!(vec!["1", "2", "3"], version_parts);
                assert_eq!("beta16", extension);
            }
            Ok((_, None)) => panic!("Has an extension"),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn version_sort_simple() {
        match version_sort("1.0.0", "2.0.0") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2.0.1", "2.0.0") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2.1.1", "2.1.1") {
            Ok(compare) => assert_eq!(compare, Ordering::Equal),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("20150521131347", "20150521131346") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn version_sort_complex() {
        match version_sort("1.0.0-alpha2", "1.0.0-alpha1") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("1.0.0-alpha1", "1.0.0-alpha2") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("1.0.0-beta1", "1.0.0-alpha1000") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2.1.1", "2.1.1-alpha2") {
            Ok(compare) => assert_eq!(compare, Ordering::Greater),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2.1.1-alpha2", "2.1.1") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn check_fully_qualified_package_id() {
        let partial = PackageIdent::new("acme", "rocket", None, None);
        let full = PackageIdent::new("acme", "rocket", Some("1.2.3"), Some("1234"));
        assert!(!partial.fully_qualified());
        assert!(full.fully_qualified());
    }
}
