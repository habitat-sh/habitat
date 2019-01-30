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
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;

use base64;
use regex::Regex;
use time;

use crate::error::{Error, Result};

use super::{
    PUBLIC_BOX_KEY_VERSION, PUBLIC_KEY_SUFFIX, PUBLIC_SIG_KEY_VERSION, SECRET_BOX_KEY_SUFFIX,
    SECRET_BOX_KEY_VERSION, SECRET_SIG_KEY_SUFFIX, SECRET_SIG_KEY_VERSION, SECRET_SYM_KEY_SUFFIX,
    SECRET_SYM_KEY_VERSION,
};

lazy_static::lazy_static! {
    static ref NAME_WITH_REV_RE: Regex = Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\z").unwrap();
    static ref KEYFILE_RE: Regex =
        Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\.(?P<suffix>[a-z]+(\.[a-z]+)?)\z").unwrap();
}

pub mod box_key_pair;
pub mod sig_key_pair;
pub mod sym_key;

enum KeyType {
    Sig,
    Box,
    Sym,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KeyType::Box => write!(f, "box"),
            KeyType::Sig => write!(f, "sig"),
            KeyType::Sym => write!(f, "sym"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum PairType {
    Public,
    Secret,
}

impl fmt::Display for PairType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                return Err(Error::CryptoError(format!(
                    "Invalid PairType conversion from {}",
                    value
                )));
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

    if suffix == PUBLIC_KEY_SUFFIX
        || suffix == SECRET_SIG_KEY_SUFFIX
        || suffix == SECRET_BOX_KEY_SUFFIX
        || suffix == SECRET_SYM_KEY_SUFFIX
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
                suffix == SECRET_SIG_KEY_SUFFIX
                    || suffix == SECRET_BOX_KEY_SUFFIX
                    || suffix == SECRET_SYM_KEY_SUFFIX
            }
            Some(&PairType::Public) => suffix == PUBLIC_KEY_SUFFIX,
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
    key_type: &KeyType,
) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    // accumulator for files that match
    let mut candidates = HashSet::new();
    let dir_entries = match fs::read_dir(cache_key_path.as_ref()) {
        Ok(dir_entries) => dir_entries,
        Err(e) => {
            return Err(Error::CryptoError(format!(
                "Error reading key directory {}: {}",
                cache_key_path.as_ref().display(),
                e
            )));
        }
    };
    for result in dir_entries {
        let dir_entry = match result {
            Ok(ref dir_entry) => dir_entry,
            Err(e) => {
                debug!("Error reading path {}", e);
                return Err(Error::CryptoError(format!("Error reading key path {}", e)));
            }
        };

        // NB: this metadata() call traverses symlinks, which is
        // exactly what we want.
        match dir_entry.path().metadata() {
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

        let file = File::open(dir_entry.path())?;
        let mut reader = BufReader::new(file);
        let mut buf = String::new();

        if let Err(e) = reader.read_line(&mut buf) {
            debug!("Invalid content: {}", e);
            continue;
        }

        if !buf.starts_with(&key_type.to_string().to_uppercase()) {
            debug!(
                "Invalid key content in {:?} for type {}",
                dir_entry, &key_type
            );
            continue;
        }

        let filename = match dir_entry.file_name().into_string() {
            Ok(f) => f,
            Err(e) => {
                // filename is still an OsString, so print it as debug output
                debug!("Invalid filename {:?}", e);
                return Err(Error::CryptoError(
                    "Invalid filename in key path".to_string(),
                ));
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
    path.as_ref()
        .join(format!("{}.{}", keyname.as_ref(), suffix.as_ref()))
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

/// Parses a string slice of a public or secret signature key.
///
/// The return valid is a tuple consisting of:
///   `(PairType, name_with_rev::String, key_body::String)`
///
/// # Examples
///
/// With a public key:
///
/// ```
/// extern crate habitat_core;
///
/// use habitat_core::crypto::keys::parse_key_str;
/// use habitat_core::crypto::keys::PairType;
///
/// fn main() {
///     let content = "SIG-PUB-1
/// unicorn-20160517220007
///
/// J+FGYVKgragA+dzQHCGORd2oLwCc2EvAnT9roz9BJh0=";
///     let (pair_type, name_with_rev, key_body) = parse_key_str(content).unwrap();
///     assert_eq!(pair_type, PairType::Public);
///     assert_eq!(name_with_rev, "unicorn-20160517220007");
///     assert_eq!(key_body, "J+FGYVKgragA+dzQHCGORd2oLwCc2EvAnT9roz9BJh0=");
/// }
/// ```
///
/// With a secret key:
///
/// ```
/// extern crate habitat_core;
///
/// use habitat_core::crypto::keys::parse_key_str;
/// use habitat_core::crypto::keys::PairType;
///
/// fn main() {
///     let content = "SIG-SEC-1
/// unicorn-20160517220007
///
/// jjQaaphB5+CHw7QzDWqMMuwhWmrrHH+SzQAgRrHfQ8sn4UZhUqCtqAD53NAcIY5F3agvAJzYS8CdP2ujP0EmHQ==";
///
///     let (pair_type, name_with_rev, key_body) = parse_key_str(content).unwrap();
/// }
/// ```
///
/// # Errors
///
/// * If there is a key version mismatch
/// * If the key version is missing
/// * If the key name with revision is missing
/// * If the key value (the Bas64 payload) is missing
pub fn parse_key_str(content: &str) -> Result<(PairType, String, String)> {
    let mut lines = content.lines();
    let pair_type = match lines.next() {
        Some(val) => match val {
            PUBLIC_SIG_KEY_VERSION | PUBLIC_BOX_KEY_VERSION => PairType::Public,
            SECRET_SIG_KEY_VERSION | SECRET_BOX_KEY_VERSION | SECRET_SYM_KEY_VERSION => {
                PairType::Secret
            }
            _ => {
                return Err(Error::CryptoError(format!(
                    "Unsupported key version: {}",
                    val
                )));
            }
        },
        None => {
            let msg = format!("write_key_from_str:1 Malformed key string:\n({})", content);
            return Err(Error::CryptoError(msg));
        }
    };
    let name_with_rev = match lines.next() {
        Some(val) => val,
        None => {
            let msg = format!("write_key_from_str:2 Malformed key string:\n({})", content);
            return Err(Error::CryptoError(msg));
        }
    };
    match lines.nth(1) {
        Some(val) => {
            base64::decode(val.trim()).map_err(|_| {
                Error::CryptoError(format!(
                    "write_key_from_str:3 Malformed key \
                     string:\n({})",
                    content
                ))
            })?;
            Ok((pair_type, name_with_rev.to_string(), val.trim().to_string()))
        }
        None => {
            let msg = format!("write_key_from_str:3 Malformed key string:\n({})", content);
            Err(Error::CryptoError(msg))
        }
    }
}

fn read_key_bytes(keyfile: &Path) -> Result<Vec<u8>> {
    let mut f = File::open(keyfile)?;
    let mut s = String::new();
    if f.read_to_string(&mut s)? == 0 {
        return Err(Error::CryptoError("Can't read key bytes".to_string()));
    }
    read_key_bytes_from_str(&s)
}

fn read_key_bytes_from_str(key: &str) -> Result<Vec<u8>> {
    match key.lines().nth(3) {
        Some(encoded) => {
            let v = base64::decode(encoded)
                .map_err(|e| Error::CryptoError(format!("Can't read raw key {}", e)))?;
            Ok(v)
        }
        None => Err(Error::CryptoError("Malformed key contents".to_string())),
    }
}

fn write_keypair_files(
    public_keyfile: Option<&Path>,
    public_content: Option<String>,
    secret_keyfile: Option<&Path>,
    secret_content: Option<String>,
) -> Result<()> {
    if let Some(public_keyfile) = public_keyfile {
        let public_content = match public_content {
            Some(c) => c,
            None => panic!("Invalid calling of this function"),
        };

        if let Some(pk_dir) = public_keyfile.parent() {
            fs::create_dir_all(pk_dir)?;
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
        let public_file = File::create(public_keyfile)?;
        let mut public_writer = BufWriter::new(&public_file);
        public_writer.write_all(public_content.as_bytes())?;
        set_permissions(public_keyfile)?;
    }

    if let Some(secret_keyfile) = secret_keyfile {
        let secret_content = match secret_content {
            Some(c) => c,
            None => panic!("Invalid calling of this function"),
        };

        if let Some(sk_dir) = secret_keyfile.parent() {
            fs::create_dir_all(sk_dir)?;
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
        let secret_file = File::create(secret_keyfile)?;
        let mut secret_writer = BufWriter::new(&secret_file);
        secret_writer.write_all(secret_content.as_bytes())?;
        set_permissions(secret_keyfile)?;
    }
    Ok(())
}

#[cfg(not(windows))]
fn set_permissions<T: AsRef<Path>>(path: T) -> Result<()> {
    use crate::util::posix_perm;

    use super::KEY_PERMISSIONS;

    posix_perm::set_permissions(path.as_ref(), KEY_PERMISSIONS)
}

#[cfg(windows)]
fn set_permissions<T: AsRef<Path>>(path: T) -> Result<()> {
    use crate::util::win_perm;

    win_perm::harden_path(path.as_ref())
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use std::{thread, time};

    use hex;
    use tempfile::Builder;

    use super::box_key_pair::BoxKeyPair;
    use super::sig_key_pair::SigKeyPair;
    use super::sym_key::SymKey;
    use super::KeyType;
    use super::PairType;

    use super::super::test_support::*;
    use super::TmpKeyfile;

    static VALID_KEY: &'static str = "ring-key-valid-20160504220722.sym.key";
    static VALID_KEY_AS_HEX: &'static str =
        "\
         44215a3bce23e351a6af359d77131db17a46767de2b88cbb330df162b8cf2ec1";

    #[test]
    fn tmp_keyfile_delete_on_drop() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
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
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
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
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let keyfile = cache.path().join(VALID_KEY);
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &keyfile).unwrap();
        println!("keyfile {:?}", keyfile);
        let result = super::read_key_bytes(keyfile.as_path()).unwrap();
        assert_eq!(hex::encode(result.as_slice()), VALID_KEY_AS_HEX);
    }

    #[test]
    #[should_panic(expected = "Can\\'t read key bytes")]
    fn read_key_bytes_empty_file() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let keyfile = cache.path().join("not-much-here");
        let _ = File::create(&keyfile).unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Malformed key contents")]
    fn read_key_bytes_missing_newlines() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let keyfile = cache.path().join("missing-newlines");
        let mut f = File::create(&keyfile).unwrap();
        f.write_all(b"SOMETHING\nELSE\n").unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can\\'t read raw key")]
    fn read_key_bytes_malformed_base64() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let keyfile = cache.path().join("missing-newlines");
        let mut f = File::create(&keyfile).unwrap();
        f.write_all(b"header\nsomething\n\nI am not base64 content")
            .unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

    #[test]
    fn get_key_revisions_can_return_everything() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("foo")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(time::Duration::from_millis(1000));
        SigKeyPair::generate_pair_for_origin("foo")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();
        let revs = super::get_key_revisions("foo", cache.path(), None, &KeyType::Sig).unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_key_revisions_can_only_return_keys_of_specified_type() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("foo")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();
        let revs = super::get_key_revisions("foo", cache.path(), None, &KeyType::Sig).unwrap();
        assert_eq!(1, revs.len());
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(time::Duration::from_millis(1000));
        let pair = BoxKeyPair::generate_pair_for_user("foo-user");
        pair.unwrap().to_pair_files(cache.path()).unwrap();
        let revs = super::get_key_revisions("foo-user", cache.path(), None, &KeyType::Sig).unwrap();
        assert_eq!(0, revs.len());
    }

    #[test]
    fn get_key_revisions_can_return_secret_keys() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("foo")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(time::Duration::from_millis(1000));
        SigKeyPair::generate_pair_for_origin("foo")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();
        let revs =
            super::get_key_revisions("foo", cache.path(), Some(&PairType::Secret), &KeyType::Sig)
                .unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_key_revisions_can_return_public_keys() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("foo")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(time::Duration::from_millis(1000));
        SigKeyPair::generate_pair_for_origin("foo")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();
        let revs =
            super::get_key_revisions("foo", cache.path(), Some(&PairType::Public), &KeyType::Sig)
                .unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_user_key_revisions() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        for _ in 0..3 {
            wait_until_ok(|| {
                let pair = BoxKeyPair::generate_pair_for_user("wecoyote")?;
                pair.to_pair_files(cache.path())?;
                Ok(())
            });
        }
        BoxKeyPair::generate_pair_for_user("wecoyote-foo")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();

        // we shouldn't see wecoyote-foo as a 4th revision
        let revisions =
            super::get_key_revisions("wecoyote", cache.path(), None, &KeyType::Box).unwrap();
        assert_eq!(3, revisions.len());

        let revisions =
            super::get_key_revisions("wecoyote-foo", cache.path(), None, &KeyType::Box).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_service_key_revisions() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        for _ in 0..3 {
            wait_until_ok(|| {
                let pair = BoxKeyPair::generate_pair_for_service("acme", "tnt.default")?;
                pair.to_pair_files(cache.path())?;
                Ok(())
            });
        }

        BoxKeyPair::generate_pair_for_service("acyou", "tnt.default")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();

        let revisions =
            super::get_key_revisions("tnt.default@acme", cache.path(), None, &KeyType::Box)
                .unwrap();
        assert_eq!(3, revisions.len());

        let revisions =
            super::get_key_revisions("tnt.default@acyou", cache.path(), None, &KeyType::Box)
                .unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_ring_key_revisions() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        for _ in 0..3 {
            wait_until_ok(|| {
                let pair = SymKey::generate_pair_for_ring("acme")?;
                pair.to_pair_files(cache.path())?;
                Ok(())
            });
        }

        SymKey::generate_pair_for_ring("acme-you")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();

        let revisions =
            super::get_key_revisions("acme", cache.path(), None, &KeyType::Sym).unwrap();
        assert_eq!(3, revisions.len());

        let revisions =
            super::get_key_revisions("acme-you", cache.path(), None, &KeyType::Sym).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_origin_key_revisions() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        for _ in 0..3 {
            wait_until_ok(|| {
                let pair = SigKeyPair::generate_pair_for_origin("mutants")?;
                pair.to_pair_files(cache.path())?;
                Ok(())
            });
        }

        SigKeyPair::generate_pair_for_origin("mutants-x")
            .unwrap()
            .to_pair_files(cache.path())
            .unwrap();

        let revisions =
            super::get_key_revisions("mutants", cache.path(), None, &KeyType::Sig).unwrap();
        assert_eq!(3, revisions.len());

        let revisions =
            super::get_key_revisions("mutants-x", cache.path(), None, &KeyType::Sig).unwrap();
        assert_eq!(1, revisions.len());
    }

    /// Keys should be able to be symlinks, not just normal
    /// files. This is particularly important in environments like
    /// Kubernetes that rely heavily on symlinks.
    ///
    /// See https://github.com/habitat-sh/habitat/issues/2939
    #[test]
    fn keys_that_are_symlinks_can_still_be_found() {
        let temp_dir = Builder::new().prefix("symlinks_are_ok").tempdir().unwrap();
        let key =
            SymKey::generate_pair_for_ring("symlinks_are_ok").expect("Could not generate ring key");
        key.to_pair_files(temp_dir.path()).unwrap();

        // Create a directory in our temp directory; this will serve
        // as the cache directory in which we look for keys.
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir(&cache_dir).expect("Could not create cache_dir");

        // Create a symlink to the key INTO that new dir
        let name = format!("{}.sym.key", key.name_with_rev());
        let src = temp_dir.path().join(&name);
        let dest = cache_dir.join(&name);
        symlink_file(&src, &dest).expect("Could not generate symlink");

        // For sanity, confirm that we are indeed dealing with a symlink
        let sym_meta = dest
            .symlink_metadata()
            .expect("Could not get file metadata");
        assert!(sym_meta.file_type().is_symlink());

        let revisions = super::get_key_revisions(
            "symlinks_are_ok",
            &cache_dir, // <-- THIS IS THE KEY PART OF THE TEST
            None,
            &KeyType::Sym,
        )
        .expect("Could not fetch key revisions!");

        assert_eq!(1, revisions.len());
        assert_eq!(revisions[0], key.name_with_rev());
    }

    // Windows and Linux platforms handle symlinking differently; this
    // abstracts that for the purposes of our tests here.
    #[cfg(target_os = "windows")]
    fn symlink_file<P, Q>(src: P, dest: Q) -> ::std::io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        ::std::os::windows::fs::symlink_file(src.as_ref(), dest.as_ref())
    }

    #[cfg(not(target_os = "windows"))]
    fn symlink_file<P, Q>(src: P, dest: Q) -> ::std::io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        ::std::os::unix::fs::symlink(src.as_ref(), dest.as_ref())
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
}
