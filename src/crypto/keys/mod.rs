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

use std::collections::HashSet;
use std::fs;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;

use base64;
use regex::Regex;
use time;

use error::{Error, Result};
use util::perm;

use super::{PUBLIC_BOX_KEY_VERSION, PUBLIC_KEY_PERMISSIONS, PUBLIC_KEY_SUFFIX,
            PUBLIC_SIG_KEY_VERSION, SECRET_BOX_KEY_SUFFIX, SECRET_BOX_KEY_VERSION,
            SECRET_KEY_PERMISSIONS, SECRET_SIG_KEY_SUFFIX, SECRET_SIG_KEY_VERSION,
            SECRET_SYM_KEY_SUFFIX, SECRET_SYM_KEY_VERSION};

lazy_static! {
    static ref NAME_WITH_REV_RE: Regex = Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\z").unwrap();
    static ref KEYFILE_RE: Regex =
        Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\.(?P<suffix>[a-z]+(\.[a-z]+)?)\z").unwrap();
    static ref ORIGIN_NAME_RE: Regex = Regex::new(r"\A[a-z0-9][a-z0-9_-]*\z").unwrap();
}

pub mod box_key_pair;
pub mod sym_key;
pub mod sig_key_pair;

enum KeyType {
    Sig,
    Box,
    Sym,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PairType {
    Public,
    Secret,
}

impl fmt::Display for PairType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PairType::Public => write!(f, "public"),
            PairType::Secret => write!(f, "secret"),
        }
    }
}

impl FromStr for PairType {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value {
            "public" => Ok(PairType::Public),
            "secret" => Ok(PairType::Secret),
            _ => {
                return Err(Error::CryptoError(
                    format!("Invalid PairType conversion from {}", value),
                ))
            }
        }
    }
}

struct TmpKeyfile {
    pub path: PathBuf,
}

impl Drop for TmpKeyfile {
    fn drop(&mut self) {
        if self.path.is_file() {
            let _ = fs::remove_file(&self.path);
        }
    }
}

/// A pair of related keys (public and secret) which have a name and revision.
///
/// Depending on the type of keypair, the public key may be empty or not apply, or one or both of
/// the keys may not be present due to the loading context. For example, the act of verifying a
/// signed message or artifact only requires the public key to be present, whereas the act of
/// signing will require the secret key to be present.
#[derive(Clone)]
pub struct KeyPair<P, S> {
    /// The name of the key, ex: "habitat"
    pub name: String,
    /// The revision of the key, which is a timestamp, ex: "201604051449"
    pub rev: String,
    /// The public key component, if relevant
    pub public: Option<P>,
    /// The private key component, if relevant
    pub secret: Option<S>,
}

impl<P, S> KeyPair<P, S> {
    /// Creates a new `KeyPair`.
    pub fn new(name: String, rev: String, p: Option<P>, s: Option<S>) -> KeyPair<P, S> {
        KeyPair {
            name: name,
            rev: rev,
            public: p,
            secret: s,
        }
    }

    /// Returns a `String` containing the combination of the `name` and `rev` fields.
    pub fn name_with_rev(&self) -> String {
        format!("{}-{}", self.name, self.rev)
    }

    pub fn public(&self) -> Result<&P> {
        match self.public.as_ref() {
            Some(s) => Ok(s),
            None => {
                let msg = format!(
                    "Public key is required but not present for {}",
                    self.name_with_rev()
                );
                return Err(Error::CryptoError(msg));
            }
        }
    }

    pub fn secret(&self) -> Result<&S> {
        match self.secret.as_ref() {
            Some(s) => Ok(s),
            None => {
                let msg = format!(
                    "Secret key is required but not present for {}",
                    self.name_with_rev()
                );
                return Err(Error::CryptoError(msg));
            }
        }
    }
}

/// If a key "belongs" to a filename revision, then add the full stem of the
/// file (without path, without .suffix) to the set. This function doesn't
/// return an error on a "bad" file, the bad file key name just doesn't get
/// added to the set.
fn check_filename(
    keyname: &str,
    filename: String,
    candidates: &mut HashSet<String>,
    pair_type: Option<&PairType>,
) {
    let caps = match KEYFILE_RE.captures(&filename) {
        Some(c) => c,
        None => {
            debug!("check_filename: Cannot parse {}", &filename);
            return;
        }
    };
    let name = match caps.name("name") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse name from {}", &filename);
            return;
        }
    };

    let rev = match caps.name("rev") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse rev from {}", &filename);
            return;
        }
    };

    let suffix = match caps.name("suffix") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse suffix from {}", &filename);
            return;
        }
    };

    if suffix == PUBLIC_KEY_SUFFIX || suffix == SECRET_SIG_KEY_SUFFIX ||
        suffix == SECRET_BOX_KEY_SUFFIX || suffix == SECRET_SYM_KEY_SUFFIX
    {
        debug!("valid key suffix");
    } else {
        debug!("check_filename: Invalid key suffix from {}", &filename);
        return;
    };

    if name == keyname {
        let thiskey = format!("{}-{}", name, rev);

        let do_insert = match pair_type {
            Some(&PairType::Secret) => {
                if suffix == SECRET_SIG_KEY_SUFFIX || suffix == SECRET_BOX_KEY_SUFFIX ||
                    suffix == SECRET_SYM_KEY_SUFFIX
                {
                    true
                } else {
                    false
                }
            }
            Some(&PairType::Public) => {
                if suffix == PUBLIC_KEY_SUFFIX {
                    true
                } else {
                    false
                }
            }
            None => true,
        };

        if do_insert {
            candidates.insert(thiskey);
        }
    }
}

/// Take a key name (ex "habitat"), and find all revisions of that
/// keyname in the default_cache_key_path().
fn get_key_revisions<P>(
    keyname: &str,
    cache_key_path: P,
    pair_type: Option<&PairType>,
) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    // accumulator for files that match
    let mut candidates = HashSet::new();
    let paths = match fs::read_dir(cache_key_path.as_ref()) {
        Ok(p) => p,
        Err(e) => {
            return Err(Error::CryptoError(format!(
                "Error reading key directory {}: {}",
                cache_key_path.as_ref().display(),
                e
            )))
        }
    };
    for path in paths {
        let p = match path {
            Ok(ref p) => p,
            Err(e) => {
                debug!("Error reading path {}", e);
                return Err(Error::CryptoError(format!("Error reading key path {}", e)));
            }
        };

        match p.metadata() {
            Ok(md) => {
                if !md.is_file() {
                    continue;
                }
            }
            Err(e) => {
                debug!("Error checking file metadata {}", e);
                continue;
            }
        };
        let filename = match p.file_name().into_string() {
            Ok(f) => f,
            Err(e) => {
                // filename is still an OsString, so print it as debug output
                debug!("Invalid filename {:?}", e);
                return Err(Error::CryptoError(format!("Invalid filename in key path")));
            }
        };
        debug!("checking file: {}", &filename);
        check_filename(keyname, filename, &mut candidates, pair_type);
    }

    // traverse the candidates set and sort the entries
    let mut candidate_vec = Vec::new();
    for c in &candidates {
        candidate_vec.push(c.clone());
    }
    candidate_vec.sort();
    // newest key first
    candidate_vec.reverse();
    Ok(candidate_vec)
}

fn mk_key_filename<P, S1, S2>(path: P, keyname: S1, suffix: S2) -> PathBuf
where
    P: AsRef<Path>,
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    path.as_ref().join(format!(
        "{}.{}",
        keyname.as_ref(),
        suffix.as_ref()
    ))
}

/// generates a revision string in the form:
/// `{year}{month}{day}{hour24}{minute}{second}`
/// Timestamps are in UTC time.
fn mk_revision_string() -> Result<String> {
    let now = time::now_utc();
    // https://github.com/rust-lang-deprecated/time/blob/master/src/display.rs
    // http://man7.org/linux/man-pages/man3/strftime.3.html
    match now.strftime("%Y%m%d%H%M%S") {
        Ok(result) => Ok(result.to_string()),
        Err(_) => return Err(Error::CryptoError("Can't parse system time".to_string())),
    }
}

pub fn parse_name_with_rev<T>(name_with_rev: T) -> Result<(String, String)>
where
    T: AsRef<str>,
{
    let caps = match NAME_WITH_REV_RE.captures(name_with_rev.as_ref()) {
        Some(c) => c,
        None => {
            let msg = format!(
                "parse_name_with_rev:1 Cannot parse {}",
                name_with_rev.as_ref()
            );
            return Err(Error::CryptoError(msg));
        }
    };
    let name = match caps.name("name") {
        Some(r) => r.as_str().to_string(),
        None => {
            let msg = format!(
                "parse_name_with_rev:2 Cannot parse name from {}",
                name_with_rev.as_ref()
            );
            return Err(Error::CryptoError(msg));
        }
    };
    let rev = match caps.name("rev") {
        Some(r) => r.as_str().to_string(),
        None => {
            let msg = format!(
                "parse_name_with_rev:3 Cannot parse rev from {}",
                name_with_rev.as_ref()
            );
            return Err(Error::CryptoError(msg));
        }
    };
    Ok((name, rev))
}

/// Is the string a valid origin name?
pub fn is_valid_origin_name(name: &str) -> bool {
    name.chars().count() <= 255 && ORIGIN_NAME_RE.is_match(name)
}

fn read_key_bytes(keyfile: &Path) -> Result<Vec<u8>> {
    let mut f = try!(File::open(keyfile));
    let mut s = String::new();
    if try!(f.read_to_string(&mut s)) <= 0 {
        return Err(Error::CryptoError("Can't read key bytes".to_string()));
    }
    match s.lines().nth(3) {
        Some(encoded) => {
            let v = try!(base64::decode(encoded).map_err(|e| {
                Error::CryptoError(format!(
                    "Can't read raw key from {}: {}",
                    keyfile.display(),
                    e
                ))
            }));
            Ok(v)
        }
        None => {
            Err(Error::CryptoError(
                format!("Malformed key contents for: {}", keyfile.display()),
            ))
        }
    }
}

fn write_keypair_files(
    key_type: KeyType,
    keyname: &str,
    public_keyfile: Option<&Path>,
    public_content: Option<&[u8]>,
    secret_keyfile: Option<&Path>,
    secret_content: Option<&[u8]>,
) -> Result<()> {
    if let Some(public_keyfile) = public_keyfile {
        let public_version = match key_type {
            KeyType::Sig => PUBLIC_SIG_KEY_VERSION,
            KeyType::Box => PUBLIC_BOX_KEY_VERSION,
            KeyType::Sym => unreachable!("Sym keys do not have a public key"),
        };

        let public_content = match public_content {
            Some(c) => c,
            None => panic!("Invalid calling of this function"),
        };

        if let Some(pk_dir) = public_keyfile.parent() {
            try!(fs::create_dir_all(pk_dir));
        } else {
            return Err(Error::BadKeyPath(
                public_keyfile.to_string_lossy().into_owned(),
            ));
        }
        if public_keyfile.exists() {
            return Err(Error::CryptoError(format!(
                "Public keyfile or a directory already \
                                                   exists {}",
                public_keyfile.display()
            )));
        }
        let public_file = try!(File::create(public_keyfile));
        let mut public_writer = BufWriter::new(&public_file);
        try!(write!(public_writer, "{}\n{}\n\n", public_version, keyname));
        try!(public_writer.write_all(public_content));
        try!(perm::set_permissions(
            public_keyfile,
            PUBLIC_KEY_PERMISSIONS,
        ));
    }

    if let Some(secret_keyfile) = secret_keyfile {
        let secret_version = match key_type {
            KeyType::Sig => SECRET_SIG_KEY_VERSION,
            KeyType::Box => SECRET_BOX_KEY_VERSION,
            KeyType::Sym => SECRET_SYM_KEY_VERSION,
        };

        let secret_content = match secret_content {
            Some(c) => c,
            None => panic!("Invalid calling of this function"),
        };

        if let Some(sk_dir) = secret_keyfile.parent() {
            try!(fs::create_dir_all(sk_dir));
        } else {
            return Err(Error::BadKeyPath(
                secret_keyfile.to_string_lossy().into_owned(),
            ));
        }
        if secret_keyfile.exists() {
            return Err(Error::CryptoError(format!(
                "Secret keyfile or a directory already \
                                                   exists {}",
                secret_keyfile.display()
            )));
        }
        let secret_file = try!(File::create(secret_keyfile));
        let mut secret_writer = BufWriter::new(&secret_file);
        try!(write!(secret_writer, "{}\n{}\n\n", secret_version, keyname));
        try!(secret_writer.write_all(secret_content));
        try!(perm::set_permissions(
            secret_keyfile,
            SECRET_KEY_PERMISSIONS,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::fs::{self, File};
    use std::io::Write;
    use std::{thread, time};

    use hex::ToHex;
    use tempdir::TempDir;

    use super::box_key_pair::BoxKeyPair;
    use super::sig_key_pair::SigKeyPair;
    use super::sym_key::SymKey;
    use super::PairType;

    use super::TmpKeyfile;
    use super::super::test_support::*;

    static VALID_KEY: &'static str = "ring-key-valid-20160504220722.sym.key";
    static VALID_KEY_AS_HEX: &'static str = "\
        44215a3bce23e351a6af359d77131db17a46767de2b88cbb330df162b8cf2ec1";

    #[test]
    fn tmp_keyfile_delete_on_drop() {
        let cache = TempDir::new("key_cache").unwrap();
        let path = cache.path().join("mykey");

        {
            let tmp_keyfile = TmpKeyfile { path: path.clone() };
            File::create(&tmp_keyfile.path).unwrap();
            assert!(tmp_keyfile.path.is_file());
        }
        assert_eq!(path.is_file(), false);
    }

    #[test]
    fn tmp_keyfile_no_file_on_drop() {
        let cache = TempDir::new("key_cache").unwrap();
        let path = cache.path().join("mykey");

        {
            let tmp_keyfile = TmpKeyfile { path: path.clone() };
            assert_eq!(tmp_keyfile.path.is_file(), false);
        }
        assert_eq!(path.is_file(), false);
    }

    #[test]
    fn parse_name_with_rev() {
        let (name, rev) = super::parse_name_with_rev("an-origin-19690114010203").unwrap();
        assert_eq!(name, "an-origin");
        assert_eq!(rev, "19690114010203");

        let (name, rev) = super::parse_name_with_rev("user-19480531051223").unwrap();
        assert_eq!(name, "user");
        assert_eq!(rev, "19480531051223");

        let (name, rev) = super::parse_name_with_rev("tnt.default@acme-19480531051223").unwrap();
        assert_eq!(name, "tnt.default@acme");
        assert_eq!(rev, "19480531051223");

        let (name, rev) = super::parse_name_with_rev("--20160420042001").unwrap();
        assert_eq!(name, "-");
        assert_eq!(rev, "20160420042001");
    }

    #[test]
    fn read_key_bytes() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join(VALID_KEY);
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &keyfile).unwrap();
        println!("keyfile {:?}", keyfile);
        let result = super::read_key_bytes(keyfile.as_path()).unwrap();
        assert_eq!(result.as_slice().to_hex(), VALID_KEY_AS_HEX);
    }

    #[test]
    #[should_panic(expected = "Can\\'t read key bytes")]
    fn read_key_bytes_empty_file() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join("not-much-here");
        let _ = File::create(&keyfile).unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Malformed key contents for")]
    fn read_key_bytes_missing_newlines() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join("missing-newlines");
        let mut f = File::create(&keyfile).unwrap();
        f.write_all("SOMETHING\nELSE\n".as_bytes()).unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can\\'t read raw key from")]
    fn read_key_bytes_malformed_base64() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join("missing-newlines");
        let mut f = File::create(&keyfile).unwrap();
        f.write_all("header\nsomething\n\nI am not base64 content".as_bytes())
            .unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

    #[test]
    fn get_key_revisions_can_return_everything() {
        let cache = TempDir::new("key_cache").unwrap();
        let _ = SigKeyPair::generate_pair_for_origin("foo", cache.path());
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(time::Duration::from_millis(1000));
        let _ = SigKeyPair::generate_pair_for_origin("foo", cache.path());
        let revs = super::get_key_revisions("foo", cache.path(), None).unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_key_revisions_can_return_secret_keys() {
        let cache = TempDir::new("key_cache").unwrap();
        let _ = SigKeyPair::generate_pair_for_origin("foo", cache.path());
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(time::Duration::from_millis(1000));
        let _ = SigKeyPair::generate_pair_for_origin("foo", cache.path());
        let revs = super::get_key_revisions("foo", cache.path(), Some(&PairType::Secret)).unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_key_revisions_can_return_public_keys() {
        let cache = TempDir::new("key_cache").unwrap();
        let _ = SigKeyPair::generate_pair_for_origin("foo", cache.path());
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(time::Duration::from_millis(1000));
        let _ = SigKeyPair::generate_pair_for_origin("foo", cache.path());
        let revs = super::get_key_revisions("foo", cache.path(), Some(&PairType::Public)).unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_user_key_revisions() {
        let cache = TempDir::new("key_cache").unwrap();
        for _ in 0..3 {
            wait_until_ok(|| {
                BoxKeyPair::generate_pair_for_user("wecoyote", cache.path())
            });
        }
        let _ = BoxKeyPair::generate_pair_for_user("wecoyote-foo", cache.path()).unwrap();

        // we shouldn't see wecoyote-foo as a 4th revision
        let revisions = super::get_key_revisions("wecoyote", cache.path(), None).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("wecoyote-foo", cache.path(), None).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_service_key_revisions() {
        let cache = TempDir::new("key_cache").unwrap();

        for _ in 0..3 {
            wait_until_ok(|| {
                BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            });
        }

        let _ = BoxKeyPair::generate_pair_for_service("acyou", "tnt.default", cache.path())
            .unwrap();

        let revisions = super::get_key_revisions("tnt.default@acme", cache.path(), None).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("tnt.default@acyou", cache.path(), None).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_ring_key_revisions() {
        let cache = TempDir::new("key_cache").unwrap();

        for _ in 0..3 {
            wait_until_ok(|| SymKey::generate_pair_for_ring("acme", cache.path()));
        }

        let _ = SymKey::generate_pair_for_ring("acme-you", cache.path()).unwrap();

        let revisions = super::get_key_revisions("acme", cache.path(), None).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("acme-you", cache.path(), None).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_origin_key_revisions() {
        let cache = TempDir::new("key_cache").unwrap();

        for _ in 0..3 {
            wait_until_ok(|| {
                SigKeyPair::generate_pair_for_origin("mutants", cache.path())
            });
        }

        let _ = SigKeyPair::generate_pair_for_origin("mutants-x", cache.path()).unwrap();

        let revisions = super::get_key_revisions("mutants", cache.path(), None).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("mutants-x", cache.path(), None).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn check_filename_for_secret_keys() {
        // only look for secret keys
        let mut candidates = HashSet::new();
        super::check_filename(
            "wecoyote",
            "wecoyote-20160519203610.pub".to_string(),
            &mut candidates,
            Some(&PairType::Secret),
        );
        super::check_filename(
            "wecoyote",
            "wecoyote-foo-20160519203610.pub".to_string(),
            &mut candidates,
            Some(&PairType::Secret),
        );
        super::check_filename(
            "wecoyote",
            "wecoyote-20160519203610.sig.key".to_string(),
            &mut candidates,
            Some(&PairType::Secret),
        );
        assert_eq!(1, candidates.len());
    }

    #[test]
    fn check_filename_for_public_keys() {
        // only look for public keys
        let mut candidates = HashSet::new();
        super::check_filename(
            "wecoyote",
            "wecoyote-20160519203610.pub".to_string(),
            &mut candidates,
            Some(&PairType::Public),
        );
        super::check_filename(
            "wecoyote",
            "wecoyote-20160519203611.pub".to_string(),
            &mut candidates,
            Some(&PairType::Public),
        );
        super::check_filename(
            "wecoyote",
            "wecoyote-20160519203610.sig.key".to_string(),
            &mut candidates,
            Some(&PairType::Public),
        );
        assert_eq!(2, candidates.len());
    }

    #[test]
    fn check_filename_key_without_dash() {
        // look for a keyname that doesn't include a dash
        let mut candidates = HashSet::new();
        super::check_filename(
            "wecoyote",
            "wecoyote-20160519203610.pub".to_string(),
            &mut candidates,
            None,
        );
        super::check_filename(
            "wecoyote",
            "wecoyote-foo-20160519203610.pub".to_string(),
            &mut candidates,
            None,
        );
        super::check_filename(
            "wecoyote",
            "wecoyote-20160519203610.box.key".to_string(),
            &mut candidates,
            None,
        );
        super::check_filename(
            "wecoyote",
            "wecoyote-foo-20160519203610.box.key".to_string(),
            &mut candidates,
            None,
        );
        assert_eq!(1, candidates.len());
    }


    #[test]
    fn check_filename_key_with_dash() {
        // look for a keyname that includes a dash
        let mut candidates = HashSet::new();
        super::check_filename(
            "wecoyote-foo",
            "wecoyote-20160519203610.pub".to_string(),
            &mut candidates,
            None,
        );
        super::check_filename(
            "wecoyote-foo",
            "wecoyote-foo-20160519203610.pub".to_string(),
            &mut candidates,
            None,
        );
        super::check_filename(
            "wecoyote-foo",
            "wecoyote-20160519203610.box.key".to_string(),
            &mut candidates,
            None,
        );
        super::check_filename(
            "wecoyote-foo",
            "wecoyote-foo-20160519203610.box.key".to_string(),
            &mut candidates,
            None,
        );
        assert_eq!(1, candidates.len());
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
        assert!(!super::is_valid_origin_name("foo!"));
        assert!(!super::is_valid_origin_name("foo bar"));
        assert!(!super::is_valid_origin_name("0xDEADBEEF"));
    }
}
