/// Create an instance of a key, along with several trait
/// implementations to handle reading from and writing to the
/// filesystem.
macro_rules! gen_key {
    (
        $(#[$attr:meta])*
        $t:ident,key_material:
        $key:ty,file_format_version:
        $version:expr,file_extension:
        $extension:expr,file_permissions:
        $permissions:expr
    ) => {


        $(#[$attr])*
        #[derive(Clone, PartialEq)]
        pub struct $t {
            named_revision: NamedRevision,
            key:            $key,
        }

        impl Key for $t {
            type Crypto = $key;

            fn key(&self) -> &$key { &self.key }

            fn named_revision(&self) -> &NamedRevision { &self.named_revision }
        }

        debug_impl_for_key!($t);

        impl crate::crypto::keys::KeyFile for $t {
            fn permissions() -> Permissions { $permissions }

            fn version() -> &'static str { $version }

            fn extension() -> &'static str { $extension }
        }

        from_str_impl_for_key!($t);

        try_from_path_buf_impl_for_key!($t);

        try_from_bytes_for_key!($t);
    };
}

/// Helper macro to generate FromStr implementations for our key
/// types.
macro_rules! from_str_impl_for_key {
    ($t:ty) => {
        impl std::str::FromStr for $t {
            type Err = Error;

            fn from_str(content: &str) -> std::result::Result<Self, Self::Err> {

                let mut lines = content.lines();

                lines.next()
                    .ok_or_else(|| Error::CryptoError("Missing key version".to_string()))
                    .map(|line| {
                        if line == <Self as crate::crypto::keys::KeyFile>::version() {
                            Ok(())
                        } else {
                            Err(Error::CryptoError(format!("Unsupported key version: {}", line)))
                        }
                    })??;

                let named_revision: NamedRevision =
                    lines.next()
                    .ok_or_else(|| Error::CryptoError("Missing name+revision".to_string()))
                    .map(str::parse)??;

                let key: <Self as crate::crypto::keys::Key>::Crypto =
                    lines.nth(1) // skip a blank line!
                    .ok_or_else(|| Error::CryptoError("Missing key material".to_string()))
                    .map(str::trim)
                    .map(base64::decode)?
                    .map_err(|_| Error::CryptoError("Invalid base64 key material".to_string()))
                    .map(|b| <Self as Key>::Crypto::from_slice(&b))?
                    .ok_or_else(|| {
                        Error::CryptoError(format!("Could not parse bytes as key for {}",
                                                   named_revision))
                    })?;


                Ok(Self {named_revision, key})
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

/// Helper macro to convert bytes into a key. Intended for use in
/// Builder; we don't really have need for this in the Supervisor or
/// CLI at the moment.
macro_rules! try_from_bytes_for_key {
    ($t:ty) => {
        impl std::convert::TryFrom<&[u8]> for $t {
            type Error = Error;

            fn try_from(bytes: &[u8]) -> Result<$t> { std::str::from_utf8(bytes)?.parse() }
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
