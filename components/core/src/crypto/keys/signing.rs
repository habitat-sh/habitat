use crate::{crypto::{hash,
                     keys::{Key,
                            KeyRevision,
                            NamedRevision},
                     PUBLIC_SIG_KEY_VERSION,
                     SECRET_SIG_KEY_VERSION},
            error::{Error,
                    Result}};
use std::{io::Read,
          path::{Path,
                 PathBuf}};

/// Private module to re-export the various sodiumoxide concepts we
/// use, to keep them all consolidated and abstracted.
mod primitives {
    pub use sodiumoxide::crypto::sign::{ed25519::{PublicKey,
                                                  SecretKey},
                                        gen_keypair,
                                        sign,
                                        verify};

    from_slice_impl_for_sodiumoxide_key!(PublicKey);
    from_slice_impl_for_sodiumoxide_key!(SecretKey);
}

/// Given the name of an origin, generate a new signing key pair.
///
/// The resulting keys will need to be saved to a cache in order to
/// persist.
pub fn generate_signing_key_pair(origin_name: &str)
                                 -> (PublicOriginSigningKey, SecretOriginSigningKey) {
    let revision = KeyRevision::new();
    let (pk, sk) = primitives::gen_keypair();

    let public = PublicOriginSigningKey::from_raw(origin_name.to_string(), revision.clone(), pk);
    let secret = SecretOriginSigningKey::from_raw(origin_name.to_string(), revision.clone(), sk);
    (public, secret)
}

////////////////////////////////////////////////////////////////////////

pub struct PublicOriginSigningKey {
    named_revision: NamedRevision,
    key:            primitives::PublicKey,
    path:           PathBuf,
}

impl Key for PublicOriginSigningKey {
    type Crypto = primitives::PublicKey;

    const EXTENSION: &'static str = "pub";
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = PUBLIC_SIG_KEY_VERSION;

    fn key(&self) -> &primitives::PublicKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

impl PublicOriginSigningKey {
    pub(crate) fn from_raw(name: String,
                           revision: KeyRevision,
                           key: primitives::PublicKey)
                           -> Self {
        let named_revision = NamedRevision::new(name, revision);
        let path = named_revision.filename::<Self>();
        Self { named_revision,
               key,
               path }
    }

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

from_str_impl_for_key!(PublicOriginSigningKey);

try_from_path_buf_impl_for_key!(PublicOriginSigningKey);

as_ref_path_impl_for_key!(PublicOriginSigningKey);

////////////////////////////////////////////////////////////////////////

pub struct SecretOriginSigningKey {
    named_revision: NamedRevision,
    key:            primitives::SecretKey,
    path:           PathBuf,
}

impl Key for SecretOriginSigningKey {
    type Crypto = primitives::SecretKey;

    const EXTENSION: &'static str = "sig.key";
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = SECRET_SIG_KEY_VERSION;

    fn key(&self) -> &primitives::SecretKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

impl SecretOriginSigningKey {
    pub(crate) fn from_raw(name: String,
                           revision: KeyRevision,
                           key: primitives::SecretKey)
                           -> Self {
        let named_revision = NamedRevision::new(name, revision);
        let path = named_revision.filename::<Self>();
        Self { named_revision,
               key,
               path }
    }

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

from_str_impl_for_key!(SecretOriginSigningKey);

try_from_path_buf_impl_for_key!(SecretOriginSigningKey);

as_ref_path_impl_for_key!(SecretOriginSigningKey);
