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
