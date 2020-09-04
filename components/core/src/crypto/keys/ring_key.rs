use crate::{crypto::{keys::NamedRevision,
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

gen_key!(
    /// Symmetric secret used to optionally encrypt
    /// Supervisor-to-Supervisor traffic. All such Supervisors must
    /// have a copy of the same key (at the same revision) in order to
    /// send and receive messages from each other.
    RingKey,
         key_material: primitives::Key,
         file_format_version: SECRET_SYM_KEY_VERSION,
         file_extension: "sym.key",
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

impl RingKey {
    /// Generate a new `RingKey` for the given name. Creates a new
    /// key, but does not write anything to the filesystem.
    pub fn new(name: &str) -> Self {
        let named_revision = NamedRevision::new(name.to_string());
        let key = primitives::gen_key();
        RingKey { named_revision,
                  key }
    }

    /// Encrypts a sequence of bytes.
    ///
    /// The return is a tuple of `Vec<u8>`s, the first being a random
    /// nonce value and the second being the ciphertext. Both are
    /// needed to decrypt the message.
    pub fn encrypt(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let nonce = primitives::gen_nonce();
        (nonce.as_ref().to_vec(), primitives::seal(data, &nonce, &self.key))
    }

    /// Decrypts a ciphertext using a given nonce value.
    ///
    /// The returns the original unencrypted bytes.
    pub fn decrypt(&self, nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let nonce = primitives::Nonce::from_slice(&nonce).ok_or_else(|| {
                                                             Error::CryptoError("Invalid size of \
                                                                                 nonce"
                                                                                       .to_string())
                                                         })?;

        primitives::open(ciphertext, &nonce, &self.key).map_err(|_| {
            Error::CryptoError("Secret key and nonce could not decrypt ciphertext".to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::test_support::{fixture_as_string,
                                      fixture_key};

    mod from_str {
        use super::*;

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

    #[test]
    fn decryption() {
        let key: RingKey = fixture_key("keys/ring-key-valid-20160504220722.sym.key");

        let nonce = [175u8, 221u8, 237u8, 184u8, 68u8, 112u8, 40u8, 80u8, 11u8, 173u8, 215u8,
                     154u8, 129u8, 39u8, 146u8, 10u8, 51u8, 143u8, 150u8, 71u8, 146u8, 97u8, 70u8,
                     76u8];
        let ciphertext = [161u8, 106u8, 124u8, 7u8, 144u8, 46u8, 9u8, 29u8, 90u8, 176u8, 207u8,
                          52u8, 61u8, 3u8, 209u8, 41u8, 144u8, 32u8, 72u8, 245u8, 159u8, 143u8,
                          192u8, 36u8, 5u8, 235u8, 241u8, 98u8, 231u8, 21u8];

        let decrypted_message = key.decrypt(&nonce, &ciphertext).unwrap();
        let decrypted_message = std::str::from_utf8(&decrypted_message).unwrap();
        assert_eq!(decrypted_message, "This is a test");
    }

    #[test]
    fn encryption_roundtrip() {
        let key = RingKey::new("beyonce");
        let original_message = "Ringonit".to_string().into_bytes();
        let (nonce, ciphertext) = key.encrypt(&original_message);
        let decrypted_message = key.decrypt(&nonce, &ciphertext).unwrap();
        let decrypted_message = std::str::from_utf8(&decrypted_message).unwrap();

        assert_eq!(decrypted_message, "Ringonit");
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
