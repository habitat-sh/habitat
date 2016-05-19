// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::collections::HashSet;
use std::fs;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;

use regex::Regex;
use rustc_serialize::base64::FromBase64;
use time;

use error::{Error, Result};
use util::perm;

use super::{PUBLIC_BOX_KEY_VERSION, PUBLIC_KEY_PERMISSIONS, PUBLIC_KEY_SUFFIX,
            PUBLIC_SIG_KEY_VERSION, SECRET_BOX_KEY_SUFFIX, SECRET_BOX_KEY_VERSION,
            SECRET_KEY_PERMISSIONS, SECRET_SIG_KEY_SUFFIX, SECRET_SIG_KEY_VERSION,
            SECRET_SYM_KEY_SUFFIX, SECRET_SYM_KEY_VERSION};

lazy_static! {
    static ref NAME_WITH_REV_RE: Regex = Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\z").unwrap();
    static ref KEYFILE_RE: Regex = Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\.(?P<suffix>[a-z]+(\.[a-z]+)?)\z").unwrap();
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
                return Err(Error::CryptoError(format!("Invalid PairType conversion from {}",
                                                      value)))
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
                let msg = format!("Public key is required but not present for {}",
                                  self.name_with_rev());
                return Err(Error::CryptoError(msg));
            }
        }
    }

    pub fn secret(&self) -> Result<&S> {
        match self.secret.as_ref() {
            Some(s) => Ok(s),
            None => {
                let msg = format!("Secret key is required but not present for {}",
                                  self.name_with_rev());
                return Err(Error::CryptoError(msg));
            }
        }
    }
}

/// If a key "belongs" to a filename revision, then add the full stem of the
/// file (without path, without .suffix) to the set. This function doesn't
/// return an error on a "bad" file, the bad file key name just doesn't get
/// added to the set.
fn check_filename(keyname: &str, filename: String, candidates: &mut HashSet<String>) -> () {
    let caps = match KEYFILE_RE.captures(&filename) {
        Some(c) => c,
        None => {
            debug!("check_filename: Cannot parse {}", &filename);
            return;
        }
    };
    let name = match caps.name("name") {
        Some(r) => r,
        None => {
            debug!("check_filename: Cannot parse name from {}", &filename);
            return;
        }
    };

    let rev = match caps.name("rev") {
        Some(r) => r,
        None => {
            debug!("check_filename: Cannot parse rev from {}", &filename);
            return;
        }
    };

    let suffix = match caps.name("suffix") {
        Some(r) => r,
        None => {
            debug!("check_filename: Cannot parse suffix from {}", &filename);
            return;
        }
    };

    if suffix == PUBLIC_KEY_SUFFIX || suffix == SECRET_SIG_KEY_SUFFIX ||
       suffix == SECRET_BOX_KEY_SUFFIX || suffix == SECRET_SYM_KEY_SUFFIX {
        debug!("valid key suffix");
    } else {
        debug!("check_filename: Invalid key suffix from {}", &filename);
        return;
    };

    if name == keyname {
        let thiskey = format!("{}-{}", name, rev);
        candidates.insert(thiskey);
    }

}

/// Take a key name (ex "habitat"), and find all revisions of that
/// keyname in the default_cache_key_path().
fn get_key_revisions(keyname: &str, cache_key_path: &Path) -> Result<Vec<String>> {
    // accumulator for files that match
    let mut candidates = HashSet::new();
    let paths = match fs::read_dir(cache_key_path) {
        Ok(p) => p,
        Err(e) => {
            return Err(Error::CryptoError(format!("Error reading key directory {}: {}",
                                                  cache_key_path.display(),
                                                  e)))
        }
    };
    for path in paths {
        match path {
            Ok(ref p) => p,
            Err(e) => {
                debug!("Error reading path {}", e);
                return Err(Error::CryptoError(format!("Error reading key path {}", e)));
            }
        };

        let p: fs::DirEntry = path.unwrap();

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
        check_filename(keyname, filename, &mut candidates);
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

fn mk_key_filename(path: &Path, keyname: &str, suffix: &str) -> PathBuf {
    path.join(format!("{}.{}", keyname, suffix))
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

pub fn parse_name_with_rev(name_with_rev: &str) -> Result<(String, String)> {
    let caps = match NAME_WITH_REV_RE.captures(name_with_rev) {
        Some(c) => c,
        None => {
            let msg = format!("parse_name_with_rev:1 Cannot parse {}", &name_with_rev);
            return Err(Error::CryptoError(msg));
        }
    };
    let name = match caps.name("name") {
        Some(r) => r,
        None => {
            let msg = format!("parse_name_with_rev:2 Cannot parse name from {}",
                              &name_with_rev);
            return Err(Error::CryptoError(msg));
        }
    };
    let rev = match caps.name("rev") {
        Some(r) => r,
        None => {
            let msg = format!("parse_name_with_rev:3 Cannot parse rev from {}",
                              &name_with_rev);
            return Err(Error::CryptoError(msg));
        }
    };
    Ok((name.to_string(), rev.to_string()))
}

/// Read a file into a Vec<u8>
fn read_key_bytes(keyfile: &Path) -> Result<Vec<u8>> {
    let mut f = try!(File::open(keyfile));
    let mut s = String::new();
    if try!(f.read_to_string(&mut s)) <= 0 {
        return Err(Error::CryptoError("Can't read key bytes".to_string()));
    }
    let start_index = match s.find("\n\n") {
        Some(i) => i + 1,
        None => {
            return Err(Error::CryptoError(format!("Malformed key contents for: {}",
                                                  keyfile.display())))
        }
    };

    match s[start_index..].as_bytes().from_base64() {
        Ok(keybytes) => Ok(keybytes),
        Err(e) => {
            return Err(Error::CryptoError(format!("Can't read raw key from {}: {}",
                                                  keyfile.display(),
                                                  e)))
        }
    }
}

fn write_keypair_files(key_type: KeyType,
                       keyname: &str,
                       public_keyfile: Option<&Path>,
                       public_content: Option<&Vec<u8>>,
                       secret_keyfile: Option<&Path>,
                       secret_content: Option<&Vec<u8>>)
                       -> Result<()> {
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
            return Err(Error::BadKeyPath(public_keyfile.to_string_lossy().into_owned()));
        }
        if public_keyfile.exists() {
            return Err(Error::CryptoError(format!("Public keyfile or a directory already exists {}",
                                                  public_keyfile.display())));
        }
        let public_file = try!(File::create(public_keyfile));
        let mut public_writer = BufWriter::new(&public_file);
        try!(write!(public_writer, "{}\n{}\n\n", public_version, keyname));
        try!(public_writer.write_all(public_content));
        try!(perm::set_permissions(public_keyfile, PUBLIC_KEY_PERMISSIONS));
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
            return Err(Error::BadKeyPath(secret_keyfile.to_string_lossy().into_owned()));
        }
        if secret_keyfile.exists() {
            return Err(Error::CryptoError(format!("Secret keyfile or a directory already exists {}",
                                                  secret_keyfile.display())));
        }
        let secret_file = try!(File::create(secret_keyfile));
        let mut secret_writer = BufWriter::new(&secret_file);
        try!(write!(secret_writer, "{}\n{}\n\n", secret_version, keyname));
        try!(secret_writer.write_all(secret_content));
        try!(perm::set_permissions(secret_keyfile, SECRET_KEY_PERMISSIONS));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::fs::{self, File};
    use std::io::Write;
    use tempdir::TempDir;
    use rustc_serialize::hex::ToHex;


    use super::box_key_pair::BoxKeyPair;
    use super::sig_key_pair::SigKeyPair;
    use super::sym_key::SymKey;

    use super::TmpKeyfile;
    use super::super::test_support::*;

    static VALID_KEY: &'static str = "ring-key-valid-20160504220722.sym.key";
    static VALID_KEY_AS_HEX: &'static str = "44215a3bce23e351a6af359d77131db17a46767de2b88cbb330df162b8cf2ec1";

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
        f.write_all("something\n\nI am not base64 content".as_bytes()).unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

 
    #[test]
    fn get_user_key_revisions() {
        let cache = TempDir::new("key_cache").unwrap();
        for _ in 0..3 {
            wait_until_ok(|| BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()));
        }
        let _ = BoxKeyPair::generate_pair_for_user("wecoyote-foo", cache.path()).unwrap();

        // we shouldn't see wecoyote-foo as a 4th revision
        let revisions = super::get_key_revisions("wecoyote", cache.path()).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("wecoyote-foo", cache.path()).unwrap();
        assert_eq!(1, revisions.len());
    }


    #[test]
    fn get_service_key_revisions() {
        let cache = TempDir::new("key_cache").unwrap();

        for _ in 0..3 {
            wait_until_ok(|| BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path()));
        }

        let _ = BoxKeyPair::generate_pair_for_service("acyou", "tnt.default", cache.path()).unwrap();

        let revisions = super::get_key_revisions("tnt.default@acme", cache.path()).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("tnt.default@acyou", cache.path()).unwrap();
        assert_eq!(1, revisions.len());
    }


    #[test]
    fn get_ring_key_revisions() {
        let cache = TempDir::new("key_cache").unwrap();

        for _ in 0..3 {
            wait_until_ok(|| SymKey::generate_pair_for_ring("acme", cache.path()));
        }

        let _ = SymKey::generate_pair_for_ring("acme-you", cache.path()).unwrap();

        let revisions = super::get_key_revisions("acme", cache.path()).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("acme-you", cache.path()).unwrap();
        assert_eq!(1, revisions.len());
    }


    #[test]
    fn get_origin_key_revisions() {
        let cache = TempDir::new("key_cache").unwrap();

        for _ in 0..3 {
            wait_until_ok(|| SigKeyPair::generate_pair_for_origin("mutants", cache.path()));
        }

        let _ = SigKeyPair::generate_pair_for_origin("mutants-x", cache.path()).unwrap();

        let revisions = super::get_key_revisions("mutants", cache.path()).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("mutants-x", cache.path()).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn check_filename_key_without_dash() {
        // look for a keyname that doesn't include a dash
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.pub".to_string(),
                              &mut candidates);
        super::check_filename("wecoyote",
                              "wecoyote-foo-20160519203610.pub".to_string(),
                              &mut candidates);
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.box.key".to_string(),
                              &mut candidates);
        super::check_filename("wecoyote",
                              "wecoyote-foo-20160519203610.box.key".to_string(),
                              &mut candidates);
        assert_eq!(1, candidates.len());

    }


    #[test]
    fn check_filename_key_with_dash() {
        // look for a keyname that includes a dash
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote-foo",
                              "wecoyote-20160519203610.pub".to_string(),
                              &mut candidates);
        super::check_filename("wecoyote-foo",
                              "wecoyote-foo-20160519203610.pub".to_string(),
                              &mut candidates);
        super::check_filename("wecoyote-foo",
                              "wecoyote-20160519203610.box.key".to_string(),
                              &mut candidates);
        super::check_filename("wecoyote-foo",
                              "wecoyote-foo-20160519203610.box.key".to_string(),
                              &mut candidates);
        assert_eq!(1, candidates.len());

    }
}
