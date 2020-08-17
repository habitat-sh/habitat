use crate::{crypto::{hash,
                     keys::{Key,
                            NamedRevision},
                     PUBLIC_SIG_KEY_VERSION,
                     SECRET_SIG_KEY_VERSION},
            error::{Error,
                    Result},
            fs::Permissions};
use std::{io::Read,
          path::Path};

/// Private module to re-export the various sodiumoxide concepts we
/// use, to keep them all consolidated and abstracted.
mod primitives {
    pub use sodiumoxide::crypto::sign::{ed25519::{PublicKey,
                                                  SecretKey},
                                        gen_keypair,
                                        sign,
                                        verify};
}

/// Given the name of an origin, generate a new signing key pair.
///
/// The resulting keys will need to be saved to a cache in order to
/// persist.
pub fn generate_signing_key_pair(origin_name: &str)
                                 -> (PublicOriginSigningKey, SecretOriginSigningKey) {
    let named_revision = NamedRevision::new(origin_name.to_string());
    let (pk, sk) = primitives::gen_keypair();

    let public = PublicOriginSigningKey { named_revision: named_revision.clone(),
                                          key:            pk, };
    let secret = SecretOriginSigningKey { named_revision,
                                          key: sk };
    (public, secret)
}

////////////////////////////////////////////////////////////////////////

gen_key!(
    /// Public key used to verify signatures of Habitat artifacts signed with
    /// a `SecretOriginSigningKey`.
    PublicOriginSigningKey,
         key_material: primitives::PublicKey,
         file_format_version: PUBLIC_SIG_KEY_VERSION,
         file_extension: "pub",
         file_permissions: crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);

impl PublicOriginSigningKey {
    /// Accept a signed Blake2b hash, along with the bytes for the
    /// content that was supposedly hashed-and-signed, in order to
    /// verify that is true.
    ///
    /// Returns the named revision of the key as well as the verified,
    /// hex-encoded Blake2b hash of the contents.
    // TODO (CM): Create a Blake2B Hash type and return that rather
    // than String
    pub fn verify(&self,
                  signed_hash: &[u8],
                  content: &mut dyn Read)
                  -> Result<(NamedRevision, String)> {
        let expected_hex_encoded_hash = match primitives::verify(signed_hash, &self.key) {
            Ok(hash_bytes) => String::from_utf8(hash_bytes).map_err(|_| {
                                  Error::CryptoError("Error parsing artifact hash".to_string())
                              })?,
            Err(_) => return Err(Error::CryptoError("Verification failed".to_string())),
        };

        let computed_hex_encoded_hash = hash::hash_reader(content)?;

        if computed_hex_encoded_hash == expected_hex_encoded_hash {
            Ok((self.named_revision().clone(), expected_hex_encoded_hash))
        } else {
            let msg = format!("Habitat artifact is invalid, hashes don't match (expected: {}, \
                               computed: {})",
                              expected_hex_encoded_hash, computed_hex_encoded_hash);
            Err(Error::CryptoError(msg))
        }
    }
}

////////////////////////////////////////////////////////////////////////

gen_key!(
    /// Key used to sign the content hashes of Habitat artifacts.
    SecretOriginSigningKey,
         key_material: primitives::SecretKey,
         file_format_version: SECRET_SIG_KEY_VERSION,
         file_extension: "sig.key",
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

impl SecretOriginSigningKey {
    /// Takes the contents of the given file and returns the signed,
    /// hex-encoded Blake2b hash of the contents.
    ///
    /// NOTE: The output is *not* a detached signature; the signed
    /// content (the content hash) is recoverable from the output, as
    /// intended.
    pub fn sign<P>(&self, path: P) -> Result<Vec<u8>>
        where P: AsRef<Path>
    {
        let hex_encoded_hash = hash::hash_file(&path)?;
        Ok(primitives::sign(&hex_encoded_hash.as_bytes(), &self.key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{keys::KeyRevision,
                        test_support::{fixture,
                                       fixture_key}};
    use std::{fs::File,
              io::BufReader};

    /// The hash of the contents of the `tests/fixtures/signme.dat`
    /// file, signed by
    /// `tests/fixtures/keys/origin-key-valid-20160509190508.sig.key`.
    const SIGNED_SIGNME_DAT_BLAKE2B_HASH: [u8; 128] =
        [148u8, 34u8, 226u8, 235u8, 2u8, 136u8, 218u8, 135u8, 130u8, 241u8, 129u8, 134u8, 193u8,
         206u8, 3u8, 15u8, 158u8, 99u8, 68u8, 169u8, 139u8, 38u8, 13u8, 140u8, 120u8, 92u8, 152u8,
         143u8, 97u8, 135u8, 22u8, 233u8, 20u8, 243u8, 48u8, 63u8, 59u8, 82u8, 26u8, 51u8, 53u8,
         63u8, 5u8, 214u8, 166u8, 231u8, 113u8, 123u8, 241u8, 33u8, 25u8, 227u8, 91u8, 201u8,
         76u8, 48u8, 199u8, 214u8, 183u8, 110u8, 173u8, 161u8, 150u8, 12u8, 50u8, 48u8, 53u8,
         57u8, 48u8, 97u8, 53u8, 50u8, 99u8, 52u8, 102u8, 48u8, 48u8, 53u8, 56u8, 56u8, 99u8,
         53u8, 48u8, 48u8, 51u8, 50u8, 56u8, 98u8, 49u8, 54u8, 100u8, 52u8, 54u8, 54u8, 99u8,
         57u8, 56u8, 50u8, 97u8, 50u8, 54u8, 102u8, 97u8, 98u8, 97u8, 97u8, 53u8, 102u8, 97u8,
         52u8, 100u8, 99u8, 99u8, 56u8, 51u8, 48u8, 53u8, 50u8, 100u8, 100u8, 48u8, 97u8, 56u8,
         52u8, 102u8, 50u8, 51u8, 51u8];

    /// The hex-encoded Blake2b hash of the contents of
    /// `tests/fixtures/signme.dat`. Case is significant.
    const SIGNME_DAT_BLAKE2B_HASH: &str =
        "20590a52c4f00588c500328b16d466c982a26fabaa5fa4dcc83052dd0a84f233";

    #[test]
    fn signing() {
        let key: SecretOriginSigningKey =
            fixture_key("keys/origin-key-valid-20160509190508.sig.key");
        let file_to_sign = fixture("signme.dat");
        let signed_message = key.sign(&file_to_sign).unwrap();
        let expected = SIGNED_SIGNME_DAT_BLAKE2B_HASH.to_vec();

        assert_eq!(signed_message.len(), expected.len());
        for (i, (actual, expected)) in signed_message.iter().zip(expected.iter()).enumerate() {
            assert_eq!(actual, expected,
                       "Signed messages differ at byte index {}; expected '{}' but got '{}'",
                       i, expected, actual);
        }
    }

    #[test]
    fn verification() {
        let key: PublicOriginSigningKey = fixture_key("keys/origin-key-valid-20160509190508.pub");

        let f = File::open(fixture("signme.dat")).unwrap();
        let mut reader = BufReader::new(f);

        let (named_revision, file_blake2b_hash) = key.verify(&SIGNED_SIGNME_DAT_BLAKE2B_HASH,
                                                             &mut reader)
                                                     .unwrap();

        assert_eq!(named_revision.name(), "origin-key-valid");
        assert_eq!(named_revision.revision(),
                   &KeyRevision::unchecked("20160509190508"));

        assert_eq!(file_blake2b_hash, SIGNME_DAT_BLAKE2B_HASH);
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let sk: SecretOriginSigningKey =
            fixture_key("keys/origin-key-valid-20160509190508.sig.key");
        let pk: PublicOriginSigningKey = fixture_key("keys/origin-key-valid-20160509190508.pub");

        let file_to_sign = fixture("signme.dat");
        let signed_message = sk.sign(&file_to_sign).unwrap();

        let f = File::open(&file_to_sign).unwrap();
        let mut reader = BufReader::new(f);

        let expected_hash = hash::hash_file(&file_to_sign).unwrap();

        let (named_revision, verified_hash) = pk.verify(&signed_message, &mut reader).unwrap();

        assert_eq!(&named_revision, sk.named_revision());
        assert_eq!(expected_hash, verified_hash);
    }
}
