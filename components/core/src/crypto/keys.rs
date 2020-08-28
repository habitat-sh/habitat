use super::{PUBLIC_BOX_KEY_VERSION,
            PUBLIC_KEY_SUFFIX,
            PUBLIC_SIG_KEY_VERSION,
            SECRET_BOX_KEY_SUFFIX,
            SECRET_BOX_KEY_VERSION,
            SECRET_SIG_KEY_SUFFIX,
            SECRET_SIG_KEY_VERSION,
            SECRET_SYM_KEY_SUFFIX,
            SECRET_SYM_KEY_VERSION};
use crate::{error::{Error,
                    Result},
            fs::{Permissions,
                 DEFAULT_PUBLIC_KEY_PERMISSIONS,
                 DEFAULT_SECRET_KEY_PERMISSIONS}};
use chrono::Utc;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashSet,
          convert::TryFrom,
          fmt,
          fs::{self,
               File},
          io::{prelude::*,
               BufReader,
               BufWriter},
          ops::Deref,
          path::{Path,
                 PathBuf},
          result,
          str::FromStr};

lazy_static::lazy_static! {
    static ref NAME_WITH_REV_RE: Regex = Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\z").unwrap();
    static ref KEYFILE_RE: Regex =
        Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\.(?P<suffix>[a-z]+(\.[a-z]+)?)\z").unwrap();
}

#[macro_use]
mod util;
pub mod box_key_pair;
mod cache;
mod encryption;
mod ring_key;
mod signing;

pub use cache::KeyCache;
pub use encryption::*;
pub use ring_key::RingKey;
pub use signing::{generate_signing_key_pair,
                  PublicOriginSigningKey,
                  SecretOriginSigningKey};

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
                &base64::encode(k))
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

// Only used for the polymorphic `KeyPair::rev` implementation so Builder can
// use things before adopting the KeyRevison type itself.
impl From<KeyRevision> for String {
    fn from(rev: KeyRevision) -> String { rev.to_string() }
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
// OLD SUPPORTING CODE BELOW
// All this can be removed once we've migrated Builder away from it.
////////////////////////////////////////////////////////////////////////

#[deprecated]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub enum PairType {
    Public,
    Secret,
}

impl fmt::Display for PairType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PairType::Public => write!(f, "public"),
            PairType::Secret => write!(f, "secret"),
        }
    }
}

impl FromStr for PairType {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value {
            "public" => Ok(PairType::Public),
            "secret" => Ok(PairType::Secret),
            _ => {
                Err(Error::CryptoError(format!("Invalid PairType conversion \
                                                from {}",
                                               value)))
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////

/// If a key "belongs" to a filename revision, then add the full stem of the
/// file (without path, without .suffix) to the set. This function doesn't
/// return an error on a "bad" file, the bad file key name just doesn't get
/// added to the set.
fn check_filename(keyname: &str,
                  filename: &str,
                  candidates: &mut HashSet<String>,
                  pair_type: Option<PairType>) {
    let caps = match KEYFILE_RE.captures(&filename) {
        Some(c) => c,
        None => {
            debug!("check_filename: Cannot parse {}", &filename);
            return;
        }
    };
    let name = match caps.name("name") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse name from {}", &filename);
            return;
        }
    };

    let rev = match caps.name("rev") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse rev from {}", &filename);
            return;
        }
    };

    let suffix = match caps.name("suffix") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse suffix from {}", &filename);
            return;
        }
    };

    if suffix == PUBLIC_KEY_SUFFIX
       || suffix == SECRET_SIG_KEY_SUFFIX
       || suffix == SECRET_BOX_KEY_SUFFIX
       || suffix == SECRET_SYM_KEY_SUFFIX
    {
        debug!("valid key suffix");
    } else {
        debug!("check_filename: Invalid key suffix from {}", &filename);
        return;
    };

    if name == keyname {
        let thiskey = format!("{}-{}", name, rev);

        let do_insert = match pair_type {
            Some(PairType::Secret) => {
                suffix == SECRET_SIG_KEY_SUFFIX
                || suffix == SECRET_BOX_KEY_SUFFIX
                || suffix == SECRET_SYM_KEY_SUFFIX
            }
            Some(PairType::Public) => suffix == PUBLIC_KEY_SUFFIX,
            None => true,
        };

        if do_insert {
            candidates.insert(thiskey);
        }
    }
}

fn mk_key_filename<P, S1, S2>(path: P, keyname: S1, suffix: S2) -> PathBuf
    where P: AsRef<Path>,
          S1: AsRef<str>,
          S2: AsRef<str>
{
    path.as_ref()
        .join(format!("{}.{}", keyname.as_ref(), suffix.as_ref()))
}

#[deprecated(note = "Please use new key types")]
pub fn parse_name_with_rev<T>(name_with_rev: T) -> Result<(String, KeyRevision)>
    where T: AsRef<str>
{
    let named_revision = name_with_rev.as_ref().parse::<NamedRevision>()?;
    Ok((named_revision.name, named_revision.revision))
}

fn write_keypair_files<P>(public: Option<(P, String)>, secret: Option<(P, String)>) -> Result<()>
    where P: AsRef<Path>
{
    if let Some((public_keyfile, public_content)) = public {
        let public_keyfile = public_keyfile.as_ref();

        if let Some(pk_dir) = public_keyfile.parent() {
            fs::create_dir_all(pk_dir)?;
        } else {
            return Err(Error::BadKeyPath(public_keyfile.to_string_lossy().into_owned()));
        }
        if public_keyfile.exists() {
            return Err(Error::CryptoError(format!("Public keyfile or a \
                                                   directory already exists {}",
                                                  public_keyfile.display())));
        }
        let public_file = File::create(public_keyfile)?;
        let mut public_writer = BufWriter::new(&public_file);
        public_writer.write_all(public_content.as_bytes())?;
        set_permissions(public_keyfile, &DEFAULT_PUBLIC_KEY_PERMISSIONS)?;
    }

    if let Some((secret_keyfile, secret_content)) = secret {
        let secret_keyfile = secret_keyfile.as_ref();

        if let Some(sk_dir) = secret_keyfile.parent() {
            fs::create_dir_all(sk_dir)?;
        } else {
            return Err(Error::BadKeyPath(secret_keyfile.to_string_lossy().into_owned()));
        }
        if secret_keyfile.exists() {
            return Err(Error::CryptoError(format!("Secret keyfile or a \
                                                   directory already exists {}",
                                                  secret_keyfile.display())));
        }
        let secret_file = File::create(secret_keyfile)?;
        let mut secret_writer = BufWriter::new(&secret_file);
        secret_writer.write_all(secret_content.as_bytes())?;
        set_permissions(secret_keyfile, &DEFAULT_SECRET_KEY_PERMISSIONS)?;
    }
    Ok(())
}

#[cfg(not(windows))]
fn set_permissions<T: AsRef<Path>>(path: T, perms: &Permissions) -> Result<()> {
    use crate::util::posix_perm;

    if let Permissions::Explicit(permissions) = perms {
        posix_perm::set_permissions(path.as_ref(), *permissions)?;
    }
    Ok(())
}

#[cfg(windows)]
fn set_permissions<T: AsRef<Path>>(path: T, _perms: &Permissions) -> Result<()> {
    use crate::util::win_perm;

    win_perm::harden_path(path.as_ref())
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{keys::RingKey,
                        test_support::*};
    use tempfile::Builder;

    static VALID_KEY: &str = "ring-key-valid-20160504220722.sym.key";
    static VALID_KEY_AS_HEX: &str = "\
         44215a3bce23e351a6af359d77131db17a46767de2b88cbb330df162b8cf2ec1";

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

    #[test]
    fn parse_name_with_rev() {
        let (name, rev) = super::parse_name_with_rev("an-origin-19690114010203").unwrap();
        assert_eq!(name, "an-origin");
        assert_eq!(rev, KeyRevision::unchecked("19690114010203"));

        let (name, rev) = super::parse_name_with_rev("user-19480531051223").unwrap();
        assert_eq!(name, "user");
        assert_eq!(rev, KeyRevision::unchecked("19480531051223"));

        let (name, rev) = super::parse_name_with_rev("tnt.default@acme-19480531051223").unwrap();
        assert_eq!(name, "tnt.default@acme");
        assert_eq!(rev, KeyRevision::unchecked("19480531051223"));

        let (name, rev) = super::parse_name_with_rev("--20160420042001").unwrap();
        assert_eq!(name, "-");
        assert_eq!(rev, KeyRevision::unchecked("20160420042001"));
    }

    #[test]
    fn check_filename_for_secret_keys() {
        // only look for secret keys
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.pub",
                              &mut candidates,
                              Some(PairType::Secret));
        super::check_filename("wecoyote",
                              "wecoyote-foo-20160519203610.pub",
                              &mut candidates,
                              Some(PairType::Secret));
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.sig.key",
                              &mut candidates,
                              Some(PairType::Secret));
        assert_eq!(1, candidates.len());
    }

    #[test]
    fn check_filename_for_public_keys() {
        // only look for public keys
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.pub",
                              &mut candidates,
                              Some(PairType::Public));
        super::check_filename("wecoyote",
                              "wecoyote-20160519203611.pub",
                              &mut candidates,
                              Some(PairType::Public));
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.sig.key",
                              &mut candidates,
                              Some(PairType::Public));
        assert_eq!(2, candidates.len());
    }

    #[test]
    fn check_filename_key_without_dash() {
        // look for a keyname that doesn't include a dash
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.pub",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote",
                              "wecoyote-foo-20160519203610.pub",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.box.key",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote",
                              "wecoyote-foo-20160519203610.box.key",
                              &mut candidates,
                              None);
        assert_eq!(1, candidates.len());
    }

    #[test]
    fn check_filename_key_with_dash() {
        // look for a keyname that includes a dash
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote-foo",
                              "wecoyote-20160519203610.pub",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote-foo",
                              "wecoyote-foo-20160519203610.pub",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote-foo",
                              "wecoyote-20160519203610.box.key",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote-foo",
                              "wecoyote-foo-20160519203610.box.key",
                              &mut candidates,
                              None);
        assert_eq!(1, candidates.len());
    }
}
