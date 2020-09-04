mod builder_key;
mod message;
mod origin_key;
mod service_key;
mod user_key;

pub use builder_key::{generate_builder_encryption_key,
                      BuilderSecretEncryptionKey,
                      BUILDER_KEY_NAME};
pub use message::{AnonymousBox,
                  SignedBox};
pub use origin_key::{generate_origin_encryption_key_pair,
                     OriginPublicEncryptionKey,
                     OriginSecretEncryptionKey};
pub use service_key::{generate_service_encryption_key_pair,
                      ServicePublicEncryptionKey,
                      ServiceSecretEncryptionKey};
pub use user_key::{generate_user_encryption_key_pair,
                   UserPublicEncryptionKey,
                   UserSecretEncryptionKey};

/// The suffix on the end of a public encryption key file
const PUBLIC_KEY_SUFFIX: &str = "pub";
/// The suffix on the end of a secret encryption key file
const SECRET_BOX_KEY_SUFFIX: &str = "box.key";
/// Format version identifier for public encryption keys.
const PUBLIC_BOX_KEY_VERSION: &str = "BOX-PUB-1";
/// Format version identifier for secret encryption keys.
const SECRET_BOX_KEY_VERSION: &str = "BOX-SEC-1";

/// Private module to re-export the various sodiumoxide concepts we
/// use, to ensure everyone is using them consistently.
mod primitives {
    pub use sodiumoxide::crypto::{box_::{curve25519xsalsa20poly1305::{gen_nonce,
                                                                      Nonce,
                                                                      PublicKey,
                                                                      SecretKey},
                                         gen_keypair,
                                         open,
                                         seal},
                                  sealedbox};
}
