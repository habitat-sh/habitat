// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::result;
use std::str::{self, FromStr};

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use libarchive::writer;
use libarchive::reader::{self, Reader};
use libarchive::archive::{Entry, ReadFilter, ReadFormat};
use regex::Regex;

use error::{Error, Result};
use gpg;

lazy_static! {
    static ref METAFILE_REGXS: HashMap<MetaFile, Regex> = {
        let mut map = HashMap::new();
        map.insert(MetaFile::CFlags, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::CFlags)).unwrap());
        map.insert(MetaFile::Config, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Config)).unwrap());
        map.insert(MetaFile::Deps, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Deps)).unwrap());
        map.insert(MetaFile::TDeps, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::TDeps)).unwrap());
        map.insert(MetaFile::Exposes, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Exposes)).unwrap());
        map.insert(MetaFile::Ident, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Ident)).unwrap());
        map.insert(MetaFile::LdRunPath, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::LdRunPath)).unwrap());
        map.insert(MetaFile::LdFlags, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::LdFlags)).unwrap());
        map.insert(MetaFile::Manifest, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Manifest)).unwrap());
        map.insert(MetaFile::Path, Regex::new(&format!(r"^opt/bldr/pkgs/([^/]+)/([^/]+)/([^/]+)/([^/]+)/{}$", MetaFile::Path)).unwrap());
        map
    };
}

#[derive(RustcEncodable, RustcDecodable, Eq, PartialEq, Debug, Clone)]
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
            Some(format!("{}-{}-{}-{}.bldr",
                         self.origin,
                         self.name,
                         self.version.as_ref().unwrap(),
                         self.release.as_ref().unwrap()))
        } else {
            None
        }
    }

    pub fn fully_qualified(&self) -> bool {
        self.version.is_some() && self.release.is_some()
    }

    pub fn satisfies<T: AsRef<Self>>(&self, ident: T) -> bool {
        let other = ident.as_ref();
        if self.origin != other.origin || self.name != other.name {
            return false;
        }
        if self.version.is_some() {
            if other.version.is_none() {
                return true;
            }
            if *self.version.as_ref().unwrap() != *other.version.as_ref().unwrap() {
                return false;
            }
        }
        if self.release.is_some() {
            if other.release.is_none() {
                return true;
            }
            if *self.release.as_ref().unwrap() != *other.release.as_ref().unwrap() {
                return false;
            }
        }
        true
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
        let ord = match version_sort(self.version.as_ref().unwrap(),
                                     other.version.as_ref().unwrap()) {
            Ok(ord) => ord,
            Err(e) => {
                error!("This was a very bad version number: {:?}", e);
                return None;
            }
        };
        match ord {
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Equal => {
                return Some(self.release.cmp(&other.release));
            }
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum MetaFile {
    CFlags,
    Config,
    Deps,
    TDeps,
    Exposes,
    Ident,
    LdRunPath,
    LdFlags,
    Manifest,
    Path,
}

impl fmt::Display for MetaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match *self {
            MetaFile::CFlags => "CFLAGS",
            MetaFile::Config => "default.toml",
            MetaFile::Deps => "DEPS",
            MetaFile::TDeps => "TDEPS",
            MetaFile::Exposes => "EXPOSES",
            MetaFile::Ident => "IDENT",
            MetaFile::LdRunPath => "LD_RUN_PATH",
            MetaFile::LdFlags => "LDFLAGS",
            MetaFile::Manifest => "MANIFEST",
            MetaFile::Path => "PATH",
        };
        write!(f, "{}", id)
    }
}

type Metadata = HashMap<MetaFile, String>;

#[derive(Debug)]
pub struct PackageArchive {
    pub path: PathBuf,
    metadata: Option<Metadata>,
}

impl PackageArchive {
    pub fn new(path: PathBuf) -> Self {
        PackageArchive {
            path: path,
            metadata: None,
        }
    }

    /// Calculate and return the checksum of the package archive in hexadecimal format.
    ///
    /// # Failures
    ///
    /// * If the archive cannot be read
    pub fn checksum(&self) -> Result<String> {
        let mut digest = Sha256::new();
        let mut file = try!(File::open(&self.path));
        let mut buffer = Vec::new();
        try!(file.read_to_end(&mut buffer));
        digest.input(&buffer);
        let hash = digest.result_str();
        Ok(hash)
    }

    pub fn cflags(&mut self) -> Result<Option<String>> {
        match self.read_metadata(MetaFile::CFlags) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn config(&mut self) -> Result<Option<String>> {
        match self.read_metadata(MetaFile::Config) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    /// Returns a list of package identifiers representing the runtime package dependencies for
    /// this archive.
    ///
    /// # Failures
    ///
    /// * If a `DEPS` metafile is not found in the archive
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn deps(&mut self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::Deps)
    }

    /// Returns a list of package identifiers representing the transitive runtime package
    /// dependencies for this archive.
    ///
    /// # Failures
    ///
    /// * If a `TDEPS` metafile is not found in the archive
    /// * If the archive cannot be read
    /// * If the archive cannot be verified
    pub fn tdeps(&mut self) -> Result<Vec<PackageIdent>> {
        self.read_deps(MetaFile::TDeps)
    }

    pub fn exposes(&mut self) -> Result<Vec<u16>> {
        match self.read_metadata(MetaFile::Exposes) {
            Ok(Some(data)) => {
                let ports: Vec<u16> = data.split(" ")
                                          .filter_map(|port| port.parse::<u16>().ok())
                                          .collect();
                Ok(ports)
            }
            Ok(None) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    pub fn ident(&mut self) -> Result<PackageIdent> {
        match self.read_metadata(MetaFile::Ident) {
            Ok(None) => Err(Error::MetaFileMalformed(MetaFile::Ident)),
            Ok(Some(data)) => PackageIdent::from_str(&data),
            Err(e) => Err(e),
        }
    }

    pub fn ld_run_path(&mut self) -> Result<Option<String>> {
        match self.read_metadata(MetaFile::LdRunPath) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn ldflags(&mut self) -> Result<Option<String>> {
        match self.read_metadata(MetaFile::LdFlags) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    pub fn manifest(&mut self) -> Result<String> {
        match self.read_metadata(MetaFile::Manifest) {
            Ok(None) => Err(Error::MetaFileMalformed(MetaFile::Manifest)),
            Ok(Some(data)) => Ok(data.clone()),
            Err(e) => Err(e),
        }
    }

    pub fn path(&mut self) -> Result<Option<String>> {
        match self.read_metadata(MetaFile::Path) {
            Ok(data) => Ok(data.cloned()),
            Err(e) => Err(e),
        }
    }

    /// A plain string representation of the archive's file name.
    pub fn file_name(&self) -> String {
        self.path.file_name().unwrap().to_string_lossy().into_owned()
    }

    /// Given a package name and a path to a file as an `&str`, verify
    /// the files gpg signature.
    ///
    /// # Failures
    ///
    /// * Fails if it cannot verify the GPG signature for any reason
    pub fn verify(&self) -> Result<()> {
        match gpg::verify(self.path.to_str().unwrap()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Given a package name and a path to a file as an `&str`, unpack
    /// the package.
    ///
    /// # Failures
    ///
    /// * If the package cannot be unpacked via gpg
    pub fn unpack(&self) -> Result<()> {
        let file = self.path.to_str().unwrap().to_string();
        let mut out = try!(gpg::verify(&file));
        try!(out.seek(SeekFrom::Start(0)));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::All));
        try!(builder.support_filter(ReadFilter::All));
        let mut reader = try!(builder.open_stream(out));
        let writer = writer::Disk::new();
        try!(writer.set_standard_lookup());
        try!(writer.write(&mut reader, Some("/")));
        try!(writer.close());
        Ok(())
    }

    fn read_deps(&mut self, file: MetaFile) -> Result<Vec<PackageIdent>> {
        let mut deps: Vec<PackageIdent> = vec![];
        match self.read_metadata(file) {
            Ok(Some(body)) => {
                let ids: Vec<String> = body.split("\n").map(|d| d.to_string()).collect();
                for id in &ids {
                    let package = try!(PackageIdent::from_str(id));
                    if !package.fully_qualified() {
                        // JW TODO: use a more appropriate erorr to describe the invalid
                        // user input here. (user because a package was generated by a user
                        // and read into program)
                        return Err(Error::InvalidPackageIdent(package.to_string()));
                    }
                    deps.push(package);
                }
                Ok(deps)
            }
            Ok(None) => Ok(vec![]),
            Err(Error::MetaFileNotFound(_)) => Ok(deps),
            Err(e) => Err(e),
        }
    }

    fn read_metadata(&mut self, file: MetaFile) -> Result<Option<&String>> {
        if let Some(ref files) = self.metadata {
            return Ok(files.get(&file));
        }

        let mut metadata = Metadata::new();
        let mut matched_count = 0u8;
        let f = self.path.to_str().unwrap().to_string();
        let mut out = try!(gpg::verify(&f));
        try!(out.seek(SeekFrom::Start(0)));
        let mut builder = reader::Builder::new();
        try!(builder.support_format(ReadFormat::All));
        try!(builder.support_filter(ReadFilter::All));
        let mut reader = try!(builder.open_stream(out));
        loop {
            let mut matched_type: Option<MetaFile> = None;
            if let Some(entry) = reader.next_header() {
                for (matched, regx) in METAFILE_REGXS.iter() {
                    if regx.is_match(entry.pathname()) {
                        matched_type = Some((*matched).clone());
                        matched_count += 1;
                        break;
                    }
                }
            } else {
                break;
            }

            if matched_type.is_none() {
                continue;
            }

            match reader.read_block() {
                Ok(Some(bytes)) => {
                    match str::from_utf8(bytes) {
                        Ok(content) => {
                            metadata.insert(matched_type.unwrap(), content.trim().to_string());
                        }
                        Err(_) => return Err(Error::MetaFileMalformed(matched_type.unwrap())),
                    }
                }
                Ok(None) => (),
                Err(_) => return Err(Error::MetaFileMalformed(matched_type.unwrap())),
            }

            if matched_count == METAFILE_REGXS.len() as u8 {
                break;
            }
        }
        self.metadata = Some(metadata);
        Ok(self.metadata.as_ref().unwrap().get(&file))
    }
}

/// Sorts two packages according to their version.
///
/// We are a bit more strict than your average package management solution on versioning.
/// What we support is the "some number of digits or dots" (the version number),
/// followed by an optional "-" and any alphanumeric string (the extension). When determining sort order, we:
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
        let a = PackageIdent::new("bldr".to_string(),
                                  "bldr".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("bldr".to_string(),
                                  "bldr".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        assert_eq!(a, b);
    }

    #[test]
    fn package_ident_partial_ord() {
        let a = PackageIdent::new("bldr".to_string(),
                                  "bldr".to_string(),
                                  Some("1.0.1".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("bldr".to_string(),
                                  "bldr".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        match a.partial_cmp(&b) {
            Some(ord) => assert_eq!(ord, Ordering::Greater),
            None => panic!("Ordering should be greater"),
        }
    }

    #[test]
    fn package_ident_partial_ord_bad_name() {
        let a = PackageIdent::new("bldr".to_string(),
                                  "snoopy".to_string(),
                                  Some("1.0.1".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("bldr".to_string(),
                                  "bldr".to_string(),
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
                                  "bldr".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131555".to_string()));
        let b = PackageIdent::new("bldr".to_string(),
                                  "bldr".to_string(),
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
                                  "bldr".to_string(),
                                  Some("1.0.0".to_string()),
                                  Some("20150521131556".to_string()));
        let b = PackageIdent::new("bldr".to_string(),
                                  "bldr".to_string(),
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
        let partial = PackageIdent::new("chef", "libarchive", None, None);
        let full = PackageIdent::new("chef", "libarchive", Some("1.2.3"), Some("1234"));
        assert!(!partial.fully_qualified());
        assert!(full.fully_qualified());
    }
}
