use crate::{crypto::{keys::{KeyExtension,
                            KeyMaterial,
                            KeyPair,
                            KeyRevision,
                            ToKeyString},
                     SECRET_SYM_KEY_SUFFIX,
                     SECRET_SYM_KEY_VERSION},
            error::{Error,
                    Result}};
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

// Ring keys are always private and deserved to be locked-down as such.
secret_permissions!(RingKey);

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
}

impl AsRef<Path> for RingKey {
    fn as_ref(&self) -> &Path { &self.path }
}

from_str_impl_for_key!(RingKey, SymSecretKey, SECRET_SYM_KEY_VERSION);

impl KeyMaterial<SymSecretKey> for RingKey {
    fn key_material(&self) -> Option<&SymSecretKey> { self.inner.secret.as_ref() }
}

impl KeyExtension for RingKey {
    const EXTENSION: &'static str = "sym.key"; // SECRET_SYM_KEY_SUFFIX;
}

to_key_string_impl_for_key!(RingKey, SECRET_SYM_KEY_VERSION);
try_from_path_buf_impl_for_key!(RingKey);

#[cfg(test)]
mod test {
    use super::{super::super::test_support::*,
                *};

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

        #[test]
        #[should_panic(expected = "Missing key version")]
        fn fails_to_parse_empty_string() { "".parse::<RingKey>().unwrap(); }

        #[test]
        #[should_panic(expected = "Missing name+revision")]
        fn fails_to_parse_only_header() { "SYM-SEC-1\n".parse::<RingKey>().unwrap(); }

        #[test]
        #[should_panic(expected = "Cannot parse named revision")]
        fn fails_to_parse_bogus_revision() {
            "SYM-SEC-1\nim-in-trouble-123\n".parse::<RingKey>().unwrap();
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
}
