use crate::{error::{Error,
                    Result},
            package::PackageTarget};
use regex::Regex;
use serde_derive::{Deserialize,
                   Serialize};
use std::{cmp::{Ordering,
                PartialOrd},
          convert::TryFrom,
          fmt,
          result,
          str::FromStr};

lazy_static::lazy_static! {
    static ref ORIGIN_NAME_RE: Regex =
        Regex::new(r"\A[a-z0-9][a-z0-9_-]*\z").expect("Unable to compile regex");
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct PackageIdent {
    pub origin:  String,
    pub name:    String,
    pub version: Option<String>,
    pub release: Option<String>,
}

pub trait Identifiable: fmt::Display {
    fn origin(&self) -> &str;
    fn name(&self) -> &str;
    fn version(&self) -> Option<&str>;
    fn release(&self) -> Option<&str>;

    fn fully_qualified(&self) -> bool { self.version().is_some() && self.release().is_some() }

    fn valid(&self) -> bool {
        let re = Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
        re.is_match(self.name())
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

impl PackageIdent {
    /// Creates a new package identifier
    pub fn new<T: Into<String>>(origin: T,
                                name: T,
                                version: Option<T>,
                                release: Option<T>)
                                -> Self {
        PackageIdent { origin:  origin.into(),
                       name:    name.into(),
                       version: version.map(Into::into),
                       release: release.map(Into::into), }
    }

    pub fn archive_name(&self) -> Result<String> {
        self.archive_name_impl(PackageTarget::active_target())
    }

    pub fn archive_name_with_target(&self, target: PackageTarget) -> Result<String> {
        self.archive_name_impl(target)
    }

    /// Produces an iterator over the ident's internal components viewed as [`&str`] slices.
    ///
    /// Note that no special interpretation should be taken from the component slices as their
    /// meaning is internal to this struct's implementation.
    ///
    /// [`&str`]: https://doc.rust-lang.org/std/primitive.str.html
    ///
    /// # Examples
    ///
    /// ```
    /// use habitat_core::package::PackageIdent;
    /// use std::str::FromStr;
    ///
    /// // All ident components are iterated through with a fully qualified ident
    /// let full_ident = PackageIdent::from_str("acme/myapp/1.2.3/20180710122645").unwrap();
    /// let mut it = full_ident.iter();
    ///
    /// assert_eq!(it.next(), Some("acme"));
    /// assert_eq!(it.next(), Some("myapp"));
    /// assert_eq!(it.next(), Some("1.2.3"));
    /// assert_eq!(it.next(), Some("20180710122645"));
    /// assert_eq!(it.next(), None);
    ///
    /// // Any optional ident components are short-circuited
    /// let fuzzy_ident = PackageIdent::from_str("acme/myapp").unwrap();
    /// let mut it = fuzzy_ident.iter();
    ///
    /// assert_eq!(it.next(), Some("acme"));
    /// assert_eq!(it.next(), Some("myapp"));
    /// assert_eq!(it.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_> {
        Iter { ident: self,
               pos:   0, }
    }

    /// Compare two `PackageIdent`s component by component:
    /// i.e. start with origin, then name, then version, then
    /// release. The first component to be not equal, then return
    /// the greater/lesser
    ///
    /// TODO: This should probably be the natural implementation of `Ord::cmp`.
    /// To be investigated why we have a different implementation
    pub fn by_parts_cmp(self: &PackageIdent, other: &PackageIdent) -> Ordering {
        if self.origin != other.origin {
            return self.origin.cmp(&other.origin);
        }

        if self.name != other.name {
            return self.name.cmp(&other.name);
        }
        match version_sort(self.version.as_ref().unwrap(),
                           other.version.as_ref().unwrap())
        {
            Ok(Ordering::Equal) => self.release.cmp(&other.release),
            Ok(ordering) => ordering,
            Err(_) => Ordering::Less,
        }
    }

    fn archive_name_impl(&self, target: PackageTarget) -> Result<String> {
        if self.fully_qualified() {
            Ok(format!("{}-{}-{}-{}-{}.hart",
                       self.origin,
                       self.name,
                       self.version.as_ref().unwrap(),
                       self.release.as_ref().unwrap(),
                       target))
        } else {
            Err(Error::FullyQualifiedPackageIdentRequired(self.to_string()))
        }
    }
}

impl Identifiable for PackageIdent {
    fn origin(&self) -> &str { &self.origin }

    fn name(&self) -> &str { &self.name }

    fn version(&self) -> Option<&str> { self.version.as_deref() }

    fn release(&self) -> Option<&str> { self.release.as_deref() }
}

// It does not make sense for `PackageIdent` to implement `Default`. This should be removed.
// See https://github.com/habitat-sh/habitat/issues/6829
impl Default for PackageIdent {
    fn default() -> PackageIdent { PackageIdent::new("", "", None, None) }
}

impl fmt::Display for PackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn as_ref(&self) -> &PackageIdent { self }
}

impl FromStr for PackageIdent {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let items: Vec<&str> = value.split('/').collect();
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
    /// * origin is ignored in the comparison - my redis and your redis compare the same.
    /// * If the names are not equal, they cannot be compared.
    /// * If the versions are greater/lesser, return that as the ordering.
    /// * If the versions are equal, return the greater/lesser for the release.
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
                           other.version.as_ref().unwrap())
        {
            ord @ Ok(Ordering::Greater) | ord @ Ok(Ordering::Less) => ord.ok(),
            Ok(Ordering::Equal) => Some(self.release.cmp(&other.release)),
            Err(_) => {
                // TODO: Can we do better than this? As long as we allow
                // non-numeric versions to co-exist with numeric ones, we
                // always have potential for incorrect ordering no matter
                // what we choose - eg, "master" vs. "0.x.x" (real examples)
                debug!("Comparing non-numeric versions: {} {}",
                       self.version.as_ref().unwrap(),
                       other.version.as_ref().unwrap());
                match self.version
                          .as_ref()
                          .unwrap()
                          .cmp(other.version.as_ref().unwrap())
                {
                    ord @ Ordering::Greater | ord @ Ordering::Less => Some(ord),
                    Ordering::Equal => Some(self.release.cmp(&other.release)),
                }
            }
        }
    }
}

impl Ord for PackageIdent {
    /// Packages can be compared according to the following:
    ///
    /// * origin is ignored in the comparison - my redis and your redis compare the same.
    /// * If the names are not equal, they cannot be compared.
    /// * If the versions are greater/lesser, return that as the ordering.
    /// * If the versions are equal, return the greater/lesser for the release.
    fn cmp(&self, other: &PackageIdent) -> Ordering {
        if self.name != other.name {
            return self.name.cmp(&other.name);
        }
        match version_sort(self.version.as_ref().unwrap(),
                           other.version.as_ref().unwrap())
        {
            Ok(Ordering::Equal) => self.release.cmp(&other.release),
            Ok(ordering) => ordering,
            Err(_) => Ordering::Less,
        }
    }
}

impl From<FullyQualifiedPackageIdent> for PackageIdent {
    fn from(full_ident: FullyQualifiedPackageIdent) -> Self { full_ident.0 }
}

/// Represents a fully-qualified Package Identifier, meaning that the normally optional version and
/// release package coordinates are guaranteed to be set. This fully-qualified-ness is checked on
/// construction and as the underlying representation is immutable, this state does not change.
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Hash)]
pub struct FullyQualifiedPackageIdent(PackageIdent);

impl FullyQualifiedPackageIdent {
    /// Creates a new fully qualified package identifier
    pub fn new<T: Into<String>>(origin: T, name: T, version: T, release: T) -> Self {
        let ident = PackageIdent { origin:  origin.into(),
                                   name:    name.into(),
                                   version: Some(version.into()),
                                   release: Some(release.into()), };
        FullyQualifiedPackageIdent(ident)
    }

    pub fn archive_name(&self) -> String {
        self.0
            .archive_name()
            .unwrap_or_else(|_| panic!("PackageIdent {} should be fully qualified", self.0))
    }

    pub fn version(&self) -> &str {
        Identifiable::version(self).unwrap_or_else(|| {
                                       panic!("PackageIdent {} should be fully qualified", self.0)
                                   })
    }

    pub fn release(&self) -> &str {
        Identifiable::release(self).unwrap_or_else(|| {
                                       panic!("PackageIdent {} should be fully qualified", self.0)
                                   })
    }
}

impl Identifiable for FullyQualifiedPackageIdent {
    fn origin(&self) -> &str { &self.0.origin }

    fn name(&self) -> &str { &self.0.name }

    fn version(&self) -> Option<&str> { self.0.version.as_deref() }

    fn release(&self) -> Option<&str> { self.0.release.as_deref() }
}

impl TryFrom<PackageIdent> for FullyQualifiedPackageIdent {
    type Error = Error;

    fn try_from(ident: PackageIdent) -> Result<Self> {
        if ident.fully_qualified() {
            Ok(FullyQualifiedPackageIdent(ident))
        } else {
            Err(Error::FullyQualifiedPackageIdentRequired(ident.to_string()))
        }
    }
}

impl<'a> TryFrom<&'a PackageIdent> for FullyQualifiedPackageIdent {
    type Error = Error;

    fn try_from(ident: &PackageIdent) -> Result<Self> {
        if ident.fully_qualified() {
            Ok(FullyQualifiedPackageIdent(ident.clone()))
        } else {
            Err(Error::FullyQualifiedPackageIdentRequired(ident.to_string()))
        }
    }
}

impl AsRef<PackageIdent> for FullyQualifiedPackageIdent {
    fn as_ref(&self) -> &PackageIdent { &self.0 }
}

impl fmt::Display for FullyQualifiedPackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

impl FromStr for FullyQualifiedPackageIdent {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        FullyQualifiedPackageIdent::try_from(value.parse::<PackageIdent>()?)
    }
}

/// An iterator over the [`&str`] slices of a [`PackageIdent`].
///
/// This `struct` is created by the [`iter`] method on [`PackageIdent`], see its documentation for
/// more.
///
/// [`&str`]: https://doc.rust-lang.org/std/primitive.str.html
/// [`iter`]: struct.PackageIdent.html#method.iter
/// [`PackageIdent`]: struct.PackageIdent.html
///
/// # Examples
///
/// ```
/// use habitat_core::package::PackageIdent;
/// use std::str::FromStr;
///
/// let target = PackageIdent::from_str("acme/myapp/1.2.3").unwrap();
///
/// for component in target.iter() {
///     println!("{}", component);
/// }
/// ```
pub struct Iter<'a> {
    // The ident to iterate over
    ident: &'a PackageIdent,
    // The position through the ident
    pos:   usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.pos += 1;
        match self.pos {
            1 => Some(self.ident.origin()),
            2 => Some(self.ident.name()),
            3 => self.ident.version(),
            4 => self.ident.release(),
            _ => None,
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
/// * Split the version numbers into an array of digits on any '.' characters. Digits are converted
///   into <u64>.
/// * Compare the version numbers by iterating over them. If 'a' is greater or lesser than 'b', we
///   return that as the result. If it is equal, we move to the next digit and repeat. If one of the
///   version numbers is exhausted before the other, it gains 0's for the missing slot.
/// * If the version numbers are equal, but either A or B has an extension (but not both) than the
///   version without the extension is greater. (1.0.0 is greater than 1.0.0-alpha6)
/// * If both have an extension, it is compared lexicographically, with the result as the final
///   ordering.
///
/// Returns a Error if we fail to match for any reason.
pub fn version_sort(a_version: &str, b_version: &str) -> Result<Ordering> {
    let (a_parts, a_extension) = split_version(a_version)?;
    let (b_parts, b_extension) = split_version(b_version)?;
    let mut a_iter = a_parts.iter();
    let mut b_iter = b_parts.iter();
    loop {
        let mut a_exhausted = false;
        let mut b_exhausted = false;
        let a_num = match a_iter.next() {
            Some(i) => i.parse::<u64>()?,
            None => {
                a_exhausted = true;
                0u64
            }
        };
        let b_num = match b_iter.next() {
            Some(i) => i.parse::<u64>()?,
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
        Ok(Ordering::Less)
    } else if a_extension.is_none() && b_extension.is_some() {
        Ok(Ordering::Greater)
    } else if a_extension.is_none() && b_extension.is_none() {
        Ok(Ordering::Equal)
    } else {
        let a = match a_extension {
            Some(a) => a,
            None => String::new(),
        };
        let b = match b_extension {
            Some(b) => b,
            None => String::new(),
        };
        Ok(a.cmp(&b))
    }
}

fn split_version(version: &str) -> Result<(Vec<&str>, Option<String>)> {
    let re = Regex::new(r"([\d\.]+)(.+)?")?;
    let caps = match re.captures(version) {
        Some(caps) => caps,
        None => return Err(Error::InvalidPackageIdent(version.to_string())),
    };
    let version_number = caps.get(1).unwrap();
    let extension = match caps.get(2) {
        Some(e) => {
            let mut estr: String = e.as_str().to_string();
            if estr.len() > 1 && estr.starts_with('-') {
                estr.remove(0);
            }
            Some(estr)
        }
        None => None,
    };
    let version_parts: Vec<&str> = version_number.as_str().split('.').collect();
    Ok((version_parts, extension))
}

/// Is the string a valid origin name?
pub fn is_valid_origin_name(origin: &str) -> bool {
    origin.chars().count() <= 255 && ORIGIN_NAME_RE.is_match(origin)
}

#[cfg(test)]
mod tests {
    use super::{split_version,
                *};
    use std::cmp::{Ordering,
                   PartialOrd};

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

        let a = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("1.0.0".to_string()),
                                  None);
        assert_ne!(a, b);
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
    fn package_ident_non_numeric_version_ord() {
        let a = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("random".to_string()),
                                  Some("20150521131556".to_string()));
        let b = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("random".to_string()),
                                  Some("20150521131555".to_string()));
        let c = PackageIdent::new("ty".to_string(),
                                  "tabor".to_string(),
                                  Some("undefined".to_string()),
                                  Some("20150521131555".to_string()));
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("Ordering should be greater"),
        }
        match b.partial_cmp(&c) {
            Some(ord) => assert_eq!(ord, Ordering::Less),
            None => panic!("Ordering should be less"),
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
        assert!(a.partial_cmp(&b).is_none(), "We tried to return an order");
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
        match version_sort("1.0", "2.0") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
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
        match version_sort("2.1.1.4", "2.1.1.5") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
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
        match version_sort("2.1.1-alpha2", "3.2.0-rc.0") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
        match version_sort("2016i", "2016j") {
            Ok(compare) => assert_eq!(compare, Ordering::Less),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn version_sort_error() {
        if let Ok(compare) = version_sort("1.0.0-alpha1", "undefined") {
            panic!("unexpected {:?}", compare);
        }
    }

    #[test]
    fn check_fully_qualified_package_id() {
        let partial = PackageIdent::new("acme", "rocket", None, None);
        let full = PackageIdent::new("acme", "rocket", Some("1.2.3"), Some("1234"));
        assert!(!partial.fully_qualified());
        assert!(full.fully_qualified());
    }

    #[test]
    fn check_valid_package_id() {
        let valid1 = PackageIdent::new("acme", "rocket", Some("1.2.3"), Some("1234"));
        let valid2 = PackageIdent::new("acme", "rocket-one", Some("1.2.3"), Some("1234"));
        let valid3 = PackageIdent::new("acme", "rocket_one", Some("1.2.3"), Some("1234"));
        let valid4 = PackageIdent::new("acme", "rocket_one", Some("foo-bar"), Some("1234"));
        let invalid1 = PackageIdent::new("acme", "rocket.one", Some("1.2.3"), Some("1234"));
        let invalid2 = PackageIdent::new("acme", "rocket%one", Some("1.2.3"), Some("1234"));

        assert!(valid1.valid());
        assert!(valid2.valid());
        assert!(valid3.valid());
        assert!(valid4.valid());
        assert!(!invalid1.valid());
        assert!(!invalid2.valid());
    }

    #[test]
    fn check_origin_name() {
        assert!(super::is_valid_origin_name("foo"));
        assert!(super::is_valid_origin_name("foo_bar"));
        assert!(super::is_valid_origin_name("foo-bar"));
        assert!(super::is_valid_origin_name("0xdeadbeef"));

        assert!(!super::is_valid_origin_name("Core"));
        assert!(!super::is_valid_origin_name(" foo"));
        assert!(!super::is_valid_origin_name("foo "));
        assert!(!super::is_valid_origin_name("!foo"));
        assert!(!super::is_valid_origin_name("foo bar"));
        assert!(!super::is_valid_origin_name("0xDEADBEEF"));
    }

    #[test]
    fn archive_name() {
        let ident = PackageIdent::from_str("tom-petty/the_last__dj/1.0.0/20180701125610").unwrap();
        let target = PackageTarget::active_target();

        assert_eq!(format!("{}-{}.hart",
                           "tom-petty-the_last__dj-1.0.0-20180701125610", target),
                   ident.archive_name().unwrap());
    }

    #[test]
    fn archive_name_with_fuzzy_ident() {
        let ident = PackageIdent::from_str("acme/not-enough").unwrap();

        match ident.archive_name() {
            Err(Error::FullyQualifiedPackageIdentRequired(i)) => {
                assert_eq!("acme/not-enough".to_string(), i)
            }
            Err(e) => panic!("Wrong expected error, found={:?}", e),
            Ok(s) => panic!("Should not have computed a result, returned={}", s),
        }
    }

    #[test]
    fn archive_name_with_target() {
        let ident = PackageIdent::from_str("tom-petty/the_last__dj/1.0.0/20180701125610").unwrap();
        let target = PackageTarget::from_str("x86_64-linux").unwrap();

        assert_eq!(String::from("tom-petty-the_last__dj-1.0.0-20180701125610-x86_64-linux.hart"),
                   ident.archive_name_with_target(target).unwrap());
    }

    #[test]
    fn archive_name_with_target_with_fuzzy_ident() {
        let ident = PackageIdent::from_str("acme/not-enough").unwrap();
        let target = PackageTarget::from_str("x86_64-linux").unwrap();

        match ident.archive_name_with_target(target) {
            Err(Error::FullyQualifiedPackageIdentRequired(i)) => {
                assert_eq!("acme/not-enough".to_string(), i)
            }
            Err(e) => panic!("Wrong expected error, found={:?}", e),
            Ok(s) => panic!("Should not have computed a result, returned={}", s),
        }
    }

    #[test]
    fn iter_with_fully_qualified() {
        let ident = PackageIdent::from_str("cypress-hill/rise-up/2.3.1/20180701141405").unwrap();
        let mut iter = ident.iter();

        assert_eq!(Some("cypress-hill"), iter.next());
        assert_eq!(Some("rise-up"), iter.next());
        assert_eq!(Some("2.3.1"), iter.next());
        assert_eq!(Some("20180701141405"), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_without_release() {
        let ident = PackageIdent::from_str("cypress-hill/rise-up/2.3.1").unwrap();
        let mut iter = ident.iter();

        assert_eq!(Some("cypress-hill"), iter.next());
        assert_eq!(Some("rise-up"), iter.next());
        assert_eq!(Some("2.3.1"), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_without_version() {
        let ident = PackageIdent::from_str("cypress-hill/rise-up").unwrap();
        let mut iter = ident.iter();

        assert_eq!(Some("cypress-hill"), iter.next());
        assert_eq!(Some("rise-up"), iter.next());
        assert_eq!(None, iter.next());
    }
}
