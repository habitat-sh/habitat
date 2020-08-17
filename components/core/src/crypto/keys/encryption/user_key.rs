//! User encryption keys are used alongside user encryption keys to
//! allow for authorized uploading of configuration and files to a
//! Habitat Supervisor network.
//!
//! User secret keys are used from workstations to encrypt
//! configuration and file rumors targeted to a specific service group
//! in a Habitat network. The service's public key must also be
//! present in order to encrypt.
//!
//! As encryption keys, this allows services to know who sent a given
//! encrypted rumor. It also allows operators to control who can send
//! such rumors by controlling which user public keys are present on a
//! Supervisor.
use crate::{crypto::keys::{box_key_pair::WrappedSealedBox,
                           encryption::{primitives,
                                        EncryptedSecret,
                                        SignedBox,
                                        PUBLIC_BOX_KEY_VERSION,
                                        PUBLIC_KEY_SUFFIX,
                                        SECRET_BOX_KEY_SUFFIX,
                                        SECRET_BOX_KEY_VERSION},
                           Key,
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
    let named_revision = NamedRevision::new(user_name.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::test_support::fixture_key;

    #[test]
    fn encryption() {
        let service: ServicePublicEncryptionKey =
            fixture_key("keys/service-key-valid.default@acme-20160509181736.pub");
        let user: UserSecretEncryptionKey = fixture_key("keys/ruby-rhod-20200813204159.box.key");

        let message = "HOT, HOT, HAAAAAWWT!".to_string();

        let encrypted_secret = user.encrypt_for_service(message.as_bytes(), &service);

        // Horrible workaround while we still use WrappedSealedBox
        let encrypted_secret = EncryptedSecret::from_bytes(encrypted_secret.as_bytes()).unwrap();

        let signed = encrypted_secret.signed().unwrap();

        // Not a whole lot we can specifically test here, since the
        // ciphertext will be different each time. If we've got the
        // right sender and recipient, and haven't raised a panic
        // before now, assume we're good.
        assert_eq!(signed.sender(), user.named_revision());
        assert_eq!(signed.receiver(), service.named_revision());
    }

    // Choosing to put the "round trip" encryption test over in
    // `service_key.rs`, since it involves both user and service, and
    // service is the one that does the decrypting.
}
