use crate::{crypto::keys::{encryption::{primitives,
                                        SignedBox,
                                        PUBLIC_BOX_KEY_VERSION,
                                        PUBLIC_KEY_SUFFIX,
                                        SECRET_BOX_KEY_SUFFIX,
                                        SECRET_BOX_KEY_VERSION},
                           Key,
                           KeyRevision,
                           NamedRevision,
                           UserPublicEncryptionKey},
            error::{Error,
                    Result}};
use std::{path::PathBuf,
          str};

/// Given the name of an org and a service group, generate a new
/// encryption key pair.
///
/// The resulting keys will need to be saved to a cache in order to
/// persist.
pub fn generate_service_encryption_key_pair(
    org_name: &str,
    service_group_name: &str)
    -> (ServicePublicEncryptionKey, ServiceSecretEncryptionKey) {
    let key_name = service_key_name(org_name, service_group_name);
    let revision = KeyRevision::new();
    let (pk, sk) = primitives::gen_keypair();

    let public = ServicePublicEncryptionKey::from_raw(key_name.clone(), revision.clone(), pk);
    let secret = ServiceSecretEncryptionKey::from_raw(key_name.clone(), revision.clone(), sk);
    (public, secret)
}

/// Generate the name of a service key.
///
/// Note that `service_group_name` is like `"redis.default"`, not
/// simply `"redis"`.
fn service_key_name(org_name: &str, service_group_name: &str) -> String {
    format!("{}@{}", service_group_name, org_name)
}

////////////////////////////////////////////////////////////////////////

pub struct ServicePublicEncryptionKey {
    named_revision: NamedRevision,
    key:            primitives::PublicKey,
    path:           PathBuf,
}

impl Key for ServicePublicEncryptionKey {
    type Crypto = primitives::PublicKey;

    const EXTENSION: &'static str = PUBLIC_KEY_SUFFIX;
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = PUBLIC_BOX_KEY_VERSION;

    fn key(&self) -> &primitives::PublicKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

from_str_impl_for_key!(ServicePublicEncryptionKey);

try_from_path_buf_impl_for_key!(ServicePublicEncryptionKey);

as_ref_path_impl_for_key!(ServicePublicEncryptionKey);

debug_impl_for_key!(ServicePublicEncryptionKey);

impl ServicePublicEncryptionKey {
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

pub struct ServiceSecretEncryptionKey {
    named_revision: NamedRevision,
    key:            primitives::SecretKey,
    path:           PathBuf,
}

impl Key for ServiceSecretEncryptionKey {
    type Crypto = primitives::SecretKey;

    const EXTENSION: &'static str = SECRET_BOX_KEY_SUFFIX;
    const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS;
    const VERSION_STRING: &'static str = SECRET_BOX_KEY_VERSION;

    fn key(&self) -> &primitives::SecretKey { &self.key }

    fn named_revision(&self) -> &NamedRevision { &self.named_revision }
}

from_str_impl_for_key!(ServiceSecretEncryptionKey);

try_from_path_buf_impl_for_key!(ServiceSecretEncryptionKey);

as_ref_path_impl_for_key!(ServiceSecretEncryptionKey);

debug_impl_for_key!(ServiceSecretEncryptionKey);

impl ServiceSecretEncryptionKey {
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

    /// Decrypt a boxed message sent from a user to a service.
    ///
    /// The message will have been encrypted using a
    /// `UserSecretEncryptionKey` and a `ServicePublicEncryptionKey`,
    /// and requires the corresponding `UserPublicEncryptionKey` and
    /// `ServiceSecretEncryptionKey`.
    ///
    /// As the service is the recipient, we attach this functionality
    /// to this struct.
    pub fn decrypt_user_message(&self,
                                signed_box: &SignedBox,
                                sender_key: &UserPublicEncryptionKey)
                                -> Result<Vec<u8>> {
        primitives::open(signed_box.ciphertext(),
                         signed_box.nonce(),
                         sender_key.key(),
                         self.key()).map_err(|_| {
            Error::CryptoError("Secret key, public key, and nonce could not \
                                                decrypt ciphertext"
                                                                   .to_string())
        })
    }
}
