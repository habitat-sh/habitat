//! Origin encryption keys (as differentiated from origin _signing_
//! keys) are used to securely transmit secrets to Builder for use in
//! package builds.
//!
//! Here, the interaction is using "anonymous" encrypted messages. A
//! user encrypts a secret with the public key, which can then only be
//! decrypted by the secret key. It is anonymous because the identity
//! of the party encrypting the message is not known. The public key
//! is freely available, while the secret key is held only by the
//! Builder service. (Actually uploading an encrypted secret to
//! Builder is governed by Builder's RBAC capabilities.)
//!
//! Even though origin encryption keys are fundamentally no different
//! than user or service encryption keys, cryptographically speaking,
//! we limit what we do with them according to our actual requirements
//! for them in the system. Thus, they only deal with anonymous
//! messages.
use crate::{crypto::keys::{AnonymousBox,
                           Key,
                           NamedRevision,
                           encryption::{PUBLIC_BOX_KEY_VERSION,
                                        PUBLIC_KEY_SUFFIX,
                                        SECRET_BOX_KEY_SUFFIX,
                                        SECRET_BOX_KEY_VERSION,
                                        primitives}},
            error::{Error,
                    Result},
            fs::Permissions,
            origin::Origin};

/// Given the name of an origin, generate a new encryption key pair.
///
/// The resulting keys will need to be saved to a cache in order to
/// persist.
///
/// This function should only ever be used in Builder; origin
/// encryption keys should never be created in the Supervisor or the
/// CLI.
pub fn generate_origin_encryption_key_pair(
    origin: &Origin)
    -> (OriginPublicEncryptionKey, OriginSecretEncryptionKey) {
    let named_revision = NamedRevision::new(origin.to_string());
    let (pk, sk) = primitives::gen_keypair();

    let public = OriginPublicEncryptionKey { named_revision: named_revision.clone(),
                                             key:            pk, };
    let secret = OriginSecretEncryptionKey { named_revision,
                                             key: sk };
    (public, secret)
}

////////////////////////////////////////////////////////////////////////

gen_key!(/// Public key used to anonymously encrypt secrets for
         /// subsequent upload to Builder for use in package builds.
         OriginPublicEncryptionKey,
         key_material: primitives::PublicKey,
         file_format_version: PUBLIC_BOX_KEY_VERSION,
         file_extension: PUBLIC_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);

impl OriginPublicEncryptionKey {
    pub fn encrypt(&self, data: &[u8]) -> AnonymousBox {
        let ciphertext = primitives::sealedbox::seal(data, self.key());
        AnonymousBox::new(self.named_revision().clone(), ciphertext)
    }
}

////////////////////////////////////////////////////////////////////////

gen_key!(/// Secret key used to decrypt anonymous secrets messages.
         ///
         /// This key will only ever be created or used in Builder for
         /// decrypting secrets that users anonymously sign with an
         /// `OriginPublicEncryptionKey`.
         ///
         /// Nothing in the Supervisor or the Habitat CLI should ever do
         /// anything with this type.
         OriginSecretEncryptionKey,
         key_material: primitives::SecretKey,
         file_format_version: SECRET_BOX_KEY_VERSION,
         file_extension: SECRET_BOX_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

impl OriginSecretEncryptionKey {
    /// Decrypt an anonymous secret encoded with the corresponding
    /// public key. Returns the decoded bytes.
    ///
    /// Only works on anonymous secrets because origin encryption keys
    /// are only ever _used_ for anonymous secrets.
    pub fn decrypt(&self, secret: &AnonymousBox) -> Result<Vec<u8>> {
        // Recover the public key material from the secret key, so we
        // don't have to pass unnecessary arguments to this function.
        let pk = self.key().public_key();

        primitives::sealedbox::open(secret.ciphertext(), &pk, self.key())
            .map_err(|_| Error::CryptoError("Could not decrypt origin encrypted secret".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::test_support::fixture_key;

    #[test]
    fn encryption() {
        let key: OriginPublicEncryptionKey =
            fixture_key("keys/fhloston-paradise-20200813211603.pub");

        let secret_message = "Leeloo Dallas Multipass".to_string();
        let anonymous = key.encrypt(secret_message.as_bytes());

        // Not a whole lot we can specifically test here, since the
        // ciphertext will be different each time. If we've got the
        // right key identity, and haven't raised a panic before now,
        // assume we're good.
        assert_eq!(anonymous.key_pair(), key.named_revision());
    }

    #[test]
    fn decryption() {
        let key: OriginSecretEncryptionKey =
            fixture_key("keys/fhloston-paradise-20200813211603.box.key");

        #[rustfmt::skip]
        let encrypted = "ANONYMOUS-BOX-1\nfhloston-paradise-20200813211603\nyCye2Rg/LtNwrNzVapYj8rrkbZpTnI3ld7oFTwGzGnsEhsxtebyW2CQDwB1IeZ2eDkNxqAQnD8AaQjK6M42fFCmadBNSMsp+AMFqnH2c".parse::<AnonymousBox>()
            .unwrap();

        let decrypted_message = key.decrypt(&encrypted).unwrap();
        let decrypted_message = std::str::from_utf8(&decrypted_message).unwrap();

        assert_eq!(decrypted_message, "Negative, I am a meat popsicle");
    }

    #[test]
    fn encryption_decrytpion_roundtrip() {
        let pk: OriginPublicEncryptionKey =
            fixture_key("keys/fhloston-paradise-20200813211603.pub");
        let sk: OriginSecretEncryptionKey =
            fixture_key("keys/fhloston-paradise-20200813211603.box.key");

        let secret_message = "Super-green".to_string();
        let encrypted = pk.encrypt(secret_message.as_bytes());

        let decrypted_message = sk.decrypt(&encrypted).unwrap();
        let decrypted_message = std::str::from_utf8(&decrypted_message).unwrap();
        assert_eq!(decrypted_message, secret_message);
    }
}
