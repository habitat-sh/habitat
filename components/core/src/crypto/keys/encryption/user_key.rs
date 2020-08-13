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
                    Result},
            fs::Permissions};

/// Given the name of a user, generate a new encryption key pair.
///
/// The resulting keys will need to be saved to a cache in order to
/// persist.
pub fn generate_user_encryption_key_pair(user_name: &str)
                                         -> (UserPublicEncryptionKey, UserSecretEncryptionKey) {
    let revision = KeyRevision::new();
    let named_revision = NamedRevision::new(user_name.to_string(), revision);
    let (pk, sk) = primitives::gen_keypair();
    let public = UserPublicEncryptionKey { named_revision: named_revision.clone(),
                                           key:            pk, };
    let secret = UserSecretEncryptionKey { named_revision,
                                           key: sk };
    (public, secret)
}

////////////////////////////////////////////////////////////////////////

gen_key!(UserPublicEncryptionKey,
         key_material: primitives::PublicKey,
         file_format_version: PUBLIC_BOX_KEY_VERSION,
         file_extension: PUBLIC_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);

////////////////////////////////////////////////////////////////////////

gen_key!(UserSecretEncryptionKey,
         key_material: primitives::SecretKey,
         file_format_version: SECRET_BOX_KEY_VERSION,
         file_extension: SECRET_BOX_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

impl UserSecretEncryptionKey {
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
