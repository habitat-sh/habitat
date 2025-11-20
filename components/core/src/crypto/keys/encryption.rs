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

/// Private module to re-export the various sodiumoxide concepts we
/// use, to ensure everyone is using them consistently.
mod primitives {
    pub use sodiumoxide::crypto::{box_::{curve25519xsalsa20poly1305::{Nonce,
                                                                      PublicKey,
                                                                      SecretKey,
                                                                      gen_nonce},
                                         gen_keypair,
                                         open,
                                         seal},
                                  sealedbox};
}
