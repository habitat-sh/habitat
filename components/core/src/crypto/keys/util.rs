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

        impl crate::crypto::keys::Key for $t {
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
        try_from_path_impl_for_key!($t);

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
                    .map(crate::base64::decode)?
                    .map_err(|_| Error::CryptoError("Invalid base64 key material".to_string()))
                    .map(|b| <Self as crate::crypto::keys::Key>::Crypto::from_slice(&b))?
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

/// Helper macro to implement conversion logic to generate an instance
/// of a key from a file.
macro_rules! try_from_path_impl_for_key {
    ($t:ty) => {
        impl std::convert::TryFrom<&std::path::Path> for $t {
            type Error = Error;

            fn try_from(path: &std::path::Path) -> Result<$t> {
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
                write!(f,
                       "{} {}",
                       stringify!($t),
                       crate::crypto::keys::Key::named_revision(self))
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
                              NamedRevision,
                              OriginPublicEncryptionKey,
                              OriginSecretEncryptionKey,
                              PublicOriginSigningKey,
                              RingKey,
                              SecretOriginSigningKey,
                              ServicePublicEncryptionKey,
                              ServiceSecretEncryptionKey,
                              UserPublicEncryptionKey,
                              UserSecretEncryptionKey};

    /// Validate the FromStr implementations
    mod from_str {
        use super::*;
        use crate::crypto::keys::KeyFile;

        /// Ensures that the FromStr implementation from
        /// `from_str_impl_for_key` macro lines up with
        /// `KeyFile::to_key_string`.
        macro_rules! assert_parse_round_trip {
            ($t:ty, $key:expr) => {
                let key_string = $key.to_key_string();
                let parsed_key: $t = key_string.parse().unwrap();
                assert_eq!($key, parsed_key,
                           "Expected to generate the same key from its string representation, \
                            but didn't!");
            };
        }

        #[test]
        fn ring_key() {
            let key = RingKey::new("beyonce");
            assert_parse_round_trip!(RingKey, key);
        }

        #[test]
        fn user_keys() {
            let (public, secret) = generate_user_encryption_key_pair("my-user");
            assert_parse_round_trip!(UserPublicEncryptionKey, public);
            assert_parse_round_trip!(UserSecretEncryptionKey, secret);
        }

        #[test]
        fn origin_keys() {
            let origin = "my-origin".parse().unwrap();
            let (public, secret) = generate_origin_encryption_key_pair(&origin);
            assert_parse_round_trip!(OriginPublicEncryptionKey, public);
            assert_parse_round_trip!(OriginSecretEncryptionKey, secret);
        }

        #[test]
        fn service_keys() {
            let (public, secret) = generate_service_encryption_key_pair("my-org", "foo.default");
            assert_parse_round_trip!(ServicePublicEncryptionKey, public);
            assert_parse_round_trip!(ServiceSecretEncryptionKey, secret);
        }

        #[test]
        fn signing_keys() {
            let origin = "my-origin".parse().unwrap();
            let (public, secret) = generate_signing_key_pair(&origin);
            assert_parse_round_trip!(PublicOriginSigningKey, public);
            assert_parse_round_trip!(SecretOriginSigningKey, secret);
        }

        /// Ensure that we can take various files as keys and
        /// correctly parse them into their appropriate types
        mod parse {
            use super::*;
            use crate::crypto::test_support::fixture_as_string;

            macro_rules! parse {
                ($name:ident, $key:ty, $fixture_path:expr) => {
                    #[test]
                    fn $name() {
                        let content = fixture_as_string($fixture_path);
                        let parsed = content.parse::<$key>();
                        assert!(
                            parsed.is_ok(),
                            "Could not parse '{}' as a {}: {:?}",
                            $fixture_path,
                            stringify!($key),
                            parsed
                        );
                    }
                };
            }

            parse!(ring_key, RingKey, "keys/ring-key-valid-20160504220722.sym.key");

            parse!(public_origin_signing_key, PublicOriginSigningKey,
                   "keys/origin-key-valid-20160509190508.pub");
            parse!(secret_origin_signing_key, SecretOriginSigningKey,
                   "keys/origin-key-valid-20160509190508.sig.key");

            parse!(service_public_encryption_key, ServicePublicEncryptionKey,
                   "keys/service-key-valid.default@acme-20160509181736.pub");
            parse!(service_secret_encryption_key, ServiceSecretEncryptionKey,
                   "keys/service-key-valid.default@acme-20160509181736.box.key");

            parse!(user_public_encryption_key, UserPublicEncryptionKey, "keys/ruby-rhod-20200813204159.pub");
            parse!(user_secret_encryption_key, UserSecretEncryptionKey,
                   "keys/ruby-rhod-20200813204159.box.key");

            parse!(origin_public_encryption_key, OriginPublicEncryptionKey,
                   "keys/fhloston-paradise-20200813211603.pub");
            parse!(origin_secret_encryption_key, OriginSecretEncryptionKey,
                   "keys/fhloston-paradise-20200813211603.box.key");

            /// While each of the three kinds of encryption keys
            /// (origin, service, and user) all have their own
            /// particular use cases, and thus different APIs, they
            /// are all fundamentally the same kind of thing. That is,
            /// nothing in the format of a user encryption key, for
            /// instance, marks it *fundamentally* as a _user_
            /// key. It could just as well be parsed as a service key.
            ///
            /// All the keys have the same internal structure, and
            /// nothing in their file serialization distinguishes
            /// between them.
            ///
            /// These tests merely reflect that fact; it is not
            /// necessarily integral to the key system that things
            /// behave this way, but if that were ever to change, it
            /// would be nice to know about it.
            mod all_encryption_keys_are_equivalent_at_some_level {
                use super::*;

                /// Pass a public encryption key instance and assert
                /// the string it produces can be parsed as any other
                /// public encryption key type.
                macro_rules! assert_public_key_equivalence {
                    ($key:expr) => {
                        let key_string = $key.to_key_string();
                        assert!(key_string.parse::<UserPublicEncryptionKey>().is_ok());
                        assert!(key_string.parse::<ServicePublicEncryptionKey>().is_ok());
                        assert!(key_string.parse::<OriginPublicEncryptionKey>().is_ok());
                    };
                }

                /// Pass a secret encryption key instance and assert
                /// the string it produces can be parsed as any other
                /// secret encryption key type.
                macro_rules! assert_secret_key_equivalence {
                    ($key:expr) => {
                        let key_string = $key.to_key_string();
                        assert!(key_string.parse::<UserSecretEncryptionKey>().is_ok());
                        assert!(key_string.parse::<ServiceSecretEncryptionKey>().is_ok());
                        assert!(key_string.parse::<OriginSecretEncryptionKey>().is_ok());
                    };
                }

                #[test]
                fn user_encryption_keys_can_parse_as_all_other_encryption_keys() {
                    let (user_public, user_secret) = generate_user_encryption_key_pair("test-user");
                    assert_public_key_equivalence!(user_public);
                    assert_secret_key_equivalence!(user_secret);
                }

                #[test]
                fn service_encryption_keys_can_parse_as_all_other_encryption_keys() {
                    let (service_public, service_secret) =
                        generate_service_encryption_key_pair("org", "testing.default");
                    assert_public_key_equivalence!(service_public);
                    assert_secret_key_equivalence!(service_secret);
                }

                #[test]
                fn origin_encryption_keys_can_parse_as_all_other_encryption_keys() {
                    let origin = "test-origin".parse().unwrap();
                    let (origin_public, origin_secret) =
                        generate_origin_encryption_key_pair(&origin);
                    assert_public_key_equivalence!(origin_public);
                    assert_secret_key_equivalence!(origin_secret);
                }
            }
        }
    }

    /// Validate the Debug implementations
    mod debug {
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
            let origin = "my-origin".parse().unwrap();
            let (public, secret) = generate_origin_encryption_key_pair(&origin);

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
            let origin = "my-origin".parse().unwrap();
            let (public, secret) = generate_signing_key_pair(&origin);
            assert_eq!(format!("PublicOriginSigningKey my-origin-{}",
                               public.named_revision().revision()),
                       format!("{:?}", public));
            assert_eq!(format!("SecretOriginSigningKey my-origin-{}",
                               secret.named_revision().revision()),
                       format!("{:?}", secret));
        }
    }

    /// Validate implementations of the KeyFile trait
    mod key_file {
        use super::*;
        use crate::crypto::keys::KeyFile;
        use std::path::PathBuf;

        #[test]
        fn filename() {
            let source = "foo-20160504220722".parse::<NamedRevision>().unwrap();
            let service_source = "redis.default@chef-20160504220722".parse::<NamedRevision>()
                                                                    .unwrap();

            assert_eq!(RingKey::filename(&source),
                       PathBuf::from("foo-20160504220722.sym.key"));

            assert_eq!(PublicOriginSigningKey::filename(&source),
                       PathBuf::from("foo-20160504220722.pub"));
            assert_eq!(SecretOriginSigningKey::filename(&source),
                       PathBuf::from("foo-20160504220722.sig.key"));

            assert_eq!(UserPublicEncryptionKey::filename(&source),
                       PathBuf::from("foo-20160504220722.pub"));
            assert_eq!(UserSecretEncryptionKey::filename(&source),
                       PathBuf::from("foo-20160504220722.box.key"));

            assert_eq!(OriginPublicEncryptionKey::filename(&source),
                       PathBuf::from("foo-20160504220722.pub"));
            assert_eq!(OriginSecretEncryptionKey::filename(&source),
                       PathBuf::from("foo-20160504220722.box.key"));

            assert_eq!(ServicePublicEncryptionKey::filename(&service_source),
                       PathBuf::from("redis.default@chef-20160504220722.pub"));
            assert_eq!(ServiceSecretEncryptionKey::filename(&service_source),
                       PathBuf::from("redis.default@chef-20160504220722.box.key"));

            // NOTE: Nothing yet explicitly prevents a named revision
            // that does not really belong to a service key from being
            // pathed as though it were.
            assert_eq!(ServicePublicEncryptionKey::filename(&source),
                       PathBuf::from("foo-20160504220722.pub"));
            assert_eq!(ServiceSecretEncryptionKey::filename(&source),
                       PathBuf::from("foo-20160504220722.box.key"));
        }

        mod own_filename {
            use super::*;

            #[test]
            fn ring_key() {
                let key = RingKey::new("beyonce");
                assert_eq!(PathBuf::from(&format!("{}.sym.key", key.named_revision())),
                           key.own_filename());
            }

            #[test]
            fn user_keys() {
                let (public, secret) = generate_user_encryption_key_pair("my-user");
                assert_eq!(PathBuf::from(&format!("{}.pub", public.named_revision())),
                           public.own_filename());
                assert_eq!(PathBuf::from(&format!("{}.box.key", secret.named_revision())),
                           secret.own_filename());
            }

            #[test]
            fn origin_keys() {
                let origin = "my-origin".parse().unwrap();
                let (public, secret) = generate_origin_encryption_key_pair(&origin);
                assert_eq!(PathBuf::from(&format!("{}.pub", public.named_revision())),
                           public.own_filename());
                assert_eq!(PathBuf::from(&format!("{}.box.key", secret.named_revision())),
                           secret.own_filename());
            }

            #[test]
            fn service_keys() {
                let (public, secret) =
                    generate_service_encryption_key_pair("my-org", "foo.default");
                assert_eq!(PathBuf::from(&format!("{}.pub", public.named_revision())),
                           public.own_filename());
                assert_eq!(PathBuf::from(&format!("{}.box.key", secret.named_revision())),
                           secret.own_filename());
            }

            #[test]
            fn signing_keys() {
                let origin = "my-origin".parse().unwrap();
                let (public, secret) = generate_signing_key_pair(&origin);
                assert_eq!(PathBuf::from(&format!("{}.pub", public.named_revision())),
                           public.own_filename());
                assert_eq!(PathBuf::from(&format!("{}.sig.key", secret.named_revision())),
                           secret.own_filename());
            }
        }

        mod extension {
            use super::*;

            macro_rules! extension {
                ($name:ident, $key:ty, $extension:expr) => {
                    #[test]
                    fn $name() {
                        let actual = <$key>::extension();
                        assert_eq!(
                            actual,
                            $extension,
                            "Expected {} to have extension '{}', but it was '{}'",
                            stringify!($key),
                            $extension,
                            actual
                        );
                    }
                };
            }

            extension!(ring_key, RingKey, "sym.key");

            extension!(public_origin_signing_key, PublicOriginSigningKey, "pub");
            extension!(secret_origin_signing_key, SecretOriginSigningKey, "sig.key");

            extension!(origin_public_encryption_key, OriginPublicEncryptionKey, "pub");
            extension!(origin_secret_encryption_key, OriginSecretEncryptionKey, "box.key");

            extension!(service_public_encryption_key, ServicePublicEncryptionKey, "pub");
            extension!(service_secret_encryption_key, ServiceSecretEncryptionKey, "box.key");

            extension!(user_public_encryption_key, UserPublicEncryptionKey, "pub");
            extension!(user_secret_encryption_key, UserSecretEncryptionKey, "box.key");
        }

        mod permissions {
            use super::*;

            macro_rules! permissions {
                ($name:ident, $key:ty, $permission:expr) => {
                    #[test]
                    fn $name() {
                        let actual = <$key>::permissions();
                        assert_eq!(
                            actual,
                            $permission,
                            "Expected {} to have permission '{:?}', but it was '{:?}'",
                            stringify!($key),
                            $permission,
                            actual
                        );
                    }
                };
            }

            permissions!(ring_key, RingKey, crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

            permissions!(user_public_encryption_key, UserPublicEncryptionKey,
                         crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);
            permissions!(user_secret_encryption_key, UserSecretEncryptionKey,
                         crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

            permissions!(origin_public_encryption_key, OriginPublicEncryptionKey,
                         crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);
            permissions!(origin_secret_encryption_key, OriginSecretEncryptionKey,
                         crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

            permissions!(service_public_encryption_key, ServicePublicEncryptionKey,
                         crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);
            permissions!(service_secret_encryption_key, ServiceSecretEncryptionKey,
                         crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);

            permissions!(public_origin_signing_key, PublicOriginSigningKey,
                         crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS);
            permissions!(secret_origin_signing_key, SecretOriginSigningKey,
                         crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS);
        }

        mod version {
            use super::*;

            macro_rules! version {
                ($name:ident, $key:ty, $version:expr) => {
                    #[test]
                    fn $name() {
                        let actual = <$key>::version();
                        assert_eq!(
                            actual,
                            $version,
                            "Expected {} to have version '{:?}', but it was '{:?}'",
                            stringify!($key),
                            $version,
                            actual
                        );
                    }
                };
            }

            version!(ring_key, RingKey, "SYM-SEC-1");

            version!(public_origin_signing_key, PublicOriginSigningKey, "SIG-PUB-1");
            version!(secret_origin_signing_key, SecretOriginSigningKey, "SIG-SEC-1");

            version!(origin_public_encryption_key, OriginPublicEncryptionKey, "BOX-PUB-1");
            version!(origin_secret_encryption_key, OriginSecretEncryptionKey, "BOX-SEC-1");

            version!(service_public_encryption_key, ServicePublicEncryptionKey, "BOX-PUB-1");
            version!(service_secret_encryption_key, ServiceSecretEncryptionKey, "BOX-SEC-1");

            version!(user_public_encryption_key, UserPublicEncryptionKey, "BOX-PUB-1");
            version!(user_secret_encryption_key, UserSecretEncryptionKey, "BOX-SEC-1");
        }
    }
}
