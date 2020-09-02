//! Encryption logic used by Builder to encrypt secrets at rest.

use crate::{crypto::keys::{encryption::{primitives,
                                        SECRET_BOX_KEY_SUFFIX,
                                        SECRET_BOX_KEY_VERSION},
                           Key,
                           NamedRevision,
                           SignedBox},
            error::{Error,
                    Result},
            fs::Permissions};

pub const BUILDER_KEY_NAME: &str = "bldr";

/// Always create a new secret encryption key named "bldr".
///
/// Interestingly, based on how we use this key, we actually don't
/// ever need the public key for this pair, so we only create the
/// secret one.
pub fn generate_builder_encryption_key() -> BuilderSecretEncryptionKey {
    let named_revision = NamedRevision::new(BUILDER_KEY_NAME.to_string());
    let (_pk, sk) = primitives::gen_keypair();

    BuilderSecretEncryptionKey { named_revision,
                                 key: sk }
}

////////////////////////////////////////////////////////////////////////

gen_key!(BuilderSecretEncryptionKey,
         key_material: primitives::SecretKey,
         file_format_version: SECRET_BOX_KEY_VERSION,
         file_extension: SECRET_BOX_KEY_SUFFIX,
         file_permissions: crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

impl BuilderSecretEncryptionKey {
    /// Encrypt bytes with our own public key, for our own
    /// consumption. This ensures that we are the only ones that could
    /// have encrypted the data, and we are the only ones that can
    /// decrypt it.
    pub fn encrypt<B>(&self, bytes: B) -> SignedBox
        where B: AsRef<[u8]>
    {
        let nonce = primitives::gen_nonce();

        // Recover the public key material from the secret key itself.
        let my_public_key = self.key().public_key();

        let ciphertext = primitives::seal(bytes.as_ref(), &nonce, &my_public_key, self.key());

        // Even though we don't have a `BuilderPublicEncryptionKey`
        // here, we know its named revision would be the same as
        // `self.named_revision()`, so we'll just copy it.
        SignedBox::new(self.named_revision.clone(),
                       self.named_revision.clone(),
                       ciphertext,
                       nonce)
    }

    /// Decrypt a signed message we sent to ourself.
    pub fn decrypt(&self, signed_box: &SignedBox) -> Result<Vec<u8>> {
        // Recover the public key material from the secret key itself.
        let my_public_key = self.key().public_key();

        primitives::open(signed_box.ciphertext(),
                         signed_box.nonce(),
                         &my_public_key,
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
    use crate::crypto::keys::SignedBox;

    #[test]
    fn encryption() {
        let key = generate_builder_encryption_key();
        let message = "He is the Kwisatz Haderach!";
        let encrypted_message = key.encrypt(message);

        // Can't assert much here since we've always got different
        // encrypted bits, but we can ensure that the encryptor and
        // decryptor are the same.
        assert_eq!(encrypted_message.encryptor(), encrypted_message.decryptor());
    }

    #[test]
    fn decryption() {
        let key: BuilderSecretEncryptionKey =
            "BOX-SEC-1\nbldr-20200825205529\n\nM9u8wuJmZMsmVG4tNgngYJDapjIJE1RnxJAFVN97Bxs="
            .parse()
            .unwrap();

        #[rustfmt::skip]
        let encrypted = "BOX-1\nbldr-20200825205529\nbldr-20200825205529\nilnFU7aVNfkq6PrNXzXh3l1FTQftMzoM\nr6B4EAUIRO2tf169nPMeDPxVzZ7tslS/Oiv2ZQCcFBRyotwv5rh0NjN6KR5pCFOPWAmp62tSQQz6FIiKqHC2bBlk3A4MLugX"
            .parse::<SignedBox>()
            .unwrap();

        let decrypted = key.decrypt(&encrypted)
                           .map(String::from_utf8)
                           .unwrap()
                           .unwrap();
        assert_eq!(decrypted,
                   "Fear is the little-death that brings total obliteration.");
    }

    #[test]
    fn encryption_decryption_round_trip() {
        let key = generate_builder_encryption_key();
        let message = "Walk without rhythm and you won't attract the worm";

        let encrypted = key.encrypt(message);
        let decrypted = key.decrypt(&encrypted)
                           .map(String::from_utf8)
                           .unwrap()
                           .unwrap();

        assert_eq!(decrypted, message);
    }
}
