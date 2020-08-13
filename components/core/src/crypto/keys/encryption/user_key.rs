use crate::{crypto::keys::{box_key_pair::WrappedSealedBox,
                           encryption::{primitives,
                                        EncryptedSecret,
                                        SignedBox,
                                        PUBLIC_BOX_KEY_VERSION,
                                        PUBLIC_KEY_SUFFIX,
                                        SECRET_BOX_KEY_SUFFIX,
                                        SECRET_BOX_KEY_VERSION},
                           Key,
                           KeyRevision,
                           NamedRevision,
                           ServicePublicEncryptionKey},
            error::{Error,
                    Result}};
use std::{path::PathBuf,
          str};

/// Given the name of a user, generate a new encryption key pair.
///
/// The resulting keys will need to be saved to a cache in order to
/// persist.
pub fn generate_user_encryption_key_pair(user_name: &str)
                                         -> (UserPublicEncryptionKey, UserSecretEncryptionKey) {
    let revision = KeyRevision::new();
    let (pk, sk) = primitives::gen_keypair();

    let public = UserPublicEncryptionKey::from_raw(user_name.to_string(), revision.clone(), pk);
    let secret = UserSecretEncryptionKey::from_raw(user_name.to_string(), revision.clone(), sk);
    (public, secret)
}

////////////////////////////////////////////////////////////////////////

pub struct UserPublicEncryptionKey {
    named_revision: NamedRevision,
    key:            primitives::PublicKey,
    path:           PathBuf,
}

impl Key for UserPublicEncryptionKey {
    type Crypto = primitives::PublicKey;

    const EXTENSION: &'static str = PUBLIC_KEY_SUFFIX;
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = PUBLIC_BOX_KEY_VERSION;

    fn key(&self) -> &primitives::PublicKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

from_str_impl_for_key!(UserPublicEncryptionKey);

try_from_path_buf_impl_for_key!(UserPublicEncryptionKey);

as_ref_path_impl_for_key!(UserPublicEncryptionKey);

debug_impl_for_key!(UserPublicEncryptionKey);

impl UserPublicEncryptionKey {
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
}

////////////////////////////////////////////////////////////////////////

pub struct UserSecretEncryptionKey {
    named_revision: NamedRevision,
    key:            primitives::SecretKey,
    path:           PathBuf,
}

impl Key for UserSecretEncryptionKey {
    type Crypto = primitives::SecretKey;

    const EXTENSION: &'static str = SECRET_BOX_KEY_SUFFIX;
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = SECRET_BOX_KEY_VERSION;

    fn key(&self) -> &primitives::SecretKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

from_str_impl_for_key!(UserSecretEncryptionKey);

try_from_path_buf_impl_for_key!(UserSecretEncryptionKey);

as_ref_path_impl_for_key!(UserSecretEncryptionKey);

debug_impl_for_key!(UserSecretEncryptionKey);

impl UserSecretEncryptionKey {
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

    /// Encrypt some data with a user's private key for decryption by
    /// a receiving service's private key.
    pub fn encrypt_for_service(&self,
                               data: &[u8],
                               receiving_service: &ServicePublicEncryptionKey)
                               -> WrappedSealedBox {
        let nonce = primitives::gen_nonce();
        let ciphertext = primitives::seal(data, &nonce, receiving_service.key(), self.key());
        let signed = SignedBox::new(self.named_revision.clone(),
                                    receiving_service.named_revision().clone(),
                                    ciphertext,
                                    nonce);
        // TODO (CM): Eventually do away with WrappedSealedBox; this
        // is just for compatibility now
        WrappedSealedBox::from(EncryptedSecret::Signed(signed))
    }
}
