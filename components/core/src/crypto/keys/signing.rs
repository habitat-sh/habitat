use crate::{crypto::{hash,
                     keys::{Key,
                            KeyRevision,
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
    let revision = KeyRevision::new();
    let named_revision = NamedRevision::new(origin_name.to_string(), revision);
    let (pk, sk) = primitives::gen_keypair();

    let public = PublicOriginSigningKey { named_revision: named_revision.clone(),
                                          key:            pk, };
    let secret = SecretOriginSigningKey { named_revision,
                                          key: sk };
    (public, secret)
}

////////////////////////////////////////////////////////////////////////

gen_key!(PublicOriginSigningKey,
         key_material: primitives::PublicKey,
         file_format_version: PUBLIC_SIG_KEY_VERSION,
         file_extension: "pub",
         file_permissions: crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);

impl PublicOriginSigningKey {
    /// Accept a signature and the bytes for the signed content to be
    /// verified. Returns the named revision of the key as well as the
    /// computed hash.
    pub fn verify(&self,
                  signature: &[u8],
                  content: &mut dyn Read)
                  -> Result<(NamedRevision, String)> {
        // TODO (CM): we should always have a public key here, by definition.
        let expected_hash = match primitives::verify(signature, &self.key) {
            Ok(signed_data) => {
                String::from_utf8(signed_data).map_err(|_| {
                                                  Error::CryptoError("Error parsing artifact \
                                                                      signature"
                                                                                .to_string())
                                              })?
            }
            Err(_) => return Err(Error::CryptoError("Verification failed".to_string())),
        };

        let computed_hash = hash::hash_reader(content)?;
        if computed_hash == expected_hash {
            Ok((self.named_revision().clone(), expected_hash))
        } else {
            let msg = format!("Habitat artifact is invalid, hashes don't match (expected: {}, \
                               computed: {})",
                              expected_hash, computed_hash);
            Err(Error::CryptoError(msg))
        }
    }
}

////////////////////////////////////////////////////////////////////////

gen_key!(SecretOriginSigningKey,
         key_material: primitives::SecretKey,
         file_format_version: SECRET_SIG_KEY_VERSION,
         file_extension: "sig.key",
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

impl SecretOriginSigningKey {
    /// Takes the contents of the given file and returns a signature
    /// based on this key.
    pub fn sign<P>(&self, path: P) -> Result<Vec<u8>>
        where P: AsRef<Path>
    {
        let hash = hash::hash_file(&path)?;
        debug!("File hash for {} = {}", path.as_ref().display(), &hash);
        Ok(primitives::sign(&hash.as_bytes(), &self.key))
    }
}
