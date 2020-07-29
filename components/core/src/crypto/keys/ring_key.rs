use crate::{crypto::{keys::{KeyCache,
                            KeyPair,
                            KeyRevision,
                            NamedRevision,
                            Permissioned,
                            ToKeyString},
                     SECRET_SYM_KEY_SUFFIX,
                     SECRET_SYM_KEY_VERSION},
            error::{Error,
                    Result},
            fs::Permissions};
use sodiumoxide::crypto::secretbox::{self,
                                     Key as SymSecretKey};
use std::{fmt,
          path::{Path,
                 PathBuf},
          str::FromStr};

#[derive(Clone, PartialEq)]
pub struct RingKey {
    inner: KeyPair<(), SymSecretKey>,
    path:  PathBuf,
}

// TODO (CM): Incorporate the name/revision of the key?
impl fmt::Debug for RingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "RingKey") }
}

impl RingKey {
    /// Generate a new `RingKey` for the given name. Creates a new
    /// key, but does not write anything to the filesystem.
    pub fn new(name: &str) -> Self {
        let revision = KeyRevision::new();
        let secret_key = secretbox::gen_key();
        RingKey::from_raw(name.to_string(), revision, Some(secret_key))
    }

    /// Create a RingKey from raw components to e.g., simulate when
    /// a requested key doesn't exist on disk.
    ///
    /// Also currently used in the KeyCache; may not be required for
    /// much longer.
    pub(crate) fn from_raw(name: String,
                           revision: KeyRevision,
                           secret: Option<SymSecretKey>)
                           -> RingKey {
        let inner = KeyPair::new(name.to_string(), revision, Some(()), secret);
        let path = Path::new(&inner.name_with_rev()).with_extension(SECRET_SYM_KEY_SUFFIX);

        RingKey { inner, path }
    }

    // Simple helper to deal with the indirection to the inner
    // KeyPair struct. Not ultimately sure if this should be kept.
    pub fn name_with_rev(&self) -> String { self.inner.name_with_rev() }

    /// Encrypts a byte slice of data using a given `RingKey`.
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
    /// use habitat_core::crypto::RingKey;
    /// use tempfile::Builder;
    ///
    /// let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    /// let ring_key = RingKey::new("beyonce");
    ///
    /// let (nonce, ciphertext) = ring_key.encrypt("Guess who?".as_bytes()).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// * If the secret key component of the `RingKey` is not present
    pub fn encrypt(&self, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let key = self.inner.secret()?;
        let nonce = secretbox::gen_nonce();
        Ok((nonce.as_ref().to_vec(), secretbox::seal(data, &nonce, &key)))
    }

    /// Decrypts a byte slice of ciphertext using a given nonce value and a `RingKey`.
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
    /// use habitat_core::crypto::RingKey;
    /// use tempfile::Builder;
    ///
    /// let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    /// let ring_key = RingKey::new("beyonce");
    /// let (nonce, ciphertext) = ring_key.encrypt("Guess who?".as_bytes()).unwrap();
    ///
    /// let message = ring_key.decrypt(&nonce, &ciphertext).unwrap();
    /// assert_eq!(message, "Guess who?".to_string().into_bytes());
    /// ```
    ///
    /// # Errors
    ///
    /// * If the secret key component of the `RingKey` is not present
    /// * If the size of the provided nonce data is not the required size
    /// * If the ciphertext was not decryptable given the nonce and symmetric key
    pub fn decrypt(&self, nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let key = self.inner.secret()?;
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
    ///                            RingKey};
    /// use tempfile::Builder;
    ///
    /// let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
    /// let content = "SYM-SEC-1
    /// beyonce-20160504220722
    ///
    /// RCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
    ///
    /// let pair = RingKey::write_file_from_str(content, cache.path()).unwrap();
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
    pub fn write_file_from_str<P>(content: &str, cache_key_path: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let parsed_key = content.parse::<RingKey>()?;
        let cache: KeyCache = cache_key_path.as_ref().into();
        cache.maybe_write_key(&parsed_key)?;
        Ok(parsed_key)
    }
}

impl AsRef<Path> for RingKey {
    fn as_ref(&self) -> &Path { &self.path }
}

impl FromStr for RingKey {
    type Err = Error;

    fn from_str(content: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = content.lines();

        match lines.next() {
            Some(val) => {
                if val != SECRET_SYM_KEY_VERSION {
                    return Err(Error::CryptoError(format!("Unsupported key version: {}", val)));
                }
            }
            None => {
                let msg = format!("Malformed ring key string:\n({})", content);
                return Err(Error::CryptoError(msg));
            }
        };

        let named_revision = match lines.next() {
            Some(val) => val.parse::<NamedRevision>()?,
            None => {
                let msg = format!("Malformed ring key string:\n({})", content);
                return Err(Error::CryptoError(msg));
            }
        };

        let key = match lines.nth(1) {
            Some(line) => {
                let key_bytes = base64::decode(line.trim()).map_err(|_| {
                                    Error::CryptoError(format!("Malformed ring key string \
                                                                (invalid base64 key \
                                                                material):\n({})",
                                                               content))
                                })?;
                match SymSecretKey::from_slice(&key_bytes) {
                    Some(sk) => sk,
                    None => {
                        return Err(Error::CryptoError(format!("Can't read ring key material \
                                                               for {}",
                                                              named_revision)));
                    }
                }
            }
            None => {
                let msg = format!("Malformed ring key string:\n({})", content);
                return Err(Error::CryptoError(msg));
            }
        };

        let (name, revision) = named_revision.into();
        Ok(RingKey::from_raw(name, revision, Some(key)))
    }
}

impl ToKeyString for RingKey {
    fn to_key_string(&self) -> Result<String> {
        match self.inner.secret {
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
}

impl Permissioned for RingKey {
    /// Ring keys are always private and deserved to be locked-down as such.
    const PERMISSIONS: Permissions = crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS;
}

#[cfg(test)]
mod test {
    use super::{super::super::test_support::*,
                *};
    use std::{fs::{self,
                   File},
              io::Read};
    use tempfile::Builder;

    static VALID_KEY: &str = "ring-key-valid-20160504220722.sym.key";
    static VALID_NAME_WITH_REV: &str = "ring-key-valid-20160504220722";

    impl RingKey {
        pub fn revision(&self) -> &KeyRevision { &self.inner.revision }

        pub fn name(&self) -> &String { &self.inner.name }

        // TODO (CM): This really shouldn't exist
        pub fn public(&self) -> crate::error::Result<&()> { self.inner.public() }

        // TODO (CM): this should probably be renamed; there's no
        // public key to distinguish it from.
        pub fn secret(&self) -> crate::error::Result<&SymSecretKey> { self.inner.secret() }
    }

    mod from_str {
        use super::*;

        #[test]
        fn can_parse() {
            let content = fixture_as_string("keys/ring-key-valid-20160504220722.sym.key");
            let key = content.parse::<RingKey>().unwrap();
            assert_eq!(key.name(), "ring-key-valid");
            assert_eq!(key.revision(), &KeyRevision::unchecked("20160504220722"));
            // TODO (CM): assert secret bytes
        }

        #[test]
        fn fails_to_parse_invalid_key() {
            let content = fixture_as_string("keys/ring-key-invalid-version-20160504221247.sym.key");
            assert!(content.parse::<RingKey>().is_err());
        }
    }

    mod to_key_string {
        use super::*;

        #[test]
        fn can_write_valid_key_string() {
            let content = fixture_as_string("keys/ring-key-valid-20160504220722.sym.key");
            let key = content.parse::<RingKey>().unwrap();

            assert_eq!(content, key.to_key_string().unwrap());
        }
    }

    mod as_ref_path {
        use super::*;

        #[test]
        fn produces_correct_filename() {
            let content = fixture_as_string("keys/ring-key-valid-20160504220722.sym.key");
            let key = content.parse::<RingKey>().unwrap();

            assert_eq!(key.as_ref(),
                       Path::new("ring-key-valid-20160504220722.sym.key"));
        }

        #[test]
        fn works_without_secret_too() {
            let key = RingKey::from_raw("foo".to_string(),
                                        KeyRevision::unchecked("20200729160923"),
                                        None);
            assert_eq!(key.as_ref(), Path::new("foo-20200729160923.sym.key"));
        }
    }

    // #[test]
    // fn empty_struct() {
    //     let pair = RingKey::new("grohl".to_string(),
    //                             KeyRevision::unchecked("201604051449"),
    //                             None,
    //                             None);

    //     assert_eq!(pair.name(), "grohl");
    //     assert_eq!(pair.revision(), KeyRevision::unchecked("201604051449"));
    //     assert_eq!(pair.name_with_rev(), "grohl-201604051449");

    //     assert_eq!(pair.public, None);
    //     assert!(pair.public().is_err(),
    //             "Empty pair should not have a public key");
    //     assert_eq!(pair.secret, None);
    //     assert!(pair.secret().is_err(),
    //             "Empty pair should not have a secret key");
    // }

    #[test]
    fn generated_ring_pair() {
        let (cache, dir) = new_cache();
        let key = RingKey::new("beyonce");
        cache.write_ring_key(&key).unwrap();

        assert_eq!(key.name(), "beyonce");
        assert!(key.public().is_ok(),
                "Generated pair should have an empty public key");
        assert!(key.secret().is_ok(),
                "Generated pair should have a secret key");
        assert!(dir.path()
                   .join(format!("{}.sym.key", key.name_with_rev()))
                   .exists());
    }

    #[test]
    fn encrypt_and_decrypt() {
        let key = RingKey::new("beyonce");
        let (nonce, ciphertext) = key.encrypt(b"Ringonit").unwrap();
        let message = key.decrypt(&nonce, &ciphertext).unwrap();
        assert_eq!(message, "Ringonit".to_string().into_bytes());
    }

    #[test]
    #[should_panic(expected = "Secret key is required but not present for")]
    fn encrypt_missing_secret_key() {
        let key = RingKey::from_raw("grohl".to_string(),
                                    KeyRevision::unchecked("201604051449"),
                                    None);

        key.encrypt(b"Not going to go well").unwrap();
    }

    #[test]
    #[should_panic(expected = "Secret key is required but not present for")]
    fn decrypt_missing_secret_key() {
        let key = RingKey::new("beyonce");
        let (nonce, ciphertext) = key.encrypt(b"Ringonit").unwrap();

        let missing = RingKey::from_raw("grohl".to_string(),
                                        KeyRevision::unchecked("201604051449"),
                                        None);
        missing.decrypt(&nonce, &ciphertext).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid size of nonce")]
    fn decrypt_invalid_nonce_length() {
        let key = RingKey::new("beyonce");
        let (_, ciphertext) = key.encrypt(b"Ringonit").unwrap();
        key.decrypt(b"crazyinlove", &ciphertext).unwrap();
    }

    #[test]
    #[should_panic(expected = "Secret key and nonce could not decrypt ciphertext")]
    fn decrypt_invalid_ciphertext() {
        let key = RingKey::new("beyonce");
        let (nonce, _) = key.encrypt(b"Ringonit").unwrap();
        key.decrypt(&nonce, b"singleladies").unwrap();
    }

    #[test]
    fn write_file_from_str() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
        let new_key_file = cache.path().join(VALID_KEY);

        assert_eq!(new_key_file.is_file(), false);
        let key = RingKey::write_file_from_str(&content, cache.path()).unwrap();
        assert_eq!(key.name_with_rev(), VALID_NAME_WITH_REV);
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

        let key = RingKey::write_file_from_str(&content, cache.path()).unwrap();
        assert_eq!(key.name_with_rev(), VALID_NAME_WITH_REV);
        assert!(new_key_file.is_file());
    }

    #[test]
    #[should_panic(expected = "Unsupported key version")]
    fn write_file_from_str_unsupported_version() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let content = fixture_as_string("keys/ring-key-invalid-version-20160504221247.sym.key");

        RingKey::write_file_from_str(&content, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Malformed ring key string")]
    fn write_file_from_str_missing_version() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        RingKey::write_file_from_str("", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Malformed ring key string")]
    fn write_file_from_str_missing_name() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        RingKey::write_file_from_str("SYM-SEC-1\n", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot parse named revision")]
    fn write_file_from_str_missing_key() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        RingKey::write_file_from_str("SYM-SEC-1\nim-in-trouble-123\n", cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Existing key file")]
    fn write_file_from_str_key_exists_but_hashes_differ() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let key = fixture("keys/ring-key-valid-20160504220722.sym.key");
        let old_content = fs::read_to_string(&key).unwrap();
        fs::copy(key,
                 cache.path().join("ring-key-valid-20160504220722.sym.key")).unwrap();

        #[rustfmt::skip]
        let new_content = "SYM-SEC-1\nring-key-valid-20160504220722\n\nkA+c03Ly5qEoOZIjJ5zCD2vHI05pAW59PfCOb8thmZw=";

        assert_ne!(old_content, new_content);

        // this should fail
        RingKey::write_file_from_str(new_content, cache.path()).unwrap();
    }
}
