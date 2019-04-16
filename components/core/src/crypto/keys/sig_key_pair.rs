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

use std::{fs,
          path::{Path,
                 PathBuf}};

use base64;
use hex;
use sodiumoxide::{crypto::sign::{self,
                                 ed25519::{PublicKey as SigPublicKey,
                                           SecretKey as SigSecretKey}},
                  randombytes::randombytes};

use super::{super::{hash,
                    PUBLIC_KEY_SUFFIX,
                    PUBLIC_SIG_KEY_VERSION,
                    SECRET_SIG_KEY_SUFFIX,
                    SECRET_SIG_KEY_VERSION},
            get_key_revisions,
            mk_key_filename,
            mk_revision_string,
            parse_name_with_rev,
            read_key_bytes,
            write_keypair_files,
            KeyPair,
            KeyType,
            PairType,
            TmpKeyfile};
use crate::error::{Error,
                   Result};

pub type SigKeyPair = KeyPair<SigPublicKey, SigSecretKey>;

impl SigKeyPair {
    pub fn generate_pair_for_origin(name: &str) -> Result<Self> {
        let revision = mk_revision_string()?;
        let (pk, sk) = sign::gen_keypair();
        Ok(Self::new(name.to_string(), revision, Some(pk), Some(sk)))
    }

    /// Return a Vec of origin keys with a given name.
    /// The newest key is listed first in the Vec.
    pub fn get_pairs_for<P: AsRef<Path> + ?Sized>(name: &str,
                                                  cache_key_path: &P,
                                                  pair_type: Option<&PairType>)
                                                  -> Result<Vec<Self>> {
        let revisions = get_key_revisions(name, cache_key_path.as_ref(), pair_type, &KeyType::Sig)?;
        debug!("revisions = {:?}", &revisions);
        let mut key_pairs = Vec::new();
        for name_with_rev in &revisions {
            debug!("Attempting to read key name_with_rev {} for {}",
                   name_with_rev, name);
            let kp = Self::get_pair_for(name_with_rev, cache_key_path)?;
            key_pairs.push(kp);
        }
        Ok(key_pairs)
    }

    pub fn get_pair_for<P: AsRef<Path> + ?Sized>(name_with_rev: &str,
                                                 cache_key_path: &P)
                                                 -> Result<Self> {
        let (name, rev) = parse_name_with_rev(name_with_rev)?;
        let pk = match Self::get_public_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!("Can't find public key for name_with_rev {}: {}",
                       name_with_rev, e);
                None
            }
        };
        let sk = match Self::get_secret_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!("Can't find secret key for name_with_rev {}: {}",
                       name_with_rev, e);
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
                                                        cache_key_path: &P,
                                                        pair_type: Option<&PairType>)
                                                        -> Result<Self> {
        let mut all = Self::get_pairs_for(name, cache_key_path, pair_type)?;
        match all.len() {
            0 => {
                let msg = format!("No revisions found for {} sig key", name);
                Err(Error::CryptoError(msg))
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

    /// Writes a sig key (public or secret) to the key cache from the contents of a string slice.
    ///
    /// The return is a `Result` of a `String` containing the key's name with revision.
    ///
    /// # Examples
    ///
    /// With a public key:
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempfile;
    ///
    /// use habitat_core::crypto::{keys::PairType,
    ///                            SigKeyPair};
    /// use std::{fs::File,
    ///           io::Read};
    /// use tempfile::Builder;
    ///
    /// fn main() {
    ///     let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    ///     let content = "SIG-PUB-1
    /// unicorn-20160517220007
    ///
    /// J+FGYVKgragA+dzQHCGORd2oLwCc2EvAnT9roz9BJh0=";
    ///     let key_path = cache.path().join("unicorn-20160517220007.pub");
    ///
    ///     let (pair, pair_type) = SigKeyPair::write_file_from_str(content, cache.path()).unwrap();
    ///     assert_eq!(pair_type, PairType::Public);
    ///     assert_eq!(pair.name_with_rev(), "unicorn-20160517220007");
    ///     assert!(key_path.is_file());
    ///     let mut f = File::open(key_path).unwrap();
    ///     let mut key_content = String::new();
    ///     f.read_to_string(&mut key_content).unwrap();
    ///     assert_eq!(&key_content, content);
    /// }
    /// ```
    ///
    /// With a secret key:
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempfile;
    ///
    /// use habitat_core::crypto::{keys::PairType,
    ///                            SigKeyPair};
    /// use std::{fs::File,
    ///           io::Read};
    /// use tempfile::Builder;
    ///
    /// fn main() {
    ///     let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    ///     let content = "SIG-SEC-1
    /// unicorn-20160517220007
    ///
    /// jjQaaphB5+CHw7QzDWqMMuwhWmrrHH+SzQAgRrHfQ8sn4UZhUqCtqAD53NAcIY5F3agvAJzYS8CdP2ujP0EmHQ==";
    ///     let key_path = cache.path().join("unicorn-20160517220007.sig.key");
    ///
    ///     let (pair, pair_type) = SigKeyPair::write_file_from_str(content, cache.path()).unwrap();
    ///     assert_eq!(pair_type, PairType::Secret);
    ///     assert_eq!(pair.name_with_rev(), "unicorn-20160517220007");
    ///     assert!(key_path.is_file());
    ///     let mut f = File::open(key_path).unwrap();
    ///     let mut key_content = String::new();
    ///     f.read_to_string(&mut key_content).unwrap();
    ///     assert_eq!(&key_content, content);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If there is a key version mismatch
    /// * If the key version is missing
    /// * If the key name with revision is missing
    /// * If the key value (the Bas64 payload) is missing
    /// * If the key file cannot be written to disk
    /// * If an existing key is already installed, but the new content is different from the
    /// existing
    pub fn write_file_from_str<P: AsRef<Path> + ?Sized>(content: &str,
                                                        cache_key_path: &P)
                                                        -> Result<(Self, PairType)> {
        let (pair_type, name_with_rev, _) = super::parse_key_str(content)?;
        let suffix = match pair_type {
            PairType::Public => PUBLIC_KEY_SUFFIX,
            PairType::Secret => SECRET_SIG_KEY_SUFFIX,
        };
        let keyfile = mk_key_filename(cache_key_path.as_ref(), &name_with_rev, &suffix);
        let tmpfile = {
            let mut t = keyfile.clone();
            t.set_file_name(format!("{}.{}",
                                    &keyfile.file_name().unwrap().to_str().unwrap(),
                                    &hex::encode(randombytes(6).as_slice())));
            TmpKeyfile { path: t }
        };

        debug!("Writing temp key file {}", tmpfile.path.display());
        match pair_type {
            PairType::Public => {
                write_keypair_files(Some(&tmpfile.path), Some(content.to_string()), None, None)?;
            }
            PairType::Secret => {
                write_keypair_files(None, None, Some(&tmpfile.path), Some(content.to_string()))?;
            }
        }

        if Path::new(&keyfile).is_file() {
            let existing_hash = hash::hash_file(&keyfile)?;
            let new_hash = hash::hash_file(&tmpfile.path)?;
            if existing_hash != new_hash {
                let msg = format!("Existing key file {} found but new version hash is different, \
                                   failing to write new file over existing. ({} = {}, {} = {})",
                                  keyfile.display(),
                                  keyfile.display(),
                                  existing_hash,
                                  tmpfile.path.display(),
                                  new_hash);
                return Err(Error::CryptoError(msg));
            } else {
                // Otherwise, hashes match and we can skip writing over the existing file
                debug!("New content hash matches existing file {} hash, removing temp key file \
                        {}.",
                       keyfile.display(),
                       tmpfile.path.display());
                fs::remove_file(&tmpfile.path)?;
            }
        } else {
            fs::rename(&tmpfile.path, keyfile)?;
        }
        Ok((Self::get_pair_for(&name_with_rev, cache_key_path)?, pair_type))
    }

    pub fn to_public_string(&self) -> Result<String> {
        match self.public {
            Some(pk) => {
                Ok(format!("{}\n{}\n\n{}",
                           PUBLIC_SIG_KEY_VERSION,
                           self.name_with_rev(),
                           &base64::encode(&pk[..])))
            }
            None => {
                Err(Error::CryptoError(format!("No public key present for {}",
                                               self.name_with_rev())))
            }
        }
    }

    pub fn to_secret_string(&self) -> Result<String> {
        match self.secret {
            Some(ref sk) => {
                Ok(format!("{}\n{}\n\n{}",
                           SECRET_SIG_KEY_VERSION,
                           self.name_with_rev(),
                           &base64::encode(&sk[..])))
            }
            None => {
                Err(Error::CryptoError(format!("No secret key present for {}",
                                               self.name_with_rev())))
            }
        }
    }

    pub fn to_pair_files<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<()> {
        let public_keyfile = mk_key_filename(path, self.name_with_rev(), PUBLIC_KEY_SUFFIX);
        let secret_keyfile = mk_key_filename(path, self.name_with_rev(), SECRET_SIG_KEY_SUFFIX);
        debug!("public sig keyfile = {}", public_keyfile.display());
        debug!("secret sig keyfile = {}", secret_keyfile.display());

        write_keypair_files(Some(&public_keyfile),
                            Some(self.to_public_string()?),
                            Some(&secret_keyfile),
                            Some(self.to_secret_string()?))
    }

    fn get_public_key(key_with_rev: &str, cache_key_path: &Path) -> Result<SigPublicKey> {
        let public_keyfile = mk_key_filename(cache_key_path, key_with_rev, PUBLIC_KEY_SUFFIX);
        let bytes = read_key_bytes(&public_keyfile)?;
        match SigPublicKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                Err(Error::CryptoError(format!("Can't read sig public key \
                                                for {}",
                                               key_with_rev)))
            }
        }
    }

    fn get_secret_key(key_with_rev: &str, cache_key_path: &Path) -> Result<SigSecretKey> {
        let secret_keyfile = mk_key_filename(cache_key_path, key_with_rev, SECRET_SIG_KEY_SUFFIX);
        let bytes = read_key_bytes(&secret_keyfile)?;
        match SigSecretKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                Err(Error::CryptoError(format!("Can't read sig secret key \
                                                for {}",
                                               key_with_rev)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{fs::{self,
                   File},
              io::Read};

    use tempfile::Builder;

    use super::{super::{super::test_support::*,
                        PairType},
                SigKeyPair};

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
        assert!(pair.public().is_err(),
                "Empty pair should not have a public key");
        assert_eq!(pair.secret, None);
        assert!(pair.secret().is_err(),
                "Empty pair should not have a secret key");
    }

    #[test]
    fn generated_origin_pair() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn").unwrap();
        pair.to_pair_files(cache.path()).unwrap();

        assert_eq!(pair.name, "unicorn");
        assert!(pair.public().is_ok(),
                "Generated pair should have a public key");
        assert!(pair.secret().is_ok(),
                "Generated pair should have a public key");
        assert!(cache.path()
                     .join(format!("{}.pub", pair.name_with_rev()))
                     .exists());
        assert!(cache.path()
                     .join(format!("{}.sig.key", pair.name_with_rev()))
                     .exists());
    }

    #[test]
    fn get_pairs_for() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(pairs.len(), 0);

        SigKeyPair::generate_pair_for_origin("unicorn").unwrap()
                                                       .to_pair_files(cache.path())
                                                       .unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(pairs.len(), 1);

        match wait_until_ok(|| {
                  let p = SigKeyPair::generate_pair_for_origin("unicorn")?;
                  p.to_pair_files(cache.path())?;
                  Ok(())
              }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(pairs.len(), 2);

        // We should not include another named key in the count
        SigKeyPair::generate_pair_for_origin("dragon").unwrap()
                                                      .to_pair_files(cache.path())
                                                      .unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(pairs.len(), 2);

        // We should be able to count public and private keys separately
        let pairs =
            SigKeyPair::get_pairs_for("unicorn", cache.path(), Some(&PairType::Secret)).unwrap();
        assert_eq!(pairs.len(), 2);

        let pairs =
            SigKeyPair::get_pairs_for("unicorn", cache.path(), Some(&PairType::Public)).unwrap();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn get_pair_for() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let p1 = SigKeyPair::generate_pair_for_origin("unicorn").unwrap();
        p1.to_pair_files(cache.path()).unwrap();
        let p2 = match wait_until_ok(|| {
                  let p = SigKeyPair::generate_pair_for_origin("unicorn")?;
                  p.to_pair_files(cache.path())?;
                  Ok(p)
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
    fn get_pair_for_nonexistent() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::get_pair_for("nope-nope-20160405144901", cache.path()).unwrap();
    }

    #[test]
    fn get_latest_pair_for_single() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn").unwrap();
        pair.to_pair_files(cache.path()).unwrap();

        let latest = SigKeyPair::get_latest_pair_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(latest.name, pair.name);
        assert_eq!(latest.rev, pair.rev);
    }

    #[test]
    fn get_latest_pair_for_multiple() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("unicorn").unwrap()
                                                       .to_pair_files(cache.path())
                                                       .unwrap();
        let p2 = match wait_until_ok(|| {
                  let p = SigKeyPair::generate_pair_for_origin("unicorn")?;
                  p.to_pair_files(cache.path())?;
                  Ok(p)
              }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let latest = SigKeyPair::get_latest_pair_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(latest.name, p2.name);
        assert_eq!(latest.rev, p2.rev);
    }

    #[test]
    fn get_latest_pair_for_secret() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let p = SigKeyPair::generate_pair_for_origin("unicorn").unwrap();
        p.to_pair_files(cache.path()).unwrap();
        let latest = SigKeyPair::get_latest_pair_for("unicorn",
                                                     cache.path(),
                                                     Some(&PairType::Secret)).unwrap();
        assert_eq!(latest.name, p.name);
        assert_eq!(latest.rev, p.rev);
    }

    #[test]
    fn get_latest_pair_for_public() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let p = SigKeyPair::generate_pair_for_origin("unicorn").unwrap();
        p.to_pair_files(cache.path()).unwrap();
        let latest = SigKeyPair::get_latest_pair_for("unicorn",
                                                     cache.path(),
                                                     Some(&PairType::Public)).unwrap();
        assert_eq!(latest.name, p.name);
        assert_eq!(latest.rev, p.rev);
    }

    #[test]
    #[should_panic(expected = "No revisions found for")]
    fn get_latest_pair_for_nonexistent() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::get_latest_pair_for("nope-nope", cache.path(), None).unwrap();
    }

    #[test]
    fn get_public_key_path() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        fs::copy(fixture(&format!("keys/{}", VALID_PUB)),
                 cache.path().join(VALID_PUB)).unwrap();

        let result = SigKeyPair::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_PUB));
    }

    #[test]
    #[should_panic(expected = "No public key found at")]
    fn get_public_key_path_nonexistent() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

    #[test]
    fn get_secret_key_path() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)),
                 cache.path().join(VALID_KEY)).unwrap();

        let result = SigKeyPair::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_KEY));
    }

    #[test]
    #[should_panic(expected = "No secret key found at")]
    fn get_secret_key_path_nonexistent() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

    #[test]
    fn write_file_from_str_secret() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
        let new_key_file = cache.path().join(VALID_KEY);

        assert_eq!(new_key_file.is_file(), false);
        let (pair, pair_type) = SigKeyPair::write_file_from_str(&content, cache.path()).unwrap();
        assert_eq!(pair_type, PairType::Secret);
        assert_eq!(pair.name_with_rev(), VALID_NAME_WITH_REV);
        assert!(new_key_file.is_file());

        let new_content = {
            let mut new_content_file = File::open(new_key_file).unwrap();
            let mut new_content = String::new();
            new_content_file.read_to_string(&mut new_content).unwrap();
            new_content
        };

        assert_eq!(new_content, content);
    }

    #[test]
    fn write_file_from_str_public() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string(&format!("keys/{}", VALID_PUB));
        let new_key_file = cache.path().join(VALID_PUB);

        assert_eq!(new_key_file.is_file(), false);
        let (pair, pair_type) = SigKeyPair::write_file_from_str(&content, cache.path()).unwrap();
        assert_eq!(pair_type, PairType::Public);
        assert_eq!(pair.name_with_rev(), VALID_NAME_WITH_REV);
        assert!(new_key_file.is_file());

        let new_content = {
            let mut new_content_file = File::open(new_key_file).unwrap();
            let mut new_content = String::new();
            new_content_file.read_to_string(&mut new_content).unwrap();
            new_content
        };

        assert_eq!(new_content, content);
    }

    #[test]
    fn write_file_from_str_with_existing_identical_secret() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
        let new_key_file = cache.path().join(VALID_KEY);

        // install the key into the cache
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &new_key_file).unwrap();

        let (pair, pair_type) = SigKeyPair::write_file_from_str(&content, cache.path()).unwrap();
        assert_eq!(pair_type, PairType::Secret);
        assert_eq!(pair.name_with_rev(), VALID_NAME_WITH_REV);
        assert!(new_key_file.is_file());
    }

    #[test]
    fn write_file_from_str_with_existing_identical_public() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string(&format!("keys/{}", VALID_PUB));
        let new_key_file = cache.path().join(VALID_PUB);

        // install the key into the cache
        fs::copy(fixture(&format!("keys/{}", VALID_PUB)), &new_key_file).unwrap();

        let (pair, pair_type) = SigKeyPair::write_file_from_str(&content, cache.path()).unwrap();
        assert_eq!(pair_type, PairType::Public);
        assert_eq!(pair.name_with_rev(), VALID_NAME_WITH_REV);
        assert!(new_key_file.is_file());
    }

    #[test]
    #[should_panic(expected = "Unsupported key version")]
    fn write_file_from_str_unsupported_version_secret() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string("keys/origin-key-invalid-version-20160518021451.sig.key");

        SigKeyPair::write_file_from_str(&content, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unsupported key version")]
    fn write_file_from_str_unsupported_version_public() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string("keys/origin-key-invalid-version-20160518021451.pub");

        SigKeyPair::write_file_from_str(&content, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_key_from_str:1 Malformed key string")]
    fn write_file_from_str_missing_version() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SigKeyPair::write_file_from_str("", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_key_from_str:2 Malformed key string")]
    fn write_file_from_str_missing_name_secret() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SigKeyPair::write_file_from_str("SIG-SEC-1\n", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_key_from_str:2 Malformed key string")]
    fn write_file_from_str_missing_name_public() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SigKeyPair::write_file_from_str("SIG-PUB-1\n", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_key_from_str:3 Malformed key string")]
    fn write_file_from_str_missing_key_secret() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SigKeyPair::write_file_from_str("SIG-SEC-1\nim-in-trouble-123\n", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_key_from_str:3 Malformed key string")]
    fn write_file_from_str_missing_key_public() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SigKeyPair::write_file_from_str("SIG-PUB-1\nim-in-trouble-123\n", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_key_from_str:3 Malformed key string")]
    fn write_file_from_str_invalid_key_secret() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SigKeyPair::write_file_from_str("SIG-SEC-1\norigin-key-valid-20160509190508\n\\
                                         nc29tZXRoaW5n%",
                                        cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_key_from_str:3 Malformed key string")]
    fn write_file_from_str_invalid_key_public() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SigKeyPair::write_file_from_str("SIG-PUB-1\nim-in-trouble-123\n\nc29tZXRoaW5n%",
                                        cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Existing key file")]
    fn write_file_from_str_key_exists_but_hashes_differ_secret() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let key = fixture("keys/origin-key-valid-20160509190508.sig.key");
        fs::copy(key,
                 cache.path().join("origin-key-valid-20160509190508.sig.key")).unwrap();
        let k = "SIG-SEC-1\norigin-key-valid-20160509190508\n\nc29tZXRoaW5n";
        SigKeyPair::write_file_from_str(k, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Existing key file")]
    fn write_file_from_str_key_exists_but_hashes_differ_public() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let key = fixture("keys/origin-key-valid-20160509190508.pub");
        fs::copy(key,
                 cache.path().join("origin-key-valid-20160509190508.pub")).unwrap();
        let k = "SIG-PUB-1\norigin-key-valid-20160509190508\n\nc29tZXRoaW5n";
        SigKeyPair::write_file_from_str(k, cache.path()).unwrap();
    }
}
