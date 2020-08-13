use crate::{crypto::{keys::{Key,
                            KeyRevision,
                            NamedRevision},
                     SECRET_SYM_KEY_VERSION},
            error::{Error,
                    Result},
            fs::Permissions};

/// Private module to re-export the various sodiumoxide concepts we
/// use, to keep them all consolidated and abstracted.
mod primitives {
    pub use sodiumoxide::crypto::secretbox::{self,
                                             gen_key,
                                             gen_nonce,
                                             open,
                                             seal,
                                             Key,
                                             Nonce};
}

gen_key!(RingKey,
         key_material: primitives::Key,
         file_format_version: SECRET_SYM_KEY_VERSION,
         file_extension: "sym.key",
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

impl RingKey {
    /// Generate a new `RingKey` for the given name. Creates a new
    /// key, but does not write anything to the filesystem.
    pub fn new(name: &str) -> Self {
        let revision = KeyRevision::new();
        let named_revision = NamedRevision::new(name.to_string(), revision);
        let key = primitives::gen_key();
        RingKey { named_revision,
                  key }
    }

    /// Encrypts a byte slice of data using a given `RingKey`.
    ///
    /// The return is a tuple of `Vec<u8>` structs, the first being the random nonce
    /// value and the second being the ciphertext.
    pub fn encrypt(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let nonce = primitives::gen_nonce();
        (nonce.as_ref().to_vec(), primitives::seal(data, &nonce, &self.key))
    }

    /// Decrypts a byte slice of ciphertext using a given nonce value and a `RingKey`.
    ///
    /// The return is a `Result` of a byte vector containing the original, unencrypted data.
    pub fn decrypt(&self, nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let nonce = match primitives::Nonce::from_slice(&nonce) {
            Some(n) => n,
            None => return Err(Error::CryptoError("Invalid size of nonce".to_string())),
        };
        match primitives::open(ciphertext, &nonce, &self.key) {
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
        pub fn secret(&self) -> crate::error::Result<&primitives::Key> { Ok(&self.key) }
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
        use crate::crypto::keys::KeyFile;

        #[test]
        fn can_write_valid_key_string() {
            let content = fixture_as_string("keys/ring-key-valid-20160504220722.sym.key");
            let key = content.parse::<RingKey>().unwrap();

            assert_eq!(content, key.to_key_string());
        }
    }

    mod as_ref_path {
        use super::*;
        use crate::crypto::keys::KeyFile;
        use std::path::Path;

        #[test]
        fn produces_correct_filename() {
            let content = fixture_as_string("keys/ring-key-valid-20160504220722.sym.key");
            let key = content.parse::<RingKey>().unwrap();

            assert_eq!(key.own_filename(),
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
                   .join(format!("{}.sym.key", key.named_revision()))
                   .exists());
    }

    #[test]
    fn encrypt_and_decrypt() {
        let key = RingKey::new("beyonce");
        let (nonce, ciphertext) = key.encrypt(b"Ringonit");
        let message = key.decrypt(&nonce, &ciphertext).unwrap();
        assert_eq!(message, "Ringonit".to_string().into_bytes());
    }

    #[test]
    #[should_panic(expected = "Invalid size of nonce")]
    fn decrypt_invalid_nonce_length() {
        let key = RingKey::new("beyonce");
        let (_, ciphertext) = key.encrypt(b"Ringonit");
        key.decrypt(b"crazyinlove", &ciphertext).unwrap();
    }

    #[test]
    #[should_panic(expected = "Secret key and nonce could not decrypt ciphertext")]
    fn decrypt_invalid_ciphertext() {
        let key = RingKey::new("beyonce");
        let (nonce, _) = key.encrypt(b"Ringonit");
        key.decrypt(&nonce, b"singleladies").unwrap();
    }
}
