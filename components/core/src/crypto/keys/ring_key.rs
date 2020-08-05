use crate::{crypto::{keys::{Key,
                            KeyRevision,
                            NamedRevision},
                     SECRET_SYM_KEY_VERSION},
            error::{Error,
                    Result}};
use sodiumoxide::crypto::secretbox::{self,
                                     Key as SymSecretKey};
use std::{fmt,
          path::PathBuf};

from_slice_impl_for_sodiumoxide_key!(SymSecretKey);

#[derive(Clone, PartialEq)]
pub struct RingKey {
    named_revision: NamedRevision,
    key:            SymSecretKey,
    path:           PathBuf, /* might not need this much longer; we
                              * can get it from the named revision */
}

impl Key for RingKey {
    type Crypto = SymSecretKey;

    // SECRET_SYM_KEY_SUFFIX;
    const EXTENSION: &'static str = "sym.key";
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = SECRET_SYM_KEY_VERSION;

    fn key(&self) -> &SymSecretKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

from_str_impl_for_key!(RingKey);

try_from_path_buf_impl_for_key!(RingKey);

as_ref_path_impl_for_key!(RingKey);
impl fmt::Debug for RingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "RingKey") }
}

impl RingKey {
    /// Generate a new `RingKey` for the given name. Creates a new
    /// key, but does not write anything to the filesystem.
    pub fn new(name: &str) -> Self {
        let revision = KeyRevision::new();
        let secret_key = secretbox::gen_key();
        RingKey::from_raw(name.to_string(), revision, secret_key)
    }

    /// Create a RingKey from raw components to e.g., simulate when
    /// a requested key doesn't exist on disk.
    ///
    /// Also currently used in the KeyCache; may not be required for
    /// much longer.
    pub(crate) fn from_raw(name: String, revision: KeyRevision, key: SymSecretKey) -> RingKey {
        let named_revision = NamedRevision::new(name, revision);
        let path = named_revision.filename::<Self>();
        Self { named_revision,
               key,
               path }
    }

    // Simple helper to deal with the indirection to the inner
    // KeyPair struct. Not ultimately sure if this should be kept.
    pub fn name_with_rev(&self) -> String { self.named_revision.to_string() }

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
        let nonce = secretbox::gen_nonce();
        Ok((nonce.as_ref().to_vec(), secretbox::seal(data, &nonce, &self.key)))
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
        let nonce = match secretbox::Nonce::from_slice(&nonce) {
            Some(n) => n,
            None => return Err(Error::CryptoError("Invalid size of nonce".to_string())),
        };
        match secretbox::open(ciphertext, &nonce, &self.key) {
            Ok(msg) => Ok(msg),
            Err(_) => {
                Err(Error::CryptoError("Secret key and nonce could not \
                                        decrypt ciphertext"
                                                           .to_string()))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{super::super::test_support::*,
                *};

    impl RingKey {
        pub fn revision(&self) -> &KeyRevision { &self.named_revision.revision }

        pub fn name(&self) -> &String { &self.named_revision.name }

        // TODO (CM): this should probably be renamed; there's no
        // public key to distinguish it from.
        pub fn secret(&self) -> crate::error::Result<&SymSecretKey> { Ok(&self.key) }
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

            assert_eq!(content, key.to_key_string());
        }
    }

    mod as_ref_path {
        use super::*;
        use std::path::Path;

        #[test]
        fn produces_correct_filename() {
            let content = fixture_as_string("keys/ring-key-valid-20160504220722.sym.key");
            let key = content.parse::<RingKey>().unwrap();

            assert_eq!(key.as_ref(),
                       Path::new("ring-key-valid-20160504220722.sym.key"));
        }
    }

    #[test]
    fn generated_ring_pair() {
        let (cache, dir) = new_cache();
        let key = RingKey::new("beyonce");
        cache.write_key(&key).unwrap();

        assert_eq!(key.name(), "beyonce");
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
