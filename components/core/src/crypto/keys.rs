use crate::{error::Error,
            fs::Permissions};
use chrono::Utc;
use regex::Regex;
use std::{self,
          fmt,
          ops::Deref,
          path::PathBuf,
          result,
          str::FromStr};

lazy_static::lazy_static! {
    static ref NAME_WITH_REV_RE: Regex = Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\z").unwrap();
    static ref KEYFILE_RE: Regex =
        Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\.(?P<suffix>[a-z]+(\.[a-z]+)?)\z").unwrap();
}

#[macro_use]
mod util;
mod cache;
mod encryption;
mod ring_key;
mod signing;

pub use cache::KeyCache;
pub use encryption::*;
pub use ring_key::RingKey;
pub use signing::{PublicOriginSigningKey,
                  SecretOriginSigningKey,
                  generate_signing_key_pair};

////////////////////////////////////////////////////////////////////////

/// Defines the basic interface that all Habitat keys use.
///
/// All Habitat keys will have a `NamedRevision`, which identifies a
/// key of this type uniquely. Additionally, each Habitat key wraps a
/// type that implements the actual cryptographic primitives need to
/// fulfill the responsibilities of the key.
pub trait Key {
    /// The actual cryptographic material used by this kind of Habitat
    /// key. Different Habitat keys will use different kinds of underlying
    /// cryptographic methods, depending on what what their purpose is.
    type Crypto: AsRef<[u8]>;

    /// Reference to the underlying bytes of actual cryptographic
    /// material.
    fn key(&self) -> &Self::Crypto;

    /// Reference to the identifier of this particular key.
    fn named_revision(&self) -> &NamedRevision;
}

/// Encapsulates properties and logic for writing Habitat keys out to
/// files on disk.
pub trait KeyFile: Key {
    /// Returns the permissions with which an item should be written
    /// to the filesystem.
    fn permissions() -> Permissions;

    /// The file format version string. This will be incorporated into
    /// files that have been exported to disk to indicate what format
    /// they are saved in.
    fn version() -> &'static str;

    /// The file extension to use when exporting this key to disk.
    fn extension() -> &'static str;

    /// Given a `NamedRevision`, return the name of the file a key of
    /// this type with that identifier would be saved as in the key
    /// cache.
    ///
    /// Only returns the name of the file itself, not its path within
    /// a particular cache directory.
    fn filename(named_revision: &NamedRevision) -> PathBuf {
        // **DO NOT** use PathBuf::with_extension here, because it fails
        // with service keys (whose name is like "core.redis@chef");
        // `with_extension` will chop off the <group>@<org> portion of
        // that string!
        PathBuf::from(format!("{}.{}", named_revision, Self::extension()))
    }

    /// Same as `filename`, but for a specific key.
    fn own_filename(&self) -> PathBuf { Self::filename(self.named_revision()) }

    /// Returns what should be the contents of a file when rendering a
    /// key out to a file on disk.
    ///
    /// While one *could* just use `ToString` / `Display`, that choice
    /// could be a little dangerous, considering the fact that the
    /// content of private keys should never potentially make it into
    /// logging or other output.
    fn to_key_string(&self) -> String {
        let k = self.key();
        format!("{}\n{}\n\n{}",
                Self::version(),
                self.named_revision(),
                &crate::base64::encode(k))
    }
}

////////////////////////////////////////////////////////////////////////

/// The combination of a key name and a revision timestamp. For any
/// given type of Habitat key, this will uniquely identify that key,
/// allowing it to be retrieved from a local key cache or from the
/// Builder API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedRevision {
    name:     String,
    revision: KeyRevision,
}

impl NamedRevision {
    /// Create a new `NamedRevision` for a given name, generating a
    /// new revision based on the current time.
    ///
    /// Only crate-public because nothing outside this crate should be
    /// creating these.
    pub(crate) fn new(name: String) -> Self { Self::from_parts(name, KeyRevision::new()) }

    pub(crate) fn from_parts(name: String, revision: KeyRevision) -> Self {
        NamedRevision { name, revision }
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn revision(&self) -> &KeyRevision { &self.revision }
}

impl FromStr for NamedRevision {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let caps = match NAME_WITH_REV_RE.captures(value) {
            Some(c) => c,
            None => {
                let msg = format!("Cannot parse named revision '{}'", value);
                return Err(Error::CryptoError(msg));
            }
        };
        let name = match caps.name("name") {
            Some(r) => r.as_str().to_string(),
            None => {
                let msg = format!("Cannot parse name from '{}'", value);
                return Err(Error::CryptoError(msg));
            }
        };
        let revision = match caps.name("rev") {
            // TODO (CM): This is a bit of an awkward constructor at the
            // moment, but we'll allow it as we've already validated this
            // with the larger regex. It would be nice to harmonize
            // this a bit more, but it's not the worst situation.
            Some(r) => KeyRevision(r.as_str().to_string()),
            None => {
                let msg = format!("Cannot parse revision from '{}'", value);
                return Err(Error::CryptoError(msg));
            }
        };

        Ok(NamedRevision { name, revision })
    }
}

impl fmt::Display for NamedRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.revision)
    }
}

////////////////////////////////////////////////////////////////////////

/// A timestamp string used to identify Habitat keys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyRevision(String);

impl KeyRevision {
    /// Generates a revision string in the form:
    /// `{year}{month}{day}{hour24}{minute}{second}`
    /// Timestamps are in UTC time.
    ///
    /// Only visible in this crate because nothing outside should be
    /// directly generating these.
    pub(crate) fn new() -> KeyRevision {
        KeyRevision(Utc::now().format("%Y%m%d%H%M%S").to_string())
    }
}

// TODO (CM): Need some tests that assert that this format string and
// the regex for parsing revisions from filenames are mutually
// consistent.

impl fmt::Display for KeyRevision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) }
}

// As a "newtype", KeyRevision can be thought of as a kind of "smart
// container", and thus we can implement Deref for it with our heads
// held high.
impl Deref for KeyRevision {
    type Target = str;

    fn deref(&self) -> &str { self.0.as_str() }
}

#[cfg(test)]
impl KeyRevision {
    /// Unchecked constructor for testing purposes only; assumes the
    /// string being passed in is valid.
    pub(crate) fn unchecked<R>(rev: R) -> KeyRevision
        where R: AsRef<str>
    {
        KeyRevision(rev.as_ref().to_string())
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    mod named_revision {
        use super::*;

        #[test]
        fn parse_valid_named_revisions() {
            let result: NamedRevision = "foo-20160504220722".parse().unwrap();
            assert_eq!("foo", result.name);
            assert_eq!(KeyRevision::unchecked("20160504220722"), result.revision);

            let result: NamedRevision = "foo-stuff-20160504220722".parse().unwrap();
            assert_eq!("foo-stuff", result.name);
            assert_eq!(KeyRevision::unchecked("20160504220722"), result.revision);
        }

        #[test]
        fn parse_invalid_named_revisions() {
            let result = "barf".parse::<NamedRevision>();
            assert!(result.is_err());

            let result = "barf-20160504220722-wheeeeeeeee".parse::<NamedRevision>();
            assert!(result.is_err());

            let result = "barf-123".parse::<NamedRevision>();
            assert!(result.is_err());
        }

        #[test]
        fn to_string() {
            let nr = NamedRevision { name:     "foo".to_string(),
                                     revision: KeyRevision::unchecked("20160504220722"), };
            assert_eq!(nr.to_string(), "foo-20160504220722");
        }

        /// These key names have a different structure!
        #[test]
        fn to_string_for_service_key_names() {
            let nr = NamedRevision { name:     "core.redis@chef".to_string(),
                                     revision: KeyRevision::unchecked("20160504220722"), };
            assert_eq!(nr.to_string(), "core.redis@chef-20160504220722");
        }

        #[test]
        fn string_roundtrip() {
            let input = "foo-20160504220722";
            assert_eq!(input.parse::<NamedRevision>().unwrap().to_string(), input);
        }
    }
}
