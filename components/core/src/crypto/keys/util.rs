/// Helper trait for use in FromStr implementations because the
/// sodiumoxide types we're parsing don't implement their `from_slice`
/// methods as traits.

// TODO (CM): This unfortunately has to be public at the moment. I
// *might* be able to get rid of it if I pull the entire parsing logic
// into the macro, though.
pub trait FromSlice<T> {
    fn from_slice(bytes: &[u8]) -> Option<T>;
}
/// Helper macro to generates `FromSlice` implementations for our
/// sodiumoxide key types, which are needed as an implementation
/// detail for our `FromStr` implementation.
macro_rules! from_slice_impl_for_sodiumoxide_key {
    ($t:ty) => {
        impl crate::crypto::keys::util::FromSlice<$t> for $t {
            fn from_slice(bytes: &[u8]) -> Option<$t> { <$t>::from_slice(bytes) }
        }
    };
}

/// Helper macro to generate FromStr implementations for our key
/// types.
macro_rules! from_str_impl_for_key {
    ($t:ty) => {
        impl std::str::FromStr for $t {
            type Err = Error;

            fn from_str(content: &str) -> std::result::Result<Self, Self::Err> {
                let (name, revision, key) = <$t>::parse_from_str(content)?;
                Ok(<$t>::from_raw(name, revision, key))
            }
        }
    };
}

/// Helper macro to implement conversion logic to generate an instance
/// of a key from a file.
macro_rules! try_from_path_buf_impl_for_key {
    ($t:ty) => {
        impl std::convert::TryFrom<std::path::PathBuf> for $t {
            type Error = Error;

            fn try_from(path: std::path::PathBuf) -> Result<$t> {
                std::fs::read_to_string(path)?.parse()
            }
        }
    };
}

/// Helper macro to create `AsRef<Path>` implementations for all our
/// Keys. They should all have a `PathBuf`-typed `path` field.
macro_rules! as_ref_path_impl_for_key {
    ($t:ty) => {
        impl std::convert::AsRef<std::path::Path> for $t {
            fn as_ref(&self) -> &std::path::Path { &self.path }
        }
    };
}

/// Helper macro to implement `Debug` for all our keys in a consistent
/// manner.
macro_rules! debug_impl_for_key {
    ($t:ty) => {
        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", stringify!($t), self.named_revision())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::crypto::keys::{generate_origin_encryption_key_pair,
                              generate_service_encryption_key_pair,
                              generate_signing_key_pair,
                              generate_user_encryption_key_pair,
                              Key,
                              OriginPublicEncryptionKey,
                              OriginSecretEncryptionKey,
                              PublicOriginSigningKey,
                              RingKey,
                              SecretOriginSigningKey,
                              ServicePublicEncryptionKey,
                              ServiceSecretEncryptionKey,
                              UserPublicEncryptionKey,
                              UserSecretEncryptionKey};

    mod debug_impl_for_key {
        use super::*;

        #[test]
        fn ring_key() {
            let key = RingKey::new("beyonce");
            assert_eq!(format!("RingKey beyonce-{}", key.named_revision().revision()),
                       format!("{:?}", key));
        }

        #[test]
        fn user_keys() {
            let (public, secret) = generate_user_encryption_key_pair("my-user");
            assert_eq!(format!("UserPublicEncryptionKey my-user-{}",
                               public.named_revision().revision()),
                       format!("{:?}", public));
            assert_eq!(format!("UserSecretEncryptionKey my-user-{}",
                               secret.named_revision().revision()),
                       format!("{:?}", secret));
        }

        #[test]
        fn origin_keys() {
            let (public, secret) = generate_origin_encryption_key_pair("my-origin");

            assert_eq!(format!("OriginPublicEncryptionKey my-origin-{}",
                               public.named_revision().revision()),
                       format!("{:?}", public));
            assert_eq!(format!("OriginSecretEncryptionKey my-origin-{}",
                               secret.named_revision().revision()),
                       format!("{:?}", secret));
        }

        #[test]
        fn service_keys() {
            let (public, secret) = generate_service_encryption_key_pair("my-org", "foo.default");

            assert_eq!(format!("ServicePublicEncryptionKey foo.default@my-org-{}",
                               public.named_revision().revision()),
                       format!("{:?}", public));
            assert_eq!(format!("ServiceSecretEncryptionKey foo.default@my-org-{}",
                               secret.named_revision().revision()),
                       format!("{:?}", secret));
        }

        #[test]
        fn signing_keys() {
            let (public, secret) = generate_signing_key_pair("my-origin");
            assert_eq!(format!("PublicOriginSigningKey my-origin-{}",
                               public.named_revision().revision()),
                       format!("{:?}", public));
            assert_eq!(format!("SecretOriginSigningKey my-origin-{}",
                               secret.named_revision().revision()),
                       format!("{:?}", secret));
        }
    }
}
