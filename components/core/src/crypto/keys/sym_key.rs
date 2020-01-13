use super::{super::{hash,
                    SECRET_SYM_KEY_SUFFIX,
                    SECRET_SYM_KEY_VERSION},
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
use base64;
use hex;
use sodiumoxide::{crypto::secretbox::{self,
                                      Key as SymSecretKey},
                  randombytes::randombytes};
use std::{fmt,
          fs,
          path::{Path,
                 PathBuf}};

pub type SymKey = KeyPair<(), SymSecretKey>;

impl fmt::Debug for SymKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "SymKey") }
}

impl SymKey {
    pub fn generate_pair_for_ring(name: &str) -> Result<Self> {
        let revision = mk_revision_string()?;
        let secret_key = secretbox::gen_key();
        Ok(SymKey::new(name.to_string(),
                       revision,
                       Some(()),
                       Some(secret_key)))
    }

    pub fn get_pairs_for<P: AsRef<Path> + ?Sized>(name: &str,
                                                  cache_key_path: &P)
                                                  -> Result<Vec<Self>> {
        let revisions = get_key_revisions(name, cache_key_path.as_ref(), None, &KeyType::Sym)?;
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
        let (name, rev) = parse_name_with_rev(&name_with_rev)?;
        let sk = match Self::get_secret_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                let msg = format!("No secret keys found for name_with_rev {}: {}",
                                  name_with_rev, e);
                return Err(Error::CryptoError(msg));
            }
        };
        Ok(Self::new(name, rev, None, sk))
    }

    pub fn get_latest_pair_for<P: AsRef<Path> + ?Sized>(name: &str,
                                                        cache_key_path: &P)
                                                        -> Result<Self> {
        let mut all = Self::get_pairs_for(name, cache_key_path)?;
        match all.len() {
            0 => {
                let msg = format!("No revisions found for {} sym key", name);
                Err(Error::CryptoError(msg))
            }
            _ => Ok(all.remove(0)),
        }
    }

    pub fn get_public_key_path<P: AsRef<Path> + ?Sized>(_key_with_rev: &str,
                                                        _cache_key_path: &P)
                                                        -> Result<PathBuf> {
        Err(Error::CryptoError("No public key exists for sym keys".to_string()))
    }

    /// Returns the full path to the secret sym key given a key name with revision.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempfile;
    ///
    /// use habitat_core::crypto::SymKey;
    /// use std::fs::File;
    /// use tempfile::Builder;
    ///
    /// let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    /// let secret_file = cache.path().join("beyonce-20160504220722.sym.key");
    /// let _ = File::create(&secret_file).unwrap();
    ///
    /// let path = SymKey::get_secret_key_path("beyonce-20160504220722", cache.path()).unwrap();
    /// assert_eq!(path, secret_file);
    /// ```
    ///
    /// # Errors
    ///
    /// * If no file exists at the the computed file path
    pub fn get_secret_key_path<P: AsRef<Path> + ?Sized>(key_with_rev: &str,
                                                        cache_key_path: &P)
                                                        -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, SECRET_SYM_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(format!("No secret key found at {}", path.display())));
        }
        Ok(path)
    }

    /// Encrypts a byte slice of data using a given `SymKey`.
    ///
    /// The return is a `Result` of a tuple of `Vec<u8>` structs, the first being the random nonce
    /// value and the second being the ciphertext.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempfile;
    ///
    /// use habitat_core::crypto::SymKey;
    /// use tempfile::Builder;
    ///
    /// let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    /// let sym_key = SymKey::generate_pair_for_ring("beyonce").unwrap();
    ///
    /// let (nonce, ciphertext) = sym_key.encrypt("Guess who?".as_bytes()).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// * If the secret key component of the `SymKey` is not present
    pub fn encrypt(&self, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let key = self.secret()?;
        let nonce = secretbox::gen_nonce();
        Ok((nonce.as_ref().to_vec(), secretbox::seal(data, &nonce, &key)))
    }

    /// Decrypts a byte slice of ciphertext using a given nonce value and a `SymKey`.
    ///
    /// The return is a `Result` of a byte vector containing the original, unencrypted data.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempfile;
    ///
    /// use habitat_core::crypto::SymKey;
    /// use tempfile::Builder;
    ///
    /// let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    /// let sym_key = SymKey::generate_pair_for_ring("beyonce").unwrap();
    /// let (nonce, ciphertext) = sym_key.encrypt("Guess who?".as_bytes()).unwrap();
    ///
    /// let message = sym_key.decrypt(&nonce, &ciphertext).unwrap();
    /// assert_eq!(message, "Guess who?".to_string().into_bytes());
    /// ```
    ///
    /// # Errors
    ///
    /// * If the secret key component of the `SymKey` is not present
    /// * If the size of the provided nonce data is not the required size
    /// * If the ciphertext was not decryptable given the nonce and symmetric key
    pub fn decrypt(&self, nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let key = self.secret()?;
        let nonce = match secretbox::Nonce::from_slice(&nonce) {
            Some(n) => n,
            None => return Err(Error::CryptoError("Invalid size of nonce".to_string())),
        };
        match secretbox::open(ciphertext, &nonce, &key) {
            Ok(msg) => Ok(msg),
            Err(_) => {
                Err(Error::CryptoError("Secret key and nonce could not \
                                        decrypt ciphertext"
                                                           .to_string()))
            }
        }
    }

    pub fn to_secret_string(&self) -> Result<String> {
        match self.secret {
            Some(ref sk) => {
                Ok(format!("{}\n{}\n\n{}",
                           SECRET_SYM_KEY_VERSION,
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
        let secret_keyfile = mk_key_filename(path, self.name_with_rev(), SECRET_SYM_KEY_SUFFIX);
        debug!("secret sym keyfile = {}", secret_keyfile.display());

        write_keypair_files(None,
                            None,
                            Some(&secret_keyfile),
                            Some(self.to_secret_string()?))
    }

    fn get_secret_key(key_with_rev: &str, cache_key_path: &Path) -> Result<SymSecretKey> {
        let secret_keyfile = mk_key_filename(cache_key_path, key_with_rev, SECRET_SYM_KEY_SUFFIX);
        let bytes = read_key_bytes(&secret_keyfile)?;
        match SymSecretKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                Err(Error::CryptoError(format!("Can't read sym secret key \
                                                for {}",
                                               key_with_rev)))
            }
        }
    }

    /// Writes a sym key to the key cache from the contents of a string slice.
    ///
    /// The return is a `Result` of a `String` containing the key's name with revision.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempfile;
    ///
    /// use habitat_core::crypto::{keys::PairType,
    ///                            SymKey};
    /// use tempfile::Builder;
    ///
    /// let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    /// let content = "SYM-SEC-1
    /// beyonce-20160504220722
    ///
    /// RCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
    ///
    /// let (pair, pair_type) = SymKey::write_file_from_str(content, cache.path()).unwrap();
    /// assert_eq!(pair_type, PairType::Secret);
    /// assert_eq!(pair.name_with_rev(), "beyonce-20160504220722");
    /// assert!(cache.path()
    ///              .join("beyonce-20160504220722.sym.key")
    ///              .is_file());
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
        let mut lines = content.lines();
        match lines.next() {
            Some(val) => {
                if val != SECRET_SYM_KEY_VERSION {
                    return Err(Error::CryptoError(format!("Unsupported key version: {}", val)));
                }
            }
            None => {
                let msg = format!("write_sym_key_from_str:1 Malformed sym key string:\n({})",
                                  content);
                return Err(Error::CryptoError(msg));
            }
        };
        let name_with_rev = match lines.next() {
            Some(val) => val,
            None => {
                let msg = format!("write_sym_key_from_str:2 Malformed sym key string:\n({})",
                                  content);
                return Err(Error::CryptoError(msg));
            }
        };
        if lines.nth(1).is_none() {
            let msg = format!("write_sym_key_from_str:3 Malformed sym key string:\n({})",
                              content);
            return Err(Error::CryptoError(msg));
        };
        let secret_keyfile = mk_key_filename(cache_key_path.as_ref(),
                                             &name_with_rev,
                                             SECRET_SYM_KEY_SUFFIX);
        let tmpfile = {
            let mut t = secret_keyfile.clone();
            t.set_file_name(format!("{}.{}",
                                    &secret_keyfile.file_name().unwrap().to_str().unwrap(),
                                    &hex::encode(randombytes(6).as_slice())));
            TmpKeyfile { path: t }
        };

        debug!("Writing temp key file {}", tmpfile.path.display());
        write_keypair_files(None, None, Some(&tmpfile.path), Some(content.to_string()))?;

        if Path::new(&secret_keyfile).is_file() {
            let existing_hash = hash::hash_file(&secret_keyfile)?;
            let new_hash = hash::hash_file(&tmpfile.path)?;
            if existing_hash != new_hash {
                let msg = format!("Existing key file {} found but new version hash is different, \
                                   failing to write new file over existing. ({} = {}, {} = {})",
                                  secret_keyfile.display(),
                                  secret_keyfile.display(),
                                  existing_hash,
                                  tmpfile.path.display(),
                                  new_hash);
                return Err(Error::CryptoError(msg));
            } else {
                // Otherwise, hashes match and we can skip writing over the existing file
                debug!("New content hash matches existing file {} hash, removing temp key file \
                        {}.",
                       secret_keyfile.display(),
                       tmpfile.path.display());
                fs::remove_file(&tmpfile.path)?;
            }
        } else {
            debug!("Moving {} to {}",
                   tmpfile.path.display(),
                   secret_keyfile.display());
            fs::rename(&tmpfile.path, secret_keyfile)?;
        }

        // Now load and return the pair to ensure everything wrote out
        Ok((Self::get_pair_for(&name_with_rev, cache_key_path)?, PairType::Secret))
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
                SymKey};

    static VALID_KEY: &str = "ring-key-valid-20160504220722.sym.key";
    static VALID_NAME_WITH_REV: &str = "ring-key-valid-20160504220722";

    #[test]
    fn empty_struct() {
        let pair = SymKey::new("grohl".to_string(), "201604051449".to_string(), None, None);

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
    fn generated_ring_pair() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SymKey::generate_pair_for_ring("beyonce").unwrap();
        pair.to_pair_files(cache.path()).unwrap();

        assert_eq!(pair.name, "beyonce");
        assert!(pair.public().is_ok(),
                "Generated pair should have an empty public key");
        assert!(pair.secret().is_ok(),
                "Generated pair should have a secret key");
        assert!(cache.path()
                     .join(format!("{}.sym.key", pair.name_with_rev()))
                     .exists());
    }

    #[test]
    fn get_pairs_for() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pairs = SymKey::get_pairs_for("beyonce", cache.path()).unwrap();
        assert_eq!(pairs.len(), 0);

        SymKey::generate_pair_for_ring("beyonce").unwrap()
                                                 .to_pair_files(cache.path())
                                                 .unwrap();
        let pairs = SymKey::get_pairs_for("beyonce", cache.path()).unwrap();
        assert_eq!(pairs.len(), 1);

        match wait_until_ok(|| {
                  let rpair = SymKey::generate_pair_for_ring("beyonce")?;
                  rpair.to_pair_files(cache.path())?;
                  Ok(())
              }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };
        let pairs = SymKey::get_pairs_for("beyonce", cache.path()).unwrap();
        assert_eq!(pairs.len(), 2);

        // We should not include another named key in the count
        SymKey::generate_pair_for_ring("jayz").unwrap()
                                              .to_pair_files(cache.path())
                                              .unwrap();
        let pairs = SymKey::get_pairs_for("beyonce", cache.path()).unwrap();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn get_pair_for() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let p1 = SymKey::generate_pair_for_ring("beyonce").unwrap();
        p1.to_pair_files(cache.path()).unwrap();
        let p2 = match wait_until_ok(|| {
                  let rpath = SymKey::generate_pair_for_ring("beyonce")?;
                  rpath.to_pair_files(cache.path())?;
                  Ok(rpath)
              }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let p1_fetched = SymKey::get_pair_for(&p1.name_with_rev(), cache.path()).unwrap();
        assert_eq!(p1.name, p1_fetched.name);
        assert_eq!(p1.rev, p1_fetched.rev);
        let p2_fetched = SymKey::get_pair_for(&p2.name_with_rev(), cache.path()).unwrap();
        assert_eq!(p2.name, p2_fetched.name);
        assert_eq!(p2.rev, p2_fetched.rev);
    }

    #[test]
    #[should_panic(expected = "No secret keys found for name_with_rev")]
    fn get_pair_for_nonexistent() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SymKey::get_pair_for("nope-nope-20160405144901", cache.path()).unwrap();
    }

    #[test]
    fn get_latest_pair_for_single() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SymKey::generate_pair_for_ring("beyonce").unwrap();
        pair.to_pair_files(cache.path()).unwrap();

        let latest = SymKey::get_latest_pair_for("beyonce", cache.path()).unwrap();
        assert_eq!(latest.name, pair.name);
        assert_eq!(latest.rev, pair.rev);
    }

    #[test]
    fn get_latest_pair_for_multiple() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SymKey::generate_pair_for_ring("beyonce").unwrap()
                                                 .to_pair_files(cache.path())
                                                 .unwrap();
        let p2 = match wait_until_ok(|| {
                  let rpath = SymKey::generate_pair_for_ring("beyonce")?;
                  rpath.to_pair_files(cache.path())?;
                  Ok(rpath)
              }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let latest = SymKey::get_latest_pair_for("beyonce", cache.path()).unwrap();
        assert_eq!(latest.name, p2.name);
        assert_eq!(latest.rev, p2.rev);
    }

    #[test]
    #[should_panic(expected = "No revisions found for")]
    fn get_latest_pair_for_nonexistent() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SymKey::get_latest_pair_for("nope-nope", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "No public key exists for sym keys")]
    fn get_public_key_path() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SymKey::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

    #[test]
    fn get_secret_key_path() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)),
                 cache.path().join(VALID_KEY)).unwrap();

        let result = SymKey::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_KEY));
    }

    #[test]
    #[should_panic(expected = "No secret key found at")]
    fn get_secret_key_path_nonexistent() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SymKey::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

    #[test]
    fn encrypt_and_decrypt() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SymKey::generate_pair_for_ring("beyonce").unwrap();
        pair.to_pair_files(cache.path()).unwrap();

        let (nonce, ciphertext) = pair.encrypt(b"Ringonit").unwrap();
        let message = pair.decrypt(&nonce, &ciphertext).unwrap();
        assert_eq!(message, "Ringonit".to_string().into_bytes());
    }

    #[test]
    #[should_panic(expected = "Secret key is required but not present for")]
    fn encrypt_missing_secret_key() {
        let pair = SymKey::new("grohl".to_string(), "201604051449".to_string(), None, None);

        pair.encrypt(b"Not going to go well").unwrap();
    }

    #[test]
    #[should_panic(expected = "Secret key is required but not present for")]
    fn decrypt_missing_secret_key() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SymKey::generate_pair_for_ring("beyonce").unwrap();
        pair.to_pair_files(cache.path()).unwrap();
        let (nonce, ciphertext) = pair.encrypt(b"Ringonit").unwrap();

        let missing = SymKey::new("grohl".to_string(), "201604051449".to_string(), None, None);
        missing.decrypt(&nonce, &ciphertext).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid size of nonce")]
    fn decrypt_invalid_nonce_length() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SymKey::generate_pair_for_ring("beyonce").unwrap();
        pair.to_pair_files(cache.path()).unwrap();

        let (_, ciphertext) = pair.encrypt(b"Ringonit").unwrap();
        pair.decrypt(b"crazyinlove", &ciphertext).unwrap();
    }

    #[test]
    #[should_panic(expected = "Secret key and nonce could not decrypt ciphertext")]
    fn decrypt_invalid_ciphertext() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SymKey::generate_pair_for_ring("beyonce").unwrap();
        pair.to_pair_files(cache.path()).unwrap();

        let (nonce, _) = pair.encrypt(b"Ringonit").unwrap();
        pair.decrypt(&nonce, b"singleladies").unwrap();
    }

    #[test]
    fn write_file_from_str() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
        let new_key_file = cache.path().join(VALID_KEY);

        assert_eq!(new_key_file.is_file(), false);
        let (pair, pair_type) = SymKey::write_file_from_str(&content, cache.path()).unwrap();
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
    fn write_file_from_str_with_existing_identical() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
        let new_key_file = cache.path().join(VALID_KEY);

        // install the key into the cache
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &new_key_file).unwrap();

        let (pair, pair_type) = SymKey::write_file_from_str(&content, cache.path()).unwrap();
        assert_eq!(pair_type, PairType::Secret);
        assert_eq!(pair.name_with_rev(), VALID_NAME_WITH_REV);
        assert!(new_key_file.is_file());
    }

    #[test]
    #[should_panic(expected = "Unsupported key version")]
    fn write_file_from_str_unsupported_version() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string("keys/ring-key-invalid-version-20160504221247.sym.key");

        SymKey::write_file_from_str(&content, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_sym_key_from_str:1 Malformed sym key string")]
    fn write_file_from_str_missing_version() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SymKey::write_file_from_str("", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_sym_key_from_str:2 Malformed sym key string")]
    fn write_file_from_str_missing_name() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SymKey::write_file_from_str("SYM-SEC-1\n", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "write_sym_key_from_str:3 Malformed sym key string")]
    fn write_file_from_str_missing_key() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        SymKey::write_file_from_str("SYM-SEC-1\nim-in-trouble-123\n", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Existing key file")]
    fn write_file_from_str_key_exists_but_hashes_differ() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let key = fixture("keys/ring-key-valid-20160504220722.sym.key");
        fs::copy(key,
                 cache.path().join("ring-key-valid-20160504220722.sym.key")).unwrap();

        SymKey::write_file_from_str("SYM-SEC-1\nring-key-valid-20160504220722\n\nsomething",
                                    cache.path()).unwrap();
    }
}
