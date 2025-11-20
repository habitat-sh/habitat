//! Service encryption keys are used alongside user encryption keys to
//! allow for authorized uploading of configuration and files to a
//! Habitat Supervisor network.
//!
//! Service secret keys are used on Supervisors to decrypt any
//! encrypted configuration and file rumors they receive. All relevant
//! user public encryption keys must also be present.
//!
//! As encryption keys, this allows services to know who sent a given
//! encrypted rumor. It also allows operators to control who can send
//! such rumors by controlling which user public keys are present on a
//! Supervisor.
use crate::{crypto::keys::{Key,
                           NamedRevision,
                           UserPublicEncryptionKey,
                           encryption::{PUBLIC_BOX_KEY_VERSION,
                                        PUBLIC_KEY_SUFFIX,
                                        SECRET_BOX_KEY_SUFFIX,
                                        SECRET_BOX_KEY_VERSION,
                                        SignedBox,
                                        primitives}},
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
    let named_revision = NamedRevision::new(key_name);
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
                                sending_user: &UserPublicEncryptionKey)
                                -> Result<Vec<u8>> {
        primitives::open(signed_box.ciphertext(),
                         signed_box.nonce(),
                         sending_user.key(),
                         self.key()).map_err(|_| {
            Error::CryptoError("Secret key, public key, and nonce could not \
                                                decrypt ciphertext"
                                                                   .to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{keys::{SignedBox,
                               UserSecretEncryptionKey},
                        test_support::fixture_key};

    #[test]
    fn decryption() {
        let key: ServiceSecretEncryptionKey =
            fixture_key("keys/service-key-valid.default@acme-20160509181736.box.key");

        let user: UserPublicEncryptionKey = fixture_key("keys/ruby-rhod-20200813204159.pub");

        #[rustfmt::skip]
        let signed = "BOX-1\nruby-rhod-20200813204159\nservice-key-valid.default@acme-20160509181736\nE6kRUTjQayKDykfgTM9WvJLFmOk2M/CR\nFivenLwuZnrKaOxNHro+StLXRKmK+acDSXE+qgGKqPHpDH6H"
            .parse::<SignedBox>()
            .unwrap();

        let decrypted_message = key.decrypt_user_message(&signed, &user).unwrap();
        let decrypted_message = std::str::from_utf8(&decrypted_message).unwrap();

        assert_eq!(decrypted_message, "HOT, HOT, HAAAAAWWT!");
    }

    #[test]
    fn encryption_decrytpion_roundtrip() {
        let user_public: UserPublicEncryptionKey = fixture_key("keys/ruby-rhod-20200813204159.pub");
        let user_secret: UserSecretEncryptionKey =
            fixture_key("keys/ruby-rhod-20200813204159.box.key");
        let service_public: ServicePublicEncryptionKey =
            fixture_key("keys/service-key-valid.default@acme-20160509181736.pub");
        let service_secret: ServiceSecretEncryptionKey =
            fixture_key("keys/service-key-valid.default@acme-20160509181736.box.key");

        let message = "Korben, sweetheart, what was that? IT WAS BAD! It had nothing! No fire, no \
                       energy, no nothin'!"
                                           .to_string();
        let signed = user_secret.encrypt_for_service(message.as_bytes(), &service_public);

        let decrypted_message = service_secret.decrypt_user_message(&signed, &user_public)
                                              .unwrap();
        let decrypted_message = std::str::from_utf8(&decrypted_message).unwrap();

        assert_eq!(decrypted_message, message);
    }
}
