use super::{KeyRevision,
            NamedRevision};
use crate::error::{Error,
                   Result};

/// Helper trait for use in FromStr implementations because the
/// sodiumoxide types we're parsing don't implement their `from_slice`
/// methods as traits.
pub(crate) trait FromSlice<T> {
    fn from_slice(bytes: &[u8]) -> Option<T>;
}

/// Common logic for parsing the content of a key file as a string. To
/// be used for `FromStr` implementations for various key types
pub(crate) fn key_bits<T>(content: &str, version_string: &str) -> Result<(String, KeyRevision, T)>
    where T: FromSlice<T>
{
    let mut lines = content.lines();

    lines.next()
         .ok_or_else(|| Error::CryptoError("Missing key version".to_string()))
         .map(|line| {
             if line == version_string {
                 Ok(())
             } else {
                 Err(Error::CryptoError(format!("Unsupported key version: {}", line)))
             }
         })??;

    let named_revision: NamedRevision =
        lines.next()
             .ok_or_else(|| Error::CryptoError("Missing name+revision".to_string()))
             .map(str::parse)??;

    let key: T = lines.nth(1) // skip a blank line!
                      .ok_or_else(|| Error::CryptoError("Missing key material".to_string()))
                      .map(str::trim)
                      .map(base64::decode)?
                      .map_err(|_| Error::CryptoError("Invalid base64 key material".to_string()))
                      .map(|b| T::from_slice(&b))?
                      .ok_or_else(|| {
                          Error::CryptoError(format!("Could not parse bytes as key for {}",
                                                     named_revision))
                      })?;

    let (name, revision) = named_revision.into();
    Ok((name, revision, key))
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
    ($t:ty, $sodiumoxide_key:ty, $version_string:expr) => {
        impl crate::crypto::keys::util::FromSlice<$sodiumoxide_key> for $sodiumoxide_key {
            fn from_slice(bytes: &[u8]) -> Option<$sodiumoxide_key> {
                <$sodiumoxide_key>::from_slice(bytes)
            }
        }

        impl FromStr for $t {
            type Err = Error;

            fn from_str(content: &str) -> std::result::Result<Self, Self::Err> {
                let (name, revision, key) =
                    crate::crypto::keys::util::key_bits(content, $version_string)?;
                Ok(<$t>::from_raw(name, revision, Some(key)))
            }
        }
    };
}

macro_rules! secret_permissions {
    ($t:ty) => {
        impl crate::crypto::keys::Permissioned for $t {
            const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_SECRET_KEY_PERMISSIONS;
        }
    };
}

macro_rules! public_permissions {
    ($t:ty) => {
        impl crate::crypto::keys::Permissioned for $t {
            const PERMISSIONS: crate::fs::Permissions = crate::fs::DEFAULT_PUBLIC_KEY_PERMISSIONS;
        }
    };
}
