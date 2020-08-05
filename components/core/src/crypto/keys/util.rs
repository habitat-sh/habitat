/// Helper trait for use in FromStr implementations because the
/// sodiumoxide types we're parsing don't implement their `from_slice`
/// methods as traits.

// TODO (CM): This unfortunately has to be public at the moment. I
// *might* be able to get rid of it if I pull the entire parsing logic
// into the macro, though.
pub trait FromSlice<T> {
    fn from_slice(bytes: &[u8]) -> Option<T>;
}

/// Helper macro to generate FromStr implementations for our key
/// types.
///
/// This also generates `FromSlice` implementations as well, since
/// that can be thought of as a `FromStr` implementation detail for
/// us.
///
/// `t` is our Habitat key type.
///
/// `sodiumoxide_key` is the key type from the sodiumoxide library
/// that `t` wraps.
///
/// `version_string` is the content of the first line of a valid file
/// for `t`.
macro_rules! from_str_impl_for_key {
    ($t:ty) => {
        impl crate::crypto::keys::util::FromSlice<<$t as Key>::Crypto> for <$t as Key>::Crypto {
            fn from_slice(bytes: &[u8]) -> Option<<$t as Key>::Crypto> {
                <$t as Key>::Crypto::from_slice(bytes)
            }
        }

        impl FromStr for $t {
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
