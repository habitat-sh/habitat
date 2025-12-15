mod builder_key;
mod message;
mod origin_key;
mod service_key;
mod user_key;

pub use builder_key::{BUILDER_KEY_NAME,
                      BuilderSecretEncryptionKey,
                      generate_builder_encryption_key};
pub use message::{AnonymousBox,
                  SignedBox};
pub use origin_key::{OriginPublicEncryptionKey,
                     OriginSecretEncryptionKey,
                     generate_origin_encryption_key_pair};
pub use service_key::{ServicePublicEncryptionKey,
                      ServiceSecretEncryptionKey,
                      generate_service_encryption_key_pair};
pub use user_key::{UserPublicEncryptionKey,
                   UserSecretEncryptionKey,
                   generate_user_encryption_key_pair};

/// The suffix on the end of a public encryption key file
const PUBLIC_KEY_SUFFIX: &str = "pub";
/// The suffix on the end of a secret encryption key file
const SECRET_BOX_KEY_SUFFIX: &str = "box.key";
/// Format version identifier for public encryption keys.
const PUBLIC_BOX_KEY_VERSION: &str = "BOX-PUB-1";
/// Format version identifier for secret encryption keys.
const SECRET_BOX_KEY_VERSION: &str = "BOX-SEC-1";

/// Private module to re-export the various libsodium-rs concepts we
/// use, to ensure everyone is using them consistently.
mod primitives {

    pub use libsodium_rs::crypto_box::{Nonce,
                                       PublicKey,
                                       open,
                                       seal};

    pub mod sealedbox {
        pub use libsodium_rs::crypto_box::{open_sealed_box as open,
                                           seal_box as seal};
    }

    pub fn gen_keypair() -> crate::Result<(PublicKey, SecretKey)> {
        let key_pair_tuple = libsodium_rs::crypto_box::KeyPair::generate().into_tuple();
        Ok((key_pair_tuple.0, SecretKey(key_pair_tuple.1)))
    }

    pub fn gen_nonce() -> Nonce { Nonce::generate() }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SecretKey(libsodium_rs::crypto_box::SecretKey);

    impl SecretKey {
        pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
            Ok(SecretKey(libsodium_rs::crypto_box::SecretKey::from_bytes(bytes)?))
        }

        pub fn public_key(&self) -> crate::Result<PublicKey> {
            use libsodium_rs::{crypto_box::SECRETKEYBYTES,
                               crypto_scalarmult::curve25519::scalarmult_base};
            let private_key: &[u8; SECRETKEYBYTES] = self.0.as_bytes();
            Ok(scalarmult_base(private_key).map(PublicKey::from_bytes_exact)?)
        }
    }

    impl core::convert::AsRef<[u8]> for SecretKey {
        fn as_ref(&self) -> &[u8] { self.0.as_ref() }
    }

    impl std::ops::Deref for SecretKey {
        type Target = libsodium_rs::crypto_box::SecretKey;

        fn deref(&self) -> &Self::Target { &self.0 }
    }
}
