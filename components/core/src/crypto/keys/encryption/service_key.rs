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
                    Result},
            fs::Permissions};

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
    let named_revision = NamedRevision::new(key_name, revision);
    let (pk, sk) = primitives::gen_keypair();

    let public = ServicePublicEncryptionKey { named_revision: named_revision.clone(),
                                              key:            pk, };
    let secret = ServiceSecretEncryptionKey { named_revision,
                                              key: sk };
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

gen_key!(ServicePublicEncryptionKey,
         key_material: primitives::PublicKey,
         file_format_version: PUBLIC_BOX_KEY_VERSION,
         file_extension: PUBLIC_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);

////////////////////////////////////////////////////////////////////////

gen_key!(ServiceSecretEncryptionKey,
         key_material: primitives::SecretKey,
         file_format_version: SECRET_BOX_KEY_VERSION,
         file_extension: SECRET_BOX_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

impl ServiceSecretEncryptionKey {
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
