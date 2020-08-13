use crate::{crypto::keys::{box_key_pair::WrappedSealedBox,
                           encryption::{primitives,
                                        AnonymousBox,
                                        EncryptedSecret,
                                        PUBLIC_BOX_KEY_VERSION,
                                        PUBLIC_KEY_SUFFIX,
                                        SECRET_BOX_KEY_SUFFIX,
                                        SECRET_BOX_KEY_VERSION},
                           Key,
                           KeyRevision,
                           NamedRevision},
            error::{Error,
                    Result}};

use std::{path::PathBuf,
          str};

/// Given the name of an origin, generate a new encryption key pair.
///
/// The resulting keys will need to be saved to a cache in order to
/// persist.
pub fn generate_origin_encryption_key_pair(
    origin_name: &str)
    -> (OriginPublicEncryptionKey, OriginSecretEncryptionKey) {
    let revision = KeyRevision::new();
    let (pk, sk) = primitives::gen_keypair();

    let public = OriginPublicEncryptionKey::from_raw(origin_name.to_string(), revision.clone(), pk);
    let secret = OriginSecretEncryptionKey::from_raw(origin_name.to_string(), revision.clone(), sk);
    (public, secret)
}

////////////////////////////////////////////////////////////////////////

pub struct OriginPublicEncryptionKey {
    named_revision: NamedRevision,
    key:            primitives::PublicKey,
    path:           PathBuf,
}

impl Key for OriginPublicEncryptionKey {
    type Crypto = primitives::PublicKey;

    const EXTENSION: &'static str = PUBLIC_KEY_SUFFIX;
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = PUBLIC_BOX_KEY_VERSION;

    fn key(&self) -> &primitives::PublicKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

from_str_impl_for_key!(OriginPublicEncryptionKey);

try_from_path_buf_impl_for_key!(OriginPublicEncryptionKey);

as_ref_path_impl_for_key!(OriginPublicEncryptionKey);

debug_impl_for_key!(OriginPublicEncryptionKey);

impl OriginPublicEncryptionKey {
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

    // corresponds to old encrypt_anonymous_box
    /// Encrypt a secret
    pub fn encrypt(&self, data: &[u8]) -> WrappedSealedBox {
        let ciphertext = primitives::sealedbox::seal(data, self.key());
        let anon = AnonymousBox::new(self.named_revision().clone(), ciphertext);
        // TODO (CM): Eventually do away with WrappedSealedBox; this
        // is just for compatibility now
        WrappedSealedBox::from(EncryptedSecret::Anonymous(anon))
    }
}

////////////////////////////////////////////////////////////////////////

pub struct OriginSecretEncryptionKey {
    named_revision: NamedRevision,
    key:            primitives::SecretKey,
    path:           PathBuf,
}

impl Key for OriginSecretEncryptionKey {
    type Crypto = primitives::SecretKey;

    const EXTENSION: &'static str = SECRET_BOX_KEY_SUFFIX;
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = SECRET_BOX_KEY_VERSION;

    fn key(&self) -> &primitives::SecretKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

from_str_impl_for_key!(OriginSecretEncryptionKey);

try_from_path_buf_impl_for_key!(OriginSecretEncryptionKey);

as_ref_path_impl_for_key!(OriginSecretEncryptionKey);

debug_impl_for_key!(OriginSecretEncryptionKey);

impl OriginSecretEncryptionKey {
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
}
