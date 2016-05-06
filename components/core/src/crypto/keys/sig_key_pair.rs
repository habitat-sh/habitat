// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::path::{Path, PathBuf};

use rustc_serialize::base64::{STANDARD, ToBase64};
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::ed25519::SecretKey as SigSecretKey;
use sodiumoxide::crypto::sign::ed25519::PublicKey as SigPublicKey;

use error::{Error, Result};
use super::{get_key_revisions, mk_key_filename, mk_revision_string, parse_name_with_rev,
            read_key_bytes, write_keypair_files, KeyPair, KeyType};
use super::super::{PUBLIC_KEY_SUFFIX, SECRET_SIG_KEY_SUFFIX};

pub type SigKeyPair = KeyPair<SigPublicKey, SigSecretKey>;

impl SigKeyPair {
    pub fn generate_pair_for_origin<P: AsRef<Path> + ?Sized>(name: &str,
                                                             cache_key_path: &P)
                                                             -> Result<Self> {
        let revision = try!(mk_revision_string());
        let keyname = Self::mk_key_name(name, &revision);
        debug!("new sig key name = {}", &keyname);
        let (public_key, secret_key) = try!(Self::generate_pair_files(&keyname,
                                                                      cache_key_path.as_ref()));
        Ok(Self::new(name.to_string(),
                     revision,
                     Some(public_key),
                     Some(secret_key)))
    }

    fn mk_key_name(name: &str, revision: &str) -> String {
        format!("{}-{}", name, revision)
    }

    fn generate_pair_files(name_with_rev: &str,
                           cache_key_path: &Path)
                           -> Result<(SigPublicKey, SigSecretKey)> {
        let (pk, sk) = sign::gen_keypair();

        let public_keyfile = mk_key_filename(cache_key_path, name_with_rev, PUBLIC_KEY_SUFFIX);
        let secret_keyfile = mk_key_filename(cache_key_path, name_with_rev, SECRET_SIG_KEY_SUFFIX);
        debug!("public sig keyfile = {}", public_keyfile.display());
        debug!("secret sig keyfile = {}", secret_keyfile.display());

        try!(write_keypair_files(KeyType::Sig,
                                 &name_with_rev,
                                 Some(&public_keyfile),
                                 Some(&pk[..].to_base64(STANDARD).into_bytes()),
                                 &secret_keyfile,
                                 &sk[..].to_base64(STANDARD).into_bytes()));
        Ok((pk, sk))
    }

    /// Return a Vec of origin keys with a given name.
    /// The newest key is listed first in the Vec
    /// Origin keys are always "sig" keys. They are used for signing/verifying
    /// packages, not for encryption.
    pub fn get_pairs_for<P: AsRef<Path> + ?Sized>(name: &str,
                                                  cache_key_path: &P)
                                                  -> Result<Vec<Self>> {
        let revisions = try!(get_key_revisions(name, cache_key_path.as_ref()));
        let mut key_pairs = Vec::new();
        for name_with_rev in &revisions {
            debug!("Attempting to read key name_with_rev {} for {}",
                   name_with_rev,
                   name);
            let kp = try!(Self::get_pair_for(name_with_rev, cache_key_path));
            key_pairs.push(kp);
        }
        Ok(key_pairs)
    }

    pub fn get_pair_for<P: AsRef<Path> + ?Sized>(name_with_rev: &str,
                                                 cache_key_path: &P)
                                                 -> Result<Self> {
        let (name, rev) = try!(parse_name_with_rev(&name_with_rev));
        let pk = match Self::get_public_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!("Can't find public key for name_with_rev {}: {}",
                       name_with_rev,
                       e);
                None
            }
        };
        let sk = match Self::get_secret_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!("Can't find secret key for name_with_rev {}: {}",
                       name_with_rev,
                       e);
                None
            }
        };
        if pk == None && sk == None {
            let msg = format!("No public or secret keys found for name_with_rev {}",
                              name_with_rev);
            return Err(Error::CryptoError(msg));
        }
        Ok(SigKeyPair::new(name, rev, pk, sk))
    }

    pub fn get_latest_pair_for<P: AsRef<Path> + ?Sized>(name: &str,
                                                        cache_key_path: &P)
                                                        -> Result<Self> {
        let mut all = try!(Self::get_pairs_for(name, cache_key_path));
        match all.len() {
            0 => {
                let msg = format!("No revisions found for {} sig key", name);
                return Err(Error::CryptoError(msg));
            }
            _ => Ok(all.remove(0)),
        }
    }

    pub fn get_public_key_path<P: AsRef<Path> + ?Sized>(key_with_rev: &str,
                                                        cache_key_path: &P)
                                                        -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, PUBLIC_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(format!("No public key found at {}", path.display())));
        }
        Ok(path)
    }

    pub fn get_secret_key_path<P: AsRef<Path> + ?Sized>(key_with_rev: &str,
                                                        cache_key_path: &P)
                                                        -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, SECRET_SIG_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(format!("No secret key found at {}", path.display())));
        }
        Ok(path)
    }

    fn get_public_key(key_with_rev: &str, cache_key_path: &Path) -> Result<SigPublicKey> {
        let public_keyfile = mk_key_filename(cache_key_path, key_with_rev, PUBLIC_KEY_SUFFIX);
        let bytes = try!(read_key_bytes(&public_keyfile));
        match SigPublicKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!("Can't read sig public key for {}",
                                                      key_with_rev)))
            }
        }
    }

    fn get_secret_key(key_with_rev: &str, cache_key_path: &Path) -> Result<SigSecretKey> {
        let secret_keyfile = mk_key_filename(cache_key_path, key_with_rev, SECRET_SIG_KEY_SUFFIX);
        let bytes = try!(read_key_bytes(&secret_keyfile));
        match SigSecretKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!("Can't read sig secret key for {}",
                                                      key_with_rev)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use tempdir::TempDir;

    use super::SigKeyPair;
    use super::super::super::test_support::*;

    static VALID_KEY: &'static str = "origin-key-valid-20160509190508.sig.key";
    static VALID_PUB: &'static str = "origin-key-valid-20160509190508.pub";
    static VALID_NAME_WITH_REV: &'static str = "origin-key-valid-20160509190508";

    #[test]
    fn empty_struct() {
        let pair = SigKeyPair::new("grohl".to_string(), "201604051449".to_string(), None, None);

        assert_eq!(pair.name, "grohl");
        assert_eq!(pair.rev, "201604051449");
        assert_eq!(pair.name_with_rev(), "grohl-201604051449");

        assert_eq!(pair.public, None);
        match pair.public() {
            Ok(_) => panic!("Empty pair should not have a public key"),
            Err(_) => assert!(true),
        }
        assert_eq!(pair.secret, None);
        match pair.secret() {
            Ok(_) => panic!("Empty pair should not have a secret key"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn generated_origin_pair() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();

        assert_eq!(pair.name, "unicorn");
        match pair.public() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a public key"),
        }
        match pair.secret() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a secret key"),
        }
        assert!(cache.path().join(format!("{}.pub", pair.name_with_rev())).exists());
        assert!(cache.path().join(format!("{}.sig.key", pair.name_with_rev())).exists());
    }

    #[test]
    fn get_pairs_for() {
        let cache = TempDir::new("key_cache").unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path()).unwrap();
        assert_eq!(pairs.len(), 0);

        let _ = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path()).unwrap();
        assert_eq!(pairs.len(), 1);

        let _ = match wait_until_ok(|| {
            SigKeyPair::generate_pair_for_origin("unicorn", cache.path())
        }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path()).unwrap();
        assert_eq!(pairs.len(), 2);

        // We should not include another named key in the count
        let _ = SigKeyPair::generate_pair_for_origin("dragon", cache.path()).unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path()).unwrap();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn get_pair_for() {
        let cache = TempDir::new("key_cache").unwrap();
        let p1 = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let p2 = match wait_until_ok(|| {
            SigKeyPair::generate_pair_for_origin("unicorn", cache.path())
        }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let p1_fetched = SigKeyPair::get_pair_for(&p1.name_with_rev(), cache.path()).unwrap();
        assert_eq!(p1.name, p1_fetched.name);
        assert_eq!(p1.rev, p1_fetched.rev);
        let p2_fetched = SigKeyPair::get_pair_for(&p2.name_with_rev(), cache.path()).unwrap();
        assert_eq!(p2.name, p2_fetched.name);
        assert_eq!(p2.rev, p2_fetched.rev);
    }

    #[test]
    #[should_panic(expected = "No public or secret keys found for")]
    fn get_pair_for_nonexistant() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_pair_for("nope-nope-20160405144901", cache.path()).unwrap();
    }

    #[test]
    fn get_latest_pair_for_single() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();

        let latest = SigKeyPair::get_latest_pair_for("unicorn", cache.path()).unwrap();
        assert_eq!(latest.name, pair.name);
        assert_eq!(latest.rev, pair.rev);
    }

    #[test]
    fn get_latest_pair_for_multiple() {
        let cache = TempDir::new("key_cache").unwrap();
        let _ = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let p2 = match wait_until_ok(|| {
            SigKeyPair::generate_pair_for_origin("unicorn", cache.path())
        }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let latest = SigKeyPair::get_latest_pair_for("unicorn", cache.path()).unwrap();
        assert_eq!(latest.name, p2.name);
        assert_eq!(latest.rev, p2.rev);
    }

    #[test]
    #[should_panic(expected = "No revisions found for")]
    fn get_latest_pair_for_nonexistant() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_latest_pair_for("nope-nope", cache.path()).unwrap();
    }

    #[test]
    fn get_public_key_path() {
        let cache = TempDir::new("key_cache").unwrap();
        fs::copy(fixture(&format!("keys/{}", VALID_PUB)),
                 cache.path().join(VALID_PUB))
            .unwrap();

        let result = SigKeyPair::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_PUB));
    }

    #[test]
    #[should_panic(expected = "No public key found at")]
    fn get_public_key_path_nonexistant() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

    #[test]
    fn get_secret_key_path() {
        let cache = TempDir::new("key_cache").unwrap();
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)),
                 cache.path().join(VALID_KEY))
            .unwrap();

        let result = SigKeyPair::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_KEY));
    }

    #[test]
    #[should_panic(expected = "No secret key found at")]
    fn get_secret_key_path_nonexistant() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }
}
