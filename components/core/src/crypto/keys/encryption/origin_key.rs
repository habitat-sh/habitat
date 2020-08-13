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
                    Result},
            fs::Permissions};

/// Given the name of an origin, generate a new encryption key pair.
///
/// The resulting keys will need to be saved to a cache in order to
/// persist.
pub fn generate_origin_encryption_key_pair(
    origin_name: &str)
    -> (OriginPublicEncryptionKey, OriginSecretEncryptionKey) {
    let revision = KeyRevision::new();
    let named_revision = NamedRevision::new(origin_name.to_string(), revision);
    let (pk, sk) = primitives::gen_keypair();

    let public = OriginPublicEncryptionKey { named_revision: named_revision.clone(),
                                             key:            pk, };
    let secret = OriginSecretEncryptionKey { named_revision,
                                             key: sk };
    (public, secret)
}

////////////////////////////////////////////////////////////////////////

gen_key!(OriginPublicEncryptionKey,
         key_material: primitives::PublicKey,
         file_format_version: PUBLIC_BOX_KEY_VERSION,
         file_extension: PUBLIC_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);

impl OriginPublicEncryptionKey {
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

gen_key!(OriginSecretEncryptionKey,
         key_material: primitives::SecretKey,
         file_format_version: SECRET_BOX_KEY_VERSION,
         file_extension: SECRET_BOX_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);
